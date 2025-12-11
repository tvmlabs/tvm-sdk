// Copyright (C) 2019-2023 EverX. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use std::cmp::min;
#[cfg(feature = "timings")]
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
#[cfg(feature = "timings")]
use std::time::Instant;

use tvm_block::AccStatusChange;
use tvm_block::Account;
use tvm_block::AccountState;
use tvm_block::AccountStatus;
use tvm_block::AddSub;
use tvm_block::CommonMsgInfo;
use tvm_block::CurrencyCollection;
use tvm_block::GlobalCapabilities;
use tvm_block::Grams;
use tvm_block::MASTERCHAIN_ID;
use tvm_block::Message;
use tvm_block::Serializable;
use tvm_block::TrBouncePhase;
use tvm_block::TrComputePhase;
use tvm_block::Transaction;
use tvm_block::TransactionDescr;
use tvm_block::TransactionDescrOrdinary;
use tvm_block::VarUInteger32;
use tvm_types::HashmapType;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::error;
use tvm_types::fail;
use tvm_vm::SmartContractInfo;
use tvm_vm::boolean;
use tvm_vm::int;
use tvm_vm::stack::Stack;
use tvm_vm::stack::StackItem;
use tvm_vm::stack::integer::IntegerData;

use crate::ActionPhaseResult;
use crate::ExecuteParams;
use crate::TransactionExecutor;
use crate::VERSION_BLOCK_REVERT_MESSAGES_WITH_ANYCAST_ADDRESSES;
use crate::blockchain_config::BlockchainConfig;
use crate::error::ExecutorError;

pub struct OrdinaryTransactionExecutor {
    config: BlockchainConfig,

    #[cfg(feature = "timings")]
    timings: [AtomicU64; 3], // 0 - preparation, 1 - compute, 2 - after compute
}

impl OrdinaryTransactionExecutor {
    pub fn new(config: BlockchainConfig) -> Self {
        Self {
            config,

            #[cfg(feature = "timings")]
            timings: [AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0)],
        }
    }

    #[cfg(feature = "timings")]
    pub fn timing(&self, kind: usize) -> u64 {
        self.timings[kind].load(Ordering::Relaxed)
    }
}

impl TransactionExecutor for OrdinaryTransactionExecutor {
    /// Create and execute transaction from message for account
    fn execute_with_params(
        &self,
        in_msg: Option<&Message>,
        account: &mut Account,
        params: ExecuteParams,
        minted_shell: &mut i128,
    ) -> Result<Transaction> {
        #[cfg(feature = "timings")]
        let mut now = Instant::now();

        let is_previous_state_active = match account.state() {
            Some(AccountState::AccountUninit {}) => false,
            None => false,
            _ => true,
        };

        let revert_anycast =
            self.config.global_version() >= VERSION_BLOCK_REVERT_MESSAGES_WITH_ANYCAST_ADDRESSES;

        let in_msg =
            in_msg.ok_or_else(|| error!("Ordinary transaction must have input message"))?;
        let in_msg_cell = in_msg.serialize()?; // TODO: get from outside
        let is_masterchain = in_msg.dst_workchain_id() == Some(MASTERCHAIN_ID);
        log::debug!(
            target: "executor",
            "Ordinary transaction executing, in message id: {:x}",
            in_msg_cell.repr_hash()
        );
        let (bounce, is_ext_msg) = match in_msg.header() {
            CommonMsgInfo::ExtOutMsgInfo(_) => fail!(ExecutorError::InvalidExtMessage),
            CommonMsgInfo::IntMsgInfo(ref hdr) => (hdr.bounce, false),
            CommonMsgInfo::CrossDappMessageInfo(ref hdr) => (hdr.bounce, false),
            CommonMsgInfo::ExtInMsgInfo(_) => (false, true),
        };

        let account_address = in_msg.dst_ref().ok_or_else(|| {
            ExecutorError::TrExecutorError(format!(
                "Input message {:x} has no dst address",
                in_msg_cell.repr_hash()
            ))
        })?;
        let account_id = match account.get_id() {
            Some(account_id) => {
                log::debug!(target: "executor", "Account = {:x}", account_id);
                account_id
            }
            None => {
                log::debug!(target: "executor", "Account = None, address = {:x}", account_address.address());
                account_address.address()
            }
        };
        let mut need_to_burn = Grams::zero();
        let mut acc_balance = account.balance().cloned().unwrap_or_default();
        let mut msg_balance = in_msg.get_value().cloned().unwrap_or_default();
        let original_msg_balance = msg_balance.clone();
        let gas_config = self.config().get_gas_config(false);
        log::debug!(target: "executor", "address = {:?}, available_credit {:?}", in_msg.int_header(), params.available_credit);
        let mut msg_balance_convert = 0;
        let mut exchanged = false;
        if let Some(h) = in_msg.int_header() {
            if Some(h.src_dapp_id()) != account.stuff().is_some().then_some(&params.dapp_id)
                && !(in_msg.have_state_init()
                    && account
                        .state()
                        .map(|s| *s == AccountState::AccountUninit {})
                        .unwrap_or(true))
                && !h.bounced
            {
                log::debug!(target: "executor", "account dapp_id {:?}", params.dapp_id);
                log::debug!(target: "executor", "msg balance {:?}, config balance {}", msg_balance.grams, (gas_config.gas_limit * gas_config.gas_price / 65536));
                msg_balance.grams = min(
                    (gas_config.gas_limit * gas_config.gas_price / 65536).into(),
                    msg_balance.grams,
                );
                need_to_burn = msg_balance.grams;
                log::debug!(target: "executor", "final msg balance {}", msg_balance.grams);
            }
            if h.is_exchange {
                if let Ok(Some(mut value)) = msg_balance.get_other(2) {
                    let mut echng = value.clone();
                    if echng > VarUInteger32::from(u64::MAX) {
                        echng = VarUInteger32::from(u64::MAX);
                    }
                    if echng != VarUInteger32::from(0) {
                        msg_balance_convert =
                            echng.value().iter_u64_digits().collect::<Vec<u64>>()[0];
                        msg_balance.grams += Grams::from(msg_balance_convert);
                        value.sub(&echng)?;
                        let digits = value.value().iter_u64_digits().collect::<Vec<u64>>();
                        let base = u64::MAX as u128 + 1;
                        let new_balance =
                            if digits.len() > 2 && digits.iter().skip(2).any(|&d| d != 0) {
                                u128::MAX
                            } else {
                                let d0 = digits.first().copied().unwrap_or(0) as u128;
                                let d1 = digits.get(1).copied().unwrap_or(0) as u128;
                                d0 + d1 * base
                            };
                        msg_balance.set_other(2, new_balance)?;
                    }
                    exchanged = true;
                }
            }
        }
        let ihr_delivered = false; // ihr is disabled because it does not work
        if !ihr_delivered {
            if let Some(h) = in_msg.int_header() {
                msg_balance.grams += h.ihr_fee;
            }
        }

        let is_special = self.config.is_special_account(account_address)?;
        log::debug!(target: "executor", "acc_balance: {}, msg_balance: {}, credit_first: {}, is_special: {}",
            acc_balance.grams, msg_balance.grams, !bounce, is_special);
        let lt = std::cmp::max(
            account.last_tr_time().unwrap_or(0),
            std::cmp::max(params.last_tr_lt.load(Ordering::Relaxed), in_msg.lt().unwrap_or(0) + 1),
        );
        let mut tr = Transaction::with_address_and_status(account_id, account.status());
        tr.set_logical_time(lt);
        tr.set_now(params.block_unixtime);
        tr.set_in_msg_cell(in_msg_cell.clone());

        let mut description = TransactionDescrOrdinary {
            credit_first: !bounce,
            ..TransactionDescrOrdinary::default()
        };

        if revert_anycast && account_address.rewrite_pfx().is_some() {
            description.aborted = true;
            tr.set_end_status(account.status());
            params.last_tr_lt.store(lt, Ordering::Relaxed);
            account.set_last_tr_time(lt);
            tr.write_description(&TransactionDescr::Ordinary(description))?;
            return Ok(tr);
        }

        // first check if contract can pay for importing external message
        if is_ext_msg && !is_special {
            let in_fwd_fee = self.config.calc_fwd_fee(is_masterchain, &in_msg_cell)?;
            log::debug!(target: "executor", "import message fee: {}, acc_balance: {}", in_fwd_fee, acc_balance.grams);
            if !acc_balance.grams.sub(&in_fwd_fee)? {
                fail!(ExecutorError::NoFundsToImportMsg)
            }
            tr.add_fee_grams(&in_fwd_fee)?;
        }

        if description.credit_first && !is_ext_msg {
            description.credit_ph = match self.credit_phase(
                account,
                &mut tr,
                &mut msg_balance,
                &mut acc_balance,
            ) {
                Ok(credit_ph) => Some(credit_ph),
                Err(e) => fail!(ExecutorError::TrExecutorError(format!(
                    "cannot create credit phase of a new transaction for smart contract for reason {}",
                    e
                ))),
            };
        }
        let due_before_storage = account.due_payment().map(|due| due.as_u128());
        let is_due = account.due_payment().map(|due| due.as_u128()).is_some_and(|due| due != 0);
        let mut storage_fee;
        description.storage_ph = match self.storage_phase(
            account,
            &mut acc_balance,
            &mut tr,
            is_masterchain,
            is_special,
            is_due,
        ) {
            Ok(storage_ph) => {
                storage_fee = storage_ph.storage_fees_collected.as_u128();
                if let Some(due) = &storage_ph.storage_fees_due {
                    storage_fee += due.as_u128()
                }
                if let Some(due) = due_before_storage {
                    storage_fee -= due;
                }
                Some(storage_ph)
            }
            Err(e) => fail!(ExecutorError::TrExecutorError(format!(
                "cannot create storage phase of a new transaction for smart contract for reason {}",
                e
            ))),
        };
        if description.credit_first && msg_balance.grams > acc_balance.grams {
            msg_balance.grams = acc_balance.grams;
        }

        log::debug!(target: "executor",
            "storage_phase: {}", if description.storage_ph.is_some() {"present"} else {"none"});
        let mut original_acc_balance = account.balance().cloned().unwrap_or_default();
        original_acc_balance.sub(tr.total_fees())?;

        if !description.credit_first && !is_ext_msg {
            description.credit_ph = match self.credit_phase(
                account,
                &mut tr,
                &mut msg_balance,
                &mut acc_balance,
            ) {
                Ok(credit_ph) => Some(credit_ph),
                Err(e) => fail!(ExecutorError::TrExecutorError(format!(
                    "cannot create credit phase of a new transaction for smart contract for reason {}",
                    e
                ))),
            };
        }
        log::debug!(target: "executor",
            "credit_phase: {}", if description.credit_ph.is_some() {"present"} else {"none"});

        let last_paid = if !is_special { params.block_unixtime } else { 0 };
        account.set_last_paid(last_paid);
        #[cfg(feature = "timings")]
        {
            self.timings[0].fetch_add(now.elapsed().as_micros() as u64, Ordering::SeqCst);
            now = Instant::now();
        }

        let config_params = self.config().raw_config().config_params.data().cloned();
        let mut smc_info = SmartContractInfo {
            capabilities: self.config().raw_config().capabilities(),
            myself: SliceData::load_builder(
                account_address.write_to_new_cell().unwrap_or_default(),
            )
            .unwrap(),
            block_lt: params.block_lt,
            trans_lt: lt,
            unix_time: params.block_unixtime,
            seq_no: params.seq_no,
            balance: acc_balance.clone(),
            config_params,
            ..Default::default()
        };
        smc_info.calc_rand_seed(
            params.seed_block.clone(),
            &account_address.address().get_bytestring(0),
        );
        let mut stack = Stack::new();
        stack
            .push(int!(acc_balance.grams.as_u128()))
            .push(int!(msg_balance.grams.as_u128()))
            .push(StackItem::Cell(in_msg_cell.clone()))
            .push(StackItem::Slice(in_msg.body().unwrap_or_default()))
            .push(boolean!(is_ext_msg));
        log::debug!(target: "executor", "compute_phase");
        let (compute_ph, actions, new_data) = match self.compute_phase(
            Some(in_msg),
            account,
            &mut acc_balance,
            &msg_balance,
            smc_info,
            stack,
            storage_fee,
            is_masterchain,
            is_special,
            &params,
        ) {
            Ok((compute_ph, actions, new_data)) => (compute_ph, actions, new_data),
            Err(e) => {
                log::debug!(target: "executor", "compute_phase error: {}", e);
                match e.downcast_ref::<ExecutorError>() {
                    Some(ExecutorError::NoAcceptError(_, _))
                    | Some(ExecutorError::TerminationDeadlineReached) => return Err(e),
                    _ => fail!(ExecutorError::TrExecutorError(e.to_string())),
                }
            }
        };
        let mut out_msgs = vec![];
        let mut action_phase_processed = false;
        let mut compute_phase_gas_fees = Grams::zero();
        let mut copyleft = None;
        description.compute_ph = compute_ph;
        let mut new_acc_balance = acc_balance.clone();
        description.action = match &description.compute_ph {
            TrComputePhase::Vm(phase) => {
                compute_phase_gas_fees = phase.gas_fees;
                tr.add_fee_grams(&phase.gas_fees)?;
                if phase.success {
                    log::debug!(target: "executor", "compute_phase: success");
                    log::debug!(target: "executor", "action_phase: lt={}", lt);
                    action_phase_processed = true;

                    let message_src_dapp_id = if let Some(AccountState::AccountActive {
                        state_init: _,
                    }) = account.state()
                    {
                        if !is_previous_state_active {
                            if let Some(header) = in_msg.int_header() {
                                header.src_dapp_id().clone()
                            } else {
                                Some(account.get_id().unwrap().get_bytestring(0).as_slice().into())
                            }
                        } else {
                            params.dapp_id
                        }
                    } else {
                        None
                    };
                    let minted_shell_orig = minted_shell.clone();
                    match self.action_phase_with_copyleft(
                        &mut tr,
                        account,
                        &original_acc_balance,
                        &mut new_acc_balance,
                        &mut msg_balance,
                        &compute_phase_gas_fees,
                        actions.unwrap_or_default(),
                        new_data,
                        account_address,
                        is_special,
                        params.available_credit,
                        minted_shell,
                        need_to_burn,
                        message_src_dapp_id,
                    ) {
                        Ok(ActionPhaseResult { phase, messages, copyleft_reward }) => {
                            if phase.success == false {
                                *minted_shell = minted_shell_orig.clone();
                            }
                            out_msgs = messages;
                            if let Some(copyleft_reward) = &copyleft_reward {
                                tr.total_fees_mut().grams.sub(&copyleft_reward.reward)?;
                            }
                            copyleft = copyleft_reward;
                            Some(phase)
                        }
                        Err(e) => {
                            *minted_shell = minted_shell_orig.clone();
                            fail!(ExecutorError::TrExecutorError(format!(
                                "cannot create action phase of a new transaction for smart contract for reason {}",
                                e
                            )))
                        }
                    }
                } else {
                    log::debug!(target: "executor", "compute_phase: failed");
                    if acc_balance.grams >= need_to_burn {
                        acc_balance.grams -= need_to_burn;
                    } else {
                        acc_balance.grams = Grams::zero();
                    }
                    None
                }
            }
            TrComputePhase::Skipped(skipped) => {
                log::debug!(target: "executor", "compute_phase: skipped reason {:?}", skipped.reason);
                if acc_balance.grams >= need_to_burn {
                    acc_balance.grams -= need_to_burn;
                } else {
                    acc_balance.grams = Grams::zero();
                }
                if is_ext_msg {
                    fail!(ExecutorError::ExtMsgComputeSkipped(skipped.reason.clone()))
                }
                None
            }
        };

        #[cfg(feature = "timings")]
        {
            self.timings[1].fetch_add(now.elapsed().as_micros() as u64, Ordering::SeqCst);
            now = Instant::now();
        }
        description.aborted = match description.action.as_ref() {
            Some(phase) => {
                log::debug!(
                    target: "executor",
                    "action_phase: present: success={}, err_code={}", phase.success, phase.result_code
                );
                if AccStatusChange::Deleted == phase.status_change {
                    *account = Account::default();
                    description.destroyed = true;
                }
                if phase.success {
                    acc_balance = new_acc_balance;
                }
                !phase.success
            }
            None => {
                log::debug!(target: "executor", "action_phase: none");
                true
            }
        };

        if description.aborted && !is_ext_msg && bounce {
            msg_balance = original_msg_balance.clone();
            if exchanged {
                let mut add_value = CurrencyCollection::new();
                add_value.set_other(2, msg_balance_convert.into())?;
                if Grams::from(msg_balance_convert) > acc_balance.grams {
                    acc_balance.grams = Grams::zero();
                    acc_balance.add(&add_value)?;
                } else {
                    acc_balance.grams -= Grams::from(msg_balance_convert);
                    acc_balance.add(&add_value)?;
                }
            }
            if !action_phase_processed
                || self.config().has_capability(GlobalCapabilities::CapBounceAfterFailedAction)
            {
                log::debug!(target: "executor", "bounce_phase");
                description.bounce = match self.bounce_phase(
                    msg_balance.clone(),
                    &mut acc_balance,
                    &compute_phase_gas_fees,
                    in_msg,
                    &mut tr,
                    account_address,
                ) {
                    Ok((bounce_ph, Some(bounce_msg))) => {
                        log::debug!(target: "executor", "bounce_phase: out_msg value: {}", bounce_msg.get_value().unwrap());
                        out_msgs.push(bounce_msg);
                        Some(bounce_ph)
                    }
                    Ok((bounce_ph, None)) => Some(bounce_ph),
                    Err(e) => fail!(ExecutorError::TrExecutorError(format!(
                        "cannot create bounce phase of a new transaction for smart contract for reason {}",
                        e
                    ))),
                };
            }
            // if money can be returned to sender
            // restore account balance - storage fee
            if let Some(TrBouncePhase::Ok(_)) = description.bounce {
                log::debug!(target: "executor", "restore balance {} => {}", acc_balance.grams, original_acc_balance.grams);
                acc_balance = original_acc_balance;
            } else if account.is_none() && !acc_balance.is_zero()? {
                *account =
                    Account::uninit(account_address.clone(), 0, last_paid, acc_balance.clone());
            }
        }
        if (account.status() == AccountStatus::AccStateUninit) && acc_balance.is_zero()? {
            *account = Account::default();
        }
        tr.set_end_status(account.status());
        log::debug!(target: "executor", "set balance {}", acc_balance.grams);
        account.set_balance(acc_balance);
        log::debug!(target: "executor", "add messages");
        params.last_tr_lt.store(lt, Ordering::Relaxed);
        let lt = self.add_messages(&mut tr, out_msgs, params.last_tr_lt)?;
        account.set_last_tr_time(lt);
        tr.write_description(&TransactionDescr::Ordinary(description))?;
        #[cfg(feature = "timings")]
        self.timings[2].fetch_add(now.elapsed().as_micros() as u64, Ordering::SeqCst);
        tr.set_copyleft_reward(copyleft);
        Ok(tr)
    }

    fn ordinary_transaction(&self) -> bool {
        true
    }

    fn config(&self) -> &BlockchainConfig {
        &self.config
    }

    fn build_stack(&self, in_msg: Option<&Message>, account: &Account) -> Stack {
        let mut stack = Stack::new();
        let in_msg = match in_msg {
            Some(in_msg) => in_msg,
            None => return stack,
        };
        let acc_balance = int!(account.balance().map_or(0, |value| value.grams.as_u128()));
        let msg_balance = int!(in_msg.get_value().map_or(0, |value| value.grams.as_u128()));
        let function_selector = boolean!(in_msg.is_inbound_external());
        let body_slice = in_msg.body().unwrap_or_default();
        let in_msg_cell = in_msg.serialize().unwrap_or_default();
        stack
            .push(acc_balance)
            .push(msg_balance)
            .push(StackItem::Cell(in_msg_cell))
            .push(StackItem::Slice(body_slice))
            .push(function_selector);
        stack
    }
}
