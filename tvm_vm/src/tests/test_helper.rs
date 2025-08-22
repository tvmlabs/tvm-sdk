#![allow(unused)]

use std::collections::HashMap;
use std::iter::repeat;

use base64ct::Encoding as bEncoding;
use num_bigint::BigUint;
use serde::Deserialize;
use serde_derive::Serialize;
use tvm_types::Cell;

use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::zk_login::CanonicalSerialize;
use crate::executor::zk_stuff::zk_login::JWK;
use crate::executor::zk_stuff::zk_login::JwkId;
use crate::executor::zk_stuff::zk_login::ZkLoginInputs;
use crate::utils::pack_data_to_cell;

pub static DEFAULT_CAPABILITIES: u64 = 0x572e;

pub fn read_boc(filename: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut file = std::fs::File::open(filename).unwrap();
    std::io::Read::read_to_end(&mut file, &mut bytes).unwrap();
    bytes
}

pub fn load_boc(filename: &str) -> tvm_types::Cell {
    let bytes = read_boc(filename);
    tvm_types::read_single_root_boc(bytes).unwrap()
}

pub fn secret_key_from_integer_map(key_data: HashMap<String, u8>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..=key_data.len()/*31*//*32*/ {
        if let Some(value) = key_data.get(&i.to_string()) {
            vec.push(value.clone());
        }
    }
    return vec;
}

#[derive(Debug, Deserialize)]
pub struct EphemeralKeyPair {
    pub keypair: Keypair,
}

#[derive(Debug, Deserialize)]
pub struct Keypair {
    pub public_key: HashMap<String, u8>,
    pub secret_key: HashMap<String, u8>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ZkProofs {
    pub proof_points: ProofPoints,
    pub iss_base64_details: IssBase64Details,
    pub header_base64: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofPoints {
    pub a: Vec<String>,
    pub b: Vec<Vec<String>>,
    pub c: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssBase64Details {
    pub value: String,
    pub index_mod4: i32,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataShort {
    pub provider: String,
    pub user_pass_to_int_format: String,
    pub ephemeral_public_key: HashMap<String, u8>,
    pub value: String,
    pub aud: String,
    pub zk_proofs: ZkProofs,
    pub modulus: String,
    pub kid: String,
    pub max_epoch: u64,
    pub verification_key_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct JwtData {
    pub provider: String,
    pub jwt: String,
    pub user_pass_to_int_format: String,
    pub ephemeral_key_pair: EphemeralKeyPair,
    // pub zk_addr: String,
    pub zk_proofs: ZkProofs,
    // pub extended_ephemeral_public_key: String,
    pub modulus: String,
    pub kid: String,
    pub max_epoch: u64,
    pub verification_key_id: u32,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart1 {
    pub alg: String,
    pub kid: String,
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart1Common {
    pub alg: String,
    pub kid: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Google {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub nbf: u32,
    pub iat: u32,
    pub exp: u32,
    pub jti: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Facebook {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub iat: u32,
    pub exp: u32,
    pub jti: String,
    pub given_name: String,
    pub family_name: String,
    pub name: String,
    pub picture: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Credenza3 {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub scope: String,
    pub nonce: String,
    pub iat: u32,
    pub exp: u32,
    pub token_type: String,
    pub token_use: String,
    pub login_type: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Apple {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub iat: u32,
    pub exp: u32,
    pub c_hash: String,
    pub auth_time: u32,
    pub nonce_supported: bool,
}

pub struct JwtDataDecodedPart2Twitch {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub iat: u32,
    pub exp: u32,
    pub azp: String,
    pub preferred_username: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Kakao {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub iat: u32,
    pub exp: u32,
    pub auth_time: u32,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Microsoft {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub iat: u32,
    pub nbf: u32,
    pub exp: u32,
    pub aio: String,
    pub rh: String,
    pub tid: String,
    pub uti: String,
    pub ver: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Slack {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub nonce: String,
    pub iat: u32,
    pub exp: u32,
    pub at_hash: String,
    pub auth_time: u32,
    pub nonce_supported: bool,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2KarrierOne {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub name: String,
    pub nonce: String,
    pub iat: u32,
    pub exp: u32,
    pub preferred_username: String,
    pub oi_au_id: String,
    pub azp: String,
    pub oi_tkn_id: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2Common {
    pub aud: String,
    pub sub: String,
}

fn to_binary_string(value: &str) -> String {
    let big_value = BigUint::parse_bytes(value.as_bytes(), 10).unwrap();
    big_value.to_str_radix(2)
}

fn pad_string_to_256(input: &str) -> String {
    let current_length = input.len();

    if current_length >= 256 {
        return input.to_string();
    }

    let zeros_to_add = 256 - current_length;
    format!("{}{}", repeat('0').take(zeros_to_add).collect::<String>(), input)
}

fn bits_to_decimal_and_reverse(bits: &str) -> Vec<u8> {
    let byte_chunks: Vec<&str> =
        bits.as_bytes().chunks(8).map(|chunk| std::str::from_utf8(chunk).unwrap()).collect();

    let decimal_numbers: Vec<u8> =
        byte_chunks.iter().map(|byte| u8::from_str_radix(byte, 2).unwrap()).collect();

    decimal_numbers.into_iter().rev().collect()
}

pub fn prepare_hex_representation(init_x: &str, y: BigUint) -> String {
    let mut binary_representation = pad_string_to_256(&to_binary_string(init_x));

    let p: BigUint = BigUint::from_bytes_be(&[
        48, 100, 78, 114, 225, 49, 160, 41, 184, 80, 69, 182, 129, 129, 88, 93, 151, 129, 106, 145,
        104, 113, 202, 141, 60, 32, 140, 22, 216, 124, 253, 71,
    ]);

    if y > &p - &y {
        binary_representation.replace_range(0..1, "1");
    }

    let reversed_byte_array = bits_to_decimal_and_reverse(&binary_representation);

    let hex_string =
        reversed_byte_array.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    hex_string
}

pub fn prepare_proof_and_public_key_cells_for_stack(
    eph_pubkey: &Vec<u8>,
    zk_login_inputs: &ZkLoginInputs,
    all_jwk: &HashMap<JwkId, JWK>,
    max_epoch: u64,
) -> (Cell, Cell) {
    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    println!("kid = {}", kid);
    println!("all_jwk = {:?}", all_jwk);

    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    // Decode modulus to bytes.
    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string()))
        .unwrap();

    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
    let public_inputs =
        &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    (proof_cell, public_inputs_cell)
}
