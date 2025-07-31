use ark_std::rand::rngs::StdRng;
use std::collections::HashSet;
use std::time::Duration;
use std::time::Instant;

use rand::rngs::OsRng;
use num_traits::Zero;
use crate::stack::integer::IntegerData;
use crate::stack::integer::serialization::Encoding;
use crate::stack::integer::serialization::SignedIntegerBigEndianEncoding;
use crate::stack::integer::serialization::UnsignedIntegerBigEndianEncoding;
use crate::stack::serialization::Deserializer;

use rand::RngCore;
use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::IBitstring;
use tvm_types::SliceData;

use crate::error::TvmError;
use crate::executor::crypto::execute_chksigns;
use crate::executor::deserialization::execute_pldu;
use crate::executor::engine::Engine;
use crate::executor::gas::gas_state::Gas;

use crate::executor::serialize_currency_collection;

use crate::executor::zk::execute_poseidon_zk_login;
use crate::executor::zk::execute_vergrth16;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::savelist::SaveList;
use crate::types::Status;
use crate::utils::pack_data_to_cell;
use crate::utils::unpack_data_from_cell;
use crate::utils::pack_string_to_cell;
use crate::utils::unpack_string_from_cell;

use crate::executor::zk_stuff::utils::gen_address_seed;



use crate::executor::test_helper::*;


use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::KeyPair;
use fastcrypto::traits::ToFromBytes;

use crate::executor::zk_stuff::zk_login::CanonicalSerialize;
use crate::executor::zk_stuff::zk_login::JWK;
use crate::executor::zk_stuff::zk_login::JwkId;
use crate::executor::zk_stuff::zk_login::OIDCProvider;
use crate::executor::zk_stuff::zk_login::ZkLoginInputs;
use crate::executor::zk_stuff::curve_utils::Bn254FrElement;
use crate::executor::zk_stuff::error::ZkCryptoError;

use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use std::collections::HashMap;

use base64::decode;
use base64ct::Encoding as bEncoding;

use rand::Rng;
use rand::SeedableRng;
use rand::thread_rng;

use crate::executor::deserialization::execute_schkrefs;
use crate::executor::math::execute_divmod;

use num_bigint::BigInt;
use num_bigint::BigUint;
use std::str::FromStr;
use num_traits::FromPrimitive;
use crate::executor::zk_stuff::zk_login::ZkLoginProof;


#[test]
fn test_modulus_bad() {
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

    let user_pass_salt = "535455565748"; 
    // Generate an ephemeral key pair.
    let secret_key = [
            222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162,
            166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
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
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    let index_mod_4 = 1;

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(),
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
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
    .unwrap();

    let max_epoch = 142; 

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|_| {
                ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
            })
            .unwrap();

    let public_inputs =
            &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    ///// Modulus handle /////

    // Decode modulus to bytes.

    println!("Too short modulus...");

    let mut modulus_spoiled = modulus.clone();
    modulus_spoiled.pop();

    let modulus_cell = pack_data_to_cell(&modulus_spoiled.clone(), &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);


    println!("Too long modulus...");

    let mut modulus_spoiled = modulus.clone();
    modulus_spoiled.push(100);

    let modulus_cell = pack_data_to_cell(&modulus_spoiled.clone(), &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    match execute_poseidon_zk_login(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true); 
            if let Some(TvmError::TvmExceptionFull(e, msg2)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            }
            else {
                assert!(false);
            }
        }
    }

    println!("Spoiled modulus...");
    let modulus_spoiled = vec![0; 2];

    let modulus_cell = pack_data_to_cell(&modulus_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);

    println!("Spoiled modulus again...");

    let mut modulus_spoiled = modulus.clone();
    modulus_spoiled[10] = 67;

    let modulus_cell = pack_data_to_cell(&modulus_spoiled.clone(), &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);
}

#[test]
fn test_iss_64_bad() {
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

    let user_pass_salt = "535455565748"; 
    // Generate an ephemeral key pair.
    let secret_key = [
            222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162,
            166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
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
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    let index_mod_4 = 1;

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(),
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
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
    .unwrap();

    let max_epoch = 142; 

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|_| {
                ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
            })
            .unwrap();

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();

    let public_inputs =
            &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    
    ///// iss_base_64 handle /////

    println!("Test cut iss_base_64...");

    let iss_base_64_spoiled = "yJpc3MiOiJodHRwczovL";

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);


    println!("Test empty iss_base_64...");

    let iss_base_64_spoiled = "";

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);

    println!("Test too  long wrong iss_base_64...");

    let iss_base_64_spoiled = "ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666";

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);
    
    
    println!("Test too too long wrong iss_base_64...");

    let iss_base_64_spoiled = "ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666";

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    match execute_poseidon_zk_login(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true); 
            if let Some(TvmError::TvmExceptionFull(e, msg2)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            }
            else {
                assert!(false);
            }
        }
    }
}

#[test]
fn test_header_bad() {
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

    let user_pass_salt = "535455565748"; 
    // Generate an ephemeral key pair.
    let secret_key = [
            222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162,
            166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
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
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    let index_mod_4 = 1;

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(),
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
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
    .unwrap();

    let max_epoch = 142; 

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|_| {
                ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
            })
            .unwrap();

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();

    let public_inputs =
            &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    
    ///// Header handle /////

    println!("Incorrect short header...");

    let header_base_64 =
            "eyJhbGciOiJSUzI1NImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzElZGEzZTMiLCJ0eQifQ";

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();


    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);


    println!("Empty header...");

    let header_base_64 = "";

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();


    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);

    println!("Too long wrong header...");

    let header_base_64 = "32789263432789263432789263432789263432789263432789263432789263432789263432786786786g32789263432789263432789263432789263432786786786g32789263432789263432789263432789263432786786786g";

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);
     
    println!("Too long wrong header...");

    let header_base_64 = "327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634ejwgdejhcg327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634dsjhgcjhwdcgjwgcdjhwgcdhc327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634dgxhwjdcg";

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();


    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    match execute_poseidon_zk_login(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true); 
            if let Some(TvmError::TvmExceptionFull(e, msg2)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            }
            else {
                assert!(false);
            }
        }
    }
}


#[test]
fn test_zk_seed_bad() {
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

    let user_pass_salt = "535455565748"; 
    // Generate an ephemeral key pair.
    let secret_key = [
            222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162,
            166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
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
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    let index_mod_4 = 1;

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(),
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
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
    .unwrap();

    let max_epoch = 142; 

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|_| {
                ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
            })
            .unwrap();

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();

    let public_inputs =
            &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    
    ///// zk seed handle /////

    println!("Empty zk seed...");

    let zk_seed_spoiled: String = String::from("");

    let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    match execute_poseidon_zk_login(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true); 
            if let Some(TvmError::TvmExceptionFull(e, msg2)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            }
            else {
                assert!(false);
            }
        }
    }


    println!("Wrong short zk seed...");

    let zk_seed_spoiled: String = String::from(
            "190149130838213916597767154061365555081978517533943138452652822856495767863",
    );

    let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);

     println!("Spoiled decimal position in zk seed...");

    let zk_seed_spoiled: String = String::from(
            "18014913083821391659776715405561365555081978517533943138452652822856495767863",
    );

    let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();


    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);
     
    println!("Not decimal symbol in zk seed...");

    let zk_seed_spoiled: String = String::from(
            "a8014913083821391659776715405561365555081978517533943138452652822856495767863",
    );

    let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    match execute_poseidon_zk_login(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true); 
            if let Some(TvmError::TvmExceptionFull(e, msg2)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            }
            else {
                assert!(false);
            }
        }
    }

    println!("Too long zk seed...");

    let zk_seed_spoiled: String = String::from(
            "119014913083821391659776715405561365555081978517533943138452652822856495767863",
    );

    let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);

    println!("Too too long zk seed...");

    let zk_seed_spoiled: String = String::from(
            "119011190141190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678636437862786427864874269130838213916597767154055613655550819785175339431384526528228564957678634913083821311901119014913083821391659776715405561365555081978517533943138452652822856495767863491308382139165977671540556136555508197851753394313845265282285649576786391659776715405561365555081978517533943138452652822856495767863119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821311901119014913083821391659776715405561365555081978517533943138452652822856495767863491308382139165977671540556136555508197851753394313845265282285649576786391659776715405561365555081978517533943138452652822856495767863119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821311901119014913083821391659776715405561365555081978517533943138452652822856495767863491308382139165977671540556136555508197851753394313845265282285649576786391659776715405561365555081978517533943138452652822856495767863",
    );

    let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();

    engine.cc.stack.push(StackItem::int(index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);

}

#[test]
fn test_other_args_bad() {
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

    let user_pass_salt = "535455565748"; 
    // Generate an ephemeral key pair.
    let secret_key = [
            222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162,
            166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    // Generate an ephemeral key pair.
    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
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
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
    let len = proof_and_jwt.bytes().len();
    println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

    println!("proof_and_jwt: {}", proof_and_jwt);

    let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
    let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
    let index_mod_4 = 1;

    let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
    let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(),
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
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
    .unwrap();

    let max_epoch = 142; 

    let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();

    let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

    let zk_seed_cell = pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();

    let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|_| {
                ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
            })
            .unwrap();

    let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();

    let public_inputs =
            &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    
    let wrong_index_mod_4 = 256;


    engine.cc.stack.push(StackItem::int(wrong_index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    match execute_poseidon_zk_login(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true); 
            if let Some(TvmError::TvmExceptionFull(e, msg2)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::RangeCheckError);
            }
            else {
                assert!(false);
            }
        }
    }


    let wrong_index_mod_4 = 255;

    engine.cc.stack.push(StackItem::int(wrong_index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);

    
    let wrong_index_mod_4 = 0;

    engine.cc.stack.push(StackItem::int(wrong_index_mod_4));
    engine.cc.stack.push(StackItem::int(max_epoch));
    engine.cc.stack.push(StackItem::integer(IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));
    engine.cc.stack.push(StackItem::cell(modulus_cell.clone()));
    engine.cc.stack.push(StackItem::cell(iss_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(header_base_64_cell.clone()));
    engine.cc.stack.push(StackItem::cell(zk_seed_cell.clone()));

    let status = execute_poseidon_zk_login(&mut engine).unwrap();
    let poseidon_res = engine.cc.stack.get(0).as_cell().unwrap();
    println!("poseidon_res : {poseidon_res}");
    println!("public_inputs_cell : {public_inputs_cell}");
    assert!(*poseidon_res != public_inputs_cell);
}