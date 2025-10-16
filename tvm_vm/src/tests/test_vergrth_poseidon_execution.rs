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
use std::collections::HashMap;
use std::time::Instant;

use base64::decode;
use base64ct::Encoding as bEncoding;
use ed25519_dalek::Signer;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::KeyPair;
use fastcrypto::traits::ToFromBytes;
use num_bigint::BigUint;
use num_traits::Zero;
use rand::rngs::OsRng;
use serde_json::Value;
use tvm_types::Cell;
use tvm_types::SliceData;

use crate::executor::crypto::execute_chksigns;
use crate::executor::engine::Engine;
use crate::executor::test_data::*;
use crate::executor::test_helper::*;
use crate::executor::zk::execute_poseidon_zk_login;
use crate::executor::zk::execute_vergrth16;
use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::utils::gen_address_seed;
use crate::executor::zk_stuff::zk_login::CanonicalSerialize;
use crate::executor::zk_stuff::zk_login::JWK;
use crate::executor::zk_stuff::zk_login::JwkId;
use crate::executor::zk_stuff::zk_login::OIDCProvider;
use crate::executor::zk_stuff::zk_login::ZkLoginInputs;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::savelist::SaveList;
use crate::utils::pack_data_to_cell;
use crate::utils::pack_string_to_cell;
use crate::utils::unpack_data_from_cell;

const FACEBOOK: &str = "facebook";
const CREDENZA3: &str = "credenza3";
const APPLE: &str = "apple";
const TWITCH: &str = "twitch";
const KAKAO: &str = "kakao";
const SLACK: &str = "slack";
const KARRIER_ONE: &str = "karrier_one";
const MICROSOFT: &str = "microsoft";

fn single_chcksgns(
    engine: &mut Engine,
    eph_pubkey: &Vec<u8>,
    zk_login_inputs: &ZkLoginInputs,
    all_jwk: &HashMap<JwkId, JWK>,
    max_epoch: u64,
) -> u128 {
    let (proof_cell, public_inputs_cell) =
        crate::executor::test_helper::prepare_proof_and_public_key_cells_for_stack(
            eph_pubkey,
            zk_login_inputs,
            all_jwk,
            max_epoch,
        );

    let mut csprng = OsRng;
    let signing_key: ed25519_dalek::SigningKey = ed25519_dalek::SigningKey::generate(&mut csprng);
    let binding = proof_cell.clone();
    let first = binding.data();
    let binding = public_inputs_cell.clone();
    let second = binding.data();
    let concatenated = [&first[..], &second[..]].concat();

    let test_cell: Cell = pack_data_to_cell(&concatenated, &mut 0).unwrap();
    let signature = signing_key.sign(test_cell.data()).to_bytes().to_vec();
    let len = signature.len() * 8;
    let signature = SliceData::from_raw(signature, len).into_cell();

    engine.cc.stack.push(StackItem::Slice(SliceData::load_cell(test_cell.clone()).unwrap()));
    engine.cc.stack.push(StackItem::Slice(SliceData::load_cell(signature.clone()).unwrap()));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(
        &signing_key.verifying_key().as_bytes().to_vec().clone(),
    )));

    let start: Instant = Instant::now();
    execute_chksigns(engine).unwrap();
    let chksigns_elapsed = start.elapsed().as_micros();

    println!("chksigns_elapsed in microsecond: {:?}", chksigns_elapsed);

    let res = engine.cc.stack.get(0).as_integer().unwrap();
    println!("res: {:?}", res);
    assert!(*res == IntegerData::minus_one());

    chksigns_elapsed
}

#[test]
fn test_poseidon_and_vergrth16_and_chksigns_for_multiple_data() {
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

    let data: Vec<&str> = vec![
        TEST_AUTH_DATA_0_GOOGLE,
        TEST_AUTH_DATA_1_GOOGLE,
        TEST_AUTH_DATA_2_GOOGLE,
        TEST_AUTH_DATA_3_GOOGLE,
        TEST_AUTH_DATA_4_GOOGLE,
        TEST_AUTH_DATA_5_GOOGLE,
        TEST_AUTH_DATA_6_GOOGLE,
        TEST_AUTH_DATA_7_GOOGLE,
        TEST_AUTH_DATA_8_GOOGLE,
        TEST_AUTH_DATA_9_GOOGLE,
        TEST_AUTH_DATA_10_GOOGLE,
        TEST_AUTH_DATA_11_GOOGLE,
        TEST_AUTH_DATA_12_GOOGLE,
        TEST_AUTH_DATA_13_GOOGLE,
        TEST_AUTH_DATA_14_GOOGLE,
        TEST_AUTH_DATA_15_GOOGLE,
        TEST_AUTH_DATA_16_GOOGLE,
        TEST_AUTH_DATA_17_GOOGLE,
        TEST_AUTH_DATA_18_GOOGLE,
        TEST_AUTH_DATA_19_GOOGLE,
        TEST_AUTH_DATA_20_GOOGLE,
        TEST_AUTH_DATA_21_GOOGLE,
        TEST_AUTH_DATA_1_FACEBOOK,
        TEST_AUTH_DATA_2_FACEBOOK,
        /*TEST_AUTH_DATA_1_KAKAO*/
        /*
        TEST_AUTH_DATA_1_APPLE,
        TEST_AUTH_DATA_1_TWITCH,
        TEST_AUTH_DATA_1_SLACK,
        TEST_AUTH_DATA_1_MICROSOFT,*/
    ];

    let mut average_poseidon: u128 = 0;
    let mut average_vergrth16: u128 = 0;
    let mut average_chcksigns: u128 = 0;

    for i in 0..data.len() {
        println!("");
        println!("====================== Iter@ is {i} =========================");
        println!("jwt_data: {:?}", data[i]);
        let jwt_data: JwtData = serde_json::from_str(&data[i]).unwrap();
        println!("jwt_data: {:?}", jwt_data);

        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: jwt_data.modulus,
            alg: "RS256".to_string(),
        };

        let provider = jwt_data.provider.as_str();
        println!("provider: {:?}", provider);

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                match provider {
                    FACEBOOK => OIDCProvider::Facebook.get_config().iss,
                    CREDENZA3 => OIDCProvider::Credenza3.get_config().iss,
                    APPLE => OIDCProvider::Apple.get_config().iss,
                    TWITCH => OIDCProvider::Twitch.get_config().iss,
                    KAKAO => OIDCProvider::Kakao.get_config().iss,
                    SLACK => OIDCProvider::Slack.get_config().iss,
                    KARRIER_ONE => OIDCProvider::KarrierOne.get_config().iss,
                    MICROSOFT => OIDCProvider::Microsoft.get_config().iss,
                    _ => OIDCProvider::Google.get_config().iss,
                },
                jwt_data.kid,
            ),
            content,
        );

        let user_pass_salt = jwt_data.user_pass_to_int_format.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        let eph_pubkey =
            secret_key_from_integer_map(jwt_data.ephemeral_key_pair.keypair.public_key);
        println!("ephemeral public_key is {:?}", eph_pubkey);
        println!("ephemeral public_key len is {:?}", eph_pubkey.len());
        let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is

        let jwt_data_decoded1: JwtDataDecodedPart1Common =
            serde_json::from_str(&jwt_string_1).unwrap();
        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2);

        let jwt_data_decoded2: JwtDataDecodedPart2Common =
            serde_json::from_str(&jwt_string_2).unwrap();
        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(),
            jwt_data_decoded2.aud.as_str(),
        )
        .unwrap();

        println!("jwt_data.zk_proofs = {:?}", jwt_data.zk_proofs);
        let proof_and_jwt = serde_json::to_string(&jwt_data.zk_proofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string()).unwrap();

        let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
        let jwk = all_jwk
            .get(&JwkId::new(iss.clone(), kid.clone()))
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
            .unwrap();

        let max_epoch = jwt_data.max_epoch;
        println!("max_epoch = {:?}", max_epoch);

        let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|_| {
                ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
            })
            .unwrap();

        let public_inputs =
            &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

        println!("modulus_ hex = {:?}", hex::encode(modulus.clone()));

        let mut public_inputs_as_bytes = vec![];
        public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
        println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
        println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

        println!("====== Start Poseidon ========");

        let index_mod_4 = jwt_data.zk_proofs.iss_base64_details.index_mod4;
        println!("index_mod4 = {:?}", jwt_data.zk_proofs.iss_base64_details.index_mod4);
        engine.cc.stack.push(StackItem::int(index_mod_4));
        engine.cc.stack.push(StackItem::int(max_epoch));
        println!(
            "IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())) : {:?} ",
            IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())
        );
        engine
            .cc
            .stack
            .push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

        let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
        println!("modulus_cell = {:?}", modulus_cell);
        engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));

        let iss_base_64 = jwt_data.zk_proofs.iss_base64_details.value;

        println!("iss_base_64 = {:?}", iss_base_64);

        let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
        println!("iss_base_64_cell = {:?}", iss_base_64_cell);
        engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));

        let header_base_64 = jwt_data.zk_proofs.header_base64;

        println!("header_base_64 = {:?}", header_base_64);

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
        println!("header_base_64_cell = {:?}", header_base_64_cell);
        engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));

        let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
        println!("zk_seed_cell = {:?}", zk_seed_cell);
        engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

        let start: Instant = Instant::now();
        let _ = execute_poseidon_zk_login(&mut engine).unwrap();
        let poseidon_elapsed = start.elapsed().as_micros();

        let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
        let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
        let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
        println!("poseidon_res from stack: {:?}", hex::encode(poseidon_res.clone()));

        println!(
            "public_inputs hex (computed in test): {:?}",
            hex::encode(public_inputs_as_bytes.clone())
        );
        assert!(poseidon_res == public_inputs_as_bytes);

        println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);

        average_poseidon = average_poseidon + poseidon_elapsed;

        println!("====== Start VERGRTH16 ========");
        let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes : {:?}", hex::encode(proof_as_bytes.clone()));
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

        let public_inputs_cell =
            pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

        let start: Instant = Instant::now();
        let _ = execute_vergrth16(&mut engine).unwrap();
        let vergrth16_elapsed = start.elapsed().as_micros();

        println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed);

        let res = engine.cc.stack.get(0).as_integer().unwrap();
        println!("res: {:?}", res);
        assert!(*res == IntegerData::minus_one());

        average_vergrth16 = average_vergrth16 + vergrth16_elapsed;

        let time_for_chcksgns =
            single_chcksgns(&mut engine, &eph_pubkey, &zk_login_inputs, &all_jwk, max_epoch);
        println!("time_for_chcksgns is {time_for_chcksgns} micro seconds");
        average_chcksigns = average_chcksigns + time_for_chcksgns;
    }

    println!("===================================");
    println!("===================================");
    println!("===================================");
    println!("===================================");

    let average_poseidon_ = average_poseidon / (data.len() as u128);
    println!("average_poseidon_ in microsecond: {:?}", average_poseidon_);
    let average_vergrth16_ = average_vergrth16 / (data.len() as u128);
    println!("average_vergrth16_ in microsecond: {:?}", average_vergrth16_);
    let average_chcksigns_ = average_chcksigns / (data.len() as u128);
    println!("average_chcksigns_ in microsecond: {:?}", average_chcksigns_);
}

#[test]
fn test_poseidon_and_vergrth16_and_for_multiple_data_short() {
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

    let data: Vec<&str> = vec![TEST_AUTH_DATA_SHORT_GOOGLE];
    let mut average_poseidon: u128 = 0;
    let mut average_vergrth16: u128 = 0;

    for i in 0..data.len() {
        println!("");
        println!("====================== Iter@ is {i} =========================");
        println!("jwt_data: {:?}", data[i]);
        let jwt_data: JwtDataShort = serde_json::from_str(&data[i]).unwrap();
        println!("jwt_data: {:?}", jwt_data);

        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: jwt_data.modulus,
            alg: "RS256".to_string(),
        };

        let mut all_jwk = HashMap::new();
        all_jwk.insert(JwkId::new(OIDCProvider::Google.get_config().iss, jwt_data.kid), content);

        let user_pass_salt = jwt_data.user_pass_to_int_format.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        let eph_pubkey = secret_key_from_integer_map(jwt_data.ephemeral_public_key);
        println!("ephemeral public_key is {:?}", eph_pubkey);

        let eph_pubkey_len = eph_pubkey.clone().len();
        println!("len eph_pubkey: {:?}", eph_pubkey_len);

        let zk_seed =
            gen_address_seed(user_pass_salt, "sub", &jwt_data.value, &jwt_data.aud).unwrap();

        println!("jwt_data.zk_proofs = {:?}", jwt_data.zk_proofs);
        let proof_and_jwt = serde_json::to_string(&jwt_data.zk_proofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string()).unwrap();

        let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
        let jwk = all_jwk
            .get(&JwkId::new(iss.clone(), kid.clone()))
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
            .unwrap();

        let max_epoch = jwt_data.max_epoch;

        let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|_| {
                ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
            })
            .unwrap();

        let public_inputs =
            &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

        println!("modulus_ hex = {:?}", hex::encode(modulus.clone()));

        let mut public_inputs_as_bytes = vec![];
        public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
        println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
        println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

        println!("====== Start Poseidon ========");

        let index_mod_4 = jwt_data.zk_proofs.iss_base64_details.index_mod4;
        engine.cc.stack.push(StackItem::int(index_mod_4));
        engine.cc.stack.push(StackItem::int(max_epoch));
        engine
            .cc
            .stack
            .push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

        let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
        println!("modulus_cell = {:?}", modulus_cell);
        engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));

        let iss_base_64 = jwt_data.zk_proofs.iss_base64_details.value;

        let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
        println!("iss_base_64_cell = {:?}", iss_base_64_cell);
        engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));

        let header_base_64 = jwt_data.zk_proofs.header_base64;

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
        println!("header_base_64_cell = {:?}", header_base_64_cell);
        engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));

        let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
        println!("zk_seed_cell = {:?}", zk_seed_cell);
        engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

        let start: Instant = Instant::now();
        let _ = execute_poseidon_zk_login(&mut engine).unwrap();
        let poseidon_elapsed = start.elapsed().as_micros();

        let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
        let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
        let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
        println!("poseidon_res from stack: {:?}", hex::encode(poseidon_res.clone()));

        println!(
            "public_inputs hex (computed in test): {:?}",
            hex::encode(public_inputs_as_bytes.clone())
        );
        assert!(poseidon_res == public_inputs_as_bytes);

        println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);

        average_poseidon = average_poseidon + poseidon_elapsed;

        println!("====== Start VERGRTH16 ========");
        let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes : {:?}", hex::encode(proof_as_bytes.clone()));
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

        let public_inputs_cell =
            pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
        engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

        let start: Instant = Instant::now();
        let _ = execute_vergrth16(&mut engine).unwrap();
        let vergrth16_elapsed = start.elapsed().as_micros();

        println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed);

        let res = engine.cc.stack.get(0).as_integer().unwrap();
        println!("res: {:?}", res);
        assert!(*res == IntegerData::minus_one());

        average_vergrth16 = average_vergrth16 + vergrth16_elapsed;
    }

    println!("===================================");
    println!("===================================");
    println!("===================================");
    println!("===================================");

    let average_poseidon_ = average_poseidon / (data.len() as u128);
    println!("average_poseidon_ in microsecond: {:?}", average_poseidon_);
    let average_vergrth16_ = average_vergrth16 / (data.len() as u128);
    println!("average_vergrth16_ in microsecond: {:?}", average_vergrth16_);
}

#[test]
fn test_proof_stuff() {
    let data = [
        TEST_AUTH_DATA_1_GOOGLE,
        TEST_AUTH_DATA_2_GOOGLE,
        TEST_AUTH_DATA_3_GOOGLE,
        TEST_AUTH_DATA_4_GOOGLE,
        TEST_AUTH_DATA_5_GOOGLE,
        TEST_AUTH_DATA_6_GOOGLE,
        TEST_AUTH_DATA_7_GOOGLE,
        TEST_AUTH_DATA_8_GOOGLE,
        TEST_AUTH_DATA_9_GOOGLE,
        TEST_AUTH_DATA_10_GOOGLE,
        TEST_AUTH_DATA_11_GOOGLE,
        TEST_AUTH_DATA_12_GOOGLE,
        TEST_AUTH_DATA_13_GOOGLE,
        TEST_AUTH_DATA_14_GOOGLE,
        TEST_AUTH_DATA_15_GOOGLE,
        TEST_AUTH_DATA_16_GOOGLE,
        TEST_AUTH_DATA_17_GOOGLE,
        TEST_AUTH_DATA_18_GOOGLE,
        TEST_AUTH_DATA_19_GOOGLE,
        TEST_AUTH_DATA_20_GOOGLE,
        TEST_AUTH_DATA_21_GOOGLE,
    ];

    for i in 0..data.len() {
        let jwt_data: JwtData = serde_json::from_str(&data[i]).unwrap();

        let user_pass_salt = jwt_data.user_pass_to_int_format.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2);
        let jwt_data_decoded2: JwtDataDecodedPart2Google =
            serde_json::from_str(&jwt_string_2).unwrap();
        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(),
            jwt_data_decoded2.aud.as_str(),
        )
        .unwrap();

        let proof_and_jwt = serde_json::to_string(&jwt_data.zk_proofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string()).unwrap();

        let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();

        println!("proof.a: {:?}", proof.a);
        println!("proof.a.y.0.to_string(): {:?}", proof.a.y.0.to_string());

        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());
        println!("----------------------------------");

        let json_string = serde_json::to_string(&jwt_data.zk_proofs).unwrap();
        println!("json_string ={:?}", json_string); // jwt_data.zk_proofs);

        let data: Value = serde_json::from_str(&*json_string).unwrap();
        println!("data = {:?}", data);

        let a_x = data["proofPoints"]["a"][0].as_str().unwrap();
        let a_y =
            BigUint::parse_bytes(data["proofPoints"]["a"][1].as_str().unwrap().as_bytes(), 10)
                .unwrap();
        println!("a_x = {:?}", a_x);
        println!("a_y = {:?}", a_y);

        let b0_x = data["proofPoints"]["b"][0][0].as_str().unwrap();
        let b1_x = data["proofPoints"]["b"][0][1].as_str().unwrap();
        let b1_y =
            BigUint::parse_bytes(data["proofPoints"]["b"][1][1].as_str().unwrap().as_bytes(), 10)
                .unwrap();
        println!("b0_x = {:?}", b0_x);
        println!("b1_x = {:?}", b1_x);
        println!("b1_y = {:?}", b1_y);

        let c_x = data["proofPoints"]["c"][0].as_str().unwrap();
        let c_y =
            BigUint::parse_bytes(data["proofPoints"]["c"][1].as_str().unwrap().as_bytes(), 10)
                .unwrap();
        println!("c_x = {:?}", c_x);
        println!("c_y = {:?}", c_y);

        let hex_ax = prepare_hex_representation(a_x, a_y);
        let hex_b0x = prepare_hex_representation(b0_x, BigUint::zero());
        let hex_b1x = prepare_hex_representation(b1_x, b1_y);
        let hex_cx = prepare_hex_representation(c_x, c_y);

        let result = format!("{}{}{}{}", hex_ax, hex_b0x, hex_b1x, hex_cx);

        println!("Serialized proof _ 0: {:?}", result);

        println!("Serialized proof _ 1: {:?}", hex::encode(&proof_as_bytes));

        assert_eq!(result, hex::encode(&proof_as_bytes));

        println!("===================");
    }
}

#[test]
fn test_poseidon() {
    let mut stack = Stack::new();

    // password was 567890 in ascii 535455565748
    let user_pass_salt = "535455565748";
    let secret_key = [
        222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166,
        87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // 
    let mut eph_pubkey = Vec::new();
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());
    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103",
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    )
    .unwrap();
    println!("zk_seed = {:?}", zk_seed);
    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

    println!("iss_and_header_base64details: {}", iss_and_header_base64details);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(), // Alina's data
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    let max_epoch = 142;

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string()))
        .unwrap();

    let public_inputs =
        &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
    println!("====== Start Poseidon ========");

    let index_mod_4 = 1;
    stack.push(StackItem::int(index_mod_4));
    stack.push(StackItem::int(max_epoch));
    stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
    println!("modulus_cell = {:?}", modulus_cell);
    stack.push(StackItem::cell(modulus_cell.clone()));

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
    println!("iss_base_64_cell = {:?}", iss_base_64_cell);
    stack.push(StackItem::cell(iss_base_64_cell.clone()));

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();
    println!("header_base_64_cell = {:?}", header_base_64_cell);
    stack.push(StackItem::cell(header_base_64_cell.clone()));

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
    println!("zk_seed_cell = {:?}", zk_seed_cell);
    stack.push(StackItem::cell(zk_seed_cell.clone()));

    let start: Instant = Instant::now();

    let mut res = Vec::<u8>::with_capacity(2);
    res.push(0xC7);
    res.push(0x32);
    res.push(0x80);

    let code = SliceData::new(res);

    let mut engine =
        Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);
    let _ = engine.execute();
    let poseidon_elapsed = start.elapsed().as_micros();

    println!("poseidon_elapsed in microsecond: {:?}", poseidon_elapsed);

    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    let slice = SliceData::load_cell(poseidon_res.clone()).unwrap();
    let poseidon_res = unpack_data_from_cell(slice, &mut engine).unwrap();
    println!("poseidon_res from stack: {:?}", poseidon_res.clone());

    println!("public_inputs hex (computed in test): {:?}", public_inputs_as_bytes.clone());
    assert!(poseidon_res == public_inputs_as_bytes);
}

#[test]
fn test_vergrth16() {
    let mut stack = Stack::new();

    // password was 567890 in ascii 535455565748
    let user_pass_salt = "535455565748";
    let secret_key = [
        222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166,
        87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // 
    let mut eph_pubkey = Vec::new();
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());
    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103",
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    )
    .unwrap();
    println!("zk_seed = {:?}", zk_seed);
    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

    println!("iss_and_header_base64details: {}", iss_and_header_base64details);

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(), // Alina's data
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    let max_epoch = 142;

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string()))
        .unwrap();

    let public_inputs =
        &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    println!("====== Start VERGRTH16 ========");
    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    stack.push(StackItem::cell(public_inputs_cell.clone()));

    let start: Instant = Instant::now();

    let mut res = Vec::<u8>::with_capacity(2);
    res.push(0xC7);
    res.push(0x31);
    res.push(0x80);

    let code = SliceData::new(res);

    let mut engine =
        Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);
    let _ = engine.execute().unwrap();
    // let status = execute_vergrth16(&mut engine).unwrap();
    let vergrth16_elapsed = start.elapsed().as_micros();

    println!("vergrth16_elapsed in microsecond: {:?}", vergrth16_elapsed);

    let res = engine.cc.stack.get(0).as_integer().unwrap();
    println!("res: {:?}", res);
    // assert!(*res == IntegerData::minus_one());
}
