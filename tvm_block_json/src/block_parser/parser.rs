use std::collections::BTreeMap;

use serde_json::Value;
use tvm_block::Account;
use tvm_block::AccountBlock;
use tvm_block::AccountStatus;
use tvm_block::BlockIdExt;
use tvm_block::BlockProcessingStatus;
use tvm_block::BlockProof;
use tvm_block::Deserializable;
use tvm_block::HashmapAugType;
use tvm_block::Transaction;
use tvm_types::fail;
use tvm_types::HashmapType;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::UInt256;

use crate::block_parser::accounts::AccountTransition;
use crate::block_parser::accounts::ParserAccounts;
use crate::block_parser::block::ParsedBlock;
use crate::block_parser::block::ParsingBlock;
use crate::block_parser::entry::get_sharding_depth;
use crate::block_parser::entry::ParsedEntry;
use crate::block_parser::is_account_none;
use crate::block_parser::transactions::ParserTransactions;
use crate::block_parser::unix_time_to_system_time;
use crate::block_parser::ParserTraceEvent;
use crate::block_parser::ParserTracer;
use crate::JsonReducer;

pub struct EntryConfig<R: JsonReducer> {
    pub sharding_depth: Option<u32>,
    pub reducer: Option<R>,
}

pub struct BlockParserConfig<R: JsonReducer> {
    pub blocks: Option<EntryConfig<R>>,
    pub proofs: Option<EntryConfig<R>>,
    pub accounts: Option<EntryConfig<R>>,
    pub transactions: Option<EntryConfig<R>>,
    pub messages: Option<EntryConfig<R>>,

    pub max_account_bytes_size: Option<usize>,
    pub is_node_se: bool,
}

pub struct BlockParser<T: ParserTracer, R: JsonReducer> {
    pub config: BlockParserConfig<R>,
    tracer: Option<T>,
    block_sharding_depth: u32,
}

impl<T: ParserTracer, R: JsonReducer> BlockParser<T, R> {
    pub fn new(config: BlockParserConfig<R>, tracer: Option<T>) -> Self {
        let block_sharding_depth = get_sharding_depth(&config.blocks);
        Self { config, tracer, block_sharding_depth }
    }

    pub fn parse(&self, block: ParsingBlock, with_proofs: bool) -> Result<ParsedBlock> {
        if self.config.accounts.is_some() && block.shard_state.is_none() {
            fail!(
                "Shard state should be specified because the block parser was configured with account parsing."
            );
        }
        let now = std::time::Instant::now();

        let block_id_str = block.id.root_hash().as_hex_string();

        let block_order = if let Some(mc_seq_no) = block.mc_seq_no {
            Some(crate::block_order(block.block, mc_seq_no)?)
        } else {
            None
        };
        log::trace!("block order for {}: {:#?}", block_id_str, block_order);

        let block_info = block.block.read_info()?;
        let ut = block_info.gen_utime();
        if let Some(tracer) = &self.tracer {
            tracer.trace(
                block.id.root_hash(),
                None,
                unix_time_to_system_time(ut.as_u32() as u64)?,
                ParserTraceEvent::BlockCollated,
            );
        }
        log::debug!("TIME: block deserialize {}ms;   {}", now.elapsed().as_millis(), block_id_str);

        let mut result = ParsedBlock::new();

        let include_accounts = self.config.accounts.is_some();
        let include_transactions = self.config.transactions.is_some();
        let include_messages = self.config.messages.is_some();

        if include_accounts || include_transactions || include_messages {
            // Transactions and messages

            let now = std::time::Instant::now();
            let workchain_id = block_info.shard().workchain_id();

            let mut block_transactions = BTreeMap::new();

            let mut accounts = ParserAccounts::new(&self.config, &block)?;
            let transactions =
                ParserTransactions::new(&self.config, &self.tracer, &block, with_proofs);
            let mut tr_count = 0;

            let block_extra = block.block.read_extra()?;
            block_extra.read_account_blocks()?.iterate_objects(
                &mut |account_block: AccountBlock| {
                    let state_upd = account_block.read_state_update()?;
                    let (mut transition, check_existed) = if is_account_none(&state_upd.new_hash) {
                        (
                            AccountTransition::Deleted,
                            self.config.is_node_se && is_account_none(&state_upd.old_hash),
                        )
                    } else {
                        (AccountTransition::Changed, false)
                    };

                    let mut account_existed = false;

                    account_block.transactions().iterate_slices(
                        &mut |_, transaction_slice: SliceData| {
                            let cell = transaction_slice.reference(0)?;
                            let transaction = Transaction::construct_from_cell(cell.clone())?;

                            if transaction.orig_status != AccountStatus::AccStateNonexist
                                || transaction.end_status != AccountStatus::AccStateNonexist
                            {
                                account_existed = true;
                            }

                            let ordering_key =
                                (transaction.logical_time(), transaction.account_id().clone());
                            block_transactions.insert(ordering_key, (cell, transaction));
                            tr_count += 1;
                            Ok(true)
                        },
                    )?;
                    if check_existed && !account_existed {
                        transition = AccountTransition::None;
                    }
                    if include_accounts {
                        accounts.set_transition(account_block.account_id(), transition)?;
                    }

                    Ok(true)
                },
            )?;
            log::debug!("TIME: prepare transactions order {}ms", now.elapsed().as_millis());
            let now = std::time::Instant::now();

            let mut prepared_messages = Default::default();
            for (index, (_, (cell, transaction))) in block_transactions.into_iter().enumerate() {
                let transaction_order = block_order
                    .as_deref()
                    .map(|b_o| format!("{}{}", b_o, crate::u64_to_string(index as u64)));

                let account_id = transaction.account_id().clone();
                if include_accounts {
                    accounts.set_last_transaction(&account_id, &transaction, &transaction_order)?;
                }

                let code_hash = accounts.get_code_hash(&account_id)?;
                if include_messages {
                    transactions.parse_messages_from_transaction(
                        &transaction,
                        cell.repr_hash(),
                        transaction_order.as_deref(),
                        &code_hash,
                        &mut prepared_messages,
                    )?;
                }

                if include_transactions {
                    result.transactions.push(transactions.prepare_transaction_entry(
                        cell,
                        transaction,
                        workchain_id,
                        transaction_order,
                        &code_hash,
                    )?);
                }
            }
            log::debug!("TIME: prepare transactions and messages {}ms", now.elapsed().as_millis());

            if include_messages {
                result.messages = transactions.finish_prepared_messages(prepared_messages)?;
            }

            if include_accounts {
                accounts.insert_entries(&mut result)?;
            }
        }

        let now = std::time::Instant::now();

        // Block

        if self.config.blocks.is_some() {
            result.block = Some(self.prepare_block_entry(&block, &block_order)?);
        }

        log::debug!("TIME: prepare block {}ms;   {}", now.elapsed().as_millis(), block_id_str);

        if self.config.proofs.is_some() {
            if let Some(proof) = block.proof {
                let now = std::time::Instant::now();
                result.proof = Some(self.prepare_block_proof_entry(&block, proof, &block_order)?);
                log::trace!("TIME: block proof {}ms;   {}", now.elapsed().as_millis(), block.id);
            }
        }

        Ok(result)
    }

    fn prepare_block_entry(
        &self,
        block: &ParsingBlock,
        block_order: &Option<String>,
    ) -> Result<ParsedEntry> {
        let set = crate::BlockSerializationSetFH {
            block: block.block,
            id: block.id.root_hash(),
            status: BlockProcessingStatus::Finalized,
            boc: block.data,
            file_hash: Some(block.id.file_hash()),
        };

        let mut doc = crate::db_serialize_block("id", set)?;
        if let Some(block_order) = block_order {
            doc.insert("chain_order".to_owned(), Value::String(block_order.clone()));
        }
        let partition = get_block_partition(self.block_sharding_depth, block.id);
        ParsedEntry::reduced(doc, partition, &self.config.blocks)
    }

    fn prepare_block_proof_entry(
        &self,
        block: &ParsingBlock,
        proof: &BlockProof,
        block_order: &Option<String>,
    ) -> Result<ParsedEntry> {
        let partition = get_block_partition(self.block_sharding_depth, block.id);
        let mut doc = crate::db_serialize_block_proof("id", proof)?;
        if let Some(chain_order) = block_order {
            doc.insert("chain_order".to_owned(), Value::String(chain_order.clone()));
        }
        ParsedEntry::reduced(doc, partition, &self.config.proofs)
    }

    pub fn prepare_account_entry(
        &self,
        account: Account,
        prev_code_hash: Option<UInt256>,
        last_trans_chain_order: Option<String>,
    ) -> Result<ParsedEntry> {
        ParserAccounts::prepare_account_entry(
            account,
            prev_code_hash,
            last_trans_chain_order,
            self.config.max_account_bytes_size,
            get_sharding_depth(&self.config.accounts),
            &self.config.accounts,
        )
    }
}

fn get_block_partition(sharding_depth: u32, block_id: &BlockIdExt) -> Option<u32> {
    if sharding_depth > 0 {
        let partitioning_info =
            u64::from_be_bytes(block_id.root_hash.as_slice()[0..8].try_into().unwrap());
        Some((partitioning_info >> (64 - sharding_depth)) as u32)
    } else {
        None
    }
}

#[cfg(test)]
#[path = "../tests/test_parser.rs"]
mod tests;
