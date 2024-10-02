// Copyright (C) 2019-2023 TON Labs. All Rights Reserved.
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
use std::iter::repeat;
use std::time::Instant;

use base64ct::Encoding as bEncoding;
use ed25519::signature::Signer;
use fastcrypto_zkp::bn254::zk_login::CanonicalSerialize;
use fastcrypto_zkp::bn254::zk_login::JwkId;
use fastcrypto_zkp::bn254::zk_login::ZkLoginInputs;
use fastcrypto_zkp::bn254::zk_login::JWK;
use num_bigint::BigUint;
use serde::Deserialize;
use serde_derive::Serialize;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::SliceData;
use tvm_vm::executor::zk_stuff::error::ZkCryptoError;
use tvm_vm::int;
use tvm_vm::stack::integer::IntegerData;
use tvm_vm::stack::Stack;
use tvm_vm::stack::StackItem;
use tvm_vm::utils::pack_data_to_cell;

use crate::test_framework::test_case_with_refs;
use crate::test_framework::Expects;

#[derive(Debug, Deserialize)]
pub struct JwtData {
    jwt: String,
    userPassToIntFormat: String,
    ephemeralKeyPair: EphemeralKeyPair,
    zkAddr: String,
    zkProofs: ZkProofs,
    extendedEphemeralPublicKey: String,
}

#[derive(Debug, Deserialize)]
pub struct EphemeralKeyPair {
    keypair: Keypair,
}

#[derive(Debug, Deserialize)]
pub struct Keypair {
    publicKey: HashMap<String, u8>, // HashMap<String,String>, // publicKey: Vec<u8>,
    secretKey: HashMap<String, u8>, // secretKey: Vec<u8> // HashMap<String,u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZkProofs {
    proofPoints: ProofPoints,
    iss_base64_details: iss_base64_details,
    header_base64: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofPoints {
    a: Vec<String>,
    b: Vec<Vec<String>>,
    c: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct iss_base64_details {
    value: String,
    index_mod4: i32,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart1 {
    alg: String,
    kid: String,
    typ: String,
}

#[derive(Debug, Deserialize)]
pub struct JwtDataDecodedPart2 {
    iss: String,
    azp: String,
    aud: String,
    sub: String,
    nonce: String,
    nbf: u32,
    iat: u32,
    exp: u32,
    jti: String,
}

fn gen_keypair() -> ed25519_dalek::Keypair {
    ed25519_dalek::Keypair::generate(&mut rand::thread_rng())
}

#[test]
fn test_poseidon_plus_vrgrth16_based_on_real_data_super_new() {
    // real data taken from our react app for zklogin tests
    // {"alg":"RS256","kid":"a3b762f871cdb3bae0044c649622fc1396eda3e3","typ":"JWT"}
    // {"iss":"https://accounts.google.com","azp":"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    // "aud":"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.
    // com","sub":"112897468626716626103", "nonce":"sS2DydHu3Ihp8ZCWCA4nzD79e08"
    // ,"nbf":1715600156,"iat":1715600456,"exp":1715604056,"jti":"
    // 27d9a159279fc60df664c6ce8cb149a4244e5dd5"} Initial password was 567890
    // in ascii 535455565748
    let user_pass_salt = "535455565748"; // Alina's data (password in ascii ), should be different for iterations

    // Generate an ephemeral key pair.
    let secret_key = [
        222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166,
        87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); //Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
    let mut eph_pubkey = Vec::new();
    // replace by Alina's data (ephemeral public key place to byte array ), depends
    // on iteration
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());

    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);

    // Get the zklogin seed.
    // This stuff is a kind of bound between  smart contract and email (some
    // account) It will be stored in smart contract (must be added during
    // contract deployment)
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103", // Alina's data (stable id, fixed by gmail alina.t@gosh.sh) from jwt
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com", // Alina's data (fixed by app id ) from jwt
    ).unwrap();

    println!("zk_seed = {:?}", zk_seed);

    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\
    \"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let issAndheader_base64Details = "{\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

    println!("issAndheader_base64Details: {}", issAndheader_base64Details);

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    // summary from 132 to 146 : need to parse jwt, see jwt header to check that kid
    // in this header is equal to one specified in line 143,... take kid from jwt if
    // not equal
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
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(), // Alina's data, fascrypto's was 6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    let max_epoch = 142; // data from the react test

    // Decode modulus to bytes.
    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string()))
        .unwrap();

    let public_inputs =
        &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    /// calcs poseidon *

    println!("====== Start Poseidon ========");

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();

    let issAndheader_base64Details_cell =
        pack_string_to_cell(&issAndheader_base64Details.clone(), &mut 0).unwrap();

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();

    let max_epoch_ = 142;

    let mut code = format!("PUSHINT {max_epoch_} \n").to_string();
    code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"POSEIDON_ZKLOGIN \n".to_string();

    println!("code : {:?}", code);

    test_case_with_refs(
        code.as_str(),
        vec![modulus_cell.clone(), issAndheader_base64Details_cell, zk_seed_cell],
    )
    .expect_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
    //.expect_success();

    /// calcs vergrth16 *

    println!("====== Start VERGRTH16 ========");

    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

    let verification_key_id: u32 = 0; //valid key id
    // let verification_key_id: u32 = 1; //invalid key id

    let mut code = "PUSHREF \n".to_string();
    code = code + "PUSHREF \n";
    code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
    code = code + "VERGRTH16";

    test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
        .expect_success();
}

#[test]
fn test_poseidon_plus_vrgrth16() {
    /// Common data generation *
    let user_pass_salt = "206703048842351542647799591018316385612";

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
    let mut eph_pubkey = Vec::new();
    eph_pubkey.extend(ephemeral_kp.public().as_ref());

    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());

    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);

    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "106294049240999307923",
        "25769832374-famecqrhe2gkebt5fvqms2263046lj96.apps.googleusercontent.com",
    )
    .unwrap();

    println!("zk_seed: {}", zk_seed);

    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"8247215875293406890829839156897863742504615191361518281091302475904551111016\",\"6872980335748205979379321982220498484242209225765686471076081944034292159666\",\"1\"],\"b\":[[\"21419680064642047510915171723230639588631899775315750803416713283740137406807\",\"21566716915562037737681888858382287035712341650647439119820808127161946325890\"],[\"17867714710686394159919998503724240212517838710399045289784307078087926404555\",\"21812769875502013113255155836896615164559280911997219958031852239645061854221\"],[\"1\",\"0\"]],\"c\":[\"7530826803702928198368421787278524256623871560746240215547076095911132653214\",\"16244547936249959771862454850485726883972969173921727256151991751860694123976\",\"1\"]},\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\"}";

    println!("proof_and_jwt: {}", proof_and_jwt);

    let issAndheader_base64Details = "{\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\"}";
    println!("issAndheader_base64Details: {}", issAndheader_base64Details);

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();

    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "oUriU8GqbRw-avcMn95DGW1cpZR1IoM6L7krfrWvLSSCcSX6Ig117o25Yk7QWBiJpaPV0FbP7Y5-DmThZ3SaF0AXW-3BsKPEXfFfeKVc6vBqk3t5mKlNEowjdvNTSzoOXO5UIHwsXaxiJlbMRalaFEUm-2CKgmXl1ss_yGh1OHkfnBiGsfQUndKoHiZuDzBMGw8Sf67am_Ok-4FShK0NuR3-q33aB_3Z7obC71dejSLWFOEcKUVCaw6DGVuLog3x506h1QQ1r0FXKOQxnmqrRgpoHqGSouuG35oZve1vgCU4vLZ6EAgBAbC0KL35I7_0wUDSMpiAvf7iZxzJVbspkQ".to_string(),
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());

    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    // Decode modulus to bytes.
    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string()))
        .unwrap();

    println!("modulus: {:?}", modulus);

    println!("modulus hex: {:?}", hex::encode(&modulus));

    let max_epoch = 10;

    // let max_epoch = 142;

    /// calcs poseidon *

    println!("====== Start Poseidon ========");

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();

    let issAndheader_base64Details_cell =
        pack_string_to_cell(&issAndheader_base64Details.clone(), &mut 0).unwrap();

    // let eph_pubkey_cell = pack_data_to_cell(&eph_pubkey.clone(), &mut
    // 0).unwrap();

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();

    let max_epoch_ = 142;

    let mut code = format!("PUSHINT {max_epoch_} \n").to_string();
    code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"POSEIDON_ZKLOGIN \n".to_string();

    test_case_with_refs(
        code.as_str(),
        vec![modulus_cell, issAndheader_base64Details_cell, zk_seed_cell],
    )
    .expect_success();

    /// calcs vergrth16 *

    println!("====== Start Vergrth16 ========");

    let pp = zk_login_inputs.get_proof();

    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
    let public_inputs =
        &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());
    println!("proof_as_bytes hex: {:?}", hex::encode(&proof_as_bytes));

    let y1 = proof.a.y.0.to_bits_le();
    let y2 = proof.a.y.0.to_bits_be();
    // let y_ = -y;

    println!("proof.a: {:?}", proof.a);

    println!("proof.a.y: {:?}", proof.a.y);

    println!("proof.a.y.0: {:?}", proof.a.y.0);

    println!("proof.a.y.0.to_string(): {:?}", proof.a.y.0.to_string());

    println!("proof.a.x.0.to_string(): {:?}", proof.a.x.0.to_string());

    println!("y1: {:?}", y1);
    for i in 0..y1.len() {
        print!("{}", y1[i] as i32);
    }
    println!("");
    println!("y2: {:?}", y2);
    for i in 0..y2.len() {
        print!("{}", y2[i] as i32);
    }
    println!("");
    // println!("y_: {:?}", y_);

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
    println!("public_inputs_as_bytes hex: {:?}", hex::encode(&public_inputs_as_bytes));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    let verification_key_id: u32 = 1;

    let mut code = "PUSHREF \n".to_string();
    code = code + "PUSHREF \n";
    code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
    code = code + "VERGRTH16";

    test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
        .expect_success();
}

#[test]
fn test_eval_time_vrgrth16_new() {
    // todo: later n must be extracted from 3d part of jwt
    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "rv95jmy91hibD7cb_BCA25jv5HrX7WoqHv-fh8wrOR5aYcM8Kvsc3mbzs2w1vCUlMRv7NdEGVBEnOZ6tHvUzGLon4ythd5XsX-wTvAtIHPkyHdo5zGpTgATO9CEn78Y-f1E8By63ttv14kXe_RMjt5aKttK4yqqUyzWUexSs7pET2zWiigd0_bGhJGYYEJlEk_JsOBFvloIBaycMfDjK--kgqnlRA8SWUkP3pEJIAo9oHzmvX6uXZTEJK10a1YNj0JVR4wZY3k60NaUX-KCroreU85iYgnecyxSdL-trpKdkg0-2OYks-_2Isymu7jPX-uKVyi-zKyaok3N64mERRQ".to_string(),
        alg: "RS256".to_string(),
    };

    // {
    // "e": "AQAB",
    // "kty": "RSA",
    // "n": "rv95jmy91hibD7cb_BCA25jv5HrX7WoqHv-fh8wrOR5aYcM8Kvsc3mbzs2w1vCUlMRv7NdEGVBEnOZ6tHvUzGLon4ythd5XsX-wTvAtIHPkyHdo5zGpTgATO9CEn78Y-f1E8By63ttv14kXe_RMjt5aKttK4yqqUyzWUexSs7pET2zWiigd0_bGhJGYYEJlEk_JsOBFvloIBaycMfDjK--kgqnlRA8SWUkP3pEJIAo9oHzmvX6uXZTEJK10a1YNj0JVR4wZY3k60NaUX-KCroreU85iYgnecyxSdL-trpKdkg0-2OYks-_2Isymu7jPX-uKVyi-zKyaok3N64mERRQ"
    // }

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "0e345fd7e4a97271dffa991f5a893cd16b8e0827".to_string(), // Alina's data, fascrypto's was 6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
        ),
        content,
    );

    let sui_data = [SUI_DATA_FROM_REACT_1_NEW];

    for i in 0..sui_data.len() {
        println!("====================== Iter@ is {i} =========================");
        // parse
        let jwt_data: JwtData = serde_json::from_str(&sui_data[i]).unwrap();
        // println!("{:?}", jwt_data);

        let user_pass_salt = jwt_data.userPassToIntFormat.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        let eph_secret_key = secretKeyFromIntegerMap(jwt_data.ephemeralKeyPair.keypair.secretKey);

        let ephemeral_kp = Ed25519KeyPair::from_bytes(&eph_secret_key).unwrap();
        let mut eph_pubkey = Vec::new(); //vec![0x00];
        eph_pubkey.extend(ephemeral_kp.public().as_ref());

        println!("ephemeral secret_key is {:?}", eph_secret_key);
        println!("ephemeral public_key is {:?}", eph_pubkey);

        let eph_pubkey_len = eph_pubkey.clone().len();
        println!("len eph_pubkey: {:?}", eph_pubkey_len);

        let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is "{\"alg\":\"RS256\",\"kid\":\"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"typ\":\"JWT\"}"

        // JwtDataDecodedPart1
        let jwt_data_decoded1: JwtDataDecodedPart1 = serde_json::from_str(&jwt_string_1).unwrap();
        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2); // "{\"iss\":\"https://accounts.google.com\",\"azp\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"aud\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"sub\":\"112897468626716626103\",\"nonce\":\"bxmnJW31ruzKMGir01YPGYL0xDY\",\"nbf\":1715687036,\"iat\":1715687336,\"exp\":1715690936,\"jti\":\"9b601d25f003640c2889a2a047789382cb1cfe87\"}"

        // JwtDataDecodedPart2
        let jwt_data_decoded2: JwtDataDecodedPart2 = serde_json::from_str(&jwt_string_2).unwrap();
        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(), /* Alina's data (stable id, fixed by gmail
                                             * alina.t@gosh.sh) from jwt */
            jwt_data_decoded2.aud.as_str(), // Alina's data (fixed by app id ) from jwt
        )
        .unwrap();

        println!("jwt_data.zkProofs = {:?}", jwt_data.zkProofs);
        let proof_and_jwt = serde_json::to_string(&jwt_data.zkProofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string()).unwrap();

        let time_for_vergrth16 = single_vrgrth16(&eph_pubkey, &zk_login_inputs, &all_jwk);
        println!("time_for_vergrth16 is {time_for_vergrth16}");

        println!("==========================================");
    }
}

#[test]
fn test_vrgrth16_based_on_real_data_new() {
    // real data taken from our react app for zklogin tests
    // {"alg":"RS256","kid":"a3b762f871cdb3bae0044c649622fc1396eda3e3","typ":"JWT"}
    // {"iss":"https://accounts.google.com","azp":"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    // "aud":"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.
    // com","sub":"112897468626716626103", "nonce":"sS2DydHu3Ihp8ZCWCA4nzD79e08"
    // ,"nbf":1715600156,"iat":1715600456,"exp":1715604056,"jti":"
    // 27d9a159279fc60df664c6ce8cb149a4244e5dd5"} Initial password was 567890
    // in ascii 535455565748
    let user_pass_salt = "535455565748"; // Alina's data (password in ascii ), should be different for iterations

    // Generate an ephemeral key pair.
    let secret_key = [
        222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166,
        87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); //Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
    let mut eph_pubkey = Vec::new(); //vec![0x00];
    // replace by Alina's data (ephemeral public key place to byte array ), depends
    // on iteration
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("eph_pubkey: {:?}", hex::encode(eph_pubkey.clone()));
    let len = eph_pubkey.clone().len();
    println!("len eph_pubkey: {:?}", len);

    // Get the zklogin seed.
    // This stuff is a kind of bound between  smart contract and email (some
    // account) It will be stored in smart contract (must be added during
    // contract deployment)
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103", // Alina's data (stable id, fixed by gmail alina.t@gosh.sh) from jwt
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com", // Alina's data (fixed by app id ) from jwt
    ).unwrap();

    println!("zk_seed = {:?}", zk_seed);

    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\
    \"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    // summary from 132 to 146 : need to parse jwt, see jwt header to check that kid
    // in this header is equal to one specified in line 143,... take kid from jwt if
    // not equal
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
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(), // Alina's data, fascrypto's was 6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    let max_epoch = 142; // data from the react test

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

    let verification_key_id: u32 = 0; //valid key id
    // let verification_key_id: u32 = 1; //invalid key id

    let mut code = "PUSHREF \n".to_string();
    code = code + "PUSHREF \n";
    code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
    code = code + "VERGRTH16";

    test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
        .expect_success();
}

#[test]
fn test_proof_serialization() {
    let user_pass_salt = "206703048842351542647799591018316385612";

    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "106294049240999307923",
        "25769832374-famecqrhe2gkebt5fvqms2263046lj96.apps.googleusercontent.com",
    )
    .unwrap();

    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"8247215875293406890829839156897863742504615191361518281091302475904551111016\",\"6872980335748205979379321982220498484242209225765686471076081944034292159666\",\"1\"],\"b\":[[\"21419680064642047510915171723230639588631899775315750803416713283740137406807\",\"21566716915562037737681888858382287035712341650647439119820808127161946325890\"],[\"17867714710686394159919998503724240212517838710399045289784307078087926404555\",\"21812769875502013113255155836896615164559280911997219958031852239645061854221\"],[\"1\",\"0\"]],\"c\":[\"7530826803702928198368421787278524256623871560746240215547076095911132653214\",\"16244547936249959771862454850485726883972969173921727256151991751860694123976\",\"1\"]},\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\"}";

    println!("proof_and_jwt: {}", proof_and_jwt);

    let zk_login_inputs = tvm_vm::executor::zk_stuff::zk_login::ZkLoginInputs::from_json(
        &*proof_and_jwt,
        &*zk_seed.to_string(),
    )
    .unwrap();

    println!("zk_login_inputs: {:?}", zk_login_inputs);

    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();

    println!("proof.a: {:?}", proof.a);

    println!("proof.a.y.0.to_string(): {:?}", proof.a.y.0.to_string());
}

#[test]
fn test_vrgrth16_fresh() {
    // Initial password was 567890
    let user_pass_salt = "535455565748";

    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103", // Alina's data (stable id, fixed by gmail alina.t@gosh.sh) from jwt
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com", // Alina's data (fixed by app id ) from jwt
    ).unwrap();

    println!("zk_seed {zk_seed}");

    let iss = "https://accounts.google.com";

    let xxx = get_zk_login_address(&Bn254FrElement::from_str(&zk_seed).unwrap(), iss).unwrap();
    let xx = hex::encode(&xxx);
    println!("xxx {xx}");

    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"20032491544466004395942516676927853848812757556091814296260914209848471949133\",\"2383319895045368406863089991961299436327009667970727469594098906910899823518\",\"1\"],\"b\":[[\"17524079199473031626933714849790290610990375813469214348846178898325828270802\",\"14967860363718375858883445892553389848174133418448836833724123534259346456965\"],[\"8012103671455598651673212917030479015077366694912593401917441922282850889728\",\"9619406946838713340504188077859322423191842838375117333667670119492063405148\"],[\"1\",\"0\"]],\"c\":[\"1155327534990006564455106296492790109069125857506281397147103620914309288350\",\"11642927414888703901346255147864200862372140915112720472429308471936285279899\",\"1\"]},\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImIyNjIwZDVlN2YxMzJiNTJhZmU4ODc1Y2RmMzc3NmMwNjQyNDlkMDQiLCJ0eXAiOiJKV1QifQ\"}";

    println!("proof_and_jwt: {}", proof_and_jwt);

    let issAndheader_base64Details = "\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImIyNjIwZDVlN2YxMzJiNTJhZmU4ODc1Y2RmMzc3NmMwNjQyNDlkMDQiLCJ0eXAiOiJKV1QifQ\"";
    println!("issAndheader_base64Details: {}", issAndheader_base64Details);

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();

    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "pi22xDdK2fz5gclIbDIGghLDYiRO56eW2GUcboeVlhbAuhuT5mlEYIevkxdPOg5n6qICePZiQSxkwcYMIZyLkZhSJ2d2M6Szx2gDtnAmee6o_tWdroKu0DjqwG8pZU693oLaIjLku3IK20lTs6-2TeH-pUYMjEqiFMhn-hb7wnvH_FuPTjgz9i0rEdw_Hf3Wk6CMypaUHi31y6twrMWq1jEbdQNl50EwH-RQmQ9bs3Wm9V9t-2-_Jzg3AT0Ny4zEDU7WXgN2DevM8_FVje4IgztNy29XUkeUctHsr-431_Iu23JIy6U4Kxn36X3RlVUKEkOMpkDD3kd81JPW4Ger_w".parse().unwrap(),
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "b2620d5e7f132b52afe8875cdf3776c064249d04".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());

    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    // Decode modulus to bytes.
    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string()))
        .unwrap();

    println!("modulus: {:?}", modulus);

    println!("modulus hex: {:?}", hex::encode(&modulus));

    let max_epoch = 142;

    // let max_epoch = 10;

    let mut eph_pubkey = vec![
        131, 177, 23, 68, 46, 252, 168, 4, 146, 173, 66, 45, 69, 248, 80, 87, 25, 27, 251, 212,
        143, 3, 173, 66, 156, 56, 155, 83, 21, 226, 161, 63,
    ];

    println!("eph_pubkey : {:?}", eph_pubkey);
    println!("eph_pubkey len : {:?}", eph_pubkey.len());

    let pp = zk_login_inputs.get_proof();

    let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
    let public_inputs =
        &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());
    println!("proof_as_bytes hex: {:?}", hex::encode(&proof_as_bytes));

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
    println!("public_inputs_as_bytes hex: {:?}", hex::encode(&public_inputs_as_bytes));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    let verification_key_id: u32 = 0;

    let mut code = "PUSHREF \n".to_string();
    code = code + "PUSHREF \n";
    code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
    code = code + "VERGRTH16";

    test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
        .expect_success();
}

#[test]
fn test_poseidon_update() {
    /// Common data generation *
    let user_pass_salt = "206703048842351542647799591018316385612";

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
    let mut eph_pubkey = Vec::new();
    eph_pubkey.extend(ephemeral_kp.public().as_ref());
    println!("eph_pubkey: {:?}", eph_pubkey);
    println!("len eph_pubkey: {:?}", eph_pubkey.len());

    let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
    println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);

    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "106294049240999307923",
        "25769832374-famecqrhe2gkebt5fvqms2263046lj96.apps.googleusercontent.com",
    )
    .unwrap();

    println!("zk_seed: {}", zk_seed);

    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"8247215875293406890829839156897863742504615191361518281091302475904551111016\",\"6872980335748205979379321982220498484242209225765686471076081944034292159666\",\"1\"],\"b\":[[\"21419680064642047510915171723230639588631899775315750803416713283740137406807\",\"21566716915562037737681888858382287035712341650647439119820808127161946325890\"],[\"17867714710686394159919998503724240212517838710399045289784307078087926404555\",\"21812769875502013113255155836896615164559280911997219958031852239645061854221\"],[\"1\",\"0\"]],\"c\":[\"7530826803702928198368421787278524256623871560746240215547076095911132653214\",\"16244547936249959771862454850485726883972969173921727256151991751860694123976\",\"1\"]},\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\"}";

    println!("proof_and_jwt: {}", proof_and_jwt);

    let issAndheader_base64Details = "{\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\"}";
    println!("issAndheader_base64Details: {}", issAndheader_base64Details);

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();

    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "oUriU8GqbRw-avcMn95DGW1cpZR1IoM6L7krfrWvLSSCcSX6Ig117o25Yk7QWBiJpaPV0FbP7Y5-DmThZ3SaF0AXW-3BsKPEXfFfeKVc6vBqk3t5mKlNEowjdvNTSzoOXO5UIHwsXaxiJlbMRalaFEUm-2CKgmXl1ss_yGh1OHkfnBiGsfQUndKoHiZuDzBMGw8Sf67am_Ok-4FShK0NuR3-q33aB_3Z7obC71dejSLWFOEcKUVCaw6DGVuLog3x506h1QQ1r0FXKOQxnmqrRgpoHqGSouuG35oZve1vgCU4vLZ6EAgBAbC0KL35I7_0wUDSMpiAvf7iZxzJVbspkQ".to_string(),
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());

    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    // Decode modulus to bytes.
    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
        .map_err(|_| ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string()))
        .unwrap();

    println!("modulus: {:?}", modulus);

    println!("modulus hex: {:?}", hex::encode(&modulus));

    println!("====== Start Poseidon ========");

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();

    let issAndheader_base64Details_cell =
        pack_string_to_cell(&issAndheader_base64Details.clone(), &mut 0).unwrap();

    // let eph_pubkey_cell = pack_data_to_cell(&eph_pubkey.clone(), &mut
    // 0).unwrap();

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();

    let max_epoch = "142"; //"200142";

    let mut code = format!("PUSHINT {max_epoch} \n").to_string();
    code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"PUSHREF \n".to_string();
    code = code + &*"POSEIDON_ZKLOGIN \n".to_string();

    println!("code : {code}");

    test_case_with_refs(
        code.as_str(),
        vec![modulus_cell, issAndheader_base64Details_cell, zk_seed_cell],
    )
    .expect_success();
}

#[test]
fn test_vrgrth16_and_chcksigns_comparison_based_on_fascrypto_data() {
    println!(
        "===================================== START VRGRTH16 TEST ====================================="
    );

    let user_pass_salt = "206703048842351542647799591018316385612";
    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
    let mut eph_pubkey = Vec::new(); //vec![0x00];
    eph_pubkey.extend(ephemeral_kp.public().as_ref());

    println!("eph_pubkey: {:?}", hex::encode(eph_pubkey.clone()));
    let len = eph_pubkey.clone().len();
    println!("len eph_pubkey: {:?}", len);

    // Get the zklogin seed.
    // This stuff is a kind of bound between  smart contract and email (some
    // account) It will be stored in smart contract (must be added during
    // contract deployment)
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "106294049240999307923",
        "25769832374-famecqrhe2gkebt5fvqms2263046lj96.apps.googleusercontent.com",
    )
    .unwrap();

    println!("zk_seed = {:?}", zk_seed);

    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"8247215875293406890829839156897863742504615191361518281091302475904551111016\",\"6872980335748205979379321982220498484242209225765686471076081944034292159666\",\"1\"],\"b\":[[\"21419680064642047510915171723230639588631899775315750803416713283740137406807\",\"21566716915562037737681888858382287035712341650647439119820808127161946325890\"],[\"17867714710686394159919998503724240212517838710399045289784307078087926404555\",\"21812769875502013113255155836896615164559280911997219958031852239645061854221\"],[\"1\",\"0\"]],\"c\":[\"7530826803702928198368421787278524256623871560746240215547076095911132653214\",\"16244547936249959771862454850485726883972969173921727256151991751860694123976\",\"1\"]},\"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();

    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "oUriU8GqbRw-avcMn95DGW1cpZR1IoM6L7krfrWvLSSCcSX6Ig117o25Yk7QWBiJpaPV0FbP7Y5-DmThZ3SaF0AXW-3BsKPEXfFfeKVc6vBqk3t5mKlNEowjdvNTSzoOXO5UIHwsXaxiJlbMRalaFEUm-2CKgmXl1ss_yGh1OHkfnBiGsfQUndKoHiZuDzBMGw8Sf67am_Ok-4FShK0NuR3-q33aB_3Z7obC71dejSLWFOEcKUVCaw6DGVuLog3x506h1QQ1r0FXKOQxnmqrRgpoHqGSouuG35oZve1vgCU4vLZ6EAgBAbC0KL35I7_0wUDSMpiAvf7iZxzJVbspkQ".to_string(),
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());

    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    let max_epoch = 10;

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

    let verification_key_id: u32 = 1;

    let mut code = "PUSHREF \n".to_string();
    code = code + "PUSHREF \n";
    code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
    code = code + "VERGRTH16";

    test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
        .expect_success();

    println!(
        "===================================== START CHKSIGNS TEST ====================================="
    );

    let pair = gen_keypair();

    let binding = proof_cell.clone();
    let first = binding.data();

    // let mut b = BuilderData::with_raw(first, len).unwrap();
    let binding = public_inputs_cell.clone();
    let second = binding.data();
    // b.append_raw(second, len);

    let concatenated = [&first[..], &second[..]].concat();
    println!("len concatenated = {}", concatenated.len());

    // test cell with data and one not empty reference
    let test_cell = pack_data_to_cell(&concatenated, &mut 0).unwrap();
    // b.into_cell().unwrap();//crate::test_app_specific::gen_test_tree_of_cells();
    // let cell_hash = test_cell.repr_hash();
    // sign hash of data cell
    let signature = pair.sign(test_cell.data()).to_bytes().to_vec();

    // put signature to separate slice
    let len = signature.len() * 8;
    let signature = SliceData::from_raw(signature, len);

    // put public key to integer
    let pub_key = BuilderData::with_raw(
        pair.public.to_bytes().to_vec(),
        ed25519_dalek::PUBLIC_KEY_LENGTH * 8,
    )
    .unwrap();

    // put hash to integer
    // let hash = BuilderData::with_raw(cell_hash.as_slice().to_vec(),
    // 256).unwrap();

    test_case_with_refs(
        "
        PUSHREFSLICE
        PUSHREFSLICE
        PUSHREFSLICE
        PLDU 256
        CHKSIGNS
    ",
        vec![test_cell, signature.into_cell(), pub_key.into_cell().unwrap()],
    )
    .expect_stack(Stack::new().push(int!(-1)));
}

#[test]
fn test_vrgrth16_based_on_real_data() {
    // real data taken from our react app for zklogin tests
    // {"alg":"RS256","kid":"a3b762f871cdb3bae0044c649622fc1396eda3e3","typ":"JWT"}
    // {"iss":"https://accounts.google.com","azp":"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
    // "aud":"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.
    // com","sub":"112897468626716626103", "nonce":"sS2DydHu3Ihp8ZCWCA4nzD79e08"
    // ,"nbf":1715600156,"iat":1715600456,"exp":1715604056,"jti":"
    // 27d9a159279fc60df664c6ce8cb149a4244e5dd5"} Initial password was 567890
    // in ascii 535455565748

    let user_pass_salt = "535455565748"; // Alina's data (password in ascii ), should be different for iterations

    // Generate an ephemeral key pair.
    let secret_key = [
        222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166,
        87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); //Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
    let mut eph_pubkey = Vec::new(); //vec![0x00]; // replace by Alina's data (ephemeral public key place to byte array ), depends on iteration
    eph_pubkey.extend(ephemeral_kp.public().as_ref());

    println!("eph_pubkey: {:?}", hex::encode(eph_pubkey.clone()));
    let len = eph_pubkey.clone().len();
    println!("len eph_pubkey: {:?}", len);

    // Get the zklogin seed.
    // This stuff is a kind of bound between  smart contract and email (some
    // account) It will be stored in smart contract (must be added during
    // contract deployment)
    let zk_seed = gen_address_seed(
        user_pass_salt,
        "sub",
        "112897468626716626103", // Alina's data (stable id, fixed by gmail alina.t@gosh.sh) from jwt
        "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com", // Alina's data (fixed by app id ) from jwt
    ).unwrap();

    println!("zk_seed = {:?}", zk_seed);

    let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
    \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
    \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
    \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
    [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
    \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
    \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
    \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
    \"iss_base64_details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"index_mod4\":1},\
    \"header_base64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    // summary from 132 to 146 : need to parse jwt, see jwt header to check that kid
    // in this header is equal to one specified in line 143,... take kid from jwt if
    // not equal
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
            "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(), // Alina's data, fascrypto's was 6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
        ),
        content,
    );

    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    let max_epoch = 142; // data from the react test

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

    let verification_key_id: u32 = 0;

    // let verification_key_id: u32 = 1;

    let mut code = "PUSHREF \n".to_string();
    code = code + "PUSHREF \n";
    code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
    code = code + "VERGRTH16";

    test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
        .expect_success();
}

fn secret_key_from_integer_map(key_data: HashMap<String, u8>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..=31 {
        if let Some(value) = key_data.get(&i.to_string()) {
            vec.push(value.clone());
        }
    }
    return vec;
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

fn prepare_hex_representation(init_x: &str, y: BigUint) -> String {
    let mut binary_representation = pad_string_to_256(&to_binary_string(init_x));

    let p: BigUint = BigUint::from_bytes_be(&[
        48, 100, 78, 114, 225, 49, 160, 41, 184, 80, 69, 182, 129, 129, 88, 93, 151, 129, 106, 145,
        104, 113, 202, 141, 60, 32, 140, 22, 216, 124, 253, 71,
    ]);

    // Сравниваем y с p - y и меняем первый бит
    if y > &p - &y {
        binary_representation.replace_range(0..1, "1");
    }

    let reversed_byte_array = bits_to_decimal_and_reverse(&binary_representation);

    // Преобразуем массив байтов в hex-строку
    let hex_string =
        reversed_byte_array.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

    hex_string
}

#[test]
fn test_proof_stuff() {
    let sui_data = [
        SUI_DATA_FROM_REACT_1,
        SUI_DATA_FROM_REACT_2,
        SUI_DATA_FROM_REACT_3,
        SUI_DATA_FROM_REACT_4,
        SUI_DATA_FROM_REACT_5,
        SUI_DATA_FROM_REACT_6,
        SUI_DATA_FROM_REACT_7,
        SUI_DATA_FROM_REACT_8,
        SUI_DATA_FROM_REACT_9,
        SUI_DATA_FROM_REACT_10,
        SUI_DATA_FROM_REACT_11,
        SUI_DATA_FROM_REACT_12,
        SUI_DATA_FROM_REACT_13,
        SUI_DATA_FROM_REACT_14,
        SUI_DATA_FROM_REACT_15,
        SUI_DATA_FROM_REACT_16,
        SUI_DATA_FROM_REACT_17,
        SUI_DATA_FROM_REACT_18,
        SUI_DATA_FROM_REACT_19,
        SUI_DATA_FROM_REACT_20,
        SUI_DATA_FROM_REACT_21,
    ];

    for i in 0..sui_data.len() {
        let jwt_data: JwtData = serde_json::from_str(&sui_data[i]).unwrap();
        let json_string = serde_json::to_string(&jwt_data.zkProofs).unwrap();
        print!("{:?}, \n", json_string);
    }

    for i in 0..sui_data.len() {
        let jwt_data: JwtData = serde_json::from_str(&sui_data[i]).unwrap();

        //////

        let user_pass_salt = jwt_data.userPassToIntFormat.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is "{\"alg\":\"RS256\",\"kid\":\"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"typ\":\"JWT\"}"

        // JwtDataDecodedPart1
        let jwt_data_decoded1: JwtDataDecodedPart1 = serde_json::from_str(&jwt_string_1).unwrap();
        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2); // "{\"iss\":\"https://accounts.google.com\",\"azp\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"aud\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"sub\":\"112897468626716626103\",\"nonce\":\"bxmnJW31ruzKMGir01YPGYL0xDY\",\"nbf\":1715687036,\"iat\":1715687336,\"exp\":1715690936,\"jti\":\"9b601d25f003640c2889a2a047789382cb1cfe87\"}"

        // JwtDataDecodedPart2
        let jwt_data_decoded2: JwtDataDecodedPart2 = serde_json::from_str(&jwt_string_2).unwrap();
        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(), /* Alina's data (stable id, fixed by gmail
                                             * alina.t@gosh.sh) from jwt */
            jwt_data_decoded2.aud.as_str(), // Alina's data (fixed by app id ) from jwt
        )
        .unwrap();

        println!("jwt_data.zkProofs = {:?}", jwt_data.zkProofs);
        let proof_and_jwt = serde_json::to_string(&jwt_data.zkProofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string()).unwrap();

        let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();

        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());
        println!("----------------------------------");

        ///////////

        let json_string = serde_json::to_string(&jwt_data.zkProofs).unwrap();
        println!("json_string ={:?}", json_string); //jwt_data.zkProofs);

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

    //  println!("Serialized proof");
    // let json_string =
    // r#"{"proofPoints":{"a":["
    // 8247215875293406890829839156897863742504615191361518281091302475904551111016"
    // ,"6872980335748205979379321982220498484242209225765686471076081944034292159666"
    // ,"1"],"b":[["
    // 21419680064642047510915171723230639588631899775315750803416713283740137406807"
    // ,"21566716915562037737681888858382287035712341650647439119820808127161946325890"
    // ],["17867714710686394159919998503724240212517838710399045289784307078087926404555"
    // ,"21812769875502013113255155836896615164559280911997219958031852239645061854221"
    // ],["1","0"]],"c":["
    // 7530826803702928198368421787278524256623871560746240215547076095911132653214"
    // ,"16244547936249959771862454850485726883972969173921727256151991751860694123976"
    // ,"1"]},"iss_base64_details":{"value":"
    // yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC","index_mod4":1},"
    // header_base64":"
    // eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ"
    // }"#;
    //
    // Парсинг JSON-строки
    // let data: Value = serde_json::from_str(json_string).unwrap();
    //
    // let a_x = data["proofPoints"]["a"][0].as_str().unwrap();
    // let a_y = BigUint::parse_bytes(data["proofPoints"]["a"][1].as_str().
    // unwrap().as_bytes(), 10).unwrap();
    //
    // let b0_x = data["proofPoints"]["b"][0][0].as_str().unwrap();
    // let b1_x = data["proofPoints"]["b"][0][1].as_str().unwrap();
    // let b1_y =
    // BigUint::parse_bytes(data["proofPoints"]["b"][1][1].as_str().unwrap().
    // as_bytes(), 10).unwrap();
    //
    // let c_x = data["proofPoints"]["c"][0].as_str().unwrap();
    // let c_y = BigUint::parse_bytes(data["proofPoints"]["c"][1].as_str().
    // unwrap().as_bytes(), 10).unwrap();
    //
    // let hex_ax = prepare_hex_representation(a_x, a_y);
    // let hex_b0x = prepare_hex_representation(b0_x, BigUint::zero());
    // let hex_b1x = prepare_hex_representation(b1_x, b1_y);
    // let hex_cx = prepare_hex_representation(c_x, c_y);
    //
    // let result = format!("{}{}{}{}", hex_ax, hex_b0x, hex_b1x, hex_cx);
    //
    // ????? ???????????
    // println!("Serialized proof: {}", result);
    // println!("Serialized proof: 68490e184c1c5279d09fafc5e5c0b77d2a61fe6262ced81ff315c1813ec23b1257c1538b36c9822e94933c0fdb49d39502b7d63c47cc75cae7264f6afa1b5b2f82c3d7dc537cc07c2969bb4454a3d423d0e998f5787d4735eed757554654aeaf9ee6f79a85b302bdf25d83a9aeb4e06361459f51c86b1dca23172500034ca690");
}

#[ignore]
#[test]
fn test_eval_time_vrgrth16() {
    // todo: later n must be extracted from 3d part of jwt

    let content: JWK = JWK {
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
        n: "xjWd1j8GmmWzuz732haG9HECXsSZBvxOBLph3FQhk_tplhWloI1ywx-RdopUZt1lndbOM9n99lZJkpQyNJ1sdy7JFgYLjqj-wtHdEaQlBGEQtmkW8zUjr_N3bmpsxGbPzOzlKe3qddtoxXvn9rI_RvHfJD1YY-6kayQeyPOBz_4ML1lvI_JHV-Bb1MSmSk3WaAh5PzeqleusmUT87Gqfu02cOPrY8cwugqo65D6-wzAEeVvceV8-c36TMoLU5csU05GBVplgd6Ouuw35ZsETG4si4QQJztC3KsZ4jhYM-aJ3jeFPt0r3cQooiXdZBp3JkXSpE-UUaOVPsXo7WiVmww".to_string(), // Alina's data
        alg: "RS256".to_string(),
    };

    let mut all_jwk = HashMap::new();
    all_jwk.insert(
        JwkId::new(
            OIDCProvider::Google.get_config().iss,
            "323b214ae6975a0f034ea77354dc0c25d03642dc".to_string(), // Alina's data, fascrypto's was 6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
        ),
        content,
    );

    // let sui_data = [SUI_DATA_FROM_REACT_1];
    let sui_data = [
        SUI_DATA_FROM_REACT_1,
        SUI_DATA_FROM_REACT_2,
        SUI_DATA_FROM_REACT_3,
        SUI_DATA_FROM_REACT_4,
        SUI_DATA_FROM_REACT_5,
        SUI_DATA_FROM_REACT_6,
        SUI_DATA_FROM_REACT_7,
        SUI_DATA_FROM_REACT_8,
        SUI_DATA_FROM_REACT_9,
        SUI_DATA_FROM_REACT_10,
        SUI_DATA_FROM_REACT_11,
        SUI_DATA_FROM_REACT_12,
        SUI_DATA_FROM_REACT_13,
        SUI_DATA_FROM_REACT_14,
        SUI_DATA_FROM_REACT_15,
        SUI_DATA_FROM_REACT_16,
        SUI_DATA_FROM_REACT_17,
        SUI_DATA_FROM_REACT_18,
        SUI_DATA_FROM_REACT_19,
        SUI_DATA_FROM_REACT_20,
        SUI_DATA_FROM_REACT_21,
    ];

    let mut sum_ratio: u128 = 0;

    for i in 0..sui_data.len() {
        println!("====================== Iter@ is {i} =========================");
        // parse
        let jwt_data: JwtData = serde_json::from_str(&sui_data[i]).unwrap();
        // println!("{:?}", jwt_data);

        let user_pass_salt = jwt_data.userPassToIntFormat.as_str();
        println!("user_pass_salt is {user_pass_salt}");

        let eph_secret_key = secretKeyFromIntegerMap(jwt_data.ephemeralKeyPair.keypair.secretKey);

        let ephemeral_kp = Ed25519KeyPair::from_bytes(&eph_secret_key).unwrap();
        let mut eph_pubkey = Vec::new(); //vec![0x00];
        eph_pubkey.extend(ephemeral_kp.public().as_ref());

        println!("ephemeral secret_key is {:?}", eph_secret_key);
        println!("ephemeral public_key is {:?}", eph_pubkey);

        let eph_pubkey_len = eph_pubkey.clone().len();
        println!("len eph_pubkey: {:?}", eph_pubkey_len);

        // let splitted_jwt_strings: Vec<_> = jwt_data.jwt.split('.').collect();
        //
        // let jwt_header = splitted_jwt_strings
        // .get(0)
        // .expect("split always returns at least one element");
        //
        // let jwt_body = splitted_jwt_strings.get(1).ok_or(Box::<dyn Error>::from(
        // "Could not find separator in jwt string.",
        // )).unwrap();
        //
        // let decoded_jwt_header = base64::decode(jwt_header).unwrap();
        // let decoded_jwt_body = base64::decode(jwt_body).unwrap();
        //
        // let converted_jwt_header =
        // String::from_utf8(decoded_jwt_header).expect("UTF-8 conversion failed");
        // let converted_jwt_body = String::from_utf8(decoded_jwt_body).expect("UTF-8
        // conversion failed");
        //
        // let parsed_jwt_header =
        // serde_json::from_str::<serde_json::Value>(&converted_jwt_header).unwrap();
        // let parsed_jwt_body =
        // serde_json::from_str::<serde_json::Value>(&converted_jwt_body).unwrap();
        //
        // println!(
        // "{}",
        // serde_json::to_string_pretty(&parsed_jwt_header)
        // .expect("to_string_pretty always works on serde_json::Value")
        // );
        // println!(
        // "{}",
        // serde_json::to_string_pretty(&parsed_jwt_body)
        // .expect("to_string_pretty always works on serde_json::Value")
        // );

        let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is "{\"alg\":\"RS256\",\"kid\":\"323b214ae6975a0f034ea77354dc0c25d03642dc\",\"typ\":\"JWT\"}"

        // JwtDataDecodedPart1
        let jwt_data_decoded1: JwtDataDecodedPart1 = serde_json::from_str(&jwt_string_1).unwrap();
        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2); // "{\"iss\":\"https://accounts.google.com\",\"azp\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"aud\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"sub\":\"112897468626716626103\",\"nonce\":\"bxmnJW31ruzKMGir01YPGYL0xDY\",\"nbf\":1715687036,\"iat\":1715687336,\"exp\":1715690936,\"jti\":\"9b601d25f003640c2889a2a047789382cb1cfe87\"}"

        // JwtDataDecodedPart2
        let jwt_data_decoded2: JwtDataDecodedPart2 = serde_json::from_str(&jwt_string_2).unwrap();
        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        // let key = DecodingKey::from_secret(&[]);
        // let mut validation = Validation::new(Algorithm::HS256);
        // validation.insecure_disable_signature_validation();

        // let jwt_data_3 = decode(jwt_data_vector[2]).expect("Base64 decoding failed");
        // let jwt_string_3 = String::from_utf8(jwt_data_3).expect("UTF-8 conversion
        // failed"); println!("jwt_string_3 is {:?}", jwt_string_3);
        // JwtDataDecodedPart3
        // let jwt_data_decoded3: JwtDataDecodedPart3 =
        // serde_json::from_str(&jwt_string_3).unwrap();

        // let jwt_data_3 = &decode(jwt_data.2).unwrap()[..];
        // println!("{:?}", encode(&jwt_data_1));
        // let jwt_string_3 = String::from_utf8(&jwt_data_3);

        // let jwt_v_3: Value = serde_json::from_str(&jwt_string_3)?;
        // let n = jwt_v_3["n"];

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(), /* Alina's data (stable id, fixed by gmail
                                             * alina.t@gosh.sh) from jwt */
            jwt_data_decoded2.aud.as_str(), // Alina's data (fixed by app id ) from jwt
        )
        .unwrap();

        println!("jwt_data.zkProofs = {:?}", jwt_data.zkProofs);
        let proof_and_jwt = serde_json::to_string(&jwt_data.zkProofs).unwrap();

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string()).unwrap();

        let time_for_vergrth16 = single_vrgrth16(&eph_pubkey, &zk_login_inputs, &all_jwk);
        let time_for_chcksgns = single_chcksgns(&eph_pubkey, &zk_login_inputs, &all_jwk);
        println!("time_for_vergrth16 is {time_for_vergrth16}");
        println!("time_for_chcksgns is {time_for_chcksgns}");

        let current_ratio = time_for_vergrth16 / time_for_chcksgns;
        println!("current_ratio is {current_ratio}");

        sum_ratio = sum_ratio + current_ratio; /**/
        println!("sum_ratio is {sum_ratio}");
        println!("==========================================");
    }
    let average_ratio = sum_ratio / (sui_data.len() as u128);

    println!("average ratio is {average_ratio}");
}

fn prepare_proof_and_public_key_cells_for_stack(
    eph_pubkey: &Vec<u8>,
    zk_login_inputs: &ZkLoginInputs,
    all_jwk: &HashMap<JwkId, JWK>,
) -> (Cell, Cell) {
    let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
    println!("kid = {}", kid);
    println!("all_jwk = {:?}", all_jwk);

    let jwk = all_jwk
        .get(&JwkId::new(iss.clone(), kid.clone()))
        .ok_or_else(|| ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid)))
        .unwrap();

    let max_epoch = 142; // data from the react test

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

fn single_vrgrth16(
    eph_pubkey: &Vec<u8>,
    zk_login_inputs: &ZkLoginInputs,
    all_jwk: &HashMap<JwkId, JWK>,
) -> u128 {
    let (proof_cell, public_inputs_cell) =
        prepare_proof_and_public_key_cells_for_stack(eph_pubkey, zk_login_inputs, all_jwk);

    // let verification_key_id: u32 = 2;
    let verification_key_id: u32 = 0;

    let mut code = "PUSHREF \n".to_string();
    code = code + "PUSHREF \n";
    code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
    code = code + "VERGRTH16";

    let start: Instant = Instant::now();
    test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
        .expect_success();
    start.elapsed().as_micros()
}

fn single_chcksgns(
    eph_pubkey: &Vec<u8>,
    zk_login_inputs: &ZkLoginInputs,
    all_jwk: &HashMap<JwkId, JWK>,
) -> u128 {
    let (proof_cell, public_inputs_cell) =
        prepare_proof_and_public_key_cells_for_stack(eph_pubkey, zk_login_inputs, all_jwk);

    let pair = gen_keypair();

    let binding = proof_cell.clone();
    let first = binding.data();

    let binding = public_inputs_cell.clone();
    let second = binding.data();

    let concatenated = [&first[..], &second[..]].concat();

    let test_cell = pack_data_to_cell(&concatenated, &mut 0).unwrap();
    let signature = pair.sign(test_cell.data()).to_bytes().to_vec();

    // put signature to separate slice
    let len = signature.len() * 8;
    let signature = SliceData::from_raw(signature, len);

    // put public key to integer
    let pub_key = BuilderData::with_raw(
        pair.public.to_bytes().to_vec(),
        ed25519_dalek::PUBLIC_KEY_LENGTH * 8,
    )
    .unwrap();

    let start: Instant = Instant::now();
    test_case_with_refs(
        "
        PUSHREFSLICE
        PUSHREFSLICE
        PUSHREFSLICE
        PLDU 256
        CHKSIGNS
    ",
        vec![test_cell, signature.into_cell(), pub_key.into_cell().unwrap()],
    )
    .expect_stack(Stack::new().push(int!(-1)));
    start.elapsed().as_micros()
}
