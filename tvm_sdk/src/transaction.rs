// Copyright 2018-2021 TON Labs LTD.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_block::AccStatusChange;
use tvm_block::ComputeSkipReason;
use tvm_block::GetRepresentationHash;
use tvm_block::TrComputePhase;
use tvm_block::TransactionDescr;
use tvm_block::TransactionProcessingStatus;
use tvm_types::Result;

use crate::Message;
use crate::MessageId;
use crate::error::SdkError;
use crate::json_helper;
use crate::types::StringId;
use crate::types::grams_to_u64;

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct ComputePhase {
    #[serde(deserialize_with = "json_helper::deserialize_skipped_reason")]
    pub skipped_reason: Option<ComputeSkipReason>,
    pub exit_code: Option<i32>,
    pub exit_arg: Option<i32>,
    pub success: Option<bool>,
    #[serde(with = "json_helper::uint")]
    pub gas_fees: u64,
    #[serde(with = "json_helper::uint")]
    pub gas_used: u64,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct StoragePhase {
    #[serde(deserialize_with = "json_helper::deserialize_acc_state_change")]
    pub status_change: AccStatusChange,
    #[serde(with = "json_helper::uint")]
    pub storage_fees_collected: u64,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct ActionPhase {
    pub success: bool,
    pub valid: bool,
    pub no_funds: bool,
    pub result_code: i32,
    #[serde(with = "json_helper::uint")]
    pub total_fwd_fees: u64,
    #[serde(with = "json_helper::uint")]
    pub total_action_fees: u64,
}

pub type TransactionId = StringId;

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Transaction {
    pub id: TransactionId,
    #[serde(deserialize_with = "json_helper::deserialize_tr_state")]
    pub status: TransactionProcessingStatus,
    pub now: u32,
    pub in_msg: Option<MessageId>,
    pub out_msgs: Vec<MessageId>,
    pub out_messages: Vec<Message>,
    pub aborted: bool,
    pub compute: ComputePhase,
    pub storage: Option<StoragePhase>,
    pub action: Option<ActionPhase>,
    #[serde(with = "json_helper::uint")]
    pub total_fees: u64,
}

impl TryFrom<&tvm_block::Transaction> for Transaction {
    type Error = tvm_types::Error;

    fn try_from(transaction: &tvm_block::Transaction) -> Result<Self> {
        let descr = if let TransactionDescr::Ordinary(descr) = transaction.read_description()? {
            descr
        } else {
            return Err(SdkError::InvalidData { msg: "Invalid transaction type".to_owned() }.into());
        };

        let storage_phase = if let Some(phase) = descr.storage_ph {
            Some(StoragePhase {
                status_change: phase.status_change,
                storage_fees_collected: grams_to_u64(&phase.storage_fees_collected)?,
            })
        } else {
            None
        };

        let compute_phase = match descr.compute_ph {
            TrComputePhase::Skipped(ph) => ComputePhase {
                skipped_reason: Some(ph.reason),
                exit_code: None,
                exit_arg: None,
                success: None,
                gas_fees: 0,
                gas_used: 0,
            },
            TrComputePhase::Vm(ph) => ComputePhase {
                skipped_reason: None,
                exit_code: Some(ph.exit_code),
                exit_arg: ph.exit_arg,
                success: Some(ph.success),
                gas_fees: grams_to_u64(&ph.gas_fees)?,
                gas_used: ph.gas_used.as_u64(),
            },
        };

        let action_phase = if let Some(phase) = descr.action {
            Some(ActionPhase {
                success: phase.success,
                valid: phase.valid,
                no_funds: phase.no_funds,
                result_code: phase.result_code,
                total_fwd_fees: grams_to_u64(&phase.total_fwd_fees.unwrap_or_default())?,
                total_action_fees: grams_to_u64(&phase.total_action_fees.unwrap_or_default())?,
            })
        } else {
            None
        };

        let in_msg = transaction.in_msg.as_ref().map(|msg| msg.hash().into());
        let mut out_msgs = vec![];
        transaction.out_msgs.iterate_slices(|slice| {
            if let Ok(cell) = slice.reference(0) {
                out_msgs.push(cell.repr_hash().into());
            }
            Ok(true)
        })?;
        let mut out_messages = vec![];
        transaction.out_msgs.iterate(|msg| {
            out_messages.push(Message::with_msg(&msg.0)?);
            Ok(true)
        })?;

        Ok(Transaction {
            id: transaction.hash()?.into(),
            status: TransactionProcessingStatus::Finalized,
            now: transaction.now(),
            in_msg,
            out_msgs,
            out_messages,
            aborted: descr.aborted,
            total_fees: grams_to_u64(&transaction.total_fees().grams)?,
            storage: storage_phase,
            compute: compute_phase,
            action: action_phase,
        })
    }
}

#[derive(Serialize, Deserialize, ApiType, Debug, PartialEq, Clone, Default)]
pub struct TransactionFees {
    /// Deprecated. Contains the same data as ext_in_msg_fee field
    pub in_msg_fwd_fee: u64,
    /// Fee for account storage
    pub storage_fee: u64,
    /// Fee for processing
    pub gas_fee: u64,
    /// Deprecated. Contains the same data as total_fwd_fees field. Deprecated
    /// because of its confusing name, that is not the same with GraphQL API
    /// Transaction type's field.
    pub out_msgs_fwd_fee: u64,
    /// Deprecated. Contains the same data as account_fees field
    pub total_account_fees: u64,
    /// Deprecated because it means total value sent in the transaction, which
    /// does not relate to any fees.
    pub total_output: u64,
    /// Fee for inbound external message import.
    pub ext_in_msg_fee: u64,
    /// Total fees the account pays for message forwarding
    pub total_fwd_fees: u64,
    /// Total account fees for the transaction execution.
    /// Compounds of storage_fee + gas_fee + ext_in_msg_fee + total_fwd_fees
    pub account_fees: u64,
}

// The struct represents performed transaction and allows to access their
// properties.
impl Transaction {
    // Returns transaction's processing status
    pub fn status(&self) -> TransactionProcessingStatus {
        self.status
    }

    // Returns id of transaction's input message if it exists
    pub fn in_message_id(&self) -> Option<MessageId> {
        self.in_msg.clone()
    }

    // Returns id of transaction's out messages if it exists
    pub fn out_messages_id(&self) -> &Vec<MessageId> {
        &self.out_msgs
    }

    // Returns message's identifier
    pub fn id(&self) -> TransactionId {
        // On client side id is ready allways. It is never be calculated, just returned.
        self.id.clone()
    }

    // Returns `aborted` flag
    pub fn is_aborted(&self) -> bool {
        self.aborted
    }

    pub fn calc_fees(&self) -> TransactionFees {
        let mut fees = TransactionFees { gas_fee: self.compute.gas_fees, ..Default::default() };

        if let Some(storage) = &self.storage {
            fees.storage_fee = storage.storage_fees_collected;
        }
        let mut total_action_fees = 0;
        if let Some(action) = &self.action {
            fees.out_msgs_fwd_fee = action.total_fwd_fees;
            fees.total_fwd_fees = action.total_fwd_fees;
            total_action_fees = action.total_action_fees;
        }
        // `transaction.total_fees` is calculated as
        // `transaction.total_fees = inbound_fwd_fees + storage_fees + gas_fees +
        // total_action_fees` but this total_fees is fees collected the
        // validators, not the all fees taken from account
        // because total_action_fees contains only part of all forward fees
        // to get all fees paid by account we need exchange `total_action_fees part` to
        // `out_msgs_fwd_fee`
        let total_account_fees =
            self.total_fees as i128 - total_action_fees as i128 + fees.out_msgs_fwd_fee as i128;
        fees.total_account_fees =
            if total_account_fees > 0 { total_account_fees as u64 } else { 0 };
        // inbound_fwd_fees is not represented in transaction fields so need to
        // calculate it
        let in_msg_fwd_fee = fees.total_account_fees as i128
            - fees.storage_fee as i128
            - fees.gas_fee as i128
            - fees.out_msgs_fwd_fee as i128;
        fees.in_msg_fwd_fee = if in_msg_fwd_fee > 0 { in_msg_fwd_fee as u64 } else { 0 };

        let total_output = self.out_messages.iter().fold(0u128, |acc, msg| acc + msg.value as u128);
        fees.total_output = if total_output <= u64::MAX as u128 { total_output as u64 } else { 0 };

        fees.ext_in_msg_fee = fees.in_msg_fwd_fee;
        fees.account_fees = fees.total_account_fees;
        fees
    }
}

#[cfg(test)]
mod tests {
    use tvm_block::AccStatusChange;
    use tvm_block::AccountStatus;
    use tvm_block::ComputeSkipReason;
    use tvm_block::CurrencyCollection;
    use tvm_block::ExtOutMessageHeader;
    use tvm_block::ExternalInboundMessageHeader;
    use tvm_block::GetRepresentationHash;
    use tvm_block::HashUpdate;
    use tvm_block::InternalMessageHeader;
    use tvm_block::Message as TvmMessage;
    use tvm_block::MsgAddressExt;
    use tvm_block::MsgAddressInt;
    use tvm_block::TrActionPhase;
    use tvm_block::TrComputePhase;
    use tvm_block::TrComputePhaseVm;
    use tvm_block::TrStoragePhase;
    use tvm_block::TransactionDescr;
    use tvm_block::TransactionDescrOrdinary;
    use tvm_block::TransactionDescrTickTock;
    use tvm_block::VarUInteger7;
    use tvm_types::AccountId;
    use tvm_types::SliceData;

    use super::*;
    use crate::MessageType;

    fn std_addr(byte: u8) -> MsgAddressInt {
        MsgAddressInt::with_standart(None, 0, AccountId::from([byte; 32])).unwrap()
    }

    fn ext_addr(byte: u8) -> MsgAddressExt {
        MsgAddressExt::with_extern(SliceData::new(vec![byte])).unwrap()
    }

    fn internal_message(value: u64, body: &[u8]) -> TvmMessage {
        let mut msg = TvmMessage::with_int_header(InternalMessageHeader::with_addresses(
            std_addr(1),
            std_addr(2),
            CurrencyCollection::with_grams(value),
        ));
        if !body.is_empty() {
            msg.set_body(SliceData::new(body.to_vec()));
        }
        msg
    }

    fn build_transaction(description: TransactionDescr) -> tvm_block::Transaction {
        let mut tx = tvm_block::Transaction::with_address_and_status(
            AccountId::from([9; 32]),
            AccountStatus::AccStateActive,
        );
        tx.set_total_fees(CurrencyCollection::with_grams(60));
        tx.write_in_msg(Some(&internal_message(1, &[0x11]))).unwrap();
        tx.add_out_message(&internal_message(7, &[0x22])).unwrap();
        tx.write_state_update(&HashUpdate::default()).unwrap();
        tx.write_description(&description).unwrap();
        tx
    }

    #[test]
    fn try_from_rejects_non_ordinary_transactions() {
        let tx = build_transaction(TransactionDescr::TickTock(TransactionDescrTickTock::default()));

        let err = Transaction::try_from(&tx).unwrap_err();

        assert_eq!(err.to_string(), "Invalid data: Invalid transaction type");
    }

    #[test]
    fn try_from_maps_skipped_compute_and_missing_optional_phases() {
        let tx = tvm_block::generate_tranzaction(AccountId::from([5; 32]));

        let transaction = Transaction::try_from(&tx).unwrap();

        assert_eq!(transaction.status(), TransactionProcessingStatus::Finalized);
        assert_eq!(transaction.compute.skipped_reason, Some(ComputeSkipReason::NoState));
        assert_eq!(transaction.compute.exit_code, None);
        assert_eq!(transaction.compute.exit_arg, None);
        assert_eq!(transaction.compute.success, None);
        assert_eq!(transaction.compute.gas_fees, 0);
        assert_eq!(transaction.compute.gas_used, 0);
        assert!(transaction.storage.is_none());
        assert!(transaction.action.is_none());
        assert!(transaction.in_message_id().is_some());
        assert_eq!(transaction.out_messages_id().len(), 3);
        assert_eq!(transaction.out_messages.len(), 3);
    }

    #[test]
    fn try_from_maps_vm_storage_action_and_message_ids() {
        let mut ordinary = TransactionDescrOrdinary::default();
        ordinary.aborted = true;
        ordinary.storage_ph =
            Some(TrStoragePhase::with_params(11u64.into(), None, AccStatusChange::Frozen));
        ordinary.compute_ph = TrComputePhase::Vm(TrComputePhaseVm {
            success: true,
            gas_fees: 13u64.into(),
            gas_used: VarUInteger7::from(21),
            exit_code: 17,
            exit_arg: Some(-3),
            ..Default::default()
        });
        ordinary.action = Some(TrActionPhase {
            success: true,
            valid: false,
            no_funds: true,
            result_code: -9,
            total_fwd_fees: Some(5u64.into()),
            total_action_fees: Some(7u64.into()),
            ..Default::default()
        });

        let mut tx = build_transaction(TransactionDescr::Ordinary(ordinary));
        let out_ext = TvmMessage::with_ext_out_header(ExtOutMessageHeader::with_addresses(
            std_addr(7),
            ext_addr(0xee),
        ));
        tx.add_out_message(&out_ext).unwrap();

        let transaction = Transaction::try_from(&tx).unwrap();

        assert!(transaction.is_aborted());
        assert_eq!(transaction.storage.as_ref().unwrap().status_change, AccStatusChange::Frozen);
        assert_eq!(transaction.storage.as_ref().unwrap().storage_fees_collected, 11);
        assert_eq!(transaction.compute.skipped_reason, None);
        assert_eq!(transaction.compute.exit_code, Some(17));
        assert_eq!(transaction.compute.exit_arg, Some(-3));
        assert_eq!(transaction.compute.success, Some(true));
        assert_eq!(transaction.compute.gas_fees, 13);
        assert_eq!(transaction.compute.gas_used, 21);
        assert_eq!(transaction.action.as_ref().unwrap().success, true);
        assert_eq!(transaction.action.as_ref().unwrap().valid, false);
        assert_eq!(transaction.action.as_ref().unwrap().no_funds, true);
        assert_eq!(transaction.action.as_ref().unwrap().result_code, -9);
        assert_eq!(transaction.action.as_ref().unwrap().total_fwd_fees, 5);
        assert_eq!(transaction.action.as_ref().unwrap().total_action_fees, 7);
        assert_eq!(transaction.out_messages.len(), 2);
        assert_eq!(transaction.out_messages[0].value, 7);
        assert_eq!(transaction.out_messages[1].msg_type(), MessageType::ExternalOutbound);
        assert_eq!(
            transaction.out_msgs[0].to_string(),
            hex::encode(tx.get_out_msg(0).unwrap().unwrap().hash().unwrap().as_slice())
        );
    }

    #[test]
    fn calc_fees_handles_positive_and_clamped_values() {
        let transaction = Transaction {
            id: "aa".into(),
            status: TransactionProcessingStatus::Finalized,
            now: 0,
            in_msg: Some("bb".into()),
            out_msgs: vec!["cc".into()],
            out_messages: vec![
                Message {
                    id: "01".into(),
                    body: None,
                    msg_type: MessageType::Internal,
                    value: u64::MAX,
                },
                Message { id: "02".into(), body: None, msg_type: MessageType::Internal, value: 1 },
            ],
            aborted: false,
            compute: ComputePhase { gas_fees: 20, ..Default::default() },
            storage: Some(StoragePhase {
                status_change: AccStatusChange::Unchanged,
                storage_fees_collected: 10,
            }),
            action: Some(ActionPhase {
                total_fwd_fees: 5,
                total_action_fees: 100,
                ..Default::default()
            }),
            total_fees: 60,
        };

        let fees = transaction.calc_fees();

        assert_eq!(fees.gas_fee, 20);
        assert_eq!(fees.storage_fee, 10);
        assert_eq!(fees.out_msgs_fwd_fee, 5);
        assert_eq!(fees.total_fwd_fees, 5);
        assert_eq!(fees.total_account_fees, 0);
        assert_eq!(fees.in_msg_fwd_fee, 0);
        assert_eq!(fees.ext_in_msg_fee, 0);
        assert_eq!(fees.account_fees, 0);
        assert_eq!(fees.total_output, 0);
    }

    #[test]
    fn message_helpers_support_external_input_fixture() {
        let ext_in = TvmMessage::with_ext_in_header(ExternalInboundMessageHeader::new(
            ext_addr(0xaa),
            std_addr(8),
        ));

        let message = Message::with_msg(&ext_in).unwrap();

        assert_eq!(message.msg_type(), MessageType::ExternalInbound);
        assert_eq!(message.value, 0);
    }
}
