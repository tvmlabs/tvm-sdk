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

    pub fn replace_with_external(&mut self, account_id: &UInt256) -> Result<Cell> {
        let mut account =
            self.shard_accounts.get(account_id)?.ok_or_else(|| error!("Account not found"))?;
        let cell = account.replace_with_external()?;
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

    pub fn replace_all_with_external(&mut self) -> Result<()> {
        let copy = self.shard_accounts.clone();
        copy.iterate_with_keys::<UInt256, _>(|account_id, mut account| {
            if !account.is_external() {
                account.replace_with_external()?;
                self.shard_accounts.set(&account_id, &account)?;
            }
            Ok(true)
        })?;
        Ok(())
    }

    pub fn is_external(&self, account_id: &UInt256) -> Result<bool> {
        Ok(self
            .shard_accounts
            .get(account_id)?
            .map(|account| account.is_external())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts::generate_test_account_by_init_code_hash;

    fn sample_account(byte: u8) -> ShardAccount {
        ShardAccount::with_params(
            &generate_test_account_by_init_code_hash(false),
            UInt256::from([byte; 32]),
            byte as u64 + 1,
            Some(UInt256::from([byte.wrapping_add(1); 32])),
        )
        .unwrap()
    }

    #[test]
    fn shard_accounts_cover_insert_iterate_replace_and_remove_paths() {
        let id1 = UInt256::from([1; 32]);
        let id2 = UInt256::from([2; 32]);
        let mut accounts = ShardAccounts::default();

        accounts.insert(&id1, &sample_account(10)).unwrap();
        accounts.insert(&id2, &sample_account(20)).unwrap();

        let mut visited = Vec::new();
        assert!(
            accounts
                .iterate_accounts(|account_id, shard_account| {
                    visited.push((account_id, shard_account.last_trans_lt()));
                    Ok(true)
                })
                .unwrap()
        );
        visited.sort_by_key(|(account_id, _)| account_id.as_slice().first().copied().unwrap_or(0));
        assert_eq!(visited.len(), 2);
        assert_eq!(visited[0], (id1.clone(), 11));
        assert_eq!(visited[1], (id2.clone(), 21));

        assert!(!accounts.is_external(&id1).unwrap());
        let original_cell = accounts.replace_with_external(&id1).unwrap();
        assert_eq!(
            accounts.account(&AccountId::from(&id1)).unwrap().unwrap().account_cell().unwrap(),
            original_cell
        );
        assert!(accounts.is_external(&id1).unwrap());

        accounts.replace_with_redirect(&id2).unwrap();
        let redirected = accounts.account(&AccountId::from(&id2)).unwrap().unwrap();
        assert!(redirected.is_redirect());
        assert_eq!(redirected.last_trans_hash(), &UInt256::from([20; 32]));
        assert_eq!(redirected.last_trans_lt(), 21);
        assert_eq!(redirected.get_dapp_id(), Some(&UInt256::from([21; 32])));

        assert!(accounts.remove(&id2).unwrap());
        assert!(accounts.account(&AccountId::from(&id2)).unwrap().is_none());
        assert!(!accounts.remove(&id2).unwrap());
    }

    #[test]
    fn shard_accounts_replace_all_with_external_and_missing_account_paths() {
        let id1 = UInt256::from([3; 32]);
        let id2 = UInt256::from([4; 32]);
        let missing = UInt256::from([9; 32]);
        let mut accounts = ShardAccounts::default();

        accounts.insert(&id1, &sample_account(30)).unwrap();
        accounts.insert(&id2, &sample_account(40)).unwrap();
        accounts.replace_all_with_external().unwrap();

        assert!(accounts.is_external(&id1).unwrap());
        assert!(accounts.is_external(&id2).unwrap());
        assert!(!accounts.is_external(&missing).unwrap());
        assert!(accounts.replace_with_external(&missing).is_err());
        assert!(accounts.replace_with_redirect(&missing).is_err());
    }
}
