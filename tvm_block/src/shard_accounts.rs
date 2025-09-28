// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use tvm_types::AccountId;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::HashmapType;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::UInt256;
use tvm_types::error;

use crate::Deserializable;
use crate::HashmapE;
use crate::Serializable;
use crate::accounts::ShardAccount;
use crate::define_HashmapE;

#[cfg(test)]
#[path = "tests/test_shard_accounts.rs"]
mod tests;

/////////////////////////////////////////////////////////////////////////////////////////
// 4.1.9. The combined state of all accounts in a shard. The split part
// of the shardchain state (cf. 1.2.1 and 1.2.2) is given by (upd from Lite
// Client v11): _ (HashmapAugE 256 ShardAccount DepthBalanceInfo) =
// ShardAccounts;
define_HashmapE!(ShardAccountsMap, 256, ShardAccount);

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ShardAccounts {
    shard_accounts: ShardAccountsMap,
}

impl ShardAccounts {
    pub fn insert(&mut self, account_id: &UInt256, shard_account: &ShardAccount) -> Result<()> {
        self.shard_accounts.set(account_id, shard_account)
    }

    pub fn account(&self, account_id: &AccountId) -> Result<Option<ShardAccount>> {
        self.shard_accounts.get(account_id)
    }

    pub fn iterate_accounts<F>(&self, f: F) -> Result<bool>
    where
        F: FnMut(UInt256, ShardAccount) -> Result<bool>,
    {
        self.shard_accounts.iterate_with_keys(f)
    }

    pub fn replace_with_unloaded_account(&mut self, account_id: &UInt256) -> Result<Cell> {
        let mut account =
            self.shard_accounts.get(account_id)?.ok_or_else(|| error!("Account not found"))?;
        let cell = account.replace_with_unloaded_account()?;
        self.shard_accounts.set(account_id, &account)?;
        Ok(cell)
    }

    pub fn replace_with_redirect(&mut self, account_id: &UInt256) -> Result<()> {
        let account =
            self.shard_accounts.get(account_id)?.ok_or_else(|| error!("Account not found"))?;
        let redirect = ShardAccount::with_redirect(
            account.last_trans_hash().clone(),
            account.last_trans_lt(),
            account.get_dapp_id().cloned(),
        )?;
        self.shard_accounts.set(account_id, &redirect)
    }

    pub fn replace_all_with_unloaded_account(&mut self) -> Result<()> {
        let copy = self.shard_accounts.clone();
        copy.iterate_with_keys::<UInt256, _>(|account_id, mut account| {
            if !account.is_unloaded() {
                account.replace_with_unloaded_account()?;
                self.shard_accounts.set(&account_id, &account)?;
            }
            Ok(true)
        })?;
        Ok(())
    }

    pub fn is_unloaded(&self, account_id: &UInt256) -> Result<bool> {
        Ok(self
            .shard_accounts
            .get(account_id)?
            .map(|account| account.is_unloaded())
            .unwrap_or(false))
    }

    pub fn remove(&mut self, account_id: &UInt256) -> Result<bool> {
        self.shard_accounts.remove(account_id)
    }
}

impl Deserializable for ShardAccounts {
    fn read_from(&mut self, cell: &mut SliceData) -> Result<()> {
        self.shard_accounts.read_from(cell)?;
        Ok(())
    }
}

impl Serializable for ShardAccounts {
    fn write_to(&self, cell: &mut BuilderData) -> Result<()> {
        self.shard_accounts.write_to(cell)?;
        Ok(())
    }
}
