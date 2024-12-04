use std::collections::HashMap;
use std::time::SystemTime;

use serde_json::Map;
use serde_json::Value;
use tvm_block::CommonMsgInfo;
use tvm_block::Deserializable;
use tvm_block::Message;
use tvm_block::MessageProcessingStatus;
use tvm_block::MsgAddressExt;
use tvm_block::Transaction;
use tvm_block::TransactionProcessingStatus;
use tvm_types::write_boc;
use tvm_types::Cell;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::UInt256;

use crate::block_parser::entry::get_sharding_depth;
use crate::block_parser::get_partition;
use crate::block_parser::is_minter_address;
use crate::BlockParserConfig;
use crate::BlockParsingError;
use crate::EntryConfig;
use crate::JsonReducer;
use crate::ParsedEntry;
use crate::ParserTraceEvent;
use crate::ParserTracer;
use crate::ParsingBlock;

pub struct PreparedMessage {
    doc: Map<String, Value>,
    src_partition: Option<u32>,
    dst_partition: Option<u32>,
}

struct MessageAdditionalFields {
    transaction_id: &'static str,
    chain_order: &'static str,
    code_hash: &'static str,
}

impl MessageAdditionalFields {
    const DST: Self = Self {
        transaction_id: "dst_transaction_id",
        chain_order: "dst_chain_order",
        code_hash: "dst_code_hash",
    };
    const SRC: Self = Self {
        transaction_id: "src_transaction_id",
        chain_order: "src_chain_order",
        code_hash: "src_code_hash",
    };
}

impl PreparedMessage {
    fn set_additional_fields(
        &mut self,
        fields: &MessageAdditionalFields,
        index: u64,
        tr_id: &UInt256,
        tr_chain_order: &Option<&str>,
        tr_code_hash: &Option<String>,
    ) {
        self.doc.insert(fields.transaction_id.to_owned(), tr_id.as_hex_string().into());
        if let Some(tr_chain_order) = tr_chain_order {
            self.doc.insert(
                fields.chain_order.to_owned(),
                format!("{}{}", tr_chain_order, crate::u64_to_string(index)).into(),
            );
        }
        if let Some(tr_code_hash) = tr_code_hash {
            self.doc.insert(fields.code_hash.to_owned(), tr_code_hash.clone().into());
        }
    }
}

pub(crate) struct ParserTransactions<'a, T: ParserTracer, R: JsonReducer> {
    parsing: &'a ParsingBlock<'a>,
    transactions_config: &'a Option<EntryConfig<R>>,
    messages_config: &'a Option<EntryConfig<R>>,
    transactions_sharding_depth: u32,
    messages_sharding_depth: u32,
    with_proofs: bool,
    tracer: &'a Option<T>,
}

impl<'a, T: ParserTracer, R: JsonReducer> ParserTransactions<'a, T, R> {
    pub(crate) fn new(
        config: &'a BlockParserConfig<R>,
        tracer: &'a Option<T>,
        parsing: &'a ParsingBlock,
        with_proofs: bool,
    ) -> Self {
        Self {
            parsing,
            transactions_config: &config.transactions,
            messages_config: &config.messages,
            transactions_sharding_depth: get_sharding_depth(&config.transactions),
            messages_sharding_depth: get_sharding_depth(&config.messages),
            with_proofs,
            tracer,
        }
    }

    pub(crate) fn parse_messages_from_transaction(
        &self,
        transaction: &Transaction,
        transaction_id: UInt256,
        transaction_order: Option<&str>,
        code_hash: &Option<String>,
        prepared_messages: &mut HashMap<UInt256, PreparedMessage>,
    ) -> Result<()> {
        let now = std::time::Instant::now();

        if let Some(message_cell) = transaction.in_msg_cell() {
            let message = Message::construct_from_cell(message_cell.clone())?;
            let message_id = message_cell.repr_hash();
            let mut prepared_message = if message.is_inbound_external() {
                if let Some(tracer) = self.tracer {
                    tracer.trace(
                        self.parsing.id.root_hash(),
                        Some(&message_id),
                        SystemTime::now(),
                        ParserTraceEvent::MsgIdFound,
                    );
                }

                let transaction_now = transaction.now();
                self.prepare_message_entry(message_cell, message, Some(transaction_now))?
            } else if message.src_ref().map(is_minter_address).unwrap_or(false) {
                self.prepare_message_entry(message_cell, message, None)?
            } else {
                let (src_partition, dst_partition) =
                    get_message_partitions(self.messages_sharding_depth, &message)?;
                if let Some(prepared) = prepared_messages.remove(&message_id) {
                    prepared
                } else {
                    let mut doc = Map::with_capacity(4);
                    doc.insert("id".to_owned(), message_id.as_hex_string().into());
                    PreparedMessage { src_partition, dst_partition, doc }
                }
            };
            prepared_message.set_additional_fields(
                &MessageAdditionalFields::DST,
                0,
                &transaction_id,
                &transaction_order,
                code_hash,
            );
            prepared_messages.insert(message_id, prepared_message);
        };
        log::debug!("TIME: prepare in messages {}ms", now.elapsed().as_millis());

        let now = std::time::Instant::now();
        let mut index = 1;
        transaction.out_msgs.iterate_slices(|slice| {
            let message_cell = slice.reference(0)?;
            let message_id = message_cell.repr_hash();
            let message = Message::construct_from_cell(message_cell.clone())?;
            let mut prepared_message = self.prepare_message_entry(
                message_cell,
                message,
                None, // transaction_now actual only for inbound messages
            )?;
            prepared_message.set_additional_fields(
                &MessageAdditionalFields::SRC,
                index,
                &transaction_id,
                &transaction_order,
                code_hash,
            );
            index += 1;
            prepared_messages.insert(message_id, prepared_message);
            Ok(true)
        })?;
        log::debug!("TIME: prepare out messages {}ms", now.elapsed().as_millis());

        Ok(())
    }

    pub fn finish_prepared_messages(
        &self,
        prepared_messages: HashMap<UInt256, PreparedMessage>,
    ) -> Result<Vec<ParsedEntry>> {
        let now = std::time::Instant::now();
        let mut messages = Vec::with_capacity(prepared_messages.len());
        for (_, prepared_message) in prepared_messages {
            let PreparedMessage { doc, src_partition, dst_partition } = prepared_message;

            messages.push(ParsedEntry::reduced(
                doc,
                src_partition.or(dst_partition),
                self.messages_config,
            )?);
        }
        log::debug!("TIME: prepare messages with chain_order {}ms", now.elapsed().as_millis());
        Ok(messages)
    }

    fn prepare_message_entry(
        &self,
        message_cell: Cell,
        message: Message,
        transaction_now: Option<u32>,
    ) -> Result<PreparedMessage> {
        let (src_partition, dst_partition) =
            get_message_partitions(self.messages_sharding_depth, &message)?;

        // parse message
        let boc = write_boc(&message_cell)?;
        let proof = if self.with_proofs {
            Some(write_boc(&message.prepare_proof(true, self.parsing.root)?)?)
        } else {
            None
        };
        let set = crate::MessageSerializationSet {
            message,
            id: message_cell.repr_hash(),
            block_id: Some(self.parsing.id.root_hash().clone()),
            transaction_id: None,
            status: MessageProcessingStatus::Finalized,
            boc,
            proof,
            transaction_now,
        };
        let mut doc = crate::db_serialize_message("id", &set)?;
        doc.insert("block_id".to_owned(), self.parsing.id.root_hash().as_hex_string().into());

        Ok(PreparedMessage { doc, src_partition, dst_partition })
    }

    pub(crate) fn prepare_transaction_entry(
        &self,
        cell: Cell,
        transaction: Transaction,
        workchain_id: i32,
        transaction_order: Option<String>,
        code_hash: &Option<String>,
    ) -> Result<ParsedEntry> {
        let boc = write_boc(&cell).unwrap();
        let proof = if self.with_proofs {
            Some(write_boc(&transaction.prepare_proof(self.parsing.root)?)?)
        } else {
            None
        };
        let address = transaction.account_id().clone();
        let set = crate::TransactionSerializationSet {
            transaction,
            id: cell.repr_hash(),
            status: TransactionProcessingStatus::Finalized,
            block_id: Some(self.parsing.id.root_hash().clone()),
            workchain_id,
            boc,
            proof,
        };
        let mut doc = crate::db_serialize_transaction("id", &set)?;
        let partition = get_partition(self.transactions_sharding_depth, address)?;
        if let Some(transaction_order) = transaction_order {
            doc.insert("chain_order".to_owned(), transaction_order.into());
        }
        if let Some(code_hash) = code_hash {
            doc.insert("code_hash".to_owned(), code_hash.clone().into());
        }

        ParsedEntry::reduced(doc, partition, self.transactions_config)
    }
}

fn get_message_partitions(
    sharding_depth: u32,
    message: &Message,
) -> Result<(Option<u32>, Option<u32>)> {
    let src_partition = msg_src_slice(message)?
        .map(|src| get_partition(sharding_depth, src))
        .transpose()?
        .flatten();
    let dst_partition =
        msg_dst_slice(message).map(|dst| get_partition(sharding_depth, dst)).transpose()?.flatten();
    Ok((src_partition, dst_partition))
}

fn msg_src_slice(msg: &Message) -> Result<Option<SliceData>> {
    let src = match msg.header() {
        CommonMsgInfo::ExtInMsgInfo(header) => ext_addr_slice(&header.src),
        CommonMsgInfo::ExtOutMsgInfo(_) | CommonMsgInfo::IntMsgInfo(_) => {
            Some(msg.src_ref().map(|addr| addr.address()).ok_or_else(|| {
                BlockParsingError::InvalidData("Message has no source address".to_owned())
            })?)
        }
    };
    Ok(src)
}

fn msg_dst_slice(msg: &Message) -> Option<SliceData> {
    match msg.header() {
        CommonMsgInfo::ExtInMsgInfo(header) => Some(header.dst.address()),
        CommonMsgInfo::ExtOutMsgInfo(header) => ext_addr_slice(&header.dst),
        CommonMsgInfo::IntMsgInfo(header) => Some(header.dst.address()),
    }
}

fn ext_addr_slice(addr: &MsgAddressExt) -> Option<SliceData> {
    match addr {
        MsgAddressExt::AddrExtern(addr) => Some(addr.external_address.clone()),
        MsgAddressExt::AddrNone => None,
    }
}
