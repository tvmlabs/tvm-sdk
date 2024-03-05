// Copyright (C) 2019-2023 Everx. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.
#![allow(clippy::too_many_arguments)]

use std::cmp::min;
use std::collections::LinkedList;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use tvm_block::AccStatusChange;
use tvm_block::Account;
use tvm_block::AccountStatus;
use tvm_block::AddSub;
use tvm_block::CommonMsgInfo;
use tvm_block::ComputeSkipReason;
use tvm_block::CopyleftReward;
use tvm_block::CurrencyCollection;
use tvm_block::Deserializable;
use tvm_block::ExtraCurrencyCollection;
use tvm_block::GasLimitsPrices;
use tvm_block::GetRepresentationHash;
use tvm_block::GlobalCapabilities;
use tvm_block::Grams;
use tvm_block::HashUpdate;
use tvm_block::Message;
use tvm_block::MsgAddressInt;
use tvm_block::OutAction;
use tvm_block::OutActions;
use tvm_block::Serializable;
use tvm_block::StateInit;
use tvm_block::StorageUsedShort;
use tvm_block::TrActionPhase;
use tvm_block::TrBouncePhase;
use tvm_block::TrComputePhase;
use tvm_block::TrComputePhaseVm;
use tvm_block::TrCreditPhase;
use tvm_block::TrStoragePhase;
use tvm_block::Transaction;
use tvm_block::WorkchainFormat;
use tvm_block::BASE_WORKCHAIN_ID;
use tvm_block::MASTERCHAIN_ID;
use tvm_block::RESERVE_ALL_BUT;
use tvm_block::RESERVE_IGNORE_ERROR;
use tvm_block::RESERVE_PLUS_ORIG;
use tvm_block::RESERVE_REVERSE;
use tvm_block::RESERVE_VALID_MODES;
use tvm_block::SENDMSG_ALL_BALANCE;
use tvm_block::SENDMSG_DELETE_IF_EMPTY;
use tvm_block::SENDMSG_IGNORE_ERROR;
use tvm_block::SENDMSG_PAY_FEE_SEPARATELY;
use tvm_block::SENDMSG_REMAINING_MSG_BALANCE;
use tvm_block::SENDMSG_VALID_FLAGS;
use tvm_types::error;
use tvm_types::fail;
use tvm_types::AccountId;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::HashmapE;
use tvm_types::HashmapType;
use tvm_types::IBitstring;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::UInt256;
use tvm_vm::error::tvm_exception;
use tvm_vm::executor::gas::gas_state::Gas;
use tvm_vm::executor::BehaviorModifiers;
use tvm_vm::executor::IndexProvider;
use tvm_vm::smart_contract_info::SmartContractInfo;
use tvm_vm::stack::Stack;

use crate::blockchain_config::BlockchainConfig;
use crate::blockchain_config::CalcMsgFwdFees;
use crate::error::ExecutorError;
use crate::vmsetup::VMSetup;
use crate::vmsetup::VMSetupContext;
use crate::VERSION_BLOCK_NEW_CALCULATION_BOUNCED_STORAGE;

const RESULT_CODE_ACTIONLIST_INVALID: i32 = 32;
const RESULT_CODE_TOO_MANY_ACTIONS: i32 = 33;
const RESULT_CODE_UNKNOWN_OR_INVALID_ACTION: i32 = 34;
const RESULT_CODE_INCORRECT_SRC_ADDRESS: i32 = 35;
const RESULT_CODE_INCORRECT_DST_ADDRESS: i32 = 36;
const RESULT_CODE_NOT_ENOUGH_GRAMS: i32 = 37;
const RESULT_CODE_NOT_ENOUGH_EXTRA: i32 = 38;
const RESULT_CODE_INVALID_BALANCE: i32 = 40;
const RESULT_CODE_BAD_ACCOUNT_STATE: i32 = 41;
const RESULT_CODE_ANYCAST: i32 = 50;
const RESULT_CODE_NOT_FOUND_LICENSE: i32 = 51;
const RESULT_CODE_UNSUPPORTED: i32 = -1;

const MAX_ACTIONS: usize = 255;

const MAX_MSG_BITS: usize = 1 << 21;
const MAX_MSG_CELLS: usize = 1 << 13;

#[derive(Eq, PartialEq, Debug)]
pub enum IncorrectCheckRewrite {
    Anycast,
    Other,
}

pub struct ExecuteParams {
    pub state_libs: HashmapE,
    pub block_unixtime: u32,
    pub block_lt: u64,
    pub seq_no: u32,
    pub last_tr_lt: Arc<AtomicU64>,
    pub seed_block: UInt256,
    pub debug: bool,
    pub trace_callback: Option<Arc<tvm_vm::executor::TraceCallback>>,
    pub index_provider: Option<Arc<dyn IndexProvider>>,
    pub behavior_modifiers: Option<BehaviorModifiers>,
    pub block_version: u32,
    #[cfg(feature = "signature_with_id")]
    pub signature_id: i32,
    pub vm_execution_is_block_related: Arc<Mutex<bool>>,
    pub block_collation_was_finished: Arc<Mutex<bool>>,
}

pub struct ActionPhaseResult {
    pub phase: TrActionPhase,
    pub messages: Vec<Message>,
    pub copyleft_reward: Option<CopyleftReward>,
}

impl ActionPhaseResult {
    fn new(
        phase: TrActionPhase,
        messages: Vec<Message>,
        copyleft_reward: Option<CopyleftReward>,
    ) -> ActionPhaseResult {
        ActionPhaseResult { phase, messages, copyleft_reward }
    }

    fn from_phase(phase: TrActionPhase) -> ActionPhaseResult {
        ActionPhaseResult::new(phase, vec![], None)
    }
}

impl Default for ExecuteParams {
    fn default() -> Self {
        Self {
            state_libs: HashmapE::with_bit_len(32),
            block_unixtime: 0,
            block_lt: 0,
            seq_no: 0,
            last_tr_lt: Arc::new(AtomicU64::new(0)),
            seed_block: UInt256::default(),
            debug: false,
            trace_callback: None,
            index_provider: None,
            behavior_modifiers: None,
            block_version: 0,
            #[cfg(feature = "signature_with_id")]
            signature_id: 0,
            vm_execution_is_block_related: Arc::new(Mutex::new(false)),
            block_collation_was_finished: Arc::new(Mutex::new(false)),
        }
    }
}

pub trait TransactionExecutor {
    fn execute_with_params(
        &self,
        in_msg: Option<&Message>,
        account: &mut Account,
        params: ExecuteParams,
    ) -> Result<Transaction>;

    fn execute_with_libs_and_params(
        &self,
        in_msg: Option<&Message>,
        account_root: &mut Cell,
        params: ExecuteParams,
    ) -> Result<Transaction> {
        let old_hash = account_root.repr_hash();
        let mut account = Account::construct_from_cell(account_root.clone())?;
        let mut transaction = self.execute_with_params(in_msg, &mut account, params)?;
        if self.config().has_capability(GlobalCapabilities::CapFastStorageStat) {
            account.update_storage_stat_fast()?;
        } else {
            account.update_storage_stat()?;
        }
        *account_root = account.serialize()?;
        let new_hash = account_root.repr_hash();
        transaction.write_state_update(&HashUpdate::with_hashes(old_hash, new_hash))?;
        Ok(transaction)
    }

    #[deprecated]
    fn build_contract_info(
        &self,
        acc_balance: &CurrencyCollection,
        acc_address: &MsgAddressInt,
        unix_time: u32,
        block_lt: u64,
        trans_lt: u64,
        seed_block: UInt256,
    ) -> SmartContractInfo {
        let config_params = self.config().raw_config().config_params.data().cloned();
        let mut smci = SmartContractInfo {
            capabilities: self.config().raw_config().capabilities(),
            myself: SliceData::load_builder(acc_address.write_to_new_cell().unwrap_or_default())
                .unwrap(),
            block_lt,
            trans_lt,
            unix_time,
            balance: acc_balance.clone(),
            config_params,
            ..Default::default()
        };
        smci.calc_rand_seed(seed_block, &acc_address.address().get_bytestring(0));
        smci
    }

    fn ordinary_transaction(&self) -> bool;
    fn config(&self) -> &BlockchainConfig;

    fn build_stack(&self, in_msg: Option<&Message>, account: &Account) -> Stack;

    /// Implementation of transaction's storage phase.
    /// If account does not exist - phase skipped.
    /// Calculates storage fees and substracts them from account balance.
    /// If account balance becomes negative after that, then account is frozen.
    fn storage_phase(
        &self,
        acc: &mut Account,
        acc_balance: &mut CurrencyCollection,
        tr: &mut Transaction,
        is_masterchain: bool,
        is_special: bool,
    ) -> Result<TrStoragePhase> {
        log::debug!(target: "executor", "storage_phase");
        if tr.now() < acc.last_paid() {
            fail!("transaction timestamp must be greater then account timestamp")
        }

        if is_special {
            log::debug!(target: "executor", "Special account: AccStatusChange::Unchanged");
            return Ok(TrStoragePhase::with_params(
                Grams::zero(),
                acc.due_payment().cloned(),
                AccStatusChange::Unchanged,
            ));
        }
        let mut fee = match acc.storage_info() {
            Some(storage_info) => {
                self.config().calc_storage_fee(storage_info, is_masterchain, tr.now())?
            }
            None => {
                log::debug!(target: "executor", "Account::None");
                return Ok(Default::default());
            }
        };
        if let Some(due_payment) = acc.due_payment() {
            fee.add(due_payment)?;
            acc.set_due_payment(None);
        }

        if acc_balance.grams >= fee {
            log::debug!(target: "executor", "acc_balance: {}, storage fee: {}", acc_balance.grams, fee);
            acc_balance.grams.sub(&fee)?;
            tr.add_fee_grams(&fee)?;
            Ok(TrStoragePhase::with_params(fee, None, AccStatusChange::Unchanged))
        } else {
            log::debug!(target: "executor", "acc_balance: {} is storage fee from total: {}", acc_balance.grams, fee);
            let storage_fees_collected = std::mem::take(&mut acc_balance.grams);
            tr.add_fee_grams(&storage_fees_collected)?;
            fee.sub(&storage_fees_collected)?;
            let need_freeze =
                fee > Grams::from(self.config().get_gas_config(is_masterchain).freeze_due_limit);
            let need_delete = (acc.status() == AccountStatus::AccStateUninit
                || acc.status() == AccountStatus::AccStateFrozen)
                && fee > Grams::from(self.config().get_gas_config(is_masterchain).delete_due_limit);

            if need_delete {
                tr.total_fees_mut().add(acc_balance)?;
                *acc = Account::default();
                acc_balance.other = Default::default();
                Ok(TrStoragePhase::with_params(
                    storage_fees_collected,
                    Some(fee),
                    AccStatusChange::Deleted,
                ))
            } else if need_freeze {
                acc.set_due_payment(Some(fee));
                if acc.status() == AccountStatus::AccStateActive {
                    acc.try_freeze()?;
                    Ok(TrStoragePhase::with_params(
                        storage_fees_collected,
                        Some(fee),
                        AccStatusChange::Frozen,
                    ))
                } else {
                    Ok(TrStoragePhase::with_params(
                        storage_fees_collected,
                        Some(fee),
                        AccStatusChange::Unchanged,
                    ))
                }
            } else {
                acc.set_due_payment(Some(fee));
                Ok(TrStoragePhase::with_params(
                    storage_fees_collected,
                    Some(fee),
                    AccStatusChange::Unchanged,
                ))
            }
        }
    }

    /// Implementation of transaction's credit phase.
    /// Increases account balance by the amount that appears in the internal
    /// message header. If account does not exist - phase skipped.
    /// If message is not internal - phase skipped.
    fn credit_phase(
        &self,
        acc: &mut Account,
        tr: &mut Transaction,
        msg_balance: &mut CurrencyCollection,
        acc_balance: &mut CurrencyCollection,
    ) -> Result<TrCreditPhase> {
        let collected = if let Some(due_payment) = acc.due_payment() {
            let collected = *min(due_payment, &msg_balance.grams);
            msg_balance.grams.sub(&collected)?;
            let mut due_payment_remaining = *due_payment;
            due_payment_remaining.sub(&collected)?;
            acc.set_due_payment(if due_payment_remaining.is_zero() {
                None
            } else {
                Some(due_payment_remaining)
            });
            tr.total_fees_mut().grams.add(&collected)?;
            if collected.is_zero() { None } else { Some(collected) }
        } else {
            None
        };
        log::debug!(
            target: "executor",
            "credit_phase: add funds {} to {}",
            msg_balance.grams, acc_balance.grams
        );
        acc_balance.add(msg_balance)?;
        Ok(TrCreditPhase::with_params(collected, msg_balance.clone()))
        // TODO: Is it need to credit with ihr_fee value in internal messages?
    }

    /// Implementation of transaction's computing phase.
    /// Evaluates new accout state and invokes TVM if account has contract code.
    fn compute_phase(
        &self,
        msg: Option<&Message>,
        acc: &mut Account,
        acc_balance: &mut CurrencyCollection,
        msg_balance: &CurrencyCollection,
        mut smc_info: SmartContractInfo,
        stack: Stack,
        storage_fee: u128,
        is_masterchain: bool,
        is_special: bool,
        params: &ExecuteParams,
    ) -> Result<(TrComputePhase, Option<Cell>, Option<Cell>)> {
        let mut result_acc = acc.clone();
        let mut vm_phase = TrComputePhaseVm::default();
        let init_code_hash = self.config().has_capability(GlobalCapabilities::CapInitCodeHash);
        let libs_disabled = !self.config().has_capability(GlobalCapabilities::CapSetLibCode);
        let is_external = if let Some(msg) = msg {
            if let Some(header) = msg.int_header() {
                log::debug!(target: "executor", "msg internal, bounce: {}", header.bounce);
                if result_acc.is_none() {
                    if let Some(new_acc) =
                        account_from_message(msg, msg_balance, true, init_code_hash, libs_disabled)
                    {
                        result_acc = new_acc;
                        result_acc.set_last_paid(if !is_special {
                            smc_info.unix_time()
                        } else {
                            0
                        });

                        // if there was a balance in message (not bounce), then account state at
                        // least become uninit
                        *acc = result_acc.clone();
                        acc.uninit_account();
                    }
                }
                false
            } else {
                log::debug!(target: "executor", "msg external");
                true
            }
        } else {
            debug_assert!(!result_acc.is_none());
            false
        };
        log::debug!(target: "executor", "acc balance: {}", acc_balance.grams);
        log::debug!(target: "executor", "msg balance: {}", msg_balance.grams);
        let is_ordinary = self.ordinary_transaction();
        if acc_balance.grams.is_zero() {
            log::debug!(target: "executor", "skip computing phase no gas");
            return Ok((TrComputePhase::skipped(ComputeSkipReason::NoGas), None, None));
        }
        let gas_config = self.config().get_gas_config(is_masterchain);
        let gas = init_gas(
            acc_balance.grams.as_u128(),
            msg_balance.grams.as_u128(),
            is_external,
            is_special,
            is_ordinary,
            gas_config,
        );
        if gas.get_gas_limit() == 0 && gas.get_gas_credit() == 0 {
            log::debug!(target: "executor", "skip computing phase no gas");
            return Ok((TrComputePhase::skipped(ComputeSkipReason::NoGas), None, None));
        }

        let mut libs = vec![];
        if let Some(msg) = msg {
            if let Some(state_init) = msg.state_init() {
                libs.push(state_init.libraries().inner());
            }
            if let Some(reason) =
                compute_new_state(&mut result_acc, acc_balance, msg, self.config())?
            {
                if !init_code_hash {
                    *acc = result_acc;
                }
                return Ok((TrComputePhase::skipped(reason), None, None));
            }
        };

        vm_phase.gas_credit = match gas.get_gas_credit() as u32 {
            0 => None,
            value => Some(value.try_into()?),
        };
        vm_phase.gas_limit = (gas.get_gas_limit() as u64).try_into()?;

        if result_acc.get_code().is_none() {
            vm_phase.exit_code = -13;
            if is_external {
                fail!(ExecutorError::NoAcceptError(vm_phase.exit_code, None))
            } else {
                vm_phase.exit_arg = None;
                vm_phase.success = false;
                vm_phase.gas_fees =
                    Grams::new(if is_special { 0 } else { gas_config.calc_gas_fee(0) })?;
                if !acc_balance.grams.sub(&vm_phase.gas_fees)? {
                    log::debug!(target: "executor", "can't sub funds: {} from acc_balance: {}", vm_phase.gas_fees, acc_balance.grams);
                    fail!("can't sub funds: from acc_balance")
                }
                *acc = result_acc;
                return Ok((TrComputePhase::Vm(vm_phase), None, None));
            }
        }
        let code = result_acc.get_code().unwrap_or_default();
        let data = result_acc.get_data().unwrap_or_default();
        libs.push(result_acc.libraries().inner());
        libs.push(params.state_libs.clone());

        smc_info.set_mycode(code.clone());
        smc_info.set_storage_fee(storage_fee);
        if let Some(init_code_hash) = result_acc.init_code_hash() {
            smc_info.set_init_code_hash(init_code_hash.clone());
        }
        let mut vm = VMSetup::with_context(
            SliceData::load_cell(code)?,
            VMSetupContext {
                capabilities: self.config().capabilites(),
                block_version: params.block_version,
                #[cfg(feature = "signature_with_id")]
                signature_id: params.signature_id,
            },
        )
        .set_smart_contract_info(smc_info)?
        .set_stack(stack)
        .set_data(data)?
        .set_libraries(libs)
        .set_gas(gas)
        .set_debug(params.debug)
        .set_block_related_flags(
            params.vm_execution_is_block_related.clone(),
            params.block_collation_was_finished.clone(),
        )
        .create();

        if let Some(modifiers) = params.behavior_modifiers.clone() {
            vm.modify_behavior(modifiers);
        }

        // TODO: set vm_init_state_hash

        if let Some(trace_callback) = params.trace_callback.clone() {
            vm.set_trace_callback(move |engine, info| trace_callback(engine, info));
        }

        let result = vm.execute();
        log::trace!(target: "executor", "execute result: {:?}", result);
        let mut raw_exit_arg = None;
        match result {
            Err(err) => {
                log::debug!(target: "executor", "VM terminated with exception: {}", err);
                let exception = tvm_exception(err)?;
                vm_phase.exit_code = if let Some(code) = exception.custom_code() {
                    code
                } else {
                    match exception.exception_code() {
                        Some(ExceptionCode::OutOfGas) => !(ExceptionCode::OutOfGas as i32), /* correct error code according cpp code */
                        Some(error_code) => error_code as i32,
                        None => ExceptionCode::UnknownError as i32,
                    }
                };
                vm_phase.exit_arg = match exception
                    .value
                    .as_integer()
                    .and_then(|value| value.into(std::i32::MIN..=std::i32::MAX))
                {
                    Err(_) | Ok(0) => None,
                    Ok(exit_arg) => Some(exit_arg),
                };
                raw_exit_arg = Some(exception.value);
            }
            Ok(exit_code) => vm_phase.exit_code = exit_code,
        };
        vm_phase.success = vm.get_committed_state().is_committed();
        log::debug!(target: "executor", "VM terminated with exit code {}", vm_phase.exit_code);

        // calc gas fees
        let gas = vm.get_gas();
        let credit = gas.get_gas_credit() as u32;
        // for external messages gas will not be exacted if VM throws the exception and
        // gas_credit != 0
        let used = gas.get_gas_used() as u64;
        vm_phase.gas_used = used.try_into()?;
        if credit != 0 {
            if is_external {
                fail!(ExecutorError::NoAcceptError(vm_phase.exit_code, raw_exit_arg))
            }
            vm_phase.gas_fees = Grams::zero();
        } else {
            // credit == 0 means contract accepted
            let gas_fees = if is_special { 0 } else { gas_config.calc_gas_fee(used) };
            vm_phase.gas_fees = gas_fees.try_into()?;
        };

        log::debug!(
            target: "executor",
            "gas after: gl: {}, gc: {}, gu: {}, fees: {}",
            gas.get_gas_limit() as u64, credit, used, vm_phase.gas_fees
        );

        // set mode
        vm_phase.mode = 0;
        vm_phase.vm_steps = vm.steps();
        // TODO: vm_final_state_hash
        log::debug!(target: "executor", "acc_balance: {}, gas fees: {}", acc_balance.grams, vm_phase.gas_fees);
        if !acc_balance.grams.sub(&vm_phase.gas_fees)? {
            log::error!(target: "executor", "This situation is unreachable: can't sub funds: {} from acc_balance: {}", vm_phase.gas_fees, acc_balance.grams);
            fail!("can't sub funds: from acc_balance")
        }

        let new_data = if let Ok(cell) = vm.get_committed_state().get_root().as_cell() {
            Some(cell.clone())
        } else {
            log::debug!(target: "executor", "invalid contract, it must be cell in c4 register");
            vm_phase.success = false;
            None
        };

        let out_actions = if let Ok(root_cell) = vm.get_committed_state().get_actions().as_cell() {
            Some(root_cell.clone())
        } else {
            log::debug!(target: "executor", "invalid contract, it must be cell in c5 register");
            vm_phase.success = false;
            None
        };

        *acc = result_acc;
        Ok((TrComputePhase::Vm(vm_phase), out_actions, new_data))
    }

    /// Implementation of transaction's action phase.
    /// If computing phase is successful then action phase is started.
    /// If TVM invoked in computing phase returned some output actions,
    /// then they will be added to transaction's output message list.
    /// Total value from all outbound internal messages will be collected and
    /// substracted from account balance. If account has enough funds this
    /// will be succeded, otherwise action phase is failed, transaction will be
    /// marked as aborted, account changes will be rollbacked.
    #[deprecated]
    fn action_phase(
        &self,
        tr: &mut Transaction,
        acc: &mut Account,
        original_acc_balance: &CurrencyCollection,
        acc_balance: &mut CurrencyCollection,
        msg_remaining_balance: &mut CurrencyCollection,
        compute_phase_fees: &Grams,
        actions_cell: Cell,
        new_data: Option<Cell>,
        my_addr: &MsgAddressInt,
        is_special: bool,
    ) -> Result<(TrActionPhase, Vec<Message>)> {
        let result = self.action_phase_with_copyleft(
            tr,
            acc,
            original_acc_balance,
            acc_balance,
            msg_remaining_balance,
            compute_phase_fees,
            actions_cell,
            new_data,
            my_addr,
            is_special,
        )?;
        Ok((result.phase, result.messages))
    }

    fn action_phase_with_copyleft(
        &self,
        tr: &mut Transaction,
        acc: &mut Account,
        original_acc_balance: &CurrencyCollection,
        acc_balance: &mut CurrencyCollection,
        msg_remaining_balance: &mut CurrencyCollection,
        compute_phase_fees: &Grams,
        actions_cell: Cell,
        new_data: Option<Cell>,
        my_addr: &MsgAddressInt,
        is_special: bool,
    ) -> Result<ActionPhaseResult> {
        let mut out_msgs = vec![];
        let mut acc_copy = acc.clone();
        let mut acc_remaining_balance = acc_balance.clone();
        let mut phase = TrActionPhase::default();
        let mut total_reserved_value = CurrencyCollection::default();
        phase.action_list_hash = actions_cell.repr_hash();
        let mut actions = match OutActions::construct_from_cell(actions_cell) {
            Err(err) => {
                log::debug!(
                    target: "executor",
                    "cannot parse action list: format is invalid, err: {}",
                    err
                );
                // Here you can select only one of 2 error codes:
                // RESULT_CODE_UNKNOWN_OR_INVALID_ACTION or RESULT_CODE_ACTIONLIST_INVALID
                phase.result_code = RESULT_CODE_UNKNOWN_OR_INVALID_ACTION;
                return Ok(ActionPhaseResult::from_phase(phase));
            }
            Ok(actions) => actions,
        };

        if actions.len() > MAX_ACTIONS {
            log::debug!(target: "executor", "too many actions: {}", actions.len());
            phase.result_code = RESULT_CODE_TOO_MANY_ACTIONS;
            return Ok(ActionPhaseResult::from_phase(phase));
        }
        phase.tot_actions = actions.len() as i16;

        let process_err_code =
            |mut err_code: i32, i: usize, phase: &mut TrActionPhase| -> Result<bool> {
                if err_code == -1 {
                    err_code = RESULT_CODE_UNKNOWN_OR_INVALID_ACTION;
                }
                if err_code != 0 {
                    log::debug!(target: "executor", "action failed: error_code={}", err_code);
                    phase.valid = true;
                    phase.result_code = err_code;
                    if i != 0 {
                        phase.result_arg = Some(i as i32);
                    }
                    if err_code == RESULT_CODE_NOT_ENOUGH_GRAMS
                        || err_code == RESULT_CODE_NOT_ENOUGH_EXTRA
                    {
                        phase.no_funds = true;
                    }
                    Ok(true)
                } else {
                    Ok(false)
                }
            };

        let mut account_deleted = false;

        let mut out_msgs0 = vec![];
        let copyleft_reward =
            self.copyleft_action_handler(compute_phase_fees, &mut phase, &actions)?;
        if phase.result_code != 0 {
            return Ok(ActionPhaseResult::from_phase(phase));
        }

        // The ChangeLibrary action can be created by VM in two ways:
        // 1) by the SETLIBCODE and CHANGELIB insns,
        // 2) manually by modifying the c5 register.
        //
        // In the case of CapSetLibCode is not set, (1) is denied by VM but (2) is still
        // available. To deny (2) too, CapSetLibCode needs to be checked here in
        // executor. However, since the executor's behavior gets modified, an
        // additional capability must be checked beforehand.
        let is_change_library_denied = self.config().has_capability(GlobalCapabilities::CapTvmV19)
            && !self.config().has_capability(GlobalCapabilities::CapSetLibCode);

        for (i, action) in actions.iter_mut().enumerate() {
            log::debug!(target: "executor", "\nAction #{}\nType: {}\nInitial balance: {}",
                i,
                action_type(action),
                balance_to_string(&acc_remaining_balance)
            );
            let mut init_balance = acc_remaining_balance.clone();
            let err_code = match std::mem::replace(action, OutAction::None) {
                OutAction::SendMsg { mode, mut out_msg } => {
                    if (mode & SENDMSG_ALL_BALANCE) != 0 {
                        out_msgs0.push((i, mode, out_msg));
                        log::debug!(target: "executor", "Message with flag `SEND_ALL_BALANCE` it will be sent last. Skip it for now.");
                        continue;
                    }
                    let result = outmsg_action_handler(
                        &mut phase,
                        mode,
                        &mut out_msg,
                        &mut acc_remaining_balance,
                        msg_remaining_balance,
                        compute_phase_fees,
                        self.config(),
                        is_special,
                        my_addr,
                        &total_reserved_value,
                        &mut account_deleted,
                    );
                    match result {
                        Ok(_) => {
                            phase.msgs_created += 1;
                            out_msgs0.push((i, mode, out_msg));
                            0
                        }
                        Err(code) => code,
                    }
                }
                OutAction::ReserveCurrency { mode, value } => {
                    match reserve_action_handler(
                        mode,
                        &value,
                        original_acc_balance,
                        &mut acc_remaining_balance,
                    ) {
                        Ok(reserved_value) => {
                            phase.spec_actions += 1;
                            match total_reserved_value.add(&reserved_value) {
                                Ok(_) => 0,
                                Err(_) => RESULT_CODE_INVALID_BALANCE,
                            }
                        }
                        Err(code) => code,
                    }
                }
                OutAction::SetCode { new_code: code } => {
                    match setcode_action_handler(&mut acc_copy, code) {
                        None => {
                            phase.spec_actions += 1;
                            0
                        }
                        Some(code) => code,
                    }
                }
                OutAction::ChangeLibrary { .. } if is_change_library_denied => {
                    RESULT_CODE_UNKNOWN_OR_INVALID_ACTION
                }
                OutAction::ChangeLibrary { mode, code, hash } => {
                    match change_library_action_handler(&mut acc_copy, mode, code, hash) {
                        None => {
                            phase.spec_actions += 1;
                            0
                        }
                        Some(code) => code,
                    }
                }
                OutAction::CopyLeft { .. } => 0,
                OutAction::None => RESULT_CODE_UNKNOWN_OR_INVALID_ACTION,
            };
            init_balance.sub(&acc_remaining_balance)?;
            log::debug!(target: "executor", "Final balance:   {}\nDelta:           {}",
                balance_to_string(&acc_remaining_balance),
                balance_to_string(&init_balance)
            );
            if process_err_code(err_code, i, &mut phase)? {
                return Ok(ActionPhaseResult::new(phase, vec![], copyleft_reward));
            }
        }

        for (i, mode, mut out_msg) in out_msgs0.into_iter() {
            if (mode & SENDMSG_ALL_BALANCE) == 0 {
                out_msgs.push(out_msg);
                continue;
            }
            log::debug!(target: "executor", "\nSend message with all balance:\nInitial balance: {}",
                balance_to_string(&acc_remaining_balance));
            let result = outmsg_action_handler(
                &mut phase,
                mode,
                &mut out_msg,
                &mut acc_remaining_balance,
                msg_remaining_balance,
                compute_phase_fees,
                self.config(),
                is_special,
                my_addr,
                &total_reserved_value,
                &mut account_deleted,
            );
            log::debug!(target: "executor", "Final balance:   {}", balance_to_string(&acc_remaining_balance));
            let err_code = match result {
                Ok(_) => {
                    phase.msgs_created += 1;
                    out_msgs.push(out_msg);
                    0
                }
                Err(code) => code,
            };
            if process_err_code(err_code, i, &mut phase)? {
                return Ok(ActionPhaseResult::new(phase, vec![], copyleft_reward));
            }
        }

        // calc new account balance
        log::debug!(target: "executor", "\nReturn reserved balance:\nInitial:  {}\nReserved: {}",
            balance_to_string(&acc_remaining_balance),
            balance_to_string(&total_reserved_value)
        );
        if let Err(err) = acc_remaining_balance.add(&total_reserved_value) {
            log::debug!(target: "executor", "failed to add account balance with reserved value {}", err);
            fail!("failed to add account balance with reserved value {}", err)
        }

        log::debug!(target: "executor", "Final:    {}", balance_to_string(&acc_remaining_balance));

        let fee = phase.total_action_fees();
        log::debug!(target: "executor", "Total action fees: {}", fee);
        tr.add_fee_grams(&fee)?;

        if account_deleted {
            log::debug!(target: "executor", "\nAccount deleted");
            phase.status_change = AccStatusChange::Deleted;
        }
        phase.valid = true;
        phase.success = true;
        *acc_balance = acc_remaining_balance;
        *acc = acc_copy;
        if let Some(new_data) = new_data {
            acc.set_data(new_data);
        }
        Ok(ActionPhaseResult::new(phase, out_msgs, copyleft_reward))
    }

    /// Implementation of transaction's bounce phase.
    /// Bounce phase occurs only if transaction 'aborted' flag is set and
    /// if inbound message is internal message with field 'bounce=true'.
    /// Generates outbound internal message for original message sender, with
    /// value equal to value of original message minus gas payments and
    /// forwarding fees and empty body. Generated message is added to
    /// transaction's output message list.
    fn bounce_phase(
        &self,
        mut remaining_msg_balance: CurrencyCollection,
        acc_balance: &mut CurrencyCollection,
        compute_phase_fees: &Grams,
        msg: &Message,
        tr: &mut Transaction,
        my_addr: &MsgAddressInt,
        block_version: u32,
    ) -> Result<(TrBouncePhase, Option<Message>)> {
        let header = msg.int_header().ok_or_else(|| error!("Not found msg internal header"))?;
        if !header.bounce {
            fail!("Bounce flag not set")
        }
        // create bounced message and swap src and dst addresses
        let mut header = header.clone();
        let msg_src =
            header.src_ref().ok_or_else(|| error!("Not found src in message header"))?.clone();
        let msg_dst = std::mem::replace(&mut header.dst, msg_src);
        header.set_src(msg_dst);
        match check_rewrite_dest_addr(&header.dst, self.config(), my_addr) {
            Ok(new_dst) => header.dst = new_dst,
            Err(_) => {
                log::warn!(target: "executor", "Incorrect destination address in a bounced message {}", header.dst);
                fail!("Incorrect destination address in a bounced message {}", header.dst)
            }
        }

        let is_masterchain = msg.is_masterchain();

        // create header for new bounced message and swap src and dst addresses
        header.ihr_disabled = true;
        header.bounce = false;
        header.bounced = true;
        header.ihr_fee = Grams::zero();

        let mut bounce_msg = Message::with_int_header(header);
        if self.config().has_capability(GlobalCapabilities::CapBounceMsgBody) {
            let mut builder = (-1i32).write_to_new_cell()?;
            if let Some(body) = msg.body() {
                let mut body_copy = body.clone();
                body_copy.shrink_data(0..256);
                builder.append_bytestring(&body_copy)?;
                if self.config().has_capability(GlobalCapabilities::CapFullBodyInBounced) {
                    builder.checked_append_reference(body.into_cell())?;
                }
            }
            bounce_msg.set_body(SliceData::load_builder(builder)?);
            if self.config().has_capability(GlobalCapabilities::CapFullBodyInBounced) {
                if let Some(init) = msg.state_init() {
                    bounce_msg.set_state_init(init.clone());
                }
            }
        }

        // calculated storage for bounced message is empty
        let serialized_message = bounce_msg.serialize()?;
        let (storage, fwd_full_fees) = if block_version
            >= VERSION_BLOCK_NEW_CALCULATION_BOUNCED_STORAGE
        {
            let mut storage = StorageUsedShort::default();
            storage.append(&serialized_message);
            let storage_bits = storage.bits() - serialized_message.bit_length() as u64;
            let storage_cells = storage.cells() - 1;
            let fwd_full_fees = self.config().calc_fwd_fee(is_masterchain, &serialized_message)?;
            (StorageUsedShort::with_values_checked(storage_cells, storage_bits)?, fwd_full_fees)
        } else {
            let fwd_full_fees = self.config().calc_fwd_fee(is_masterchain, &Cell::default())?;
            (StorageUsedShort::with_values_checked(0, 0)?, fwd_full_fees)
        };
        let fwd_prices = self.config().get_fwd_prices(is_masterchain);
        let fwd_mine_fees = fwd_prices.mine_fee_checked(&fwd_full_fees)?;
        let fwd_fees = fwd_full_fees - fwd_mine_fees;

        log::debug!(target: "executor", "get fee {} from bounce msg {}", fwd_full_fees, remaining_msg_balance);

        if remaining_msg_balance.grams < fwd_full_fees + *compute_phase_fees {
            log::debug!(
                target: "executor", "bounce phase - not enough grams {} to get fwd fee {}",
                remaining_msg_balance, fwd_full_fees
            );
            return Ok((TrBouncePhase::no_funds(storage, fwd_full_fees), None));
        }

        acc_balance.sub(&remaining_msg_balance)?;
        remaining_msg_balance.grams.sub(&fwd_full_fees)?;
        remaining_msg_balance.grams.sub(compute_phase_fees)?;
        match bounce_msg.header_mut() {
            CommonMsgInfo::IntMsgInfo(header) => {
                header.value = remaining_msg_balance.clone();
                header.fwd_fee = fwd_fees;
            }
            _ => fail!("Error during getting message header"),
        }

        log::debug!(
            target: "executor",
            "bounce fees: {} bounce value: {}",
            fwd_mine_fees, bounce_msg.get_value().unwrap().grams
        );
        tr.add_fee_grams(&fwd_mine_fees)?;
        Ok((TrBouncePhase::ok(storage, fwd_mine_fees, fwd_fees), Some(bounce_msg)))
    }

    fn copyleft_action_handler(
        &self,
        compute_phase_fees: &Grams,
        phase: &mut TrActionPhase,
        actions: &LinkedList<OutAction>,
    ) -> Result<Option<CopyleftReward>> {
        let mut copyleft_reward = Grams::zero();
        let mut copyleft_address = AccountId::default();
        let mut was_copyleft_instruction = false;

        for (i, action) in actions.iter().enumerate() {
            if let OutAction::CopyLeft { license, address } = action {
                if was_copyleft_instruction {
                    fail!("Duplicated copyleft action")
                }
                let copyleft_config = self.config().raw_config().copyleft_config()?;
                phase.spec_actions += 1;
                if let Some(copyleft_percent) = copyleft_config.license_rates.get(license)? {
                    if copyleft_percent >= 100 {
                        fail!(
                            "copyleft percent on license {} is too big {}",
                            license,
                            copyleft_percent
                        )
                    }
                    log::debug!(
                        target: "executor",
                        "Found copyleft action: license: {}, address: {}",
                        license, address
                    );
                    copyleft_reward = (*compute_phase_fees * copyleft_percent as u128) / 100;
                    copyleft_address = address.clone();
                    was_copyleft_instruction = true;
                } else {
                    log::debug!(target: "executor", "Not found license {} in config", license);
                    phase.result_code = RESULT_CODE_NOT_FOUND_LICENSE;
                    phase.result_arg = Some(i as i32);
                    return Ok(None);
                }
            }
        }

        if was_copyleft_instruction && !copyleft_reward.is_zero() {
            Ok(Some(CopyleftReward { reward: copyleft_reward, address: copyleft_address }))
        } else {
            Ok(None)
        }
    }

    fn add_messages(
        &self,
        tr: &mut Transaction,
        out_msgs: Vec<Message>,
        lt: Arc<AtomicU64>,
    ) -> Result<u64> {
        let mut lt = lt.fetch_add(1 + out_msgs.len() as u64, Ordering::Relaxed);
        lt += 1;
        for mut msg in out_msgs {
            msg.set_at_and_lt(tr.now(), lt);
            tr.add_out_message(&msg)?;
            lt += 1;
        }
        Ok(lt)
    }
}

/// Calculate new account state according to inbound message and current account
/// state. If account does not exist - it can be created with uninitialized
/// state. If account is uninitialized - it can be created with active state.
/// If account exists - it can be frozen.
/// Returns computed initial phase.
fn compute_new_state(
    acc: &mut Account,
    acc_balance: &CurrencyCollection,
    in_msg: &Message,
    config: &BlockchainConfig,
) -> Result<Option<ComputeSkipReason>> {
    log::debug!(target: "executor", "compute_account_state");
    let init_code_hash = config.has_capability(GlobalCapabilities::CapInitCodeHash);
    let disable_set_lib = !config.has_capability(GlobalCapabilities::CapSetLibCode);
    match acc.status() {
        AccountStatus::AccStateNonexist => {
            log::error!(target: "executor", "account must exist");
            Ok(Some(if in_msg.state_init().is_none() {
                ComputeSkipReason::NoState
            } else {
                ComputeSkipReason::BadState
            }))
        }
        // Account exists, but can be in different states.
        AccountStatus::AccStateActive => {
            if config.has_capability(GlobalCapabilities::CapSuspendedList) {
                if let Some(suspended_addresses) = config.raw_config().suspended_addresses()? {
                    let addr =
                        acc.get_addr().ok_or_else(|| error!("active account must have address"))?;
                    let wc = addr.workchain_id();
                    let addr = UInt256::construct_from(&mut addr.address())?;

                    if suspended_addresses.is_suspended(wc, addr)? {
                        log::debug!(target: "executor", "account is suspended");
                        return Ok(Some(ComputeSkipReason::Suspended));
                    }
                }
            }

            // account is active, just return it
            log::debug!(target: "executor", "account state: AccountActive");
            Ok(None)
        }
        AccountStatus::AccStateUninit => {
            log::debug!(target: "executor", "AccountUninit");
            if let Some(state_init) = in_msg.state_init() {
                // if msg is a constructor message then
                // borrow code and data from it and switch account state to 'active'.
                log::debug!(target: "executor", "message for uninitialized: activated");
                let text = "Cannot construct account from message with hash";
                if !check_libraries(state_init, disable_set_lib, text, in_msg) {
                    return Ok(Some(ComputeSkipReason::BadState));
                }
                match acc.try_activate_by_init_code_hash(state_init, init_code_hash) {
                    Err(err) => {
                        log::debug!(target: "executor", "reason: {}", err);
                        Ok(Some(ComputeSkipReason::BadState))
                    }
                    Ok(_) => Ok(None),
                }
            } else {
                log::debug!(target: "executor", "message for uninitialized: skip computing phase");
                Ok(Some(ComputeSkipReason::NoState))
            }
        }
        AccountStatus::AccStateFrozen => {
            log::debug!(target: "executor", "AccountFrozen");
            // account balance was credited and if it positive after that
            // and inbound message bear code and data then make some check and unfreeze
            // account
            if !acc_balance.grams.is_zero() {
                // This check is redundant
                if let Some(state_init) = in_msg.state_init() {
                    let text = "Cannot unfreeze account from message with hash";
                    if !check_libraries(state_init, disable_set_lib, text, in_msg) {
                        return Ok(Some(ComputeSkipReason::BadState));
                    }
                    log::debug!(target: "executor", "message for frozen: activated");
                    return match acc.try_activate_by_init_code_hash(state_init, init_code_hash) {
                        Err(err) => {
                            log::debug!(target: "executor", "reason: {}", err);
                            Ok(Some(ComputeSkipReason::BadState))
                        }
                        Ok(_) => Ok(None),
                    };
                }
            }
            // skip computing phase, because account is frozen (bad state)
            log::debug!(target: "executor", "account is frozen (bad state): skip computing phase");
            Ok(Some(ComputeSkipReason::NoState))
        }
    }
}

fn check_replace_src_addr<'a>(
    src: &'a Option<MsgAddressInt>,
    acc_addr: &'a MsgAddressInt,
) -> Option<&'a MsgAddressInt> {
    match src {
        None => Some(acc_addr),
        Some(src) => match src {
            MsgAddressInt::AddrStd(_) => {
                if src != acc_addr {
                    None
                } else {
                    Some(src)
                }
            }
            MsgAddressInt::AddrVar(_) => None,
        },
    }
}

fn is_valid_addr_len(
    addr_len: u16,
    min_addr_len: u16,
    max_addr_len: u16,
    addr_len_step: u16,
) -> bool {
    (addr_len >= min_addr_len)
        && (addr_len <= max_addr_len)
        && ((addr_len == min_addr_len)
            || (addr_len == max_addr_len)
            || ((addr_len_step != 0) && ((addr_len - min_addr_len) % addr_len_step == 0)))
}

fn check_rewrite_dest_addr(
    dst: &MsgAddressInt,
    config: &BlockchainConfig,
    my_addr: &MsgAddressInt,
) -> std::result::Result<MsgAddressInt, IncorrectCheckRewrite> {
    let (anycast_opt, addr_len, workchain_id, address, repack);
    match dst {
        MsgAddressInt::AddrVar(dst) => {
            repack = (dst.addr_len.as_u32() == 256)
                && (dst.workchain_id >= -128)
                && (dst.workchain_id < 128);
            anycast_opt = dst.anycast.clone();
            addr_len = dst.addr_len.as_u16();
            workchain_id = dst.workchain_id;
            address = dst.address.clone();
        }
        MsgAddressInt::AddrStd(dst) => {
            repack = false;
            anycast_opt = dst.anycast.clone();
            addr_len = 256;
            workchain_id = dst.workchain_id as i32;
            address = dst.address.clone();
        }
    }

    let cap_workchains = config.raw_config().has_capability(GlobalCapabilities::CapWorkchains);
    let is_masterchain = workchain_id == MASTERCHAIN_ID;
    if !is_masterchain {
        if !cap_workchains
            && my_addr.workchain_id() != workchain_id
            && my_addr.workchain_id() != MASTERCHAIN_ID
        {
            log::debug!(
                target: "executor",
                "cannot send message from {} to {} it doesn't allow yet",
                my_addr, dst
            );
            return Err(IncorrectCheckRewrite::Other);
        }
        let workchains = config.raw_config().workchains().unwrap_or_default();
        if let Ok(Some(wc)) = workchains.get(&workchain_id) {
            if !wc.accept_msgs {
                log::debug!(
                    target: "executor",
                    "destination address belongs to workchain {} not accepting new messages",
                    workchain_id
                );
                return Err(IncorrectCheckRewrite::Other);
            }
            let (min_addr_len, max_addr_len, addr_len_step) = match wc.format {
                WorkchainFormat::Extended(wf) => {
                    (wf.min_addr_len(), wf.max_addr_len(), wf.addr_len_step())
                }
                WorkchainFormat::Basic(_) => (256, 256, 0),
            };
            if !is_valid_addr_len(addr_len, min_addr_len, max_addr_len, addr_len_step) {
                log::debug!(
                    target: "executor",
                    "destination address has length {} invalid for destination workchain {}",
                    addr_len, workchain_id
                );
                return Err(IncorrectCheckRewrite::Other);
            }
        } else {
            log::debug!(
                target: "executor",
                "destination address contains unknown workchain_id {}",
                workchain_id
            );
            return Err(IncorrectCheckRewrite::Other);
        }
    } else {
        if !cap_workchains
            && my_addr.workchain_id() != MASTERCHAIN_ID
            && my_addr.workchain_id() != BASE_WORKCHAIN_ID
        {
            log::debug!(
                target: "executor",
                "masterchain cannot accept from {} workchain",
                my_addr.workchain_id()
            );
            return Err(IncorrectCheckRewrite::Other);
        }
        if addr_len != 256 {
            log::debug!(
                target: "executor",
                "destination address has length {} invalid for destination workchain {}",
                addr_len, workchain_id
            );
            return Err(IncorrectCheckRewrite::Other);
        }
    }

    if anycast_opt.is_some() {
        log::debug!(target: "executor", "address cannot be anycast");
        return Err(IncorrectCheckRewrite::Anycast);
        // if is_masterchain {
        // log::debug!(target: "executor", "masterchain address cannot be
        // anycast"); return None
        // }
        // match src.address().get_slice(0, anycast.depth.as_usize()) {
        // Ok(pfx) => {
        // if pfx != anycast.rewrite_pfx {
        // match AnycastInfo::with_rewrite_pfx(pfx) {
        // Ok(anycast) => {
        // repack = true;
        // anycast_opt = Some(anycast)
        // }
        // Err(err) => {
        // log::debug!(target: "executor", "Incorrect anycast prefix {}", err);
        // return None
        // }
        // }
        // }
        // }
        // Err(err) => {
        // log::debug!(target: "executor", "Incorrect src address {}", err);
        // return None
        // }
        // }
    }

    if !repack {
        Ok(dst.clone())
    } else if addr_len == 256 && (-128..128).contains(&workchain_id) {
        // repack as an addr_std
        MsgAddressInt::with_standart(anycast_opt, workchain_id as i8, address)
            .map_err(|_| IncorrectCheckRewrite::Other)
    } else {
        // repack as an addr_var
        MsgAddressInt::with_variant(anycast_opt, workchain_id, address)
            .map_err(|_| IncorrectCheckRewrite::Other)
    }
}

fn outmsg_action_handler(
    phase: &mut TrActionPhase,
    mut mode: u8,
    msg: &mut Message,
    acc_balance: &mut CurrencyCollection,
    msg_balance: &mut CurrencyCollection,
    compute_phase_fees: &Grams,
    config: &BlockchainConfig,
    is_special: bool,
    my_addr: &MsgAddressInt,
    reserved_value: &CurrencyCollection,
    account_deleted: &mut bool,
) -> std::result::Result<CurrencyCollection, i32> {
    // we cannot send all balance from account and from message simultaneously ?
    let invalid_flags = SENDMSG_REMAINING_MSG_BALANCE | SENDMSG_ALL_BALANCE;
    if (mode & !SENDMSG_VALID_FLAGS) != 0
        || (mode & invalid_flags) == invalid_flags
        || ((mode & SENDMSG_DELETE_IF_EMPTY) != 0 && (mode & SENDMSG_ALL_BALANCE) == 0)
    {
        log::error!(target: "executor", "outmsg mode has unsupported flags");
        return Err(RESULT_CODE_UNSUPPORTED);
    }
    let skip = if (mode & SENDMSG_IGNORE_ERROR) != 0 { None } else { Some(()) };
    let (fwd_mine_fee, total_fwd_fees);
    let mut result_value; // to sub from acc_balance

    if let Some(new_src) = check_replace_src_addr(&msg.src(), my_addr) {
        msg.set_src_address(new_src.clone());
    } else {
        log::warn!(target: "executor", "Incorrect source address {:?}", msg.src());
        return Err(RESULT_CODE_INCORRECT_SRC_ADDRESS);
    }

    let fwd_prices = config.get_fwd_prices(msg.is_masterchain());
    let compute_fwd_fee = if is_special {
        Grams::default()
    } else {
        msg
            .serialize()
            .and_then(|cell| config.calc_fwd_fee(msg.is_masterchain(), &cell))
            .map_err(|err| {
                log::error!(target: "executor", "cannot serialize message in action phase : {}", err);
                RESULT_CODE_ACTIONLIST_INVALID
            })?
    };

    if let Some(int_header) = msg.int_header_mut() {
        match check_rewrite_dest_addr(&int_header.dst, config, my_addr) {
            Ok(new_dst) => int_header.dst = new_dst,
            Err(type_error) => {
                if type_error == IncorrectCheckRewrite::Anycast {
                    log::warn!(target: "executor", "Incorrect destination anycast address {}", int_header.dst);
                    return Err(skip.map(|_| RESULT_CODE_ANYCAST).unwrap_or_default());
                } else {
                    log::warn!(target: "executor", "Incorrect destination address {}", int_header.dst);
                    return Err(skip
                        .map(|_| RESULT_CODE_INCORRECT_DST_ADDRESS)
                        .unwrap_or_default());
                }
            }
        }

        int_header.bounced = false;
        result_value = int_header.value.clone();

        if cfg!(feature = "ihr_disabled") {
            int_header.ihr_disabled = true;
        }
        if !int_header.ihr_disabled {
            let compute_ihr_fee = fwd_prices
                .ihr_fee_checked(&compute_fwd_fee)
                .map_err(|_| RESULT_CODE_UNSUPPORTED)?;
            if int_header.ihr_fee < compute_ihr_fee {
                int_header.ihr_fee = compute_ihr_fee
            }
        } else {
            int_header.ihr_fee = Grams::zero();
        }
        let fwd_fee = *std::cmp::max(&int_header.fwd_fee, &compute_fwd_fee);
        fwd_mine_fee =
            fwd_prices.mine_fee_checked(&fwd_fee).map_err(|_| RESULT_CODE_UNSUPPORTED)?;
        total_fwd_fees = fwd_fee + int_header.ihr_fee;

        let fwd_remain_fee = fwd_fee - fwd_mine_fee;
        if (mode & SENDMSG_ALL_BALANCE) != 0 {
            // send all remaining account balance
            result_value = acc_balance.clone();
            int_header.value = acc_balance.clone();

            mode &= !SENDMSG_PAY_FEE_SEPARATELY;
        }
        if (mode & SENDMSG_REMAINING_MSG_BALANCE) != 0 {
            // send all remainig balance of inbound message
            result_value.add(msg_balance).ok();
            if (mode & SENDMSG_PAY_FEE_SEPARATELY) == 0 {
                if &result_value.grams < compute_phase_fees {
                    return Err(skip.map(|_| RESULT_CODE_NOT_ENOUGH_GRAMS).unwrap_or_default());
                }
                result_value.grams.sub(compute_phase_fees).map_err(|err| {
                    log::error!(target: "executor", "cannot subtract msg balance : {}", err);
                    RESULT_CODE_ACTIONLIST_INVALID
                })?;
            }
            int_header.value = result_value.clone();
        }
        if (mode & SENDMSG_PAY_FEE_SEPARATELY) != 0 {
            // we must pay the fees, sum them with msg value
            result_value.grams += total_fwd_fees;
        } else if int_header.value.grams < total_fwd_fees {
            // msg value is too small, reciever cannot pay the fees
            log::warn!(
                target: "executor",
                "msg balance {} is too small, cannot pay fwd+ihr fees: {}",
                int_header.value.grams, total_fwd_fees
            );
            return Err(skip.map(|_| RESULT_CODE_NOT_ENOUGH_GRAMS).unwrap_or_default());
        } else {
            // reciever will pay the fees
            int_header.value.grams -= total_fwd_fees;
        }

        // set evaluated fees and value back to msg
        int_header.fwd_fee = fwd_remain_fee;
    } else if msg.ext_out_header().is_some() {
        fwd_mine_fee = compute_fwd_fee;
        total_fwd_fees = compute_fwd_fee;
        result_value = CurrencyCollection::from_grams(compute_fwd_fee);
    } else {
        return Err(-1);
    }

    if acc_balance.grams < result_value.grams {
        log::warn!(
            target: "executor",
            "account balance {} is too small, cannot send {}", acc_balance.grams, result_value.grams
        );
        return Err(skip.map(|_| RESULT_CODE_NOT_ENOUGH_GRAMS).unwrap_or_default());
    }
    match acc_balance.sub(&result_value) {
        Ok(false) | Err(_) => {
            log::warn!(
                target: "executor",
                "account balance {} is too small, cannot send {}", acc_balance, result_value
            );
            return Err(skip.map(|_| RESULT_CODE_NOT_ENOUGH_EXTRA).unwrap_or_default());
        }
        _ => (),
    }
    let mut acc_balance_copy = ExtraCurrencyCollection::default();
    let predicate = |key: u32, b: tvm_block::VarUInteger32| -> Result<bool> {
        if !b.is_zero() {
            acc_balance_copy.set(&key, &b)?;
        }
        Ok(true)
    };
    match acc_balance.other.iterate_with_keys(predicate) {
        Ok(false) | Err(_) => {
            log::warn!(target: "executor", "Cannot reduce account extra balance");
            return Err(skip.map(|_| RESULT_CODE_INVALID_BALANCE).unwrap_or_default());
        }
        _ => (),
    }
    std::mem::swap(&mut acc_balance.other, &mut acc_balance_copy);

    if (mode & SENDMSG_DELETE_IF_EMPTY) != 0
        && (mode & SENDMSG_ALL_BALANCE) != 0
        && acc_balance.grams.is_zero()
        && reserved_value.grams.is_zero()
    {
        *account_deleted = true;
    }

    // total fwd fees is sum of messages full fwd and ihr fees
    phase.add_fwd_fees(total_fwd_fees);

    // total action fees is sum of messages fwd mine fees
    phase.add_action_fees(fwd_mine_fee);

    let msg_cell = msg.serialize().map_err(|err| {
        log::error!(target: "executor", "cannot serialize message in action phase : {}", err);
        RESULT_CODE_ACTIONLIST_INVALID
    })?;
    phase.tot_msg_size.append(&msg_cell);

    if phase.tot_msg_size.bits() as usize > MAX_MSG_BITS
        || phase.tot_msg_size.cells() as usize > MAX_MSG_CELLS
    {
        log::warn!(target: "executor", "message too large : bits: {}, cells: {}", phase.tot_msg_size.bits(), phase.tot_msg_size.cells());
        return Err(RESULT_CODE_INVALID_BALANCE);
    }

    if (mode & (SENDMSG_ALL_BALANCE | SENDMSG_REMAINING_MSG_BALANCE)) != 0 {
        *msg_balance = CurrencyCollection::default();
    }

    log::debug!(target: "executor", "Message details:\n\tFlag: {}\n\tValue: {}\n\tSource: {}\n\tDestination: {}\n\tBody: {}\n\tStateInit: {}",
        mode,
        balance_to_string(&result_value),
        msg.src().map_or("None".to_string(), |addr| addr.to_string()),
        msg.dst().map_or("None".to_string(), |addr| addr.to_string()),
        msg.body().map_or("None".to_string(), |data| data.to_string()),
        msg.state_init().map_or("None".to_string(), |_| "Present".to_string())
    );

    Ok(result_value)
}

/// Reserves some grams from accout balance.
/// Returns calculated reserved value. its calculation depends on mode.
/// Reduces balance by the amount of the reserved value.
fn reserve_action_handler(
    mode: u8,
    val: &CurrencyCollection,
    original_acc_balance: &CurrencyCollection,
    acc_remaining_balance: &mut CurrencyCollection,
) -> std::result::Result<CurrencyCollection, i32> {
    if mode & !RESERVE_VALID_MODES != 0 {
        return Err(RESULT_CODE_UNKNOWN_OR_INVALID_ACTION);
    }
    log::debug!(target: "executor", "Reserve with mode = {} value = {}", mode, balance_to_string(val));

    let mut reserved;
    if mode & RESERVE_PLUS_ORIG != 0 {
        // Append all currencies
        if mode & RESERVE_REVERSE != 0 {
            reserved = original_acc_balance.clone();
            let result = reserved.sub(val);
            match result {
                Err(_) => return Err(RESULT_CODE_INVALID_BALANCE),
                Ok(false) => return Err(RESULT_CODE_UNSUPPORTED),
                Ok(true) => (),
            }
        } else {
            reserved = val.clone();
            reserved.add(original_acc_balance).or(Err(RESULT_CODE_INVALID_BALANCE))?;
        }
    } else {
        if mode & RESERVE_REVERSE != 0 {
            // flag 8 without flag 4 unacceptable
            return Err(RESULT_CODE_UNKNOWN_OR_INVALID_ACTION);
        }
        reserved = val.clone();
    }
    if mode & RESERVE_IGNORE_ERROR != 0 {
        // Only grams
        reserved.grams = min(reserved.grams, acc_remaining_balance.grams);
    }

    let mut remaining = acc_remaining_balance.clone();
    if remaining.grams.as_u128() < reserved.grams.as_u128() {
        return Err(RESULT_CODE_NOT_ENOUGH_GRAMS);
    }
    let result = remaining.sub(&reserved);
    match result {
        Err(_) => return Err(RESULT_CODE_INVALID_BALANCE),
        Ok(false) => return Err(RESULT_CODE_NOT_ENOUGH_EXTRA),
        Ok(true) => (),
    }
    std::mem::swap(&mut remaining, acc_remaining_balance);

    if mode & RESERVE_ALL_BUT != 0 {
        // swap all currencies
        std::mem::swap(&mut reserved, acc_remaining_balance);
    }

    Ok(reserved)
}

fn setcode_action_handler(acc: &mut Account, code: Cell) -> Option<i32> {
    log::debug!(target: "executor", "OutAction::SetCode\nPrevious code hash: {:x}\nNew code hash:      {:x}",
        acc.get_code().unwrap_or_default().repr_hash(),
        code.repr_hash(),
    );
    match acc.set_code(code) {
        true => None,
        false => Some(RESULT_CODE_BAD_ACCOUNT_STATE),
    }
}

fn change_library_action_handler(
    acc: &mut Account,
    mode: u8,
    code: Option<Cell>,
    hash: Option<UInt256>,
) -> Option<i32> {
    let result = match (code, hash) {
        (Some(code), None) => {
            log::debug!(target: "executor", "OutAction::ChangeLibrary mode: {}, code: {}", mode, code);
            if mode == 0 {
                // TODO: Wrong codes. Look tvm_block/out_actions::SET_LIB_CODE_REMOVE
                acc.delete_library(&code.repr_hash())
            } else {
                acc.set_library(code, (mode & 2) == 2)
            }
        }
        (None, Some(hash)) => {
            log::debug!(target: "executor", "OutAction::ChangeLibrary mode: {}, hash: {:x}", mode, hash);
            if mode == 0 {
                acc.delete_library(&hash)
            } else {
                acc.set_library_flag(&hash, (mode & 2) == 2)
            }
        }
        _ => false,
    };
    match result {
        true => None,
        false => Some(RESULT_CODE_BAD_ACCOUNT_STATE),
    }
}

fn init_gas(
    acc_balance: u128,
    msg_balance: u128,
    is_external: bool,
    is_special: bool,
    is_ordinary: bool,
    gas_info: &GasLimitsPrices,
) -> Gas {
    let gas_max = if is_special {
        gas_info.special_gas_limit
    } else {
        min(
            (1 << (7 * 8)) - 1, // because gas_limit is stored as VarUInteger7
            min(gas_info.gas_limit, gas_info.calc_gas(acc_balance)),
        )
    };
    let mut gas_credit = 0;
    let gas_limit = if !is_ordinary {
        gas_max
    } else {
        if is_external {
            gas_credit = min(
                (1 << (3 * 8)) - 1, // because gas_credit is stored as VarUInteger3
                min(gas_info.gas_credit, gas_max),
            );
        }
        min(gas_max, gas_info.calc_gas(msg_balance))
    };
    log::debug!(
        target: "executor",
        "gas before: gm: {}, gl: {}, gc: {}, price: {}",
        gas_max, gas_limit, gas_credit, gas_info.get_real_gas_price()
    );
    Gas::new(
        gas_limit as i64,
        gas_credit as i64,
        gas_max as i64,
        gas_info.get_real_gas_price() as i64,
    )
}

fn check_libraries(init: &StateInit, disable_set_lib: bool, text: &str, msg: &Message) -> bool {
    match init.libraries().len() {
        Ok(len) => {
            if !disable_set_lib || len == 0 {
                true
            } else {
                log::trace!(
                    target: "executor",
                    "{} {:x} because libraries are disabled",
                        text, msg.hash().unwrap_or_default()
                );
                false
            }
        }
        Err(err) => {
            log::trace!(
                target: "executor",
                "{} {:x} because libraries are broken {}",
                    text, msg.hash().unwrap_or_default(), err
            );
            false
        }
    }
}

/// Calculate new account according to inbound message.
/// If message has no value, account will not created.
/// If hash of state_init is equal to account address (or flag check address is
/// false), account will be active. Otherwise, account will be nonexist or
/// uninit according bounce flag: if bounce, account will be uninit that save
/// money.
fn account_from_message(
    msg: &Message,
    msg_remaining_balance: &CurrencyCollection,
    check_address: bool,
    init_code_hash: bool,
    disable_set_lib: bool,
) -> Option<Account> {
    let hdr = msg.int_header()?;
    if let Some(init) = msg.state_init() {
        if init.code().is_some() {
            if !check_address || (init.hash().ok()? == hdr.dst.address()) {
                let text = "Cannot construct account from message with hash";
                if check_libraries(init, disable_set_lib, text, msg) {
                    return Account::active_by_init_code_hash(
                        hdr.dst.clone(),
                        msg_remaining_balance.clone(),
                        0,
                        init.clone(),
                        init_code_hash,
                    )
                    .ok();
                }
            } else if check_address {
                log::trace!(
                    target: "executor",
                    "Cannot construct account from message with hash {:x} \
                        because the destination address does not math with hash message code",
                        msg.hash().unwrap_or_default()
                );
            }
        }
    }
    if hdr.bounce {
        log::trace!(
            target: "executor",
            "Account will not be created. Value of {:x} message will be returned",
            msg.hash().unwrap_or_default()
        );
        None
    } else {
        Some(Account::uninit(hdr.dst.clone(), 0, 0, msg_remaining_balance.clone()))
    }
}

fn balance_to_string(balance: &CurrencyCollection) -> String {
    let value = balance.grams.as_u128();
    format!(
        "{}.{:03} {:03} {:03}      ({})",
        value / 1e9 as u128,
        (value % 1e9 as u128) / 1e6 as u128,
        (value % 1e6 as u128) / 1e3 as u128,
        value % 1e3 as u128,
        value,
    )
}

fn action_type(action: &OutAction) -> String {
    match action {
        OutAction::SendMsg { mode: _, out_msg: _ } => "SendMsg".to_string(),
        OutAction::SetCode { new_code: _ } => "SetCode".to_string(),
        OutAction::ReserveCurrency { mode: _, value: _ } => "ReserveCurrency".to_string(),
        OutAction::ChangeLibrary { mode: _, code: _, hash: _ } => "ChangeLibrary".to_string(),
        _ => "Unknown".to_string(),
    }
}
