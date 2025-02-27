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

    pub const KAKAO_DATA: &str = "{\"jwt\":\"eyJraWQiOiI5ZjI1MmRhZGQ1ZjIzM2Y5M2QyZmE1MjhkMTJmZWEiLCJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiJjYzdjY2EwM2UwMTg0MjAzYzc3OTAzOWQ4MTFhMmIwZCIsInN1YiI6IjM4MTkxODQ2NjAiLCJhdXRoX3RpbWUiOjE3MzMyNTg1MTYsImlzcyI6Imh0dHBzOi8va2F1dGgua2FrYW8uY29tIiwiZXhwIjoxNzMzMjY1NzE2LCJpYXQiOjE3MzMyNTg1MTYsIm5vbmNlIjoiMkZSZlExX2FOVHNQQ2UxRHFoUldMMHk3Y09nIn0.WVGo_c4OwWqC0VTJ4FkHBlwxJg0YC5WmH_w3h1V-6dJN2f9DjSfQBzouD1BOVpro5clSEZvhC71nxHrXiBr0JJBqJDLuGNsOHLYEB7-NTMm5DusHrFIaHPGyFYiYlJy_wCQhBhOY-Y4Gl3s9i5W1o2l8gClTg0tbw_6zd_D14m27gKI1p64zxu7w7fW7pDJ6rHn0YHxCF4bD9BLyzL3o-2cj48TJeATWCFDCRgJ--FSKrZ-7__MM5LbtQ2aV4C1eJQWSdKZL3Grtp_9tJ_X1t983VcYJcJ56l3BMxG0vqaaA_8x-kUc7Zecjl3bWZoNJCBEG-PpCE1IlPNrPzJ2G2Q\",\"user_pass_to_int_format\":\"49505152\",\"zk_addr\":\"0xb124ee8b8a4b42f6bde8f992f27aab9c262878b1d61561dfdb4eaf1276de2c93\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":172,\"1\":187,\"2\":22,\"3\":103,\"4\":24,\"5\":70,\"6\":244,\"7\":203,\"8\":99,\"9\":251,\"10\":80,\"11\":199,\"12\":117,\"13\":49,\"14\":149,\"15\":28,\"16\":18,\"17\":209,\"18\":74,\"19\":139,\"20\":228,\"21\":45,\"22\":83,\"23\":140,\"24\":138,\"25\":82,\"26\":2,\"27\":247,\"28\":157,\"29\":117,\"30\":81,\"31\":184},\"secret_key\":{\"0\":197,\"1\":155,\"2\":75,\"3\":9,\"4\":95,\"5\":81,\"6\":48,\"7\":143,\"8\":201,\"9\":87,\"10\":103,\"11\":14,\"12\":105,\"13\":174,\"14\":254,\"15\":41,\"16\":68,\"17\":96,\"18\":241,\"19\":107,\"20\":24,\"21\":100,\"22\":113,\"23\":188,\"24\":236,\"25\":185,\"26\":113,\"27\":50,\"28\":151,\"29\":138,\"30\":249,\"31\":113,\"32\":172,\"33\":187,\"34\":22,\"35\":103,\"36\":24,\"37\":70,\"38\":244,\"39\":203,\"40\":99,\"41\":251,\"42\":80,\"43\":199,\"44\":117,\"45\":49,\"46\":149,\"47\":28,\"48\":18,\"49\":209,\"50\":74,\"51\":139,\"52\":228,\"53\":45,\"54\":83,\"55\":140,\"56\":138,\"57\":82,\"58\":2,\"59\":247,\"60\":157,\"61\":117,\"62\":81,\"63\":184}}},\"maxEpoch\":142,\"extended_ephemeral_public_key\":\"AKy7FmcYRvTLY/tQx3UxlRwS0UqL5C1TjIpSAveddVG4\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"4399464064442579559654731118204280055996139508436800044836210429355263825965\",\"5812531802997059214010956073655982972909613942920475584935335876555809665932\",\"1\"],\"b\":[[\"9189455423970304827149060791654507119083274852625632203619598406940460114303\",\"4857125762980786731043320166303404474076867315029491314704817028822366317078\"],[\"19102338593794999694577274987122287080191847841751195356661510370625053684715\",\"1807963682955372890006419651834382966812143925894805966129420265395373916597\"],[\"1\",\"0\"]],\"c\":[\"16438961670238980222017939455383463005453300560805303030780615266761463481671\",\"4459978613549646983224566437570106603554420773865861900287931039924604179165\",\"1\"]},\"issBase64Details\":{\"value\":\"ImlzcyI6Imh0dHBzOi8va2F1dGgua2FrYW8uY29tIiw\",\"indexMod4\":0},\"headerBase64\":\"eyJraWQiOiI5ZjI1MmRhZGQ1ZjIzM2Y5M2QyZmE1MjhkMTJmZWEiLCJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9\"}}";

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
        pub exp: u32,
        pub auth_time: u32,
    }

    //#[ignore]
    #[test]
    //#[cfg(feature = "gosh")]
    fn test_vrgrth16() {
        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "qGWf6RVzV2pM8YqJ6by5exoixIlTvdXDfYj2v7E6xkoYmesAjp_1IYL7rzhpUYqIkWX0P4wOwAsg-Ud8PcMHggfwUNPOcqgSk1hAIHr63zSlG8xatQb17q9LrWny2HWkUVEU30PxxHsLcuzmfhbRx8kOrNfJEirIuqSyWF_OBHeEgBgYjydd_c8vPo7IiH-pijZn4ZouPsEg7wtdIX3-0ZcXXDbFkaDaqClfqmVCLNBhg3DKYDQOoyWXrpFKUXUFuk2FTCqWaQJ0GniO4p_ppkYIf4zhlwUYfXZEhm8cBo6H2EgukntDbTgnoha8kNunTPekxWTDhE5wGAt6YpT4Yw".to_string(), 
            alg: "RS256".to_string(),
        };

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Kakao.get_config().iss,
                "9f252dadd5f233f93d2fa528d12fea".to_string(),
            ),
            content,
        );

        let data = [KAKAO_DATA];

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
