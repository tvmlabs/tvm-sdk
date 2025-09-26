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

use super::*;
use crate::{define_HashmapE, AccountCellStruct};
use crate::generate_test_account_by_init_code_hash;
use crate::write_read_and_assert;
use blake3::Hasher;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tvm_types::HashmapE;

#[test]
fn test_serialization_shard_account() {
    let mut shard_acc = ShardAccounts::default();

    for n in 5..8u8 {
        let acc = generate_test_account_by_init_code_hash(false);
        shard_acc
            .insert(
                &UInt256::from([n].as_slice()),
                &ShardAccount::with_params(&acc, UInt256::default(), 0, Some(Default::default()))
                    .unwrap(),
            )
            .unwrap();
    }
    write_read_and_assert(shard_acc);
}

#[test]
fn test_external_account_serialization() {
    let mut shard_acc = ShardAccounts::default();

    for n in 5..6u8 {
        let acc = generate_test_account_by_init_code_hash(false);
        shard_acc
            .insert(
                &UInt256::from([n].as_slice()),
                &ShardAccount::with_params(&acc, UInt256::default(), 0, Some(Default::default()))
                    .unwrap(),
            )
            .unwrap();
    }

    let account_id = UInt256::from([5u8].as_slice());
    let account = shard_acc.account(&account_id.clone().into()).unwrap().unwrap();
    assert!(matches!(account.read_account().unwrap(), AccountCellStruct::Struct(_)));

    let serialized_full = shard_acc.serialize().unwrap();
    let acc_cell = shard_acc.replace_with_unloaded_account(&account_id).unwrap();

    shard_acc = write_read_and_assert(shard_acc);

    let serialized_unloaded = shard_acc.serialize().unwrap();
    assert_eq!(serialized_full.repr_hash(), serialized_unloaded.repr_hash());

    let account = shard_acc.account(&account_id.clone().into()).unwrap().unwrap();
    assert!(matches!(account.read_account().unwrap(), AccountCellStruct::Unloaded(_)));

    let account = ShardAccount::with_account_root(
        acc_cell,
        account.last_trans_hash().clone(),
        account.last_trans_lt(),
        account.get_dapp_id().cloned(),
    );
    shard_acc.insert(&account_id, &account).unwrap();

    let account = shard_acc.account(&account_id.into()).unwrap().unwrap();
    assert!(matches!(account.read_account().unwrap(), AccountCellStruct::Struct(_)));

    let serialized_full = shard_acc.serialize().unwrap();
    assert_eq!(serialized_full.repr_hash(), serialized_unloaded.repr_hash());
}

define_HashmapE!(ShardAccounts2, 256, ShardAccount);

#[test]
fn test_shard_account_insert_time() {
    let mut accounts = HashMap::new();
    let time = Instant::now();
    for n in 5..100_000u32 {
        let account_id = UInt256::from(n.to_be_bytes().as_slice());
        let acc = generate_test_account_by_init_code_hash(false);
        accounts.insert(
            account_id.clone(),
            ShardAccount::with_params(&acc, UInt256::default(), 0, Some(Default::default()))
                .unwrap(),
        );
    }
    println!("Generation time {:?}", time.elapsed());

    let mut shard_accounts = ShardAccounts::default();
    let time = Instant::now();
    for (id, shard_acc) in &accounts {
        shard_accounts.insert(&id, &shard_acc).unwrap();
    }
    println!("Shard accounts insertion time {:?}", time.elapsed());

    let mut shard_accounts2 = ShardAccounts2::default();
    let time = Instant::now();
    for (id, shard_acc) in &accounts {
        shard_accounts2.set(id, &shard_acc).unwrap();
    }
    println!("Shard accounts 2 insertion time {:?}", time.elapsed());

    let subset =
        accounts.iter().map(|(k, v)| (k.clone(), v.clone())).take(300).collect::<Vec<_>>();

    let time = Instant::now();
    for (id, shard_acc) in &subset {
        shard_accounts.insert(id, shard_acc).unwrap();
    }
    println!("Shard accounts reinsertion time {:?}", time.elapsed());

    let time = Instant::now();
    for (id, shard_acc) in &subset {
        shard_accounts2.set(id, shard_acc).unwrap();
    }
    println!("Shard accounts 2 reinsertion time {:?}", time.elapsed());
}
