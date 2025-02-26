#[allow(dead_code)]
#[cfg(test)]
//#[cfg(feature = "gosh")]
mod tests {
    use std::collections::HashMap;

    use ark_std::rand::SeedableRng;
    use ark_std::rand::rngs::StdRng;
    use base64::decode;
    use fastcrypto::ed25519::Ed25519KeyPair;
    use fastcrypto::traits::KeyPair;
    use num_bigint::BigUint;
    use serde::Deserialize;
    use tvm_vm::executor::zk_stuff::utils::gen_address_seed;
    use tvm_vm::executor::zk_stuff::zk_login::JWK;
    use tvm_vm::executor::zk_stuff::zk_login::JwkId;
    use tvm_vm::executor::zk_stuff::zk_login::OIDCProvider;
    use tvm_vm::executor::zk_stuff::zk_login::ZkLoginInputs;

    use crate::instructions_tests::zk::test_helper::single_vrgrth16;

    #[derive(Debug, Deserialize)]
    pub struct JwtDataDecodedPart1 {
        pub alg: String,
        pub typ: String,
        pub kid: String,
        pub x5t: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct JwtDataDecodedPart2 {
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

    //#[ignore]
    #[test]
    //#[cfg(feature = "gosh")]
    fn test_vrgrth16() {
        let parsed_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjYyNzA1RUEwMjMwMDAyNTFENUUwRDZCQkQyMkQzODFDMEVFQzlBOTgiLCJ4NXQiOiJZbkJlb0NNQUFsSFY0TmE3MGkwNEhBN3NtcGciLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmthcnJpZXIub25lLyIsImV4cCI6MTcxNjI2NTM4MSwiaWF0IjoxNzE2MjY0MTgxLCJhdWQiOiJkYXNoYm9hcmQtZGV2Iiwic3ViIjoiYzM3OWI2NmYtOTJmNC00MTUxLTkyZmQtNjczNTFlMjRkOWQ5IiwiZW1haWwiOiJhYnVjaGFuYW5Aa2Fycmllci5vbmUiLCJuYW1lIjoiKzE3ODA4ODQ1MTk3IiwicHJlZmVycmVkX3VzZXJuYW1lIjoiKzE3ODA4ODQ1MTk3Iiwib2lfYXVfaWQiOiIwOGZiYjNjZS0wNWJkLTRhM2QtYmZmZC1iMDRkOWVmMGM4ZDMiLCJhenAiOiJkYXNoYm9hcmQtZGV2Iiwibm9uY2UiOiJoVFBwZ0Y3WEFLYlczN3JFVVM2cEVWWnFtb0kiLCJvaV90a25faWQiOiJjZjYwZmY5YS02MTY2LTQ3MjktYWFjYy05ZDQ1MGE4MDYzMTIifQ.SK7CT1joG64b-ybyFbm0cD4PbOGoiAAmWehc35PzdPo74EYPRKwBDfxEVCADtKBvPeG0vVXkT0bCMGbfGdztJY5s0WdjzwqjsPrl4IUz4hYhhIiA0kMlShUx65DzY-5Z015ldP0z4fEMeW-FI2B2Atri0gYMuJDLem4oRt-MrPpINgia9xle9L6hRjwE0u45us7iEY6Xuwab_LILlJsFt3u9QhpRX-vJWxbEj0YL28OkYORWJk1XK-FGt7wkEGwWDkU6enMRpVv0psDHJOlv8RQsWy5nS0jfcXlUFGkT-BdOCoyZGHF3mbcRweuddNDc2bfXNhScKyuLzVtXQdgWnA";

        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "z8dTyyTn-tMqElE6U93mmFafAk6st6IX_YSSu1UZAdxo9LZlTY-RKTtAg3dgcyYdEgTFCy4ws7Gq1Fu7zlg3UM41k6OfsKV2bCKAWdKIivqOr7hh5r976N58YSVjpC-wLLWanc8AnrdVtZT0CGC40PzIUVjpzFvfTO46dmjOiv2HtSxnTFp7Cqk2UfZYYp2p4KuMGDapdfsoV5MnociiOnRxD9Q0zMnPua6DnlMwE2tGTr4RvHpPHps99BObEr3YrxQrUljaiBmntjApzJXmxN0A8UcL2VFPFUSN1ds629vK9xj_3vZSIhlZ7vpp3PWcR436bwU5TIMZbUboo63ctQ".to_string(),
            alg: "RS256".to_string(),};

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::KarrierOne.get_config().iss,
                "62705EA023000251D5E0D6BBD22D381C0EEC9A98".to_string(),
            ),
            content,
        );

        println!("all_jwk = {:?}", all_jwk);

        let max_epoch = 10;
        // let jwt_randomness = "100681567828351849884072155819400689117";
        // A dummy salt
        let user_pass_salt = "129390038577185583942388216820280642146";

        // Generate an ephemeral key pair.
        let kp = Ed25519KeyPair::generate(&mut StdRng::from_seed([0; 32]));
        let mut eph_pubkey = vec![0x00]; //Vec::new();//
        eph_pubkey.extend(kp.public().as_ref());
        let kp_bigint = BigUint::from_bytes_be(&eph_pubkey).to_string();

        // let kp_bigint =
        // "84029355920633174015103288781128426107680789454168570548782290541079926444544"
        // ;

        println!("kp_bigint = {:?} ", kp_bigint);

        let jwt_data_vector: Vec<&str> = parsed_token.split(".").collect();
        let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

        let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
        println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is

        // JwtDataDecodedPart1
        let jwt_data_decoded1: JwtDataDecodedPart1 = serde_json::from_str(&jwt_string_1).unwrap();

        println!("kid: {:?}", jwt_data_decoded1.kid);

        let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2);

        let jwt_data_decoded2: JwtDataDecodedPart2 = serde_json::from_str(&jwt_string_2).unwrap();

        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);

        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            jwt_data_decoded2.sub.as_str(),
            jwt_data_decoded2.aud.as_str(),
        )
        .unwrap();

        let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"812448915722006790501929450771670438652382136516840850464035157301231341781\",\"20606343486020729810075161167405138191390640162527319578598684677624333039129\",\"1\"],\"b\":[[\"10056567425825077775789754191571270990729417215312365936935379254172085747424\",\"2088939852002971421270958325067372566815928163518865801233831310338769976728\"],[\"1769995345188670140066251309709227605187837157925817392971466728816260693607\",\"19085836306204549098609030411494551382498415364670352061926212959015966168617\"],[\"1\",\"0\"]],\"c\":[\"19528699632920317846374171575867261639392861967053284521697473256333671614570\",\"19219047279279298578962437921210344809131187973087332036999604532473720425752\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmthcnJpZXIub25lLyIs\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjYyNzA1RUEwMjMwMDAyNTFENUUwRDZCQkQyMkQzODFDMEVFQzlBOTgiLCJ4NXQiOiJZbkJlb0NNQUFsSFY0TmE3MGkwNEhBN3NtcGciLCJ0eXAiOiJKV1QifQ\"}";
        let zk_login_inputs =
            ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string()).unwrap();

        let verification_key_id: u32 = 0;
        let time_for_vergrth16 = single_vrgrth16(
            &eph_pubkey,
            &zk_login_inputs,
            &all_jwk,
            verification_key_id,
            max_epoch,
        );
        println!("time_for_vergrth16 is {time_for_vergrth16} micro seconds");
    }
}
