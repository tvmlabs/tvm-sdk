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

use std::fmt;

use tvm_types::AccountId;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::HashmapRemover;
use tvm_types::HashmapSubtree;
use tvm_types::HashmapType;
use tvm_types::IBitstring;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_types::UInt256;
use tvm_types::error;
use tvm_types::fail;
use tvm_types::hm_label;

use crate::Augmentation;
use crate::Deserializable;
use crate::Serializable;
use crate::accounts::ShardAccount;
use crate::define_HashmapAugE;
use crate::hashmapaug::Augmentable;
use crate::hashmapaug::HashmapAugType;
use crate::types::CurrencyCollection;
use crate::types::Number5;

#[cfg(test)]
#[path = "tests/test_shard_accounts.rs"]
mod tests;

/////////////////////////////////////////////////////////////////////////////////////////
// 4.1.9. The combined state of all accounts in a shard. The split part
// of the shardchain state (cf. 1.2.1 and 1.2.2) is given by (upd from Lite
// Client v11): _ (HashmapAugE 256 ShardAccount DepthBalanceInfo) =
// ShardAccounts;
define_HashmapAugE!(ShardAccountsMap, 256, UInt256, ShardAccount, DepthBalanceInfo);
impl HashmapSubtree for ShardAccountsMap {}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ShardAccounts {
    shard_accounts: ShardAccountsMap,
}

impl ShardAccounts {
    pub fn insert(&mut self, account_id: &UInt256, shard_account: &ShardAccount) -> Result<()> {
        let depth_balance_info = shard_account.aug()?;
        self.shard_accounts.set(account_id, shard_account, &depth_balance_info)
    }

    pub fn insert_with_aug(
        &mut self,
        account_id: &UInt256,
        shard_account: &ShardAccount,
        aug: &DepthBalanceInfo,
    ) -> Result<()> {
        self.shard_accounts.set(account_id, shard_account, aug)
    }

    pub fn account(&self, account_id: &AccountId) -> Result<Option<ShardAccount>> {
        self.shard_accounts.get_serialized(account_id.clone())
    }

    pub fn balance(&self, account_id: &AccountId) -> Result<Option<DepthBalanceInfo>> {
        match self.shard_accounts.get_serialized_raw(account_id.clone())? {
            Some(mut slice) => Ok(Some(DepthBalanceInfo::construct_from(&mut slice)?)),
            None => Ok(None),
        }
    }

    pub fn full_balance(&self) -> &CurrencyCollection {
        &self.shard_accounts.root_extra().balance
    }

    pub fn split_for(&mut self, split_key: &SliceData) -> Result<&DepthBalanceInfo> {
        self.shard_accounts.into_subtree_with_prefix(split_key, &mut 0)?;
        self.shard_accounts.update_root_extra()
    }

    pub fn iterate_accounts<F>(&self, f: F) -> Result<bool>
    where
        F: FnMut(UInt256, ShardAccount, DepthBalanceInfo) -> Result<bool>,
    {
        self.shard_accounts.iterate_with_keys_and_aug(f)
    }

    pub fn replace_with_unloaded_account(&mut self, account_id: &UInt256) -> Result<Cell> {
        let (mut shard_account, aug) = self
            .shard_accounts
            .get_with_aug(account_id)?
            .ok_or_else(|| error!("Account not found"))?;
        let cell = shard_account.replace_with_unloaded_account()?;
        self.shard_accounts.set(account_id, &shard_account, &aug)?;
        Ok(cell)
    }

    pub fn replace_all_with_unloaded_account(&mut self) -> Result<()> {
        let copy = self.shard_accounts.clone();
        copy.iterate_with_keys_and_aug(|account_id, mut shard_account, aug| {
            if !shard_account.is_unloaded() {
                shard_account.replace_with_unloaded_account()?;
                self.shard_accounts.set(&account_id, &shard_account, &aug)?;
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

    pub fn remove(&mut self, account_id: &UInt256) -> Result<Option<SliceData>> {
        self.shard_accounts.remove(account_id.into())
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

impl Augmentation<DepthBalanceInfo> for ShardAccount {
    fn aug(&self) -> Result<DepthBalanceInfo> {
        self.read_account()?.as_struct()?.aug()
    }
}

/// depth_balance$_ split_depth:(#<= 30) balance:CurrencyCollection =
/// DepthBalanceInfo;
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct DepthBalanceInfo {
    split_depth: Number5,
    balance: CurrencyCollection,
}

impl DepthBalanceInfo {
    pub fn new(split_depth: u8, balance: &CurrencyCollection) -> Result<Self> {
        Ok(Self {
            split_depth: Number5::new_checked(split_depth as u32, 30)?,
            balance: balance.clone(),
        })
    }

    pub fn set_split_depth(&mut self, split_depth: Number5) {
        self.split_depth = split_depth
    }

    pub fn set_balance(&mut self, balance: CurrencyCollection) {
        self.balance = balance
    }

    pub fn balance(&self) -> &CurrencyCollection {
        &self.balance
    }
}

impl Augmentable for DepthBalanceInfo {
    fn calc(&mut self, other: &Self) -> Result<bool> {
        self.balance.calc(&other.balance)
        // TODO: do something with split_depth
    }
}

impl Deserializable for DepthBalanceInfo {
    fn read_from(&mut self, cell: &mut SliceData) -> Result<()> {
        self.split_depth.read_from(cell)?;
        self.balance.read_from(cell)?;
        Ok(())
    }
}

impl Serializable for DepthBalanceInfo {
    fn write_to(&self, cell: &mut BuilderData) -> Result<()> {
        self.split_depth.write_to(cell)?;
        self.balance.write_to(cell)?;
        Ok(())
    }
}
