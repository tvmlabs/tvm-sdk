// Copyright (C) 2019-2022 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use std::sync::atomic::Ordering;

use tvm_block::Account;
use tvm_block::CurrencyCollection;
use tvm_block::Grams;
use tvm_block::Message;
use tvm_block::Serializable;
use tvm_block::TrComputePhase;
use tvm_block::Transaction;
use tvm_block::TransactionDescr;
use tvm_block::TransactionDescrTickTock;
use tvm_block::TransactionTickTock;
use tvm_types::error;
use tvm_types::fail;
use tvm_types::HashmapType;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_vm::boolean;
use tvm_vm::int;
use tvm_vm::stack::integer::IntegerData;
use tvm_vm::stack::Stack;
use tvm_vm::stack::StackItem;
use tvm_vm::SmartContractInfo;

use crate::blockchain_config::BlockchainConfig;
use crate::error::ExecutorError;
use crate::ActionPhaseResult;
use crate::ExecuteParams;
use crate::TransactionExecutor;

pub struct TickTockTransactionExecutor {
    pub config: BlockchainConfig,
    pub tt: TransactionTickTock,
}

impl TickTockTransactionExecutor {
    pub fn new(config: BlockchainConfig, tt: TransactionTickTock) -> Self {
        Self { config, tt }
    }
}

impl TransactionExecutor for TickTockTransactionExecutor {
    /// Create end execute tick or tock transaction for special account
    fn execute_with_params(
        &self,
        in_msg: Option<&Message>,
        account: &mut Account,
        params: ExecuteParams,
    ) -> Result<Transaction> {
        if in_msg.is_some() {
            fail!("Tick Tock transaction must not have input message")
        }
        let account_id = match account.get_id() {
            Some(addr) => addr,
            None => fail!("Tick Tock contract should have Standard address"),
        };
        match account.get_tick_tock() {
            Some(tt) => {
                if tt.tock != self.tt.is_tock() && tt.tick != self.tt.is_tick() {
                    fail!("wrong type of account's tick tock flag")
                }
            }
            None => fail!("Account {:x} is not special account for tick tock", account_id),
        }
        let account_address = account.get_addr().cloned().unwrap_or_default();
        log::debug!(target: "executor", "tick tock transation account {:x}", account_id);
        let mut acc_balance = account.balance().cloned().unwrap_or_default();

        let is_masterchain = true;
        let is_special = true;
        let lt = std::cmp::max(
            account.last_tr_time().unwrap_or_default(),
            params.last_tr_lt.load(Ordering::Relaxed),
        );
        let mut tr = Transaction::with_address_and_status(account_id.clone(), account.status());
        tr.set_logical_time(lt);
        tr.set_now(params.block_unixtime);
        account.set_last_paid(0);
        let due_before_storage = account.due_payment().map_or(0, |due| due.as_u128());
        let (storage, storage_fee) = match self.storage_phase(
            account,
            &mut acc_balance,
            &mut tr,
            is_masterchain,
            is_special,
        ) {
            Ok(storage_ph) => {
                let mut storage_fee = storage_ph.storage_fees_collected.as_u128();
                if let Some(due) = &storage_ph.storage_fees_due {
                    storage_fee += due.as_u128()
                }
                storage_fee -= due_before_storage;
                (storage_ph, storage_fee)
            }
            Err(e) => fail!(ExecutorError::TrExecutorError(format!(
                "cannot create storage phase of a new transaction for \
                         smart contract for reason {}",
                e
            ))),
        };
        let mut description = TransactionDescrTickTock {
            tt: self.tt.clone(),
            storage,
            ..TransactionDescrTickTock::default()
        };

        let old_account = account.clone();
        let original_acc_balance = acc_balance.clone();

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
            .push(int!(account.balance().map_or(0, |value| value.grams.as_u128())))
            .push(StackItem::integer(IntegerData::from_unsigned_bytes_be(
                account_id.get_bytestring(0),
            )))
            .push(boolean!(self.tt.is_tock()))
            .push(int!(-2));
        log::debug!(target: "executor", "compute_phase {}", lt);
        let (compute_ph, actions, new_data) = match self.compute_phase(
            None,
            account,
            &mut acc_balance,
            &CurrencyCollection::default(),
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
                    Some(ExecutorError::NoAcceptError(_, _)) => return Err(e),
                    _ => fail!(ExecutorError::TrExecutorError(e.to_string())),
                }
            }
        };
        let mut out_msgs = vec![];
        description.compute_ph = compute_ph;
        description.action = match description.compute_ph {
            TrComputePhase::Vm(ref phase) => {
                tr.add_fee_grams(&phase.gas_fees)?;
                if phase.success {
                    log::debug!(target: "executor", "compute_phase: TrComputePhase::Vm success");
                    log::debug!(target: "executor", "action_phase {}", lt);
                    match self.action_phase_with_copyleft(
                        &mut tr,
                        account,
                        &original_acc_balance,
                        &mut acc_balance,
                        &mut CurrencyCollection::default(),
                        &Grams::zero(),
                        actions.unwrap_or_default(),
                        new_data,
                        &account_address,
                        is_special,
                    ) {
                        Ok(ActionPhaseResult { phase, messages, .. }) => {
                            out_msgs = messages;
                            // ignore copyleft reward because account is special
                            Some(phase)
                        }
                        Err(e) => fail!(ExecutorError::TrExecutorError(format!(
                            "cannot create action phase of a new transaction \
                                     for smart contract for reason {}",
                            e
                        ))),
                    }
                } else {
                    log::debug!(target: "executor", "compute_phase: TrComputePhase::Vm failed");
                    None
                }
            }
            TrComputePhase::Skipped(ref skipped) => {
                log::debug!(target: "executor", 
                    "compute_phase: skipped: reason {:?}", skipped.reason);
                None
            }
        };

        description.aborted = match description.action {
            Some(ref phase) => {
                log::debug!(target: "executor", 
                    "action_phase: present: success={}, err_code={}", phase.success, phase.result_code);
                !phase.success
            }
            None => {
                log::debug!(target: "executor", "action_phase: none");
                true
            }
        };

        log::debug!(target: "executor", "Desciption.aborted {}", description.aborted);
        tr.set_end_status(account.status());
        account.set_balance(acc_balance);
        if description.aborted {
            *account = old_account;
        }
        params.last_tr_lt.store(lt, Ordering::Relaxed);
        let lt = self.add_messages(&mut tr, out_msgs, params.last_tr_lt)?;
        account.set_last_tr_time(lt);
        tr.write_description(&TransactionDescr::TickTock(description))?;
        Ok(tr)
    }

    fn ordinary_transaction(&self) -> bool {
        false
    }

    fn config(&self) -> &BlockchainConfig {
        &self.config
    }

    fn build_stack(&self, _in_msg: Option<&Message>, account: &Account) -> Stack {
        let account_balance = account.balance().unwrap().grams.as_u128();
        let account_id = account.get_id().unwrap();
        let mut stack = Stack::new();
        stack
            .push(int!(account_balance))
            .push(StackItem::integer(IntegerData::from_unsigned_bytes_be(
                account_id.get_bytestring(0),
            )))
            .push(boolean!(self.tt.is_tock()))
            .push(int!(-2));
        stack
    }
}
