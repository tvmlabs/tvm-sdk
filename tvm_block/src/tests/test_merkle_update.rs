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

use std::fs::read;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;

use tvm_types::AccountId;
use tvm_types::BocWriter;
use tvm_types::ExceptionCode;
use tvm_types::UsageTree;
use tvm_types::read_single_root_boc;
use tvm_types::write_boc;

use super::*;
use crate::Account;
use crate::Block;
use crate::CurrencyCollection;
use crate::Grams;
use crate::HashmapE;
use crate::HashmapType;
use crate::InternalMessageHeader;
use crate::MerkleProof;
use crate::Message;
use crate::MsgAddressInt;
use crate::MsgEnvelope;
use crate::Number5;
use crate::OutMsgQueueInfo;
use crate::OutMsgQueueKey;
use crate::OutQueueUpdate;
use crate::OutQueueUpdates;
use crate::ProcessedInfoKey;
use crate::ProcessedUpto;
use crate::ShardAccount;
use crate::ShardAccounts;
use crate::ShardIdent;
use crate::ShardState;
use crate::ShardStateSplit;
use crate::ShardStateUnsplit;
use crate::StateInit;
use crate::TickTock;
use crate::define_HashmapE;
use crate::generate_test_account_by_init_code_hash;
use crate::hashmapaug::HashmapAugType;

#[test]
fn test_merkle_update() {
    let mut acc = generate_test_account_by_init_code_hash(false);

    let old_cell = acc.serialize().unwrap();

    let f = CurrencyCollection::with_grams(20);
    acc.add_funds(&f).unwrap();

    let mut data = SliceData::new(vec![
        0b0011111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        0b11110100,
    ]);
    let data1 = SliceData::new(vec![
        0b001111, 0b11111111, 0b11111111, 0b1110111, 0b11111111, 0b11111111, 0b11111111, 0b11110100,
    ]);
    let data2 = SliceData::new(vec![
        0b00111111, 0b111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        0b11110100,
    ]);
    let data3 = SliceData::new(vec![
        0b0111, 0b11111111, 0b1111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11110100,
    ]);
    let data4 = SliceData::new(vec![
        0b0111111, 0b111111, 0b111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11110100,
    ]);
    data.append_reference(data1);
    data.append_reference(data2);
    data.append_reference(data3);
    data.append_reference(data4);
    acc.set_data(data.into_cell());

    let new_cell = acc.serialize().unwrap();

    assert_ne!(old_cell, new_cell);

    let mupd = MerkleUpdate::create(&old_cell, &new_cell).unwrap();

    let updated_cell = mupd.apply_for(&old_cell).unwrap();

    assert_eq!(new_cell, updated_cell);
}

#[test]
fn test_merkle_update_serialization() {
    let mut acc = generate_test_account_by_init_code_hash(false);

    let old_cell = acc.serialize().unwrap();

    let f = CurrencyCollection::with_grams(20);
    acc.add_funds(&f).unwrap();

    let data = SliceData::new(vec![
        0b0011111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        0b11110100,
    ]);
    acc.set_data(data.into_cell());

    let new_cell = acc.serialize().unwrap();

    assert_ne!(old_cell, new_cell);

    let mupd = MerkleUpdate::create(&old_cell, &new_cell).unwrap();

    let mupd_bytes = write_boc(&mupd.serialize().unwrap()).unwrap();

    let mupd2 = MerkleUpdate::construct_from_bytes(&mupd_bytes).unwrap();

    let updated_cell = mupd2.apply_for(&old_cell).unwrap();

    assert_eq!(new_cell, updated_cell);
}

#[test]
fn test_empty_merkle_update() {
    let ss = ShardState::default();
    let cell = ss.serialize().unwrap();
    let mupd = MerkleUpdate::create(&cell, &cell).unwrap();
    let cell2 = mupd.apply_for(&cell).unwrap();
    assert_eq!(cell, cell2);
}

#[test]
fn test_empty_merkle_update2() {
    let ss = ShardState::default();
    let cell1 = ss.serialize().unwrap();
    let cell2 = Cell::default();
    let mupd = MerkleUpdate::create(&cell1, &cell2).unwrap();
    let cell3 = mupd.apply_for(&cell1).unwrap();
    assert_eq!(cell2, cell3);
}

#[test]
fn test_merkle_update_for_other_bags() {
    let cell1 = BuilderData::with_raw(vec![1, 2, 3, 0x80], 4).unwrap().into_cell().unwrap();
    let cell2 = BuilderData::with_raw(vec![5, 6, 7, 0x80], 4).unwrap().into_cell().unwrap();
    let mupd = MerkleUpdate::create(&cell1, &cell2).unwrap();
    let cell3 = mupd.apply_for(&cell1).unwrap();
    assert_eq!(cell2, cell3);
}

#[test]
fn test_merkle_update_with_hasmaps() {
    define_HashmapE! {MerkleUpdates, 32, MerkleUpdate}
    let gen = |a: u32| {
        let mut acc = generate_test_account_by_init_code_hash(false);

        let old_cell = acc.serialize().unwrap();

        let mut f = CurrencyCollection::new();
        f.grams = Grams::new(a as u128).unwrap();
        acc.add_funds(&f).unwrap();

        let data = SliceData::new(vec![
            (a & 0xff) as u8,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11111111,
            0b11110100,
        ]);
        acc.set_data(data.into_cell());

        let new_cell = acc.serialize().unwrap();

        assert_ne!(old_cell, new_cell);

        MerkleUpdate::create(&old_cell, &new_cell).unwrap()
    };

    let _rng = rand::thread_rng();
    let mut map = MerkleUpdates::default();
    for _ in 0..100 {
        map.set(&rand::random::<u32>(), &gen(rand::random::<u32>())).unwrap();
    }

    let map_cell = map.serialize().unwrap();

    let _map_bag = BocWriter::with_root(&map_cell);
}

#[test]
fn test_merkle_update3() {
    let mut root1 = BuilderData::new();
    let mut a = BuilderData::new();
    let mut b = BuilderData::new();

    root1.append_raw(&[0], 8).unwrap();
    a.append_raw(&[1], 8).unwrap();
    b.append_raw(&[2], 8).unwrap();

    a.checked_append_reference(b.into_cell().unwrap()).unwrap();
    root1.checked_append_reference(a.into_cell().unwrap()).unwrap();

    let mut root2 = BuilderData::new();
    let mut a = BuilderData::new();
    let mut b = BuilderData::new();

    root2.append_raw(&[0], 8).unwrap();
    a.append_raw(&[1], 8).unwrap();
    b.append_raw(&[2], 8).unwrap();

    a.checked_append_reference(b.clone().into_cell().unwrap()).unwrap();
    root2.checked_append_reference(b.into_cell().unwrap()).unwrap();
    root2.checked_append_reference(a.into_cell().unwrap()).unwrap();

    let root1 = root1.into_cell().unwrap();
    let root2 = root2.into_cell().unwrap();

    let mupd = MerkleUpdate::create(&root1, &root2).unwrap();
    let root3 = mupd.apply_for(&root1).unwrap();

    assert_eq!(root2, root3);
}

const PATH_TO_SS: &str = "src/tests/data/block_with_ss/shard-states/";
const PATH_TO_BLOCK: &str = "src/tests/data/block_with_ss/blocks/";

fn check_one_mu(index: u64) {
    let (block, _block_len) = block_from_file(&format!("{}{}", PATH_TO_BLOCK, index));
    let (shard_state, _ss_len) = ss_from_file(&format!("{}{}", PATH_TO_SS, index - 1));
    let (new_shard_state, _new_ss_len) = ss_from_file(&format!("{}{}", PATH_TO_SS, index));

    // apply update from block and compare result with new state
    let updated_shard_state = block.read_state_update().unwrap().apply_for(&shard_state).unwrap();
    assert_eq!(new_shard_state.repr_hash(), updated_shard_state.repr_hash());

    // calculate own mu, apply it and compare result with new state
    let mu = MerkleUpdate::create(&shard_state, &new_shard_state).unwrap();

    let updated_shard_state_2 = mu.apply_for(&shard_state).unwrap();
    assert_eq!(new_shard_state.repr_hash(), updated_shard_state_2.repr_hash());
}

fn block_from_file(path: &str) -> (Block, usize) {
    let orig_bytes =
        read(Path::new(path)).unwrap_or_else(|_| panic!("Error reading file {:?}", path));

    let block = Block::construct_from_bytes(&orig_bytes).expect("Error deserializing Block");

    (block, orig_bytes.len())
}

fn ss_from_file(path: &str) -> (Cell, usize) {
    let orig_bytes =
        read(Path::new(path)).unwrap_or_else(|_| panic!("Error reading file {:?}", path));

    let root_cell = read_single_root_boc(&orig_bytes).expect("Error deserializing ShardState");
    (root_cell, orig_bytes.len())
}

#[test]
fn test_merkle_update_real_data() {
    for i in 2660..=2665
    // 2690
    {
        check_one_mu(i);
    }
    for i in 571525..=571527
    // 571555
    {
        check_one_mu(i);
    }
}

#[test]
fn test_merkle_update_create_fast() {
    for index in 2660..=2665 {
        let (shard_state, _ss_len) = ss_from_file(&format!("{}{}", PATH_TO_SS, index - 1));
        let (new_shard_state, _new_ss_len) = ss_from_file(&format!("{}{}", PATH_TO_SS, index));

        let usage_tree = UsageTree::with_root(shard_state.clone());

        // calculate MU regular way to fill usage tree
        MerkleUpdate::create(&shard_state, &new_shard_state).unwrap();

        let mu =
            MerkleUpdate::create_fast(&shard_state, &new_shard_state, |h| usage_tree.contains(h))
                .unwrap();

        let updated_shard_state_2 = mu.apply_for(&shard_state).unwrap();
        assert_eq!(new_shard_state.repr_hash(), updated_shard_state_2.repr_hash());
    }
}

fn prepare_data_for_bench(
    root_path: &str,
    shard: &str,
    start_block: u32,
    blocks_count: u32,
) -> (Cell, Vec<MerkleUpdate>) {
    let (ss, _) = ss_from_file(&format!("{}/states/{}/{}", root_path, shard, start_block));
    let mut updates = vec![];
    for seqno in start_block + 1..=start_block + blocks_count {
        let (block, _) = block_from_file(&format!("{}/blocks/{}/{}", root_path, shard, seqno));
        updates.push(block.read_state_update().unwrap());
    }
    (ss, updates)
}

// To perform benchmark you should provide needed number of blocks
// (`blocks_count`) named by their seqno starting from `start_number` in the
// `root_path`/blocks dir, and shard state for start block in `root_path`/states
// dir (named like the start block)
#[ignore]
#[test]
fn merkle_update_apply_benchmark() {
    let max_threads = 4;
    let blocks_count = 300;
    let root_path = "/full-node-test";
    let shard = "0c00000000000000";
    let start_block = 4440457;

    for threads in 1..=max_threads {
        // Prepare
        let mut data = vec![];
        for _ in 0..threads {
            data.push(prepare_data_for_bench(root_path, shard, start_block, blocks_count));
        }

        // Go
        print!("\nmerkle_update_apply_benchmark {} thread(s): ", threads);
        let mut handles = vec![];
        for _ in 0..threads {
            let (mut ss, updates) = data.pop().unwrap();
            handles.push(std::thread::spawn(move || {
                let now = Instant::now();

                for update in updates {
                    ss = update.apply_for(&ss).unwrap();
                }

                print!("{} ", now.elapsed().as_millis());
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }
    println!();
}

#[test]
fn test_merkle_update4() {
    let mut root1 = BuilderData::new();
    root1.append_raw(&[0], 8).unwrap();

    for i in 0..1024 {
        let mut new_root = BuilderData::new();
        new_root.append_raw(&[i as u8], 8).unwrap();
        new_root.checked_append_reference(root1.clone().into_cell().unwrap()).unwrap();
        new_root.checked_append_reference(root1.into_cell().unwrap()).unwrap();
        root1 = new_root;
    }

    let mut root2 = BuilderData::new();
    let mut a = BuilderData::new();
    let mut b = BuilderData::new();

    root2.append_raw(&[0], 8).unwrap();
    a.append_raw(&[1], 8).unwrap();
    b.append_raw(&[2], 8).unwrap();

    a.checked_append_reference(b.clone().into_cell().unwrap()).unwrap();
    root2.checked_append_reference(b.into_cell().unwrap()).unwrap();
    root2.checked_append_reference(a.into_cell().unwrap()).unwrap();

    let root1 = root1.into_cell().unwrap();
    let root2 = root2.into_cell().unwrap();

    let usage_tree = UsageTree::with_root(root1.clone());
    let mut uc = usage_tree.root_cell();
    while let Ok(c) = uc.reference(1) {
        uc = c;
    }

    let mupd = MerkleUpdate::create_fast(&root1, &root2, |h| usage_tree.contains(h)).unwrap();
    let root3 = mupd.apply_for(&root1).unwrap();

    assert_eq!(root2, root3);
}

#[test]
fn test_merkle_update5() {
    std::env::set_var("RUST_BACKTRACE", "full");

    fn create_cell(bytes: &[u8], refs: &[&Cell]) -> Cell {
        let mut c = BuilderData::new();
        c.append_raw(bytes, bytes.len() * 8).unwrap();
        for child in refs {
            c.checked_append_reference((*child).clone()).unwrap();
        }
        c.into_cell().unwrap()
    }

    // old tree
    // root
    // c5        c6
    // c1  c2    c3  c4
    // c1  c2
    let c1 = create_cell(&[1, 1, 1], &[]);
    let c2 = create_cell(&[2, 2, 2], &[]);
    let c3 = create_cell(&[3, 3, 3], &[]);
    let c4 = create_cell(&[4, 4, 4], &[]);
    let c5 = create_cell(&[5, 5, 5], &[&c1, &c2]);
    let c6 = create_cell(&[6, 6, 6], &[&c3, &c4]);
    let old_tree = create_cell(&[1], &[&c5, &c6]);

    // new tree
    // root'
    // c5        c6'
    // c1  c2    c3'  c4'
    // c1
    let c3_ = create_cell(&[3, 3, 4], &[]);
    let c4_ = create_cell(&[4, 4, 5, 6], &[]);
    let c6_ = create_cell(&[6, 6, 6], &[&c3_, &c4_]);
    let new_tree = create_cell(&[1], &[&c5, &c6_]);

    // merkle proof of c6 subtree in old tree
    let cells = [
        old_tree.repr_hash(),
        c6.repr_hash(),
        c3.repr_hash(),
        c4.repr_hash(),
        c1.repr_hash(),
        c2.repr_hash(),
    ];
    let old_proof =
        MerkleProof::create(&old_tree, |h| cells.contains(h)).unwrap().serialize().unwrap();

    // merkle proof of c6' subtree in new tree
    let cells =
        [new_tree.repr_hash(), c6_.repr_hash(), c3_.repr_hash(), c4_.repr_hash(), c1.repr_hash()];
    let new_proof =
        MerkleProof::create(&new_tree, |h| cells.contains(h)).unwrap().serialize().unwrap();

    for i in 0..2 {
        println!("old_proof\n{:#.100}", old_proof);
        println!("new_proof\n{:#.100}", new_proof);

        // merkle update old -> new proof
        let update = if i == 0 {
            // without optimisations
            let update = MerkleUpdate::create(&old_proof, &new_proof).unwrap();
            println!("update (without optimisations)\n{:#.100}", update.serialize().unwrap());
            update.serialize().unwrap()
        } else {
            // "fast"
            let cells = [
                old_tree.repr_hash(),
                c6.repr_hash(), // c3.repr_hash(), c4.repr_hash(), c1.repr_hash()
            ];

            let update =
                MerkleUpdate::create_fast(&old_proof, &new_proof, |h| cells.contains(h)).unwrap();
            println!("update (fast)\n{:#.100}", update.serialize().unwrap());
            update.serialize().unwrap()
        };

        // merkle update as a subtree of big tree
        let b1 = create_cell(&[1], &[&update]);
        let b2 = create_cell(&[2], &[]);
        let b3 = create_cell(&[3], &[]);
        let b4 = create_cell(&[3], &[&b1, &b2, &b3]);
        let b5 = create_cell(&[3], &[&b4]);

        // merkle proof of merkle update in the big tree
        let mut cells = vec![b1.repr_hash(), b4.repr_hash(), b5.repr_hash()];
        fn visit(c: &Cell, cells: &mut Vec<UInt256>) {
            cells.push(c.repr_hash());
            for child in c.clone_references() {
                visit(&child, cells);
            }
        }
        visit(&update, &mut cells);
        let proof = MerkleProof::create(&b5, |h| cells.contains(h)).unwrap();

        // ser-de
        let proof = MerkleProof::construct_from_bytes(&proof.write_to_bytes().unwrap()).unwrap();
        println!("proof\n{:#.100}", proof.serialize().unwrap());

        // apply merkle update from the last tree
        let block = proof.proof.clone().virtualize(1);

        let update = MerkleUpdate::construct_from_cell(
            block.reference(0).unwrap().reference(0).unwrap().reference(0).unwrap(),
        )
        .unwrap();

        let new_proof_2 = update.apply_for(&old_proof).unwrap();
        assert_eq!(new_proof, new_proof_2);
    }
}

fn get_message_with_addrs(src: AccountId, dst: AccountId) -> Message {
    let mut msg = Message::with_int_header(InternalMessageHeader::with_addresses(
        MsgAddressInt::with_standart(None, 0, src).unwrap(),
        MsgAddressInt::with_standart(None, 0, dst).unwrap(),
        CurrencyCollection::default(),
    ));

    let mut stinit = StateInit::default();
    stinit.set_split_depth(Number5::new(23).unwrap());
    stinit.set_special(TickTock::with_values(false, true));
    let code = SliceData::new(vec![
        0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        0b11110100,
    ]);
    stinit.set_code(code.into_cell());
    let data = SliceData::new(vec![
        0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        0b11110100,
    ]);
    stinit.set_data(data.into_cell());
    let library = SliceData::new(vec![
        0b00111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111,
        0b11110100,
    ]);
    stinit.set_library_code(library.into_cell(), true).unwrap();

    msg.set_state_init(stinit);

    msg
}

fn get_message(val: u8) -> MsgEnvelope {
    MsgEnvelope::with_message_and_fee(
        &get_message_with_addrs(AccountId::from([val; 32]), AccountId::from([val + 1; 32])),
        Grams::from(val as u64 * 2),
    )
    .unwrap()
}

#[test]
#[ignore = "reason: this test runs too long"]
fn test_out_msg_queue_updates() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    // generate "old" shard state with out messages

    let mut old_state = ShardStateUnsplit::construct_from_file(
        "src/tests/data/7992DD77CEB677577A7D5A8B6F388CDA76B4D0DDE16FF5004C87215E6ADF84DD.boc",
    )?;

    let mut out_msg_queue_info = old_state.read_out_msg_queue_info()?;
    out_msg_queue_info.out_queue_mut().insert(0, 0x1234_5678_8765_4321, &get_message(1), 100500)?;
    out_msg_queue_info.out_queue_mut().insert(0, 0x1234_5678_8765_4322, &get_message(2), 100501)?;
    out_msg_queue_info.out_queue_mut().insert(1, 0x1234_5678_8765_4323, &get_message(3), 100502)?;
    out_msg_queue_info.out_queue_mut().insert(1, 0x1234_5678_8765_4324, &get_message(4), 100503)?;
    out_msg_queue_info.out_queue_mut().insert(2, 0x1234_5678_8765_4325, &get_message(5), 100504)?;
    out_msg_queue_info.out_queue_mut().insert(3, 0x1234_5670_8765_4325, &get_message(6), 100505)?;
    out_msg_queue_info.out_queue_mut().insert(2, 0x1234_5678_8765_4326, &get_message(7), 100504)?;

    let msg = get_message(7);
    let wc2_msg_key = OutMsgQueueKey::with_workchain_id_and_prefix(
        2,
        0x1234_5678_8765_4326,
        msg.message_cell().repr_hash(),
    );
    out_msg_queue_info.out_queue().get(&wc2_msg_key).unwrap().unwrap();

    old_state.write_out_msg_queue_info(&out_msg_queue_info)?;

    // generate "new" shard state

    let mut new_state = old_state.clone();
    let mut out_msg_queue_info = new_state.read_out_msg_queue_info()?;
    out_msg_queue_info.out_queue_mut().insert(0, 0x1234_5678_8765_4322, &get_message(8), 200500)?;
    out_msg_queue_info.out_queue_mut().insert(0, 0x1234_5678_8765_4333, &get_message(9), 200501)?;
    out_msg_queue_info.out_queue_mut().insert(
        1,
        0x1234_5678_8765_4344,
        &get_message(11),
        200502,
    )?;

    let mut key = ProcessedInfoKey::default();
    let mut val = ProcessedUpto::default();
    out_msg_queue_info.proc_info().iterate_with_keys(|k: ProcessedInfoKey, v: ProcessedUpto| {
        key = k;
        val = v;
        Ok(false)
    })?;
    val.last_msg_lt += 11000;
    out_msg_queue_info.proc_info_mut().set(&key, &val)?;
    new_state.write_out_msg_queue_info(&out_msg_queue_info)?;

    // Create proofs

    let old_root = old_state.serialize()?;
    // println!("old_root\n{:#.100}", old_root);
    let old_proof = OutMsgQueueInfo::prepare_proof_for_wc(&old_root, 1)?;
    let old_proof_root = old_proof.serialize()?;
    // println!("\nold_proof\n{:#.100}", old_proof_root);

    let new_root = new_state.serialize()?;
    // println!("new_root\n{:#.100}", new_root);
    let new_proof = OutMsgQueueInfo::prepare_proof_for_wc(&new_root, 1)?;
    let new_proof_root = new_proof.serialize()?;
    // println!("\nnew_proof\n{:#.100}", new_proof_root);

    // Create merkle update from old proof to the new one

    // empty update
    let usage_tree_e = UsageTree::with_root(old_root.clone());
    let _ = format!("{:#.100}", usage_tree_e.root_cell());
    let update_e = OutMsgQueueInfo::prepare_update_for_wc(&old_root, &usage_tree_e, &new_root, 2)?;
    assert!(update_e.is_empty);

    // Visit all cells in old state - in the test it is ok for optimised merkle
    // update alg. In real collator we will use real usage tree.
    let usage_tree_1 = UsageTree::with_root(old_root.clone());
    let _ = format!("{:#.100}", usage_tree_1.root_cell());

    let update = OutMsgQueueInfo::prepare_update_for_wc(&old_root, &usage_tree_1, &new_root, 1)?;
    assert!(!update.is_empty);
    // println!("\nupdate\n{:#.100}", update.serialize()?);

    let update = OutQueueUpdate::construct_from_bytes(&update.write_to_bytes()?)?;

    // println!("\n\n\nupdate\n{:#.100}", update.serialize()?);

    // Generate block

    let mut out_msg_queue_updates = OutQueueUpdates::default();
    out_msg_queue_updates.set(&1_i32, &update)?;
    let block = Block { out_msg_queue_updates: Some(out_msg_queue_updates), ..Block::default() };

    let block = Block::construct_from_bytes(&block.write_to_bytes()?)?;

    // println!("\n\n\nblock\n{:#.100}", block.serialize()?);

    // Apply update from block to part of queue

    let update = block.out_msg_queue_updates.as_ref().unwrap().get(&1_i32)?.unwrap();
    let new_proof_root_2 = update.update.apply_for(&old_proof_root)?;

    assert_eq!(new_proof_root, new_proof_root_2);

    // Check result

    let virt_state: ShardStateUnsplit =
        MerkleProof::construct_from_cell(new_proof_root_2)?.virtualize()?;
    let out_msg_queue =
        virt_state.read_out_msg_queue_info()?.out_queue().queue_for_wc_with_prefix(1)?;

    out_msg_queue.iterate_with_keys(|_k, v| {
        let _m = v.read_out_msg()?.read_message()?;
        // println!("{:x}  {}", k, m);
        Ok(true)
    })?;

    assert_eq!(out_msg_queue.len()?, 3);

    // Test some special cases

    pub fn test_for_pruned_branch_error<T>(result: Result<T>) {
        match result {
            Ok(_) => panic!("Expected Error"),
            Err(error) => {
                match error.downcast_ref::<ExceptionCode>() {
                    Some(ExceptionCode::PrunedCellAccess) => return,
                    Some(_) => panic!("Expected ExceptionCode::PrunedCellAccess, but {}", error),
                    _ => (),
                }
                match error.downcast_ref::<BlockError>() {
                    Some(BlockError::PrunedCellAccess(_)) => (),
                    _ => panic!("Expected BlockError::PrunedCellAccess, but {}", error),
                }
            }
        }
    }

    // Try to get message for another (pruned) WC. Make sure we get erorr about
    // pruned branch
    let result = virt_state.read_out_msg_queue_info()?.out_queue().queue_for_wc_with_prefix(3);
    assert!(result.is_err());
    test_for_pruned_branch_error(result);

    // Try to get pruned value
    let result = virt_state.read_out_msg_queue_info()?.out_queue().get(&wc2_msg_key);
    assert!(result.is_err());
    test_for_pruned_branch_error(result);

    Ok(())
}

#[test]
fn test_prepare_empty_update_for_wc() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    let mut old_state = ShardStateUnsplit::construct_from_file(
        "src/tests/data/7992DD77CEB677577A7D5A8B6F388CDA76B4D0DDE16FF5004C87215E6ADF84DD.boc",
    )?;

    let mut out_msg_queue_info = old_state.read_out_msg_queue_info()?;
    out_msg_queue_info.out_queue_mut().insert(0, 0x1234_5678_8765_4321, &get_message(1), 100500)?;
    out_msg_queue_info.out_queue_mut().insert(1, 0x1234_5678_8765_4323, &get_message(2), 100502)?;
    let msg = get_message(3);
    let hash = msg.message_cell().repr_hash();
    let wc2_msg_key = OutMsgQueueKey::with_workchain_id_and_prefix(1, 0x1234_5678_8765_4326, hash);
    out_msg_queue_info.out_queue_mut().insert(1, 0x1234_5678_8765_4326, &msg, 100504)?;

    out_msg_queue_info.out_queue().get(&wc2_msg_key).unwrap().unwrap();

    old_state.write_out_msg_queue_info(&out_msg_queue_info)?;
    let old_root = old_state.serialize()?;

    // generate "new" shard state

    let mut new_state = old_state.clone();
    let mut out_msg_queue_info = new_state.read_out_msg_queue_info()?;
    out_msg_queue_info.out_queue_mut().insert(
        0,
        0x1234_5678_8765_4322,
        &get_message(33),
        200500,
    )?;

    let mut key = ProcessedInfoKey::default();
    let mut val = ProcessedUpto::default();
    out_msg_queue_info.proc_info().iterate_with_keys(|k: ProcessedInfoKey, v: ProcessedUpto| {
        key = k;
        val = v;
        Ok(false)
    })?;
    val.last_msg_lt += 11000;
    out_msg_queue_info.proc_info_mut().set(&key, &val)?;
    new_state.write_out_msg_queue_info(&out_msg_queue_info)?;
    let new_root = new_state.serialize()?;

    // Visit all cells in old state - in the test it is ok for optimised merkle
    // update alg. In real collator we will use real usage tree.
    let usage_tree_1 = UsageTree::with_root(old_root.clone());
    let _ = format!("{:#.100}", usage_tree_1.root_cell());

    // prepare update for nonexistend WC
    let update = OutMsgQueueInfo::prepare_update_for_wc(&old_root, &usage_tree_1, &new_root, 2)?;
    assert!(update.is_empty);

    let old_root = old_state.serialize()?;
    // println!("old_root\n{:#.100}", old_root);
    let old_proof = OutMsgQueueInfo::prepare_proof_for_wc(&old_root, 2)?;
    let old_proof_root = old_proof.serialize()?;
    // println!("\nold_proof\n{:#.100}", old_proof_root);

    let new_root = new_state.serialize()?;
    // println!("new_root\n{:#.100}", new_root);
    let new_proof = OutMsgQueueInfo::prepare_proof_for_wc(&new_root, 2)?;
    let new_proof_root = new_proof.serialize()?;
    // println!("\nnew_proof\n{:#.100}", new_proof_root);

    let new_proof_root_2 = update.update.apply_for(&old_proof_root)?;
    assert_eq!(new_proof_root, new_proof_root_2);

    Ok(())
}

#[test]
#[ignore = "reason: this test runs too long"]
fn test_out_msg_queue_merge_updates() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    // generate "old left" shard state with out messages

    let mut old_state_left = ShardStateUnsplit::construct_from_file(
        "src/tests/data/7992DD77CEB677577A7D5A8B6F388CDA76B4D0DDE16FF5004C87215E6ADF84DD.boc",
    )?;
    old_state_left.set_shard(ShardIdent::with_tagged_prefix(0, 0x4000000000000000).unwrap());

    let mut out_msg_queue_info = old_state_left.read_out_msg_queue_info()?;
    out_msg_queue_info.out_queue_mut().insert(0, 0x1234_5678_8765_4321, &get_message(1), 100500)?;
    out_msg_queue_info.out_queue_mut().insert(0, 0x1234_5678_8765_4322, &get_message(2), 100501)?;
    out_msg_queue_info.out_queue_mut().insert(1, 0x1234_5678_8765_4323, &get_message(3), 100502)?;
    old_state_left.write_out_msg_queue_info(&out_msg_queue_info)?;

    // generate "old right" shard state with out messages

    let mut old_state_right = ShardStateUnsplit::construct_from_file(
        "src/tests/data/7992DD77CEB677577A7D5A8B6F388CDA76B4D0DDE16FF5004C87215E6ADF84DD.boc",
    )?;
    old_state_right.set_shard(ShardIdent::with_tagged_prefix(0, 0xc000000000000000).unwrap());

    let mut out_msg_queue_info = old_state_right.read_out_msg_queue_info()?;
    out_msg_queue_info.out_queue_mut().insert(1, 0x1234_5678_8765_4324, &get_message(4), 100503)?;
    out_msg_queue_info.out_queue_mut().insert(2, 0x1234_5678_8765_4325, &get_message(5), 100504)?;
    out_msg_queue_info.out_queue_mut().insert(3, 0x1234_5670_8765_4325, &get_message(6), 100505)?;
    out_msg_queue_info.out_queue_mut().insert(2, 0x1234_5678_8765_4326, &get_message(7), 100504)?;
    old_state_right.write_out_msg_queue_info(&out_msg_queue_info)?;

    // generate "new" shard state

    let mut new_state = ShardStateUnsplit::construct_from_file(
        "src/tests/data/7992DD77CEB677577A7D5A8B6F388CDA76B4D0DDE16FF5004C87215E6ADF84DD.boc",
    )?;
    new_state.set_shard(ShardIdent::with_tagged_prefix(0, 0x8000000000000000).unwrap());
    let mut out_msg_queue_info = new_state.read_out_msg_queue_info()?;
    out_msg_queue_info.out_queue_mut().insert(0, 0x1234_5678_8765_4321, &get_message(1), 100500)?;
    out_msg_queue_info.out_queue_mut().insert(1, 0x1234_5678_8765_4323, &get_message(3), 100502)?;
    out_msg_queue_info.out_queue_mut().insert(1, 0x1234_5678_8765_4324, &get_message(4), 100503)?;
    out_msg_queue_info.out_queue_mut().insert(3, 0x1234_5670_8765_4325, &get_message(6), 100505)?;
    out_msg_queue_info.out_queue_mut().insert(2, 0x1234_5678_8765_4326, &get_message(7), 100504)?;
    out_msg_queue_info.out_queue_mut().insert(2, 0x1234_5678_8765_4111, &get_message(8), 100544)?;

    let mut key = ProcessedInfoKey::default();
    let mut val = ProcessedUpto::default();
    out_msg_queue_info.proc_info().iterate_with_keys(|k: ProcessedInfoKey, v: ProcessedUpto| {
        key = k;
        val = v;
        Ok(false)
    })?;
    val.last_msg_lt += 11000;
    out_msg_queue_info.proc_info_mut().set(&key, &val)?;
    new_state.write_out_msg_queue_info(&out_msg_queue_info)?;

    // Create queue proofs

    let old_root_left = old_state_left.serialize()?;
    let old_proof_left = OutMsgQueueInfo::prepare_proof_for_wc(&old_root_left, 1)?;

    let old_root_right = old_state_right.serialize()?;
    let old_proof_right = OutMsgQueueInfo::prepare_proof_for_wc(&old_root_right, 1)?;

    let new_root = new_state.serialize()?;
    let new_proof = OutMsgQueueInfo::prepare_proof_for_wc(&new_root, 1)?;
    let new_proof_root = new_proof.serialize()?;

    // create merkle update from old queue proof to new

    // Visit all cells in old state - in the test it is ok for optimised merkle
    // update alg. In real collator we will use real usage tree.
    let old_root = (ShardStateSplit { left: old_root_left, right: old_root_right }).serialize()?;
    let usage_tree_1 = UsageTree::with_root(old_root.clone());
    let _ = format!("{:#.100}", usage_tree_1.root_cell());

    let update = OutMsgQueueInfo::prepare_update_for_wc(&old_root, &usage_tree_1, &new_root, 1)?;
    assert!(!update.is_empty);
    let update = OutQueueUpdate::construct_from_bytes(&update.write_to_bytes()?)?;

    // construct split root
    let ss_split = ShardStateSplit { left: old_proof_left.proof, right: old_proof_right.proof };
    let old_proof_root = ss_split.serialize()?;
    let old_proof_root = MerkleProof {
        hash: old_proof_root.hash(0),
        depth: old_proof_root.depth(0),
        proof: old_proof_root,
    }
    .serialize()?;

    // apply update
    let new_proof_root_2 = update.update.apply_for(&old_proof_root)?;
    assert_eq!(new_proof_root, new_proof_root_2);

    Ok(())
}

#[test]
fn test_prepare_first_update_for_wc() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "full");

    let zerostate = ShardStateUnsplit::construct_from_file(
        "src/tests/data/969b6b42350754c691dfce198e7f1419d57815fd92bfdf44f3afc17d30ae1911.boc",
    )?;

    let mut next_state = zerostate.clone();
    next_state.set_seq_no(1);
    next_state.set_gen_time(zerostate.gen_time() + 15);

    let zerostate_root = zerostate.serialize()?;
    let next_state_root = next_state.serialize()?;

    let update =
        OutMsgQueueInfo::prepare_first_update_for_wc(&zerostate_root, &next_state_root, 0)?;

    let queue_proof = MerkleProof::construct_from_cell(update.update.apply_for(&zerostate_root)?)?;

    // println!("\n\n\n queue_proof \n{:#.100}", queue_proof);
    // println!("\n\n\nupdate\n{:#.100}", update.serialize()?);
    // println!("\n\n\n next_state_root \n{:#.100}", next_state_root);

    assert_eq!(queue_proof.proof.virtualize(1).repr_hash(), next_state_root.repr_hash());

    Ok(())
}

#[test]
fn test_update_shard_state_with_unloaded_account() {
    let mut shard_state_full = ShardStateUnsplit::default();
    let mut shard_accounts_full = ShardAccounts::default();

    for n in 5..70u8 {
        let mut acc = generate_test_account_by_init_code_hash(false);
        let f = CurrencyCollection::with_grams(n as u64);
        acc.add_funds(&f).unwrap();
        shard_accounts_full
            .insert(
                &UInt256::from([n].as_slice()),
                &ShardAccount::with_params(&acc, UInt256::default(), 0, Some(Default::default()))
                    .unwrap(),
            )
            .unwrap();
    }
    shard_state_full.write_accounts(&shard_accounts_full).unwrap();
    let account_id5 = UInt256::from([60].as_slice());
    let account_id6 = UInt256::from([50].as_slice());

    let create_apply_and_check = |old_state: &ShardStateUnsplit,
                                  apply_to: &ShardStateUnsplit,
                                  fast: bool,
                                  modifier: &dyn Fn(
        &mut ShardStateUnsplit,
        &mut ShardStateUnsplit,
    ) -> Option<ShardAccount>|
     -> Result<MerkleUpdate> {
        let mut old_state = old_state.clone();
        let old_state_root = old_state.serialize().unwrap();
        let usage_tree = UsageTree::with_params(old_state_root, true);
        let old_state_root = usage_tree.root_cell();
        let mut new_state = ShardStateUnsplit::construct_from_cell(old_state_root.clone()).unwrap();

        let acc = modifier(&mut old_state, &mut new_state);

        let update = if fast {
            let usage = usage_tree.take_visited_set();
            MerkleUpdate::create_fast(
                &old_state.serialize().unwrap(),
                &new_state.serialize().unwrap(),
                |hash| usage.contains(hash),
            )?
        } else {
            MerkleUpdate::create(&old_state.serialize().unwrap(), &new_state.serialize().unwrap())?
        };

        let new_state_acc = apply_to
            .read_accounts()
            .unwrap()
            .account(&account_id6.clone().into())
            .unwrap()
            .unwrap()
            .read_account()
            .unwrap();

        let new_state_constructed = ShardStateUnsplit::construct_from_cell(
            update.apply_for(&apply_to.serialize().unwrap())?,
        )
        .unwrap();
        let shard_accounts = new_state_constructed.read_accounts().unwrap();
        assert_eq!(acc, shard_accounts.account(&account_id5.clone().into()).unwrap());
        let shard_acc = shard_accounts.account(&account_id6.clone().into()).unwrap().unwrap();
        assert_eq!(shard_acc.read_account().unwrap(), new_state_acc);

        Ok(update)
    };

    let add_balance = |_old_state: &mut ShardStateUnsplit,
                       new_state: &mut ShardStateUnsplit|
     -> Option<ShardAccount> {
        let mut shard_accounts = new_state.read_accounts().unwrap();
        let mut shard_acc = shard_accounts.account(&account_id5.clone().into()).unwrap().unwrap();
        let mut acc = shard_acc.read_account().unwrap().as_struct().unwrap();
        acc.add_funds(&CurrencyCollection::with_grams(20)).unwrap();
        shard_acc.set_account_cell(acc.serialize().unwrap());
        shard_accounts.insert(&account_id5, &shard_acc).unwrap();
        new_state.write_accounts(&shard_accounts).unwrap();

        Some(shard_acc)
    };

    let mut shard_accounts_unl5 = shard_accounts_full.clone();
    let acc5_root = shard_accounts_unl5.replace_with_unloaded_account(&account_id5).unwrap();
    let mut shard_state_unl5 = shard_state_full.clone();
    shard_state_unl5.write_accounts(&shard_accounts_unl5).unwrap();

    let add_balance_ext = |_old_state: &mut ShardStateUnsplit,
                           new_state: &mut ShardStateUnsplit|
     -> Option<ShardAccount> {
        let mut shard_accounts = new_state.read_accounts().unwrap();
        let mut shard_acc = shard_accounts.account(&account_id5.clone().into()).unwrap().unwrap();
        shard_acc.read_account().unwrap();
        let mut acc = Account::construct_from_cell(acc5_root.clone()).unwrap();
        acc.add_funds(&CurrencyCollection::with_grams(20)).unwrap();
        shard_acc.set_account_cell(acc.serialize().unwrap());
        shard_accounts.insert(&account_id5, &shard_acc).unwrap();
        new_state.write_accounts(&shard_accounts).unwrap();

        Some(shard_acc)
    };

    let add_balance_ext_update = |old_state: &mut ShardStateUnsplit,
                                  new_state: &mut ShardStateUnsplit|
     -> Option<ShardAccount> {
        let result = add_balance_ext(old_state, new_state);

        let mut shard_accounts = old_state.read_accounts().unwrap();
        let mut shard_acc = shard_accounts.account(&account_id5.clone().into()).unwrap().unwrap();
        shard_acc.set_account_cell(acc5_root.clone());
        shard_accounts.insert(&account_id5, &shard_acc).unwrap();
        old_state.write_accounts(&shard_accounts).unwrap();

        result
    };

    let mut shard_acc = shard_accounts_unl5.account(&account_id5.clone().into()).unwrap().unwrap();
    shard_acc.set_account_cell(acc5_root.clone());
    shard_accounts_unl5.insert(&account_id5, &shard_acc).unwrap();
    let mut shard_state_loaded = shard_state_unl5.clone();
    shard_state_loaded.write_accounts(&shard_accounts_unl5).unwrap();

    let mut shard_accounts_unl6 = shard_accounts_full.clone();
    shard_accounts_unl6.replace_with_unloaded_account(&account_id6).unwrap();
    let mut shard_state_unl6 = shard_state_full.clone();
    shard_state_unl6.write_accounts(&shard_accounts_unl6).unwrap();

    let update_full_full =
        create_apply_and_check(&shard_state_full, &shard_state_full, false, &add_balance).unwrap();

    assert!(
        create_apply_and_check(&shard_state_full, &shard_state_unl5, false, &add_balance).is_err()
    );
    assert!(
        create_apply_and_check(&shard_state_unl5, &shard_state_full, false, &add_balance_ext)
            .is_err()
    );
    assert!(
        create_apply_and_check(&shard_state_unl5, &shard_state_unl5, false, &add_balance_ext)
            .is_err()
    );

    let update_unl_full = create_apply_and_check(
        &shard_state_unl5,
        &shard_state_full,
        false,
        &add_balance_ext_update,
    )
    .unwrap();
    assert_eq!(update_full_full, update_unl_full);
    assert!(
        create_apply_and_check(
            &shard_state_unl5,
            &shard_state_unl5,
            false,
            &add_balance_ext_update
        )
        .is_err()
    );

    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_loaded, false, &add_balance)
            .unwrap();
    let update_ext_ext = create_apply_and_check(
        &shard_state_unl5,
        &shard_state_loaded,
        false,
        &add_balance_ext_update,
    )
    .unwrap();
    assert_eq!(update_full_full, update_full_ext);
    assert_eq!(update_full_full, update_ext_ext);

    let update_ext_full =
        create_apply_and_check(&shard_state_unl6, &shard_state_full, false, &add_balance).unwrap();
    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_unl6, false, &add_balance).unwrap();
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl6, &shard_state_unl6, false, &add_balance).unwrap();

    assert_eq!(update_full_full, update_ext_full);
    assert_eq!(update_full_full, update_full_ext);
    assert_eq!(update_full_full, update_ext_ext);

    let update_full_full =
        create_apply_and_check(&shard_state_full, &shard_state_full, true, &add_balance).unwrap();

    assert!(
        create_apply_and_check(&shard_state_unl5, &shard_state_full, true, &add_balance_ext)
            .is_err()
    );
    assert!(
        create_apply_and_check(&shard_state_full, &shard_state_unl5, true, &add_balance).is_err()
    );
    assert!(
        create_apply_and_check(&shard_state_unl5, &shard_state_unl5, true, &add_balance_ext)
            .is_err()
    );

    let update_ext_full =
        create_apply_and_check(&shard_state_unl5, &shard_state_full, true, &add_balance_ext_update)
            .unwrap();
    assert_ne!(update_full_full, update_ext_full);
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl5, &shard_state_unl5, true, &add_balance_ext_update)
            .unwrap();
    assert_ne!(update_full_full, update_ext_ext);

    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_loaded, true, &add_balance).unwrap();
    let update_ext_ext = create_apply_and_check(
        &shard_state_unl5,
        &shard_state_loaded,
        true,
        &add_balance_ext_update,
    )
    .unwrap();
    assert_eq!(update_full_full, update_full_ext);
    assert_ne!(update_full_full, update_ext_ext);

    let update_ext_full =
        create_apply_and_check(&shard_state_unl6, &shard_state_full, true, &add_balance).unwrap();
    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_unl6, true, &add_balance).unwrap();
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl6, &shard_state_unl6, true, &add_balance).unwrap();

    assert_eq!(update_full_full, update_ext_full);
    assert_eq!(update_full_full, update_full_ext);
    assert_eq!(update_full_full, update_ext_ext);

    let remove = |_old_state: &mut ShardStateUnsplit,
                  new_state: &mut ShardStateUnsplit|
     -> Option<ShardAccount> {
        let mut shard_accounts = new_state.read_accounts().unwrap();
        let shard_acc = shard_accounts.account(&account_id5.clone().into()).unwrap().unwrap();
        shard_acc.read_account().unwrap();
        shard_accounts.remove(&account_id5).unwrap();
        new_state.write_accounts(&shard_accounts).unwrap();

        None
    };

    let remove_update = |old_state: &mut ShardStateUnsplit,
                         new_state: &mut ShardStateUnsplit|
     -> Option<ShardAccount> {
        remove(old_state, new_state);

        let mut shard_accounts = old_state.read_accounts().unwrap();
        let mut shard_acc = shard_accounts.account(&account_id5.clone().into()).unwrap().unwrap();
        shard_acc.set_account_cell(acc5_root.clone());
        shard_accounts.insert(&account_id5, &shard_acc).unwrap();
        old_state.write_accounts(&shard_accounts).unwrap();

        None
    };

    let update_full_full =
        create_apply_and_check(&shard_state_full, &shard_state_full, false, &remove).unwrap();
    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_unl5, false, &remove).unwrap();
    assert_eq!(update_full_full, update_full_ext);

    assert!(create_apply_and_check(&shard_state_unl5, &shard_state_full, false, &remove).is_err());
    assert!(create_apply_and_check(&shard_state_unl5, &shard_state_unl5, false, &remove).is_err());

    let update_ext_full =
        create_apply_and_check(&shard_state_unl5, &shard_state_full, false, &remove_update)
            .unwrap();
    assert_eq!(update_full_full, update_ext_full);
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl5, &shard_state_unl5, false, &remove_update)
            .unwrap();
    assert_eq!(update_full_full, update_ext_ext);

    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_loaded, false, &remove_update)
            .unwrap();
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl5, &shard_state_loaded, false, &remove_update)
            .unwrap();
    assert_eq!(update_full_full, update_full_ext);
    assert_eq!(update_full_full, update_ext_ext);

    let update_ext_full =
        create_apply_and_check(&shard_state_unl6, &shard_state_full, false, &remove).unwrap();
    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_unl6, false, &remove).unwrap();
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl6, &shard_state_unl6, false, &remove).unwrap();

    assert_eq!(update_full_full, update_ext_full);
    assert_eq!(update_full_full, update_full_ext);
    assert_eq!(update_full_full, update_ext_ext);

    let update_full_full =
        create_apply_and_check(&shard_state_full, &shard_state_full, true, &remove).unwrap();
    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_unl5, true, &remove).unwrap();
    assert_eq!(update_full_full, update_full_ext);

    assert!(create_apply_and_check(&shard_state_unl5, &shard_state_full, true, &remove).is_err());
    assert!(create_apply_and_check(&shard_state_unl5, &shard_state_unl5, true, &remove).is_err());

    let update_ext_full =
        create_apply_and_check(&shard_state_unl5, &shard_state_full, true, &remove_update).unwrap();
    assert_eq!(update_full_full, update_ext_full);
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl5, &shard_state_unl5, true, &remove_update).unwrap();
    assert_eq!(update_full_full, update_ext_ext);

    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_loaded, true, &remove_update)
            .unwrap();
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl5, &shard_state_loaded, true, &remove_update)
            .unwrap();
    assert_eq!(update_full_full, update_full_ext);
    assert_eq!(update_full_full, update_ext_ext);

    let update_ext_full =
        create_apply_and_check(&shard_state_unl6, &shard_state_full, true, &remove).unwrap();
    let update_full_ext =
        create_apply_and_check(&shard_state_full, &shard_state_unl6, true, &remove).unwrap();
    let update_ext_ext =
        create_apply_and_check(&shard_state_unl6, &shard_state_unl6, true, &remove).unwrap();

    assert_eq!(update_full_full, update_ext_full);
    assert_eq!(update_full_full, update_full_ext);
    assert_eq!(update_full_full, update_ext_ext);
}

#[test]
fn test_fast_merkle_update() {
    const PATH_TO_DATA: &str = "src/tests/data";

    let mut paths = vec![];
    for entry in std::fs::read_dir(PATH_TO_DATA).unwrap().flatten() {
        let path = entry.path().to_str().unwrap().to_string();
        if let Some(path) = path.strip_suffix(".old_state") {
            paths.push(path.to_string());
        }
    }
    for path in paths {
        println!("Testing {}", path);
        let (old_state, new_state, usages) = read_states_and_usages(&path);

        let start = Instant::now();
        let update_fast =
            MerkleUpdate::create_fast(&old_state, &new_state, |x| usages.contains(x)).unwrap();
        println!("Created fast: {:?}", start.elapsed());
        // println!("fast old:\n{}", cell_to_string(&update_fast.old));
        // println!("fast new:\n{}", cell_to_string(&update_fast.new));

        let verify_new = update_fast.apply_for(&old_state).unwrap();
        assert_eq!(verify_new, new_state);

        let update = MerkleUpdate::create(&old_state, &new_state).unwrap();
        // println!("slow old:\n{}", cell_to_string(&update.old));
        // println!("slow new:\n{}", cell_to_string(&update.new));

        let verify_new = update.apply_for(&old_state).unwrap();
        assert_eq!(verify_new, new_state);
    }
}

fn read_states_and_usages(path: &str) -> (Cell, Cell, HashSet<UInt256>) {
    let path = Path::new(path);
    let old_state = Cell::construct_from_file(path.with_extension("old_state")).unwrap();
    let new_state = Cell::construct_from_file(path.with_extension("new_state")).unwrap();
    let usages = std::fs::read_to_string(path.with_extension("usages"))
        .unwrap()
        .split(",")
        .map(|x| UInt256::from_str(x).unwrap())
        .collect();
    (old_state, new_state, usages)
}

#[allow(dead_code)]
fn cell_to_string(cell: &Cell) -> String {
    let mut s = String::new();
    write_cell_to_string(cell, 0, &mut s);
    s
}

#[allow(dead_code)]
fn write_cell_to_string(cell: &Cell, indent: usize, s: &mut String) {
    *s += &" ".repeat(indent);
    *s += cell.repr_hash().to_hex_string().split_at(4).0;
    match cell.cell_type() {
        CellType::Unknown => *s += "Unknown ",
        CellType::Ordinary => {
            if cell.references_count() == 0 {
                *s += " Data";
            } else {
                *s += &format!(" [{}]", cell.references_count());
            }
        }
        CellType::PrunedBranch => *s += " Pruned Branch",
        CellType::LibraryReference => *s += " Library Reference",
        CellType::MerkleProof => *s += " Merkle Proof",
        CellType::MerkleUpdate => *s += " Merkle Update",
        CellType::Big => *s += " Big",
        CellType::UnloadedAccount => *s += " External",
    }
    *s += "\n";
    for child in cell.clone_references() {
        write_cell_to_string(&child, indent + 1, s);
    }
}
