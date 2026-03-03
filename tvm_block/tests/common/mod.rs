#![allow(dead_code)]

use tvm_block::*;
use tvm_types::*;

pub fn write_read_and_assert<T>(s: T) -> T
where
    T: Serializable + Deserializable + Default + std::fmt::Debug + PartialEq,
{
    let cell = s.write_to_new_cell().unwrap();
    let mut slice = tvm_types::SliceData::load_builder(cell).unwrap();
    println!("slice: {}", slice);
    let s2 = T::construct_from(&mut slice).unwrap();
    s2.serialize().unwrap();
    pretty_assertions::assert_eq!(s, s2);
    s2
}

pub fn generate_account_block(address: AccountId, tr_count: usize) -> tvm_types::Result<AccountBlock> {
    let s_status_update = HashUpdate::default();
    let mut acc_block = AccountBlock::with_address(address.clone());

    for _ in 0..tr_count {
        let transaction = generate_tranzaction(address.clone());
        acc_block.add_transaction(&transaction)?;
    }
    acc_block.write_state_update(&s_status_update).unwrap();

    Ok(acc_block)
}

pub fn generate_test_shard_account_block() -> ShardAccountBlocks {
    let mut shard_block = ShardAccountBlocks::default();

    for n in 0..10 {
        let address = AccountId::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, n as u8,
        ]);
        let account_block = generate_account_block(address.clone(), n + 1).unwrap();
        shard_block.insert(&account_block).unwrap();
    }
    shard_block
}
