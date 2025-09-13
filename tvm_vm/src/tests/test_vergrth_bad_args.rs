use std::collections::HashMap;

use base64ct::Encoding as bEncoding;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::KeyPair;
use fastcrypto::traits::ToFromBytes;
use tvm_types::SliceData;

use crate::error::TvmError;
use crate::executor::engine::Engine;
use crate::executor::test_helper::*;
use crate::executor::zk::execute_vergrth16;
use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::utils::gen_address_seed;
use crate::executor::zk_stuff::zk_login::CanonicalSerialize;
use crate::executor::zk_stuff::zk_login::JWK;
use crate::executor::zk_stuff::zk_login::JwkId;
use crate::executor::zk_stuff::zk_login::OIDCProvider;
use crate::executor::zk_stuff::zk_login::ZkLoginInputs;
use crate::executor::zk_stuff::zk_login::ZkLoginProof;
use crate::executor::zk::{GLOBAL_VK_SERIALIZED, INSECURE_VK_SERIALIZED, MY_TEST_VK_1_SERIALIZED};
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::savelist::SaveList;
use crate::utils::pack_data_to_cell;

pub struct TestPrecomputedData {
    pub public_inputs_as_bytes: Vec<u8>,
    pub proof: ZkLoginProof,
}

pub fn do_initial_work() -> TestPrecomputedData {
    let user_pass_salt = "535455565748";

    let secret_key = [
        222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166,
        87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
    ];

    let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap();
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

    let proof = zk_login_inputs.get_proof();

    TestPrecomputedData { public_inputs_as_bytes, proof: proof.clone() }
}

#[test]
fn test_vrgrth16_short_public_input() {
    let data = do_initial_work();
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

    let proof = data.proof.as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let mut public_inputs_as_bytes = data.public_inputs_as_bytes;
    public_inputs_as_bytes.pop();

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());


    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));
    

    match execute_vergrth16(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true);
            if let Some(TvmError::TvmExceptionFull(e, _)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn test_vrgrth16_long_public_input() {
    let data = do_initial_work();
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

    let proof = data.proof.as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let mut public_inputs_as_bytes = data.public_inputs_as_bytes;

    println!("public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("public_inputs_as_bytes len: {:?}", public_inputs_as_bytes.len());

    public_inputs_as_bytes.push(1);

    println!("public_inputs_as_bytes len: {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());

    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));
    

    match execute_vergrth16(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true);
            if let Some(TvmError::TvmExceptionFull(e, _)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn test_vrgrth16_bad_public_input() {
    let data = do_initial_work();
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

    let proof = data.proof.as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let mut public_inputs_as_bytes = data.public_inputs_as_bytes.clone();

    println!("public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
    println!("public_inputs_as_bytes len: {:?}", public_inputs_as_bytes.len());

    public_inputs_as_bytes[0] = 0;

    println!("public_inputs_as_bytes len: {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());

    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    match execute_vergrth16(&mut engine) {
        Ok(_) => {
            let res = engine.cc.stack.get(0).as_integer().unwrap();
            println!("res: {:?}", res);
            assert!(*res == IntegerData::zero());
        }
        Err(_) => {
            assert!(false);
        }
    };

    let mut public_inputs_as_bytes = data.public_inputs_as_bytes.clone();
    public_inputs_as_bytes[20] = 50;
    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    match execute_vergrth16(&mut engine) {
        Ok(_) => {
            let res = engine.cc.stack.get(0).as_integer().unwrap();
            println!("res: {:?}", res);
            assert!(*res == IntegerData::zero());
        }
        Err(_) => {
            assert!(false);
        }
    };

    let public_inputs_as_bytes = vec![0; 32];
    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    match execute_vergrth16(&mut engine) {
        Ok(_) => {
            let res = engine.cc.stack.get(0).as_integer().unwrap();
            println!("res: {:?}", res);
            assert!(*res == IntegerData::zero());
        }
        Err(_) => {
            assert!(false);
        }
    };
}

#[test]
fn test_vrgrth16_short_proof() {
    let data = do_initial_work();
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

    let proof = data.proof.as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();

    // INTENTIONALLY SPOIL PROOF
    proof_as_bytes.pop();

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let public_inputs_as_bytes = data.public_inputs_as_bytes;

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    match execute_vergrth16(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true);
            if let Some(TvmError::TvmExceptionFull(e, _)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn test_vrgrth16_long_proof() {
    let data = do_initial_work();
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

    let proof = data.proof.as_arkworks().unwrap();
    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();

    // INTENTIONALLY SPOIL PROOF
    proof_as_bytes.push(1);

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let public_inputs_as_bytes = data.public_inputs_as_bytes;

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    match execute_vergrth16(&mut engine) {
        Ok(_) => {
            let res = engine.cc.stack.get(0).as_integer().unwrap();
            println!("res: {:?}", res);
            assert!(*res == IntegerData::minus_one());
        }
        Err(_) => {
            assert!(false);
        }
    };
}

#[test]
fn test_vrgrth16_long_incorrect_proof() {
    let data = do_initial_work();
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

    let proof_as_bytes = vec![1; 129];

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let public_inputs_as_bytes = data.public_inputs_as_bytes;

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    match execute_vergrth16(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true);
            if let Some(TvmError::TvmExceptionFull(e, _)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn test_vrgrth16_incorrect_proof() {
    let data = do_initial_work();
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

    let proof_as_bytes = vec![2; 128];

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let public_inputs_as_bytes = data.public_inputs_as_bytes;

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    match execute_vergrth16(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true);
            if let Some(TvmError::TvmExceptionFull(e, _)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn test_vrgrth16_invalid_proof() {
    let data = do_initial_work();
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

    let proof = data.proof.as_arkworks().unwrap();

    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    // INTENTIONALLY SPOIL PROOF

    proof_as_bytes[0] = 1;

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let public_inputs_as_bytes = data.public_inputs_as_bytes;

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));

    match execute_vergrth16(&mut engine) {
        Ok(_) => assert!(false),
        Err(ref err) => {
            assert!(true);
            if let Some(TvmError::TvmExceptionFull(e, _)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            } else {
                assert!(false);
            }
        }
    }
}

#[test]
fn test_vrgrth16_invalid_proof_one_more_case() {
    let data = do_initial_work();
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

    let proof = data.proof.as_arkworks().unwrap();

    let mut proof_as_bytes = vec![];
    proof.serialize_compressed(&mut proof_as_bytes).unwrap();
    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    // INTENTIONALLY SPOIL PROOF

    proof_as_bytes[120] = 25;

    println!("proof_as_bytes : {:?}", proof_as_bytes);
    println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

    let public_inputs_as_bytes = data.public_inputs_as_bytes;

    let vk_index: u32 = 0;
    let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };

    engine.set_vergrth16_verififcation_key_serialized(vk_bytes.clone());

    let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(public_inputs_cell.clone()));


    match execute_vergrth16(&mut engine) {
        Ok(_) => assert!(false),
        Err(err) => {
            assert!(true);
            if let Some(TvmError::TvmExceptionFull(e, _)) = err.downcast_ref() {
                assert!(e.exception_code().unwrap() == tvm_types::ExceptionCode::FatalError);
            } else {
                assert!(false);
            }
        }
    }
}
