#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use base64ct::Encoding as bEncoding;
    use fastcrypto::ed25519::Ed25519KeyPair;
    use fastcrypto::traits::KeyPair;
    use fastcrypto::traits::ToFromBytes;
    use fastcrypto_zkp::bn254::utils::gen_address_seed;
    use fastcrypto_zkp::bn254::zk_login::CanonicalSerialize;
    use fastcrypto_zkp::bn254::zk_login::JWK;
    use fastcrypto_zkp::bn254::zk_login::JwkId;
    use fastcrypto_zkp::bn254::zk_login::OIDCProvider;
    use fastcrypto_zkp::bn254::zk_login::ZkLoginInputs;
    use fastcrypto_zkp::bn254::zk_login::ZkLoginProof;
    use serde::Deserialize;
    use serde_derive::Serialize;
    use tvm_types::Cell;
    use tvm_vm::executor::zk_stuff::error::ZkCryptoError;
    use tvm_vm::int;
    use tvm_vm::stack::Stack;
    use tvm_vm::stack::StackItem;
    use tvm_vm::stack::integer::IntegerData;
    use tvm_vm::utils::pack_data_to_cell;

    use crate::test_framework::Expects;
    use crate::test_framework::test_case_with_refs;

    #[derive(Debug, Deserialize)]
    pub struct JwtData {
        pub jwt: String,
        pub user_pass_to_int_format: String,
        pub ephemeral_key_pair: EphemeralKeyPair,
        pub zk_addr: String,
        pub zk_proofs: ZkProofs,
        pub extended_ephemeral_public_key: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct EphemeralKeyPair {
        keypair: Keypair,
    }

    #[derive(Debug, Deserialize)]
    pub struct Keypair {
        public_key: HashMap<String, u8>,
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


    pub struct TestPrecomputedData {
        pub public_inputs_cell: Cell,
        pub proof: ZkLoginProof,
    }

    pub fn do_initial_work() -> TestPrecomputedData {
        // real data taken from our react app for zklogin tests
        // {"alg":"RS256","kid":"a3b762f871cdb3bae0044c649622fc1396eda3e3","typ":"JWT"}
        // {"iss":"https://accounts.google.com","azp":"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
        // "aud":"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.
        // com","sub":"112897468626716626103",
        // "nonce":"sS2DydHu3Ihp8ZCWCA4nzD79e08","nbf":1715600156,"iat":1715600456,"exp"
        // :1715604056,"jti":"27d9a159279fc60df664c6ce8cb149a4244e5dd5"} Initial
        // password was 567890 in ascii 535455565748
        let user_pass_salt = "535455565748"; // Alina's data (password in ascii ), should be different for iterations

        // Generate an ephemeral key pair.
        let secret_key = [
            222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162,
            166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14,
        ];

        // Generate an ephemeral key pair.
        let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); // Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
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
    \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
    \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
        let len = proof_and_jwt.bytes().len();
        println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);

        println!("proof_and_jwt: {}", proof_and_jwt);

        let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

        println!("iss_and_header_base64details: {}", iss_and_header_base64details);

        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
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

        let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
        let jwk = all_jwk
            .get(&JwkId::new(iss.clone(), kid.clone()))
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
            .unwrap();

        let max_epoch = 142; // data from the react test

        // Decode modulus to bytes.
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

        let proof = zk_login_inputs.get_proof();

        TestPrecomputedData{
            public_inputs_cell,
            proof: proof.clone()
        }
    }

    #[test]
    fn test_vrgrth16_short_proof() {
        
        let data = do_initial_work();

        let public_inputs_cell = data.public_inputs_cell;
        let proof = data.proof.as_arkworks().unwrap();

        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        //INTENTIONALLY SPOIL PROOF

        proof_as_bytes.pop();

        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

        let verification_key_id: u32 = 0; // valid key id
        // let verification_key_id: u32 = 1; //invalid key id

        let mut code = "PUSHREF \n".to_string();
        code = code + "PUSHREF \n";
        code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
        code = code + "VERGRTH16";

        test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
            .expect_failure(tvm_types::ExceptionCode::FatalError);
    }

    #[test]
    fn test_vrgrth16_long_proof() {
        
        let data = do_initial_work();

        let public_inputs_cell = data.public_inputs_cell;
        let proof = data.proof.as_arkworks().unwrap();

        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        //INTENTIONALLY SPOIL PROOF

        proof_as_bytes.push(1);

        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

        let verification_key_id: u32 = 0; // valid key id
        // let verification_key_id: u32 = 1; //invalid key id

        let mut code = "PUSHREF \n".to_string();
        code = code + "PUSHREF \n";
        code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
        code = code + "VERGRTH16";

        test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
            .expect_stack(Stack::new().push(int!(-1)));
            //.expect_failure(tvm_types::ExceptionCode::FatalError);
    }

    #[test]
    fn test_vrgrth16_long_incorrect_proof() {
        
        let data = do_initial_work();

        let public_inputs_cell = data.public_inputs_cell;

        let proof_as_bytes = vec![1; 129];

        println!("proof_as_bytes: {:?}", proof_as_bytes);


        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

        let verification_key_id: u32 = 0; // valid key id
        // let verification_key_id: u32 = 1; //invalid key id

        let mut code = "PUSHREF \n".to_string();
        code = code + "PUSHREF \n";
        code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
        code = code + "VERGRTH16";

        test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()]).
            expect_failure(tvm_types::ExceptionCode::FatalError);
    }

    #[test]
    fn test_vrgrth16_incorrect_proof() {
        
        let data = do_initial_work();

        let public_inputs_cell = data.public_inputs_cell;

        let proof_as_bytes = vec![2; 128];

        println!("proof_as_bytes: {:?}", proof_as_bytes);


        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

        let verification_key_id: u32 = 0; // valid key id
        // let verification_key_id: u32 = 1; //invalid key id

        let mut code = "PUSHREF \n".to_string();
        code = code + "PUSHREF \n";
        code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
        code = code + "VERGRTH16";

        test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()]).
            expect_failure(tvm_types::ExceptionCode::FatalError);
    }

    #[test]
    fn test_vrgrth16_invalid_proof() {
        
        let data = do_initial_work();

        let public_inputs_cell = data.public_inputs_cell;
        let proof = data.proof.as_arkworks().unwrap();

        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        //INTENTIONALLY SPOIL PROOF

        proof_as_bytes[0] = 1;

        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

        let verification_key_id: u32 = 0; // valid key id
        // let verification_key_id: u32 = 1; //invalid key id

        let mut code = "PUSHREF \n".to_string();
        code = code + "PUSHREF \n";
        code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
        code = code + "VERGRTH16";

        test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
            .expect_failure(tvm_types::ExceptionCode::FatalError);
    }

    #[test]
    fn test_vrgrth16_invalid_proof_one_more_case() {
        
        let data = do_initial_work();

        let public_inputs_cell = data.public_inputs_cell;
        let proof = data.proof.as_arkworks().unwrap();

        let mut proof_as_bytes = vec![];
        proof.serialize_compressed(&mut proof_as_bytes).unwrap();
        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        //INTENTIONALLY SPOIL PROOF

        proof_as_bytes[120] = 25;

        println!("proof_as_bytes : {:?}", proof_as_bytes);
        println!("proof_as_bytes len: {:?}", proof_as_bytes.len());

        let proof_cell = pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();

        let verification_key_id: u32 = 0; // valid key id
        // let verification_key_id: u32 = 1; //invalid key id

        let mut code = "PUSHREF \n".to_string();
        code = code + "PUSHREF \n";
        code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
        code = code + "VERGRTH16";

        test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
            .expect_failure(tvm_types::ExceptionCode::FatalError);
    }
}
