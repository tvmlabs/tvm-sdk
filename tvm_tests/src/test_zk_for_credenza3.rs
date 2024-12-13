#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
  
    use ark_std::rand::{rngs::StdRng, SeedableRng};
    use num_bigint::BigUint;

    use base64::decode;
    use fastcrypto::ed25519::Ed25519KeyPair;
    use fastcrypto::traits::KeyPair;
    
    use tvm_vm::executor::zk_stuff::utils::gen_address_seed;
    use tvm_vm::executor::zk_stuff::zk_login::JWK;
    use tvm_vm::executor::zk_stuff::zk_login::JwkId;
    use tvm_vm::executor::zk_stuff::zk_login::OIDCProvider;
    use tvm_vm::executor::zk_stuff::zk_login::ZkLoginInputs;

    use serde::Deserialize;
    use crate::test_helper::test_helper::{single_vrgrth16};

    #[derive(Debug, Deserialize)]
    pub struct JwtDataDecodedPart1 {
        pub alg: String,
        pub typ: String,
        pub kid: String
    }

    #[derive(Debug, Deserialize)]
    pub struct JwtDataDecodedPart2 {
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

    #[ignore]
    #[test]
    fn test_vrgrth16() {
        let parsed_token = "eyJ0eXAiOiJqd3QiLCJhbGciOiJSUzI1NiIsImtpZCI6IlFfeGNBejhnalRWZm5pdnZSZjZXSEF6MDBpeUZodHNlcl9BVnVWeF8wRmMifQ.eyJpYXQiOjE3MTU3MjUyNzcsImV4cCI6MTcxNTcyNTg3NywiYXVkIjoiNjU5NTRlYzVkMDNkYmEwMTk4YWMzNDNhIiwiaXNzIjoiaHR0cHM6Ly9hY2NvdW50cy5jcmVkZW56YTMuY29tIiwic3ViIjoiNjY0M2UzZGNkYTA0MTliZTg1NWMxMzU4Iiwic2NvcGUiOiJvcGVuaWQgcHJvZmlsZSBlbWFpbCBwaG9uZSBibG9ja2NoYWluLmV2bSBibG9ja2NoYWluLmV2bS53cml0ZSIsInRva2VuX3R5cGUiOiJCZWFyZXIiLCJ0b2tlbl91c2UiOiJpZCIsIm5vbmNlIjoiaFRQcGdGN1hBS2JXMzdyRVVTNnBFVlpxbW9JIiwibG9naW5fdHlwZSI6ImNyZWRlbnRpYWxzIiwiZW1haWwiOiJqb3lAbXlzdGVubGFicy5jb20ifQ.e70Kj00oV6vlQfK7_n4ca0P4UH0Fk8kJ3nGtHsgWtH1OAlpKPP51QNcAaig8TSN0UnJhakiXEB5KJ_lc6XsQ84s4NxBVnlLfwCxBQJ5NCkyqIRw5k4oVyeJIodj61JOo6wqMvsZhfb86MZRYPQv7-369tDRLnezZn0DrQYOD5NAwDzj2YKeuWTxRDlcUDeNjyPVzh3Um1eFUFjWOFzHDqtes3O7-9kvzDKiGnewzb8-7aKZFkie2ggcPgnLapz1SnqflzQB_YL3g_57eKUogPMHCENklG1bbhdKxemczFQXzrFqs1Wl4X-_XOXB_Z59b7SFswqPqk1APdBoYGxkALA";
    
        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "qlPW7v3-QzyfBvsDO0FK67HaunFgflRsjKdsVHgJs53ZTbaC1x0mDBkrqiwpZQAdMPDP-jMLzyq_0T3BDjA0v5wrvFjvQdIFHp9kVNS6UaiDRDSxRXOhJWGt23HafTnMWjYQJfUrEHgbMcke7qsRRewV3fcQHy0d7khMJ5SgSPWo7c42WybRP9eF5EyWkDwZppZQH_XYdlo3ucG-j4JvV6Mz85hJkxhszn303V12q-CckqZ-HT3drenpCKLZoZ5yvZwlKVIgmwCothT5lRqwj-U1jZaoTPn8su4HCF2ujv80DZNKBVHGOcenNNfwbVTdnHF5G0z_BCP2rgFmhPm0BQ".to_string(),
            alg: "RS256".to_string(),};

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Credenza3.get_config().iss,
                "Q_xcAz8gjTVfnivvRf6WHAz00iyFhtser_AVuVx_0Fc".to_string(), 
            ),
            content,
        );

        println!("all_jwk = {:?}", all_jwk);

        let max_epoch = 10;
        //let jwt_randomness = "100681567828351849884072155819400689117";
        // A dummy salt
        let user_pass_salt = "129390038577185583942388216820280642146";

        // Generate an ephemeral key pair.
        let kp = Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
        let mut eph_pubkey = vec![0x00]; //Vec::new();//
        eph_pubkey.extend(kp.public().as_ref());
        let kp_bigint = BigUint::from_bytes_be(&eph_pubkey).to_string();

        //let kp_bigint = "84029355920633174015103288781128426107680789454168570548782290541079926444544";
        
        println!("kp_bigint = {:?} ", kp_bigint);

        let jwt_data_vector: Vec<&str> = parsed_token.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is
      

        // JwtDataDecodedPart1
        let jwt_data_decoded1: JwtDataDecodedPart1 =
                serde_json::from_str(&jwt_string_1).unwrap();

        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2); 


        let jwt_data_decoded2: JwtDataDecodedPart2 =
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

        let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"18523619921984701350997646243356070880390102932143055245871365877811644985476\",\"500218213585829849458692616749186043011745842753125633416993470304428465786\",\"1\"],\"b\":[[\"21056361482901031897935490250005077440619242903573104366577829786342773516521\",\"12542592672958726404552043928797494976165015480123521559270584962324089051712\"],[\"18303153658765010724412226066945720817302171223468756845913694544653033669476\",\"12035817243318280091407363690782291765125669297577234573208078322179355767968\"],[\"1\",\"0\"]],\"c\":[\"6070532770347865721935992301158253677663412015380269462364542577053267095295\",\"16369601657744272109670546956191233205982012077647802519239302449452526249745\",\"1\"]},\"issBase64Details\":{\"value\":\"wiaXNzIjoiaHR0cHM6Ly9hY2NvdW50cy5jcmVkZW56YTMuY29tIiw\",\"indexMod4\":2},\"headerBase64\":\"eyJ0eXAiOiJqd3QiLCJhbGciOiJSUzI1NiIsImtpZCI6IlFfeGNBejhnalRWZm5pdnZSZjZXSEF6MDBpeUZodHNlcl9BVnVWeF8wRmMifQ\"}";
        let zk_login_inputs =
                ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string())
                    .unwrap();

        let verification_key_id: u32 = 0;

        let time_for_vergrth16 = single_vrgrth16(&eph_pubkey, &zk_login_inputs, &all_jwk, verification_key_id, max_epoch);
        println!("time_for_vergrth16 is {time_for_vergrth16}");
        
        
    }


    /*fn prepare_proof_and_public_key_cells_for_stack(
        eph_pubkey: &Vec<u8>,
        zk_login_inputs: &ZkLoginInputs,
        all_jwk: &HashMap<JwkId, JWK>,
    ) -> (Cell, Cell) {
        let (iss, kid) =
            (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
        println!("kid = {}", kid);
        println!("all_jwk = {:?}", all_jwk);

        let jwk = all_jwk
            .get(&JwkId::new(iss.clone(), kid.clone()))
            .ok_or_else(|| {
                ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
            })
            .unwrap();

        let max_epoch = 10; 

        // Decode modulus to bytes.
        let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
            .map_err(|_| {
                ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
            })
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

        let verification_key_id: u32 = 0;

        let mut code = "PUSHREF \n".to_string();
        code = code + "PUSHREF \n";
        code = code + "PUSHINT " + &*verification_key_id.to_string() + "\n";
        code = code + "VERGRTH16";

        let start: Instant = Instant::now();
        test_case_with_refs(code.as_str(), vec![proof_cell.clone(), public_inputs_cell.clone()])
            .expect_success();
        start.elapsed().as_micros()
    }*/

 
}
