mod accounts;
mod block;
mod entry;
mod parser;
mod reducers;
mod transactions;

use std::time::Duration;
use std::time::SystemTime;

pub use block::ParsedBlock;
pub use block::ParsingBlock;
pub use entry::ParsedEntry;
pub use parser::BlockParser;
pub use parser::BlockParserConfig;
pub use parser::EntryConfig;
pub use reducers::JsonFieldsReducer;
use serde_json::Map;
use serde_json::Value;
use thiserror::Error;
use tvm_block::MsgAddrStd;
use tvm_block::MsgAddressInt;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::UInt256;

#[derive(Debug, Error)]
pub enum BlockParsingError {
    #[error("Invalid data: {}", 0)]
    InvalidData(String),
}

#[derive(Copy, Clone)]
pub enum ParserTraceEvent {
    BlockCollated,
    BlockPushedByNode,
    BlockPulledByParser,
    MsgIdFound,
    BlockParsed,
    ProducerStarted,
    ProducerFinished,
}

pub trait ParserTracer {
    fn trace(
        &self,
        block_id: &UInt256,
        message_id: Option<&UInt256>,
        time: SystemTime,
        event: ParserTraceEvent,
    );
}

pub struct NoTrace();

impl ParserTracer for NoTrace {
    fn trace(
        &self,
        _block_id: &UInt256,
        _message_id: Option<&UInt256>,
        _time: SystemTime,
        _event: ParserTraceEvent,
    ) {
    }
}

pub trait JsonReducer {
    fn reduce(&self, json: Map<String, Value>) -> Result<Map<String, Value>>;
}

pub struct NoReduce();

impl JsonReducer for NoReduce {
    fn reduce(&self, json: Map<String, Value>) -> Result<Map<String, Value>> {
        Ok(json)
    }
}

pub fn unix_time_to_system_time(utime: u64) -> Result<SystemTime> {
    SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(utime))
        .ok_or_else(|| anyhow::anyhow!("Can't convert unix timestamp bytes to SystemTime"))
}

pub(crate) fn get_partition(
    sharding_depth: u32,
    mut partitioning_info: SliceData,
) -> Result<Option<u32>> {
    if sharding_depth > 0 && partitioning_info.remaining_bits() > 0 {
        let partition = if partitioning_info.remaining_bits() >= 32 {
            partitioning_info.get_next_u32()?
        } else {
            let bytes = partitioning_info.get_next_bits(partitioning_info.remaining_bits())?;
            let mut array = [0u8; 4];
            array[..bytes.len()].copy_from_slice(&bytes);
            u32::from_be_bytes(array)
        };
        Ok(Some(partition >> (32 - sharding_depth)))
    } else {
        Ok(None)
    }
}

lazy_static::lazy_static!(
    static ref MINTER_ADDRESS: MsgAddressInt =
        MsgAddressInt::AddrStd(MsgAddrStd::with_address(None, -1, [0; 32].into()));
);

const ACCOUNT_NONE_HASH: UInt256 = UInt256::with_array([
    144, 174, 200, 150, 90, 250, 187, 22, 235, 195, 203, 155, 64, 142, 186, 231, 27, 97, 141, 120,
    120, 139, 200, 13, 9, 132, 53, 147, 202, 201, 141, 164,
]);

pub(crate) fn is_account_none(hash: &UInt256) -> bool {
    *hash == ACCOUNT_NONE_HASH
}

pub(crate) fn is_minter_address(address: &MsgAddressInt) -> bool {
    *address == *MINTER_ADDRESS
}
