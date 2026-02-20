use gosh_dark_dex_halo2_circuit::circuit::*;
use gosh_dark_dex_halo2_circuit::snark_utils::*;
use gosh_dark_dex_halo2_circuit::proof::*;
use crate::executor::Engine;
use crate::stack::StackItem;
use crate::stack::StackItem::Cell;
use crate::stack::integer::IntegerData;
use crate::stack::integer::serialization::UnsignedIntegerBigEndianEncoding;
use crate::types::Exception;
use crate::types::Status;
use crate::error::TvmError;
use crate::executor::zk_halo2_utils::*;

use tvm_types::SliceData;
use crate::executor::engine::storage::fetch_stack;
use crate::utils::pack_data_to_cell;
use crate::utils::unpack_data_from_cell;
use crate::utils::unpack_string_from_cell;
use std::io::Cursor;
use halo2_base::halo2_proofs::{
    arithmetic::CurveAffine,
    halo2curves::{bn256::{Fr, Bn256}},
    plonk::Fixed,
};

use tvm_types::error;
use tvm_types::fail;

use tvm_types::ExceptionCode;


use halo2_base::utils::ScalarField;

use hex::*;

use halo2_base::halo2_proofs::{
    circuit::Layouter,
    circuit::SimpleFloorPlanner,
    circuit::Value,
    dev::MockProver,
    halo2curves::bn256,
    halo2curves::secp256k1,
    plonk::{self, Advice, Circuit, Column, ConstraintSystem, Expression, Instance, Selector, ProvingKey, VerifyingKey, keygen_vk, keygen_pk, create_proof, verify_proof},
    poly::{
        commitment::ParamsProver,
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::VerifierSHPLONK,
        },
        kzg::{multiopen::ProverSHPLONK, strategy::SingleStrategy},
    },
    transcript::{
        Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
    },
    SerdeFormat
};

use halo2_base::gates::circuit::builder::BaseCircuitBuilder;
use halo2_base::gates::circuit::BaseCircuitParams;

fn pop(barry: &[u8]) -> &[u8; 8] {
    barry.try_into().expect("slice with incorrect length")
}

pub fn consume_uint64(b: &[u8]) -> tvm_types::Result<u64> {
    if b.len() != 32 {
        fail!("Not enough bytes for u64");
    }
    let result = (b[31] as u64)
        | ((b[30] as u64) << 8)
        | ((b[29] as u64) << 16)
        | ((b[28] as u64) << 24)
        | ((b[27] as u64) << 32)
        | ((b[26] as u64) << 40)
        | ((b[25] as u64) << 48)
        | ((b[24] as u64) << 56)
        ;
    
    Ok(result)
}

pub fn check_is_u64(b: &[u8]) -> tvm_types::Result<bool> {
    if b.len() != 32 {
        fail!("Not enough bytes");
    }
    for i in 0..24 {
        if (b[i] != 0){
            return Ok(false);
        }
    }
    Ok(true)
}

//Note: for now this instruction works with circuits handling only one Column of public inputs
pub(crate) fn execute_halo2_proof_verification(engine: &mut Engine) -> Status {
    let concrete_params: BaseCircuitParams = BaseCircuitParams{
        k: 12,
        num_advice_per_phase: vec![4],
        num_fixed: 1,
        num_lookup_advice_per_phase: vec![1, 0, 0],
        lookup_bits: Some(11),
        num_instance_columns: 1
    }; 

    engine.load_instruction(crate::executor::types::Instruction::new("ZKHALO2VERIFY"))?;
    
    fetch_stack(engine, 2)?;
    let proof_slice = SliceData::load_cell_ref(engine.cmd.var(0).as_cell()?)?;
    let proof = unpack_data_from_cell(proof_slice, engine)?;
    println!("proof: {:?}", hex::encode(proof.clone()));
    let proof = Proof::new(proof);

    let pub_inputs_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let pub_inputs_bytes = unpack_data_from_cell(pub_inputs_slice, engine)?;
    println!("pub_inputs_bytes len: {:?}", pub_inputs_bytes.len());

    if (pub_inputs_bytes.len() % 32 != 0) {
        fail!(ExceptionCode::FatalError);
    }
    let num_of_pub_inputs = pub_inputs_bytes.len()  / 32;
    let mut pub_inputs: Vec<Fr> = Vec::new();
    for i in 0..num_of_pub_inputs {
        let pub_input_bytes: &[u8; 32] = &pub_inputs_bytes[i*32..(i+1)*32].try_into().unwrap();
        println!("portion of pub_input_bytes: {:?}", pub_input_bytes);
        let check_pub_input_bytes_is_u64 = match check_is_u64(pub_input_bytes) {
            Ok(check_res) => { check_res }
            Err(err) => { fail!("Invalid length {}", err) }
        };
        if (check_pub_input_bytes_is_u64) {
            let elem: u64 = match consume_uint64(pub_input_bytes){
                Ok(el) => { el }
                Err(err) => { fail!("Invalid length {}", err) }
            };
            let pub_input = Fr::from(elem as u64);
            pub_inputs.push(pub_input);
        }
        else {
            //TODO: take care later of splitting pub_input_bytes, it may be tooo big (with small probability however)
            let pub_input = Fr::from_bytes_le(pub_input_bytes);
            pub_inputs.push(pub_input);
        }
    }

    println!("pub_inputs: {:?}", pub_inputs);
    let mut cursor = Cursor::new(KZG_PARAMS.to_vec());
    let params = match ParamsKZG::<Bn256>::read_custom(&mut cursor, SerdeFormat::RawBytesUnchecked){
        Ok(params) => params,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect KZG params {}", err);
        }
    };

    let res = proof.verify_with_vk_from_bytes::<BaseCircuitBuilder<Fr>>(&mut DARK_DEX_VERIFICATION_HALO2_KEY, &params, concrete_params, &[&pub_inputs]);
    
    let res =  boolean!(res);
    engine.cc.stack.push(res);

    Ok(())
}