#[allow(dead_code)]

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use base64ct::Encoding as bEncoding;
    use fastcrypto::ed25519::Ed25519KeyPair;
    use fastcrypto::traits::KeyPair;
    use fastcrypto::traits::ToFromBytes;
    
    use tvm_vm::executor::zk_stuff::utils::gen_address_seed;
    use tvm_vm::executor::zk_stuff::zk_login::CanonicalSerialize;
    use tvm_vm::executor::zk_stuff::zk_login::JWK;
    use tvm_vm::executor::zk_stuff::zk_login::JwkId;
    use tvm_vm::executor::zk_stuff::zk_login::OIDCProvider;
    use tvm_vm::executor::zk_stuff::zk_login::ZkLoginInputs;
    
    use serde::Deserialize;
    use serde_derive::Serialize;

    use tvm_vm::executor::zk_stuff::error::ZkCryptoError;
    use tvm_vm::stack::Stack;
    use tvm_vm::stack::StackItem;
    use tvm_vm::utils::pack_data_to_cell;
    use tvm_vm::utils::pack_string_to_cell;

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
    pub struct JwtDataDecodedPart1 {
        pub alg: String,
        pub kid: String,
        pub typ: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct JwtDataDecodedPart2 {
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

    #[test]
    fn test_modulus_bad() {
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

        let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
        let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
        let index_mod_4 = "1";

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

        let mut code = format!("PUSHINT {index_mod_4} \n").to_string();
        code = code + &*format!("PUSHINT {max_epoch} \n").to_string();
        code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"POSEIDON \n".to_string(); //

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        //expect_failure(tvm_types::ExceptionCode::FatalError);
        //.expect_success();

        println!("Too long modulus...");

        let mut modulus_spoiled = modulus.clone();
        modulus_spoiled.push(100);

        let modulus_cell = pack_data_to_cell(&modulus_spoiled.clone(), &mut 0).unwrap();

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_failure(tvm_types::ExceptionCode::FatalError);

        println!("Spoiled modulus...");

        let modulus_spoiled = vec![0; 2];

        let modulus_cell = pack_data_to_cell(&modulus_spoiled, &mut 0).unwrap();

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));

        println!("Spoiled modulus again...");

        let mut modulus_spoiled = modulus.clone();
        modulus_spoiled[10] = 67;

        let modulus_cell = pack_data_to_cell(&modulus_spoiled.clone(), &mut 0).unwrap();

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
       
    }

    #[test]
    fn test_iss_64_bad() {
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

        let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";

        let index_mod_4 = "1";

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

        ///// iss_base_64 handle /////
        
        println!("Test cut iss_base_64...");
        
        let iss_base_64_spoiled = "yJpc3MiOiJodHRwczovL";

        let iss_base_64_cell = pack_string_to_cell(&iss_base_64_spoiled, &mut 0).unwrap();


        let mut code = format!("PUSHINT {index_mod_4} \n").to_string();
        code = code + &*format!("PUSHINT {max_epoch} \n").to_string();
        code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"POSEIDON \n".to_string(); //

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        //.expect_failure(tvm_types::ExceptionCode::FatalError);
        //.expect_success();

        println!("Test empty iss_base_64...");
        
        let iss_base_64_spoiled = "";


        let iss_base_64_cell = pack_string_to_cell(&iss_base_64_spoiled, &mut 0).unwrap();

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));

          
        println!("Test too long wrong iss_base_64...");
        
        let iss_base_64_spoiled = "ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666";
 
 
        let iss_base_64_cell = pack_string_to_cell(&iss_base_64_spoiled, &mut 0).unwrap();
 
        println!("code : {code}");
 
        test_case_with_refs(code.as_str(), vec![
             modulus_cell.clone(),
             iss_base_64_cell.clone(),
             header_base_64_cell.clone(),
             zk_seed_cell.clone(),
         ])
         .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));

        println!("Test too too long wrong iss_base_64...");
        
        let iss_base_64_spoiled = "ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666ghg67666";
 
 
        let iss_base_64_cell = pack_string_to_cell(&iss_base_64_spoiled, &mut 0).unwrap();
 
        println!("code : {code}");
 
        test_case_with_refs(code.as_str(), vec![
             modulus_cell.clone(),
             iss_base_64_cell.clone(),
             header_base_64_cell.clone(),
             zk_seed_cell.clone(),
         ])
         .expect_failure(tvm_types::ExceptionCode::FatalError);
         //.expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));

    }

    #[test]
    fn test_header_bad() {
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

        let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
        let index_mod_4 = "1";

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

        let header_base_64 = "eyJhbGciOiJSUzI1NImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzElZGEzZTMiLCJ0eQifQ";

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();


        let mut code = format!("PUSHINT {index_mod_4} \n").to_string();
        code = code + &*format!("PUSHINT {max_epoch} \n").to_string();
        code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"POSEIDON \n".to_string(); //

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        
        println!("Empty header...");

        let header_base_64 = "";

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        
        println!("Too long wrong header...");

        let header_base_64 = "32789263432789263432789263432789263432789263432789263432789263432789263432786786786g32789263432789263432789263432789263432786786786g32789263432789263432789263432789263432786786786g";

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        
        println!("Too long wrong header...");

        let header_base_64 = "327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634ejwgdejhcg327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634dsjhgcjhwdcgjwgcdjhwgcdhc327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634327892634dgxhwjdcg";

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_failure(tvm_types::ExceptionCode::FatalError);
      

       
    }


    #[test]
    fn test_zk_seed_bad() {
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

        let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
        let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
        let index_mod_4 = "1";

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

        let header_base_64_cell = pack_string_to_cell(&header_base_64, &mut 0).unwrap();

        let iss_base_64_cell = pack_string_to_cell(&iss_base_64, &mut 0).unwrap();


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

        let modulus_cell = pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();


        ///// zk seed handle /////

        println!("Empty zk seed...");

        let zk_seed_spoiled: String = String::from("");

        let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();


        let mut code = format!("PUSHINT {index_mod_4} \n").to_string();
        code = code + &*format!("PUSHINT {max_epoch} \n").to_string();
        code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"POSEIDON \n".to_string(); //

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        //.expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        .expect_failure(tvm_types::ExceptionCode::FatalError);
        //.expect_success();

        println!("Wrong short zk seed...");

        let zk_seed_spoiled: String = String::from("190149130838213916597767154061365555081978517533943138452652822856495767863");

        let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();



        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        //.expect_failure(tvm_types::ExceptionCode::FatalError);
        //.expect_success();


        println!("Spoiled decimal position in zk seed...");

        let zk_seed_spoiled: String = String::from("18014913083821391659776715405561365555081978517533943138452652822856495767863");

        let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();



        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        //.expect_failure(tvm_types::ExceptionCode::FatalError);
        //.expect_success();

        println!("Not decimal symbol in zk seed...");

        let zk_seed_spoiled: String = String::from("a8014913083821391659776715405561365555081978517533943138452652822856495767863");

        let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();



        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        //.expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        .expect_failure(tvm_types::ExceptionCode::FatalError);
        //.expect_success();

        println!("Too long zk seed...");

        let zk_seed_spoiled: String = String::from("119014913083821391659776715405561365555081978517533943138452652822856495767863");

        let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();



        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        //.expect_failure(tvm_types::ExceptionCode::FatalError);
        //.expect_success();


        println!("Too too long zk seed...");

        let zk_seed_spoiled: String = String::from("119011190141190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678631190111901491308382139165977671540556136555508197851753394313845265282285649576786349130838213119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821391659776715405561365555081978517533943138452652822856495767863916597767154055613655550819785175339431384526528228564957678636437862786427864874269130838213916597767154055613655550819785175339431384526528228564957678634913083821311901119014913083821391659776715405561365555081978517533943138452652822856495767863491308382139165977671540556136555508197851753394313845265282285649576786391659776715405561365555081978517533943138452652822856495767863119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821311901119014913083821391659776715405561365555081978517533943138452652822856495767863491308382139165977671540556136555508197851753394313845265282285649576786391659776715405561365555081978517533943138452652822856495767863119011190149130838213916597767154055613655550819785175339431384526528228564957678634913083821311901119014913083821391659776715405561365555081978517533943138452652822856495767863491308382139165977671540556136555508197851753394313845265282285649576786391659776715405561365555081978517533943138452652822856495767863");

        let zk_seed_cell = pack_string_to_cell(&zk_seed_spoiled, &mut 0).unwrap();



        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        //.expect_failure(tvm_types::ExceptionCode::FatalError);
        //.expect_success();
       
    }




    #[test]
    fn test_other_args_bad() {
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

        let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
        let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";
        //let index_mod_4 = "1";

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

    

        let wrong_index_mod_4 = "256"; 

        let mut code = format!("PUSHINT {wrong_index_mod_4} \n").to_string();
        code = code + &*format!("PUSHINT {max_epoch} \n").to_string();
        code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"POSEIDON \n".to_string(); //

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        //.expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));
        .expect_failure(tvm_types::ExceptionCode::RangeCheckError);
        //.expect_success();

        let wrong_index_mod_4 = "255"; 

        let mut code = format!("PUSHINT {wrong_index_mod_4} \n").to_string();
        code = code + &*format!("PUSHINT {max_epoch} \n").to_string();
        code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"POSEIDON \n".to_string(); //

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));

        let wrong_index_mod_4 = "0"; 

        let mut code = format!("PUSHINT {wrong_index_mod_4} \n").to_string();
        code = code + &*format!("PUSHINT {max_epoch} \n").to_string();
        code = code + &*format!("PUSHINT {eph_pubkey_hex_number} \n").to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"PUSHREF \n".to_string();
        code = code + &*"POSEIDON \n".to_string(); //

        println!("code : {code}");

        test_case_with_refs(code.as_str(), vec![
            modulus_cell.clone(),
            iss_base_64_cell.clone(),
            header_base_64_cell.clone(),
            zk_seed_cell.clone(),
        ])
        .expect_not_stack(Stack::new().push(StackItem::Cell(public_inputs_cell.clone())));



       
    }

   

    
}
