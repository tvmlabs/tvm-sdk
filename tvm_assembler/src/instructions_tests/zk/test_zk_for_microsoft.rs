#[allow(dead_code)]
#[cfg(test)]
//#[cfg(feature = "gosh")]
mod tests {
    use std::collections::HashMap;

    use base64::decode;
    use fastcrypto::ed25519::Ed25519KeyPair;
    use fastcrypto::traits::KeyPair;
    use fastcrypto::traits::ToFromBytes;
    use serde::Deserialize;
    use tvm_vm::executor::zk_stuff::utils::gen_address_seed;
    use tvm_vm::executor::zk_stuff::zk_login::JWK;
    use tvm_vm::executor::zk_stuff::zk_login::JwkId;
    use tvm_vm::executor::zk_stuff::zk_login::OIDCProvider;
    use tvm_vm::executor::zk_stuff::zk_login::ZkLoginInputs;

    use crate::instructions_tests::zk::test_helper::JwtData;
    use crate::instructions_tests::zk::test_helper::secret_key_from_integer_map;
    use crate::instructions_tests::zk::test_helper::single_vrgrth16;

    pub const MICROSOFT_DATA: &str = "{\"jwt\":\"eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6Inp4ZWcyV09OcFRrd041R21lWWN1VGR0QzZKMCJ9.eyJhdWQiOiI3MTNjYzk3MS1kMTBiLTQyZjAtYThlZS03ZWJjYWY3M2I3NTQiLCJpc3MiOiJodHRwczovL2xvZ2luLm1pY3Jvc29mdG9ubGluZS5jb20vMTliYTQzNWQtZTQ2Yy00MzZhLTg0ZjItMWIwMWU2OTNlNDgwL3YyLjAiLCJpYXQiOjE3MzM0MDMzMzQsIm5iZiI6MTczMzQwMzMzNCwiZXhwIjoxNzMzNDA3MjM0LCJhaW8iOiJBVFFBeS84WUFBQUF6cTBQWXZ3YXVhR2NYSGtzcEIxdjdUL1o1akNBdlBKb280a1NUWXNlaHNjNW56WE1OUWZzby91eHNtOG5zVllYIiwibm9uY2UiOiJldHpSWXNtTXRTMnVHMDhwNEhjZWtiVXNiTk0iLCJyaCI6IjEuQVZ3QVhVTzZHV3prYWtPRThoc0I1cFBrZ0hISlBIRUwwZkJDcU81LXZLOXp0MVJjQUFwY0FBLiIsInN1YiI6IklKNnNxdDNRTm9NNjFhUV9vV2dIQXRjTUJzUXh2eXlPOS00ck1iTExrT2ciLCJ0aWQiOiIxOWJhNDM1ZC1lNDZjLTQzNmEtODRmMi0xYjAxZTY5M2U0ODAiLCJ1dGkiOiI0ZzdhUnFlRUFVYVVkZk9mTGNVM0FBIiwidmVyIjoiMi4wIn0.lKBzgKNETElu5So53v-7CNw6CVWciw2a5Zog3MLis0dP4fRQvIz-44FyEhMdDdirGxHVLLAQerOOS5lYx0vCihz0EspxpP5CrtW5uOM_KnMYJk6Hkulrq48-yuKXfKB3l1JK7579kSpZc4UzZz4ZKCtGsIgFzBiRHg1eFI6i2YiQwqpLqrbh2ME-tulXtYkuPB5hkOxx66_52yinZtm2br_GoXjyBvlmKFQyELmbd9cs8683qy7hwipSse7KDqySB04pBqYUGyp2ALWVPdb8CrMsPjozU4fwHnTvcJPXVOmX0u7T9H4-NAPp5g83ROpCS55I8ybqd6Ve-0Uil2ovug&session_state=e91dee2b-9c5f-40f4-8af6-719c87466def\", \"user_pass_to_int_format\":\"49505152\", \"zk_addr\":\"0x64f157d6fb92d4389db431f0828516651d3805ce3268ee2d56abd188f202b169\", \"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":39,\"1\":48,\"2\":197,\"3\":39,\"4\":92,\"5\":45,\"6\":123,\"7\":135,\"8\":43,\"9\":76,\"10\":191,\"11\":233,\"12\":207,\"13\":156,\"14\":128,\"15\":14,\"16\":204,\"17\":16,\"18\":68,\"19\":248,\"20\":229,\"21\":215,\"22\":203,\"23\":189,\"24\":162,\"25\":76,\"26\":154,\"27\":178,\"28\":122,\"29\":44,\"30\":8,\"31\":179}, \"secret_key\":{\"0\":159,\"1\":214,\"2\":18,\"3\":254,\"4\":191,\"5\":115,\"6\":156,\"7\":226,\"8\":229,\"9\":146,\"10\":211,\"11\":51,\"12\":81,\"13\":111,\"14\":101,\"15\":33,\"16\":160,\"17\":179,\"18\":53,\"19\":95,\"20\":164,\"21\":134,\"22\":167,\"23\":65,\"24\":71,\"25\":190,\"26\":169,\"27\":219,\"28\":136,\"29\":78,\"30\":14,\"31\":136,\"32\":39,\"33\":48,\"34\":197,\"35\":39,\"36\":92,\"37\":45,\"38\":123,\"39\":135,\"40\":43,\"41\":76,\"42\":191,\"43\":233,\"44\":207,\"45\":156,\"46\":128,\"47\":14,\"48\":204,\"49\":16,\"50\":68,\"51\":248,\"52\":229,\"53\":215,\"54\":203,\"55\":189,\"56\":162,\"57\":76,\"58\":154,\"59\":178,\"60\":122,\"61\":44,\"62\":8,\"63\":179}}},\"maxEpoch\":142, \"extended_ephemeral_public_key\":\"ACcwxSdcLXuHK0y/6c+cgA7MEET45dfLvaJMmrJ6LAiz\", \"zk_proofs\":{\"proofPoints\":{\"a\":[\"17137990231957793317975225132965381963730913983728324875139538151641767739907\",\"15220444311807827196475559074300170686944326391368342123537808259707001188246\",\"1\"],\"b\":[[\"3759089320316495642126774422368579915055767796485787191870421544522615881403\",\"3084706229311317057422402941985513602639281686518596184473515255610713104923\"],[\"8493915494628037526451233519569362746582715621568744471888007467628912451304\",\"792173932412042019876114170987950593509307280947942955952893142973028901309\"],[\"1\",\"0\"]],\"c\":[\"8744954679061282555124535766628336249898222760451969135599179136137078963670\",\"2186498562620143679547775617437575369206327134194558292650559386954874303549\",\"1\"]}, \"issBase64Details\":{\"value\":\"CJpc3MiOiJodHRwczovL2xvZ2luLm1pY3Jvc29mdG9ubGluZS5jb20vMTliYTQzNWQtZTQ2Yy00MzZhLTg0ZjItMWIwMWU2OTNlNDgwL3YyLjAiLC\",\"indexMod4\":1},\"headerBase64\":\"eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6Inp4ZWcyV09OcFRrd041R21lWWN1VGR0QzZKMCJ9\"}}";

    #[derive(Debug, Deserialize)]
    pub struct JwtDataDecodedPart1 {
        pub alg: String,
        pub kid: String,
        pub typ: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct JwtDataDecodedPart2 {
        pub iss: String,
        pub aud: String,
        pub sub: String,
        pub nonce: String,
        pub iat: u32,
        pub nbf: u32,
        pub exp: u32,
        pub aio: String,
        pub rh: String,
        pub tid: String,
        pub uti: String,
        pub ver: String,
    }

    //#[ignore]
    #[test]
    //#[cfg(feature = "gosh")]
    fn test_vrgrth16() {
        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "vqEEyvePAnDTT2fd-E_MZm5i6AfwrkHwmWicYmHhsdLXnxVPNSuPjxmTe3UedZBU2Q6OUU5Dv5I4vjryhChnHIxamu4iZsumig8LL2_BqYQVqR6E1mmzpQubanJepJWvKW4aEgLelcK0EXnZSf-_uEPbR2MLgmXo7MW6a3qSqnxLYyQClxbtQML06E7ALXspTaQt7RA6qgtgK8sIuArhcdSghFnfWsQ9Ksr_CI96H50snLTQk9PBHAfwpPK80ha67tQ2uU08zZS_lADdpm0D3r_PgAyhTgaSOvSTGC02-Qv4vht3mG8B1oHprb0XB23B82jUEr6VQL6wbVkEwHU6Tw".to_string(),
            alg: "RS256".to_string(),
        };

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Microsoft.get_config().iss,
                "zxeg2WONpTkwN5GmeYcuTdtC6J0".to_string(),
            ),
            content,
        );

        let data = [MICROSOFT_DATA];

        let max_epoch = 142;
        let verification_key_id: u32 = 0;

        for i in 0..data.len() {
            println!("====================== Iter@ is {i} =========================");
            // parse
            let jwt_data: JwtData = serde_json::from_str(&data[i]).unwrap();
            // println!("{:?}", jwt_data);

            let user_pass_salt = jwt_data.user_pass_to_int_format.as_str();
            println!("user_pass_salt is {user_pass_salt}");

            println!("{:?}", jwt_data.ephemeral_key_pair.keypair.public_key);
            let eph_secret_key =
                secret_key_from_integer_map(jwt_data.ephemeral_key_pair.keypair.secret_key);

            let ephemeral_kp = Ed25519KeyPair::from_bytes(&eph_secret_key).unwrap();
            let mut eph_pubkey = Vec::new(); // vec![0x00];
            eph_pubkey.extend(ephemeral_kp.public().as_ref());

            println!("ephemeral secret_key is {:?}", eph_secret_key);
            println!("ephemeral public_key is {:?}", eph_pubkey);

            let eph_pubkey_len = eph_pubkey.clone().len();
            println!("len eph_pubkey: {:?}", eph_pubkey_len);

            let jwt_data_vector: Vec<&str> = jwt_data.jwt.split(".").collect();
            let jwt_data_1 = decode(jwt_data_vector[0]).expect("Base64 decoding failed");

            let jwt_string_1 = String::from_utf8(jwt_data_1).expect("UTF-8 conversion failed");
            println!("jwt_string_1 is {:?}", jwt_string_1); // jwt_string_1 is

            // JwtDataDecodedPart1
            let jwt_data_decoded1: JwtDataDecodedPart1 =
                serde_json::from_str(&jwt_string_1).unwrap();
            println!("kid: {:?}", jwt_data_decoded1.kid);

            let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
            let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
            println!("jwt_string_2 is {:?}", jwt_string_2); // "{\"iss\":\"https://accounts.google.com\",\"azp\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"aud\":\"232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com\",\"sub\":\"112897468626716626103\",\"nonce\":\"bxmnJW31ruzKMGir01YPGYL0xDY\",\"nbf\":1715687036,\"iat\":1715687336,\"exp\":1715690936,\"jti\":\"9b601d25f003640c2889a2a047789382cb1cfe87\"}"

            // JwtDataDecodedPart2
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

            println!("jwt_data.zk_proofs = {:?}", jwt_data.zk_proofs);
            let proof_and_jwt = serde_json::to_string(&jwt_data.zk_proofs).unwrap();

            let zk_login_inputs =
                ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string())
                    .unwrap();

            let time_for_vergrth16 = single_vrgrth16(
                &eph_pubkey,
                &zk_login_inputs,
                &all_jwk,
                verification_key_id,
                max_epoch,
            );
            println!("time_for_vergrth16 is {time_for_vergrth16} micro seconds");

            println!("==========================================");
        }
    }
}
