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

#[cfg(test)]
mod tests {
    use super::ParsedBlock;

    #[test]
    fn parsed_block_new_and_default_are_empty() {
        let parsed = ParsedBlock::new();
        assert!(parsed.block.is_none());
        assert!(parsed.proof.is_none());
        assert!(parsed.accounts.is_empty());
        assert!(parsed.transactions.is_empty());
        assert!(parsed.messages.is_empty());

        let default = ParsedBlock::default();
        assert!(default.block.is_none());
        assert!(default.proof.is_none());
        assert!(default.accounts.is_empty());
        assert!(default.transactions.is_empty());
        assert!(default.messages.is_empty());
    }
}
