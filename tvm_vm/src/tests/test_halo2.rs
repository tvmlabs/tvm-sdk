use std::collections::HashMap;
use std::env;
use std::time::Instant;
use tvm_types::Cell;

use tvm_types::SliceData;
use crate::executor::zk_halo2::execute_halo2_proof_verification;

use crate::executor::engine::Engine;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::savelist::SaveList;
use crate::utils::pack_data_to_cell;
use crate::utils::pack_string_to_cell;
use crate::utils::unpack_data_from_cell;

use crate::executor::test_helper::*;

use gosh_dark_dex_halo2_circuit::prover::*;
use gosh_dark_dex_halo2_circuit::circuit::poseidon_hash;

use halo2_base::halo2_proofs::{
    arithmetic::CurveAffine,
    halo2curves::{bn256::Fr, secp256k1::{Fp, Fq, Secp256k1Affine}},
    plonk::Fixed,
};
use std::io::Cursor;

#[test]
fn test() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );

    let sk_u: u64 = 23;
    let token_type: u64 = 1;
    let private_note_sum: u64 = 1000;

    let sk_u_ = Fr::from(sk_u);
    let token_type_ = Fr::from(token_type);
    let private_note_sum_ = Fr::from(private_note_sum);
 
    let sk_u_commitment = poseidon_hash([sk_u_, Fr::zero()]);
    let data_to_hash = [sk_u_commitment, private_note_sum_, token_type_, sk_u_];
    let digest = poseidon_hash(data_to_hash);

    
    let digest: [u8; 32] = digest.to_bytes();

    let digest_hex = hex::encode(&digest);

    println!("digest here here: {:?}", digest.clone());
    println!("digest_hex: {:?}", digest_hex);

    let i = IntegerData::from_str_radix(digest_hex.as_str(), 16).unwrap();
    /*let i = IntegerData::from_unsigned_bytes_be(&digest.clone());*/
    engine.cc.stack.push(StackItem::integer(i));
    engine.cc.stack.push(StackItem::int(token_type));
    engine.cc.stack.push(StackItem::int(private_note_sum));

    let params = read_kzg_params("kzg_params.bin".to_string());
    let mut pub_inputs = vec![private_note_sum_, token_type_, Fr::from_bytes(&digest).unwrap()];
    let proof = generate_proof(&params, Some(token_type_), Some(private_note_sum_), Some(sk_u_), Some(sk_u_commitment),  &mut pub_inputs);

    
    let proof_cell = pack_data_to_cell(&proof.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let start: Instant = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_micros();

    println!("elapsed in microsecond: {:?}", elapsed);

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("res: {:?}", res);
    assert!(res == true);
}

#[test]
fn test_negative() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );

    let sk_u: u64 = 23;
    let token_type: u64 = 1;
    let private_note_sum: u64 = 1000;

    let sk_u_ = Fr::from(sk_u);
    let token_type_ = Fr::from(token_type);
    let private_note_sum_ = Fr::from(private_note_sum);
 
    let sk_u_commitment = poseidon_hash([sk_u_, Fr::zero()]);
    let data_to_hash = [sk_u_commitment, private_note_sum_, token_type_, sk_u_];
    let digest = poseidon_hash(data_to_hash);
    let digest: [u8; 32] = digest.to_bytes();

    let token_type_wrong: u64 = 2;
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&digest.clone())));
    engine.cc.stack.push(StackItem::int(token_type_wrong));
    engine.cc.stack.push(StackItem::int(private_note_sum));

    let params = read_kzg_params("kzg_params.bin".to_string());
    let mut pub_inputs = vec![private_note_sum_, token_type_, Fr::from_bytes(&digest).unwrap()];
    let proof = generate_proof(&params, Some(token_type_), Some(private_note_sum_), Some(sk_u_), Some(sk_u_commitment),  &mut pub_inputs);

    
    let proof_cell = pack_data_to_cell(&proof.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let start: Instant = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_micros();

    println!("elapsed in microsecond: {:?}", elapsed);

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("res: {:?}", res);
    assert!(res == false);
}