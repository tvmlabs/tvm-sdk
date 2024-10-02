// use std::collections::HashMap;
// use std::slice;
// use ark_std::rand::rngs::StdRng;
// use ark_std::rand::SeedableRng;
// use base64ct::Encoding as bEncoding;
// use fastcrypto::ed25519::Ed25519KeyPair;
//
//
// use fastcrypto::traits::KeyPair;
// use rand::Rng;
// use similar::DiffableStr;
// use tvm_block::{
// GlobalCapabilities, MsgAddressInt, Serializable, ACTION_CHANGE_LIB,
// ACTION_COPYLEFT, ACTION_RESERVE, ACTION_SEND_MSG, ACTION_SET_CODE,
// };
// use tvm_assembler::compile_code_to_cell;
// use tvm_types::{
// types::ExceptionCode, AccountId, HashmapE, HashmapType, IBitstring, Result,
// Sha256
// };
//
// #[cfg(feature = "signature_no_check")]
// use ton_vm::executor::BehaviorModifiers;
// use tvm_vm::{
// boolean,
// executor::serialize_currency_collection,
// int,
// stack::{
// integer::{
// serialization::{Encoding, UnsignedIntegerBigEndianEncoding},
// IntegerData,
// },
// serialization::{Deserializer, Serializer},
// Stack, StackItem,
// },
// SmartContractInfo,
// utils::{pack_string_to_cell, pack_data_to_cell, unpack_string_from_cell},
// };
//
// use tvm_vm::executor::zk::ZkCryptoError;
// use tvm_assembler::CompileError;
// use tvm_types::{BuilderData, Cell, SliceData};
//
// use fastcrypto_zkp::bn254::zk_login::{CanonicalSerialize, JWK, JwkId,
// OIDCProvider, ZkLoginInputs};
// use fastcrypto_zkp::bn254::utils::gen_address_seed;
//
// pub const VALUE_PORTION_SIZE: usize = 126;
//
// #[test]
// fn test_vergrth16() {
// let user_pass_salt = "206703048842351542647799591018316385612";
//
// Generate an ephemeral key pair.
// let ephemeral_kp = Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
// let mut eph_pubkey = vec![0x00];
// eph_pubkey.extend(ephemeral_kp.public().as_ref());
//
// println!("eph_pubkey: {:?}", hex::encode(eph_pubkey.clone()));
// let len = eph_pubkey.clone().len();
// println!("len eph_pubkey: {:?}", len);
//
// Get the zklogin seed.
// This stuff is a kind of bound between  smart contract and email (some
// account) It will be stored in smart contract (must be added during contract
// deployment) let zk_seed = gen_address_seed(
// user_pass_salt,
// "sub",
// "106294049240999307923",
// "25769832374-famecqrhe2gkebt5fvqms2263046lj96.apps.googleusercontent.com",
// ).unwrap();
//
// println!("zk_seed = {:?}", zk_seed);
//
// let proof_and_jwt =
// "{\"proofPoints\":{\"a\":[\"
// 8247215875293406890829839156897863742504615191361518281091302475904551111016\
// ",\"6872980335748205979379321982220498484242209225765686471076081944034292159666\
// ",\"1\"],\"b\":[[\"
// 21419680064642047510915171723230639588631899775315750803416713283740137406807\
// ",\"21566716915562037737681888858382287035712341650647439119820808127161946325890\
// "],[\"17867714710686394159919998503724240212517838710399045289784307078087926404555\
// ",\"21812769875502013113255155836896615164559280911997219958031852239645061854221\
// "],[\"1\",\"0\"]],\"c\":[\"
// 7530826803702928198368421787278524256623871560746240215547076095911132653214\
// ",\"16244547936249959771862454850485726883972969173921727256151991751860694123976\
// ",\"1\"]},\"issBase64Details\":{\"value\":\"
// yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"
// headerBase64\":\"
// eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\
// "}"; let len = proof_and_jwt.bytes().len();
// println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);
//
// let zk_login_inputs = ZkLoginInputs::from_json(
// &*proof_and_jwt, &*zk_seed.to_string()).unwrap();
//
// let content: JWK = JWK {
// kty: "RSA".to_string(),
// e: "AQAB".to_string(),
// n: "oUriU8GqbRw-avcMn95DGW1cpZR1IoM6L7krfrWvLSSCcSX6Ig117o25Yk7QWBiJpaPV0FbP7Y5-DmThZ3SaF0AXW-3BsKPEXfFfeKVc6vBqk3t5mKlNEowjdvNTSzoOXO5UIHwsXaxiJlbMRalaFEUm-2CKgmXl1ss_yGh1OHkfnBiGsfQUndKoHiZuDzBMGw8Sf67am_Ok-4FShK0NuR3-q33aB_3Z7obC71dejSLWFOEcKUVCaw6DGVuLog3x506h1QQ1r0FXKOQxnmqrRgpoHqGSouuG35oZve1vgCU4vLZ6EAgBAbC0KL35I7_0wUDSMpiAvf7iZxzJVbspkQ".to_string(),
// alg: "RS256".to_string(),
// };
//
// let mut all_jwk = HashMap::new();
// all_jwk.insert(
// JwkId::new(
// OIDCProvider::Google.get_config().iss,
// "6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
// ),
// content,
// );
//
// let (iss, kid) = (zk_login_inputs.get_iss().to_string(),
// zk_login_inputs.get_kid().to_string()); let jwk = all_jwk
// .get(&JwkId::new(iss.clone(), kid.clone()))
// .ok_or_else(|| {
// ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
// }).unwrap();
//
// let max_epoch = 10;
//
// Decode modulus to bytes.
// let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n).map_err(|_| {
// ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
// }).unwrap();
//
// let proof  = &zk_login_inputs.get_proof().as_arkworks().unwrap();
// let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey,
// &modulus, max_epoch).unwrap()];
//
// let mut proof_as_bytes = vec![];
// proof.serialize_compressed(&mut proof_as_bytes).unwrap();
// println!("proof_as_bytes : {:?}", proof_as_bytes);
//
// let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
//
// let mut public_inputs_as_bytes = vec![];
// public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
// println!("public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
//
// let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut
// 0).unwrap();
//
// let verification_key_id: u32 = 1;
//
// let mut code = "PUSHREF \n".to_string();
// code = code + "PUSHREF \n";
// code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
// code = code + "VERGRTH16";
//
// ... run 'code', providing proof_cell, public_inputs_cell into stack..
// }
//
//
//
