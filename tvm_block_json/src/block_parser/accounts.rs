use std::collections::HashMap;
use std::collections::HashSet;

use tvm_block::Account;
use tvm_block::ChildCell;
use tvm_block::Serializable;
use tvm_block::ShardAccounts;
use tvm_block::Transaction;
use tvm_types::fail;
use tvm_types::write_boc;
use tvm_types::AccountId;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::UInt256;

use crate::block_parser::entry::get_sharding_depth;
use crate::block_parser::get_partition;
use crate::BlockParserConfig;
use crate::BlockParsingError;
use crate::EntryConfig;
use crate::JsonReducer;
use crate::ParsedBlock;
use crate::ParsedEntry;
use crate::ParsingBlock;

pub(crate) enum AccountTransition {
    None,
    Changed,
    Deleted,
}

pub(crate) struct ParserAccounts<'a, R: JsonReducer> {
    parsing: &'a ParsingBlock<'a>,
    max_account_bytes_size: Option<usize>,
    accounts_sharding_depth: u32,
    accounts_config: &'a Option<EntryConfig<R>>,
    changed: HashSet<AccountId>,
    deleted: HashSet<AccountId>,
    last_trans_chain_order: HashMap<AccountId, String>,
    last_trans_lt: HashMap<AccountId, u64>,
    update: Option<(ShardAccounts, ShardAccounts)>,
}

fn read_accounts(cell: Cell) -> Result<ShardAccounts> {
    const SHARD_STATE_UNSPLIT_PFX: u32 = 0x9023afe2;
    const SHARD_STATE_UNSPLIT_PFX_2: u32 = 0x9023aeee;
    let mut cell = SliceData::load_cell(cell)?;
    let cell = &mut cell;
    let tag = cell.get_next_u32()?;
    if tag != SHARD_STATE_UNSPLIT_PFX && tag != SHARD_STATE_UNSPLIT_PFX_2 {
        Err(tvm_block::BlockError::InvalidConstructorTag {
            t: tag,
            s: "ShardStateUnsplit".to_string(),
        })?;
    }
    // out_msg_queue_info
    cell.checked_drain_reference()?;

    let mut accounts = ChildCell::<ShardAccounts>::default();
    accounts.read_from_reference(cell)?;
    accounts.read_struct()
}

enum UpdateSide {
    Old,
    New,
}

impl<'a, R: JsonReducer> ParserAccounts<'a, R> {
    pub(crate) fn new(config: &'a BlockParserConfig<R>, parsing: &'a ParsingBlock) -> Result<Self> {
        let state_update = parsing.block.state_update.read_struct()?;
        let updates = if state_update.old_hash != state_update.new_hash {
            Some((read_accounts(state_update.old)?, read_accounts(state_update.new)?))
        } else {
            None
        };
        Ok(Self {
            parsing,
            max_account_bytes_size: config.max_account_bytes_size,
            accounts_sharding_depth: get_sharding_depth(&config.accounts),
            accounts_config: &config.accounts,
            changed: HashSet::new(),
            deleted: HashSet::new(),
            last_trans_chain_order: HashMap::new(),
            last_trans_lt: HashMap::new(),
            update: updates,
        })
    }

    pub(crate) fn insert_entries(&mut self, result: &mut ParsedBlock) -> Result<()> {
        let now = std::time::Instant::now();
        let workchain_id = self.parsing.id.shard().workchain_id();
        let Some(shard_state) = self.parsing.shard_state else {
            Err(BlockParsingError::InvalidData(
                "Can not parse accounts: required shard state is not specified.".to_string(),
            ))?
        };
        let shard_accounts = shard_state.read_accounts()?;
        for account_id in self.changed.iter() {
            let acc = shard_accounts.account(account_id)?.ok_or_else(|| {
                BlockParsingError::InvalidData(
                    "Block and shard state mismatch: \
                                    state doesn't contain changed account"
                        .to_string(),
                )
            })?;
            let acc = acc.read_account()?;

            let last_trans_chain_order = self.last_trans_chain_order.remove(account_id);
            result.accounts.push(Self::prepare_account_entry(
                acc,
                self.get_code_hash_from(UpdateSide::Old, account_id)?,
                last_trans_chain_order,
                self.max_account_bytes_size,
                self.accounts_sharding_depth,
                self.accounts_config,
            )?);
        }

        for account_id in &self.deleted {
            let last_trans_chain_order = self.last_trans_chain_order.remove(account_id);
            let last_trans_lt = self.last_trans_lt.remove(account_id);
            result.accounts.push(self.prepare_deleted_account_entry(
                account_id.clone(),
                workchain_id,
                self.get_code_hash_from(UpdateSide::Old, account_id)?,
                last_trans_chain_order,
                last_trans_lt,
            )?);
        }
        log::trace!(
            "TIME: accounts {} {}ms;   {}",
            self.changed.len(),
            now.elapsed().as_millis(),
            self.parsing.id
        );

        metrics::histogram!("accounts_parsing_time").record(now.elapsed());
        metrics::histogram!("parsed_accounts_count").record(self.changed.len() as f64);

        Ok(())
    }

    pub(crate) fn set_transition(
        &mut self,
        account_id: &AccountId,
        transition: AccountTransition,
    ) -> Result<()> {
        match transition {
            AccountTransition::Changed => {
                self.changed.insert(account_id.clone());
            }
            AccountTransition::Deleted => {
                self.deleted.insert(account_id.clone());
            }
            AccountTransition::None => {}
        }
        Ok(())
    }

    pub(crate) fn set_last_transaction(
        &mut self,
        account_id: &AccountId,
        transaction: &Transaction,
        chain_order: &Option<String>,
    ) -> Result<()> {
        if let Some(chain_order) = chain_order {
            self.last_trans_chain_order.insert(account_id.clone(), chain_order.clone());
        }
        let last_trans_lt =
            transaction.logical_time() + transaction.out_msgs.len().unwrap_or(0) as u64 + 1;
        self.last_trans_lt.insert(account_id.clone(), last_trans_lt);
        Ok(())
    }

    pub(crate) fn get_code_hash(&self, account_id: &AccountId) -> Result<Option<String>> {
        Ok(if let Some(hash) = self.get_code_hash_from(UpdateSide::Old, account_id)? {
            Some(hash.to_hex_string())
        } else {
            self.get_code_hash_from(UpdateSide::New, account_id)?.map(|hash| hash.to_hex_string())
        })
    }

    fn get_code_hash_from(&self, source: UpdateSide, id: &AccountId) -> Result<Option<UInt256>> {
        let acc = if let Some(updates) = &self.update {
            let accounts = match source {
                UpdateSide::Old => &updates.0,
                UpdateSide::New => &updates.1,
            };
            accounts.account(id)
        } else if let Some(state) = self.parsing.shard_state {
            state.read_accounts()?.account(id)
        } else {
            Ok(None)
        };
        if let Err(err) = &acc {
            if let Some(ExceptionCode::PrunedCellAccess) = err.downcast_ref::<ExceptionCode>() {
                return Ok(None);
            }
        }
        Ok(if let Some(acc) = acc? { acc.read_account()?.get_code_hash() } else { None })
    }

    pub(crate) fn prepare_account_entry(
        account: Account,
        prev_code_hash: Option<UInt256>,
        last_trans_chain_order: Option<String>,
        max_account_bytes_size: Option<usize>,
        accounts_sharding_depth: u32,
        accounts_config: &Option<EntryConfig<R>>,
    ) -> Result<ParsedEntry> {
        let mut boc1 = None;
        let mut boc = vec![];
        let mut skip_data = false;
        if let Some(max_size) = max_account_bytes_size {
            let size =
                account.storage_info().map(|si| si.used().bits() / 8).unwrap_or_else(|| 0) as usize;
            if max_size < size {
                log::warn!(
                    "Too big account ({}, {} bytes), skipped",
                    account.get_addr().map(|a| a.to_string()).unwrap_or_else(|| "unknown".into()),
                    size
                );
                skip_data = true;
            }
        }
        if !skip_data {
            if account.init_code_hash().is_some() {
                // new format
                let mut builder = BuilderData::new();
                account.write_original_format(&mut builder)?;
                boc1 = Some(write_boc(&builder.into_cell()?)?);
            }
            boc = write_boc(&account.serialize()?)?;
        }

        let account_id = match account.get_id() {
            Some(id) => id,
            None => fail!("Account without id in external db processor"),
        };
        let set =
            crate::AccountSerializationSet { account, prev_code_hash, proof: None, boc, boc1 };

        let partition = get_partition(accounts_sharding_depth, account_id.clone())?;
        let mut doc = crate::db_serialize_account("id", &set)?;
        if let Some(last_trans_chain_order) = last_trans_chain_order {
            doc.insert("last_trans_chain_order".to_owned(), last_trans_chain_order.into());
        }
        ParsedEntry::reduced(doc, partition, accounts_config)
    }

    fn prepare_deleted_account_entry(
        &self,
        account_id: AccountId,
        workchain_id: i32,
        prev_code_hash: Option<UInt256>,
        last_trans_chain_order: Option<String>,
        last_trans_lt: Option<u64>,
    ) -> Result<ParsedEntry> {
        let partition = get_partition(self.accounts_sharding_depth, account_id.clone())?;
        let set =
            crate::DeletedAccountSerializationSet { account_id, workchain_id, prev_code_hash };

        let mut doc = crate::db_serialize_deleted_account("id", &set)?;
        if let Some(last_trans_chain_order) = last_trans_chain_order {
            doc.insert("last_trans_chain_order".to_owned(), last_trans_chain_order.into());
        }
        if let Some(lt) = last_trans_lt {
            doc.insert("last_trans_lt".to_owned(), crate::u64_to_string(lt).into());
        }
        ParsedEntry::reduced(doc, partition, self.accounts_config)
    }
}
