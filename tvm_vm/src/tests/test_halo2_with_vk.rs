//! Real round-trip tests for `ZKHALO2VERIFYWITHVK` (opcode `0xC7 0x4A`).
//!
//! Builds a v2 RLC [`Halo2TvmBundle`] from the checked-in deposit-prover
//! fixtures (`tvm_vm/halo2_test_data/deposit_rlc_{vk_blob,public_inputs,proof}.
//! bin` or the pre-assembled `deposit_rlc_bundle.bin`), pushes it as a single
//! cell onto the VM stack, runs the handler, and asserts the boolean result.
//!
//! Dark DEX W=128 proofs belong to the legacy `ZKHALO2VERIFY` opcode path
//! (`test_halo2.rs`); they use a different KZG ceremony than deposit proofs.

use tvm_types::SliceData;

use crate::executor::Engine;
use crate::executor::test_helper::*;
use crate::executor::zk_halo2_with_vk::execute_zkhalo2_verify_with_vk;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::savelist::SaveList;
use crate::utils::pack_data_to_cell;

const DEPOSIT_RLC_VK_BLOB: &str = "halo2_test_data/deposit_rlc_vk_blob.bin";
const DEPOSIT_RLC_INSTANCES: &str = "halo2_test_data/deposit_rlc_public_inputs.bin";
const DEPOSIT_RLC_PROOF: &str = "halo2_test_data/deposit_rlc_proof.bin";
const DEPOSIT_RLC_BUNDLE: &str = "halo2_test_data/deposit_rlc_bundle.bin";

fn parse_vk_blob_chunks(vk_blob: &[u8]) -> (Vec<u8>, Vec<u8>) {
    const VK_MAGIC: &[u8; 8] = b"VKBLOB\x00\x00";
    const HEADER_LEN: usize = 16;
    const LEN: usize = 4;
    assert!(vk_blob.starts_with(VK_MAGIC), "expected VKBLOB magic");
    let mut off = HEADER_LEN;
    let read_u32 = |buf: &[u8], o: &mut usize| -> usize {
        let n = u32::from_le_bytes(buf[*o..*o + LEN].try_into().unwrap()) as usize;
        *o += LEN;
        n
    };
    let cfg_len = read_u32(vk_blob, &mut off);
    let cfg = vk_blob[off..off + cfg_len].to_vec();
    off += cfg_len;
    let vk_len = read_u32(vk_blob, &mut off);
    let vk = vk_blob[off..off + vk_len].to_vec();
    (cfg, vk)
}

fn build_deposit_bundle(config: &[u8], vk: &[u8], instances: &[u8], proof: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(b"HALO2TVM");
    out.push(2); // BUNDLE_VERSION_V2
    out.push(0); // TRANSCRIPT_BLAKE2B
    out.push(1); // CIRCUIT_SHAPE_RLC
    out.extend_from_slice(&[0u8; 5]);
    for chunk in [config, vk, instances, proof] {
        out.extend_from_slice(&(chunk.len() as u32).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    out
}

fn load_deposit_bundle() -> Vec<u8> {
    let vk_blob = std::fs::read(DEPOSIT_RLC_VK_BLOB).expect("deposit_rlc_vk_blob.bin must exist");
    let (cfg, vk) = parse_vk_blob_chunks(&vk_blob);
    let instances = std::fs::read(DEPOSIT_RLC_INSTANCES).expect("deposit_rlc_public_inputs.bin");
    let proof = std::fs::read(DEPOSIT_RLC_PROOF).expect("deposit_rlc_proof.bin");
    build_deposit_bundle(&cfg, &vk, &instances, &proof)
}

fn setup_engine() -> Engine {
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
    Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls),
        Some(stack),
        None,
        vec![],
    )
}

fn run_with_bundle(bundle_bytes: &[u8]) -> tvm_types::Status {
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(bundle_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));
    execute_zkhalo2_verify_with_vk(&mut engine)?;
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    if res { Ok(()) } else { Err(tvm_types::error!("verifier returned false")) }
}

#[test]
fn round_trip_deposit_rlc_real_proof_returns_true() {
    let bundle = std::fs::read(DEPOSIT_RLC_BUNDLE).expect("deposit_rlc_bundle.bin must exist");
    run_with_bundle(&bundle).expect("pre-assembled deposit RLC bundle must verify true");
}

#[test]
fn round_trip_deposit_rlc_assembled_bundle_returns_true() {
    run_with_bundle(&load_deposit_bundle())
        .expect("deposit RLC bundle assembled from triple must verify true");
}

#[test]
fn flipped_proof_byte_rejected_as_false() {
    let mut bundle = load_deposit_bundle();
    let mid = bundle.len() / 2;
    bundle[mid] ^= 0xFF;

    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    match execute_zkhalo2_verify_with_vk(&mut engine) {
        Ok(()) => {
            let res = engine.cc.stack.get(0).as_bool().unwrap();
            assert!(!res, "flipped proof byte must NOT verify as true");
        }
        Err(_) => {}
    }
}

#[test]
fn tweaked_instance_fr_rejected_as_false() {
    let vk_blob = std::fs::read(DEPOSIT_RLC_VK_BLOB).unwrap();
    let (cfg, vk) = parse_vk_blob_chunks(&vk_blob);
    let mut instances = std::fs::read(DEPOSIT_RLC_INSTANCES).unwrap();
    let proof = std::fs::read(DEPOSIT_RLC_PROOF).unwrap();
    instances[0] ^= 0x01;

    let bundle = build_deposit_bundle(&cfg, &vk, &instances, &proof);
    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    execute_zkhalo2_verify_with_vk(&mut engine).expect("handler must not fatal on valid Fr");
    let res = engine.cc.stack.get(0).as_bool().unwrap();
    assert!(!res, "tweaked instance Fr must NOT verify as true");
}

#[test]
fn bad_magic_returns_fatal_error() {
    let mut bundle = load_deposit_bundle();
    bundle[0] = b'X';

    let mut engine = setup_engine();
    let cell = pack_data_to_cell(&bundle, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(cell));

    let err = execute_zkhalo2_verify_with_vk(&mut engine)
        .expect_err("bad bundle magic must trigger FatalError");
    assert!(
        err.to_string().contains("magic mismatch"),
        "expected magic mismatch error, got: {err}"
    );
}

#[test]
fn lru_cache_reused_across_two_invocations() {
    let bundle = load_deposit_bundle();
    for _ in 0..2 {
        run_with_bundle(&bundle).expect("identical deposit bundle must verify on every call");
    }
}
