use tvm_block::Block;
use tvm_block::BlockIdExt;
use tvm_block::BlockProof;
use tvm_block::ShardStateUnsplit;
use tvm_types::Cell;

use crate::block_parser::entry::ParsedEntry;

pub struct ParsedBlock {
    pub block: Option<ParsedEntry>,
    pub proof: Option<ParsedEntry>,
    pub accounts: Vec<ParsedEntry>,
    pub transactions: Vec<ParsedEntry>,
    pub messages: Vec<ParsedEntry>,
}

impl Default for ParsedBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl ParsedBlock {
    pub fn new() -> Self {
        Self {
            block: None,
            proof: None,
            accounts: Vec::new(),
            transactions: Vec::new(),
            messages: Vec::new(),
        }
    }
}

pub struct ParsingBlock<'a> {
    pub id: &'a BlockIdExt,
    pub block: &'a Block,
    pub root: &'a Cell,
    pub data: &'a [u8],

    pub mc_seq_no: Option<u32>,
    pub proof: Option<&'a BlockProof>,
    pub shard_state: Option<&'a ShardStateUnsplit>,
}
