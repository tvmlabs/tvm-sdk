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
        pub kid: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct JwtDataDecodedPart2 {
        pub iss: String,
        pub aud: String,
        pub sub: String,
        pub nonce: String,
        pub iat: u32,
        pub exp: u32,
        pub c_hash: String,
        pub auth_time: u32,
        pub nonce_supported: bool
    }

    #[ignore]
    #[test]
    fn test_vrgrth16() {
        let parsed_token = "eyJraWQiOiJXNldjT0tCIiwiYWxnIjoiUlMyNTYifQ.eyJpc3MiOiJodHRwczovL2FwcGxlaWQuYXBwbGUuY29tIiwiYXVkIjoibmwuZGlna2FzLndhbGxldC5jbGllbnQiLCJleHAiOjE2OTc4MjEwNzQsImlhdCI6MTY5NzczNDY3NCwic3ViIjoiMDAxMzkzLjc0YTEzNTRlZjc0YjRiOGViMWQyMDdkMzRkNzE2OGQ2LjE2MjkiLCJub25jZSI6ImhUUHBnRjdYQUtiVzM3ckVVUzZwRVZacW1vSSIsImNfaGFzaCI6Inl4dlh3Y1VXaHFUa1dpazQtQWh1UXciLCJhdXRoX3RpbWUiOjE2OTc3MzQ2NzQsIm5vbmNlX3N1cHBvcnRlZCI6dHJ1ZX0.LmGVSJY8rOpvsNob4fEqUecm_Y1ZitbW3lIK64f2QjgNUqnIpkO5sV0wXlVzlRWwGI4k3qURbwtTQO7Dw7kORaQIhlLzA1cZNHU22aXdQyQ9FIHPFgQecuudk-_0dvHB1IqhGsmvLv_qLJBQiuB7MGztVeZsgDYtXFs4dw04LCht0DNTEh_ihBRcJZkxHR9K13ItDiVUH5fLIRlfT70VgZWNuaGkKYfxeWg9nMD6medJU7VawWvXPt48YGtxIYcZqv6hlZwW14qGx-F2qg64NWjCSqwdBk5wqyhzpJdnErP79ESgGxpskNIZNn1JEzspJtgAS7Pmc0peV0hyg9FHtg";

        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "2Zc5d0-zkZ5AKmtYTvxHc3vRc41YfbklflxG9SWsg5qXUxvfgpktGAcxXLFAd9Uglzow9ezvmTGce5d3DhAYKwHAEPT9hbaMDj7DfmEwuNO8UahfnBkBXsCoUaL3QITF5_DAPsZroTqs7tkQQZ7qPkQXCSu2aosgOJmaoKQgwcOdjD0D49ne2B_dkxBcNCcJT9pTSWJ8NfGycjWAQsvC8CGstH8oKwhC5raDcc2IGXMOQC7Qr75d6J5Q24CePHj_JD7zjbwYy9KNH8wyr829eO_G4OEUW50FAN6HKtvjhJIguMl_1BLZ93z2KJyxExiNTZBUBQbbgCNBfzTv7JrxMw".to_string(),
            alg: "RS256".to_string()};

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Apple.get_config().iss,
                "W6WcOKB".to_string(), 
            ),
            content,
        );

        println!("all_jwk = {:?}", all_jwk);

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

        let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"21794804979959302369359978211243776052836338401081346616312255187078925599939\",\"3706467084233039313722071713030544276367996607128623823530648331506911438873\",\"1\"],\"b\":[[\"13409824287381659519546448219933458314514745471624578819993662473930468203082\",\"3640232423013629511755470986608600373810487641760916719963602388721548725313\"],[\"9655926467996979864874964416742099089084007635664099225748158947536392590786\",\"1677320540635567596740799862935493558153172684812817536434055465980655300270\"],[\"1\",\"0\"]],\"c\":[\"5884557104270886253680500406732541266038296156369520477108316933485454121938\",\"17029316959044275295449609805859499980068821464861424009238911367039275599407\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FwcGxlaWQuYXBwbGUuY29tIiw\",\"indexMod4\":1},\"headerBase64\":\"eyJraWQiOiJXNldjT0tCIiwiYWxnIjoiUlMyNTYifQ\"}";


        let zk_login_inputs =
                ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string())
                    .unwrap();

        let verification_key_id: u32 = 0;

        let max_epoch = 10; 

        let time_for_vergrth16 = single_vrgrth16(&eph_pubkey, &zk_login_inputs, &all_jwk, verification_key_id, max_epoch);
                    println!("time_for_vergrth16 is {time_for_vergrth16}");   
        
    }
 
}
