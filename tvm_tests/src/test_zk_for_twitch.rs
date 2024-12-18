#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use base64::decode;
    use fastcrypto::ed25519::Ed25519KeyPair;
    use fastcrypto::traits::KeyPair;
    use fastcrypto::traits::ToFromBytes;
    
    use tvm_vm::executor::zk_stuff::utils::gen_address_seed;
    use tvm_vm::executor::zk_stuff::zk_login::JWK;
    use tvm_vm::executor::zk_stuff::zk_login::JwkId;
    use tvm_vm::executor::zk_stuff::zk_login::OIDCProvider;
    use tvm_vm::executor::zk_stuff::zk_login::ZkLoginInputs;

    use serde::Deserialize;

    use crate::test_helper::test_helper::{JwtData, single_vrgrth16, secret_key_from_integer_map};

    pub const TWITCH_DATA: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ.eyJhdWQiOiI2cm05Z254cXZvNDJvcHJmbnF4OGI3aHB0cWtmbjkiLCJleHAiOjE3MzMyMTI3MjgsImlhdCI6MTczMzIxMTgyOCwiaXNzIjoiaHR0cHM6Ly9pZC50d2l0Y2gudHYvb2F1dGgyIiwic3ViIjoiMTIxMDM3MTI4NCIsImF6cCI6IjZybTlnbnhxdm80Mm9wcmZucXg4YjdocHRxa2ZuOSIsIm5vbmNlIjoidXo4bGYzRFhQaWI3T3pEVHVtQnRPZEg0Uk5ZIiwicHJlZmVycmVkX3VzZXJuYW1lIjoiYWxpbmF0OTUifQ.NyHeUJRbME6fKKlMf8EGpZLbuu6egmKfWPueiLRuGjuzHaJJClzatI9-Xf526KpnWS7cKt10rhbZx6VOV7tjRxb3ARzZu52jY1CWyBvBgfqct5XJN67kE0GybBNXo40EwhCKRuS3pNjNHsrP9G5fdNYDZRG6d3hVvvD9rsTS6Rtxl-uUz-JI_1C2pomUM9qNPcXQhCy-WFvq1fJ2AZi9qE5ZeQBVB9LhW0tWA0oF0ho5pggS4H4_wX2qx1_b_WMBfDW8oYybuAfuW9Uf4J7rgze9pePIkktna4ZIoiMDhFc7TUKQiW8k-z9RJ0oqF1NenSDqruRC7RaVGjpqJnsYdw\",\"user_pass_to_int_format\":\"49505152\",\"zk_addr\":\"0x54570f826deb9cca5498c43fb92b63a175a929214ce9a849d5ae7b29d20f3b5b\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":67,\"1\":151,\"2\":191,\"3\":17,\"4\":230,\"5\":112,\"6\":244,\"7\":134,\"8\":200,\"9\":230,\"10\":47,\"11\":135,\"12\":250,\"13\":196,\"14\":155,\"15\":97,\"16\":34,\"17\":196,\"18\":158,\"19\":38,\"20\":32,\"21\":184,\"22\":255,\"23\":219,\"24\":229,\"25\":253,\"26\":9,\"27\":39,\"28\":153,\"29\":110,\"30\":29,\"31\":244},\"secret_key\":{\"0\":118,\"1\":234,\"2\":132,\"3\":235,\"4\":35,\"5\":86,\"6\":14,\"7\":125,\"8\":235,\"9\":218,\"10\":61,\"11\":111,\"12\":125,\"13\":207,\"14\":12,\"15\":70,\"16\":116,\"17\":99,\"18\":48,\"19\":180,\"20\":122,\"21\":120,\"22\":169,\"23\":150,\"24\":41,\"25\":77,\"26\":68,\"27\":112,\"28\":98,\"29\":164,\"30\":80,\"31\":226,\"32\":67,\"33\":151,\"34\":191,\"35\":17,\"36\":230,\"37\":112,\"38\":244,\"39\":134,\"40\":200,\"41\":230,\"42\":47,\"43\":135,\"44\":250,\"45\":196,\"46\":155,\"47\":97,\"48\":34,\"49\":196,\"50\":158,\"51\":38,\"52\":32,\"53\":184,\"54\":255,\"55\":219,\"56\":229,\"57\":253,\"58\":9,\"59\":39,\"60\":153,\"61\":110,\"62\":29,\"63\":244}}},\"maxEpoch\":142,\"extended_ephemeral_public_key\":\"ALkW5W6lM6+D2DUfiDU0JnmKuete4r4kScr71UmD7m4S\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"5512737220132661764763433666303745333622817838157246990491883457014772558790\",\"7741191131945326076605655541116327963387701820619204753669967744720810301194\",\"1\"],\"b\":[[\"975939824369696756004366458915319869031560554018320503553850551436103240713\",\"3318646493849785946054690844048321975990817279130676318569276416356932186331\"],[\"20363373817397831189796445832313036107992446943102660272248262372612972428627\",\"10352628036168685925901531114370646835780137031944433752597548025858647162549\"],[\"1\",\"0\"]],\"c\":[\"16764924754241855698364440343184497893704852638018863579795588761000070680187\",\"12850393671928953693043671631466186518426175933356485428915985566432485937123\",\"1\"]},\"issBase64Details\":{\"value\":\"wiaXNzIjoiaHR0cHM6Ly9pZC50d2l0Y2gudHYvb2F1dGgyIiw\",\"indexMod4\":2},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ\"}}";
    
    
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
        pub azp: String,
        pub preferred_username: String,
    }

    #[ignore]
    #[test]
    fn test_vrgrth16() {

        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "6lq9MQ-q6hcxr7kOUp-tHlHtdcDsVLwVIw13iXUCvuDOeCi0VSuxCCUY6UmMjy53dX00ih2E4Y4UvlrmmurK0eG26b-HMNNAvCGsVXHU3RcRhVoHDaOwHwU72j7bpHn9XbP3Q3jebX6KIfNbei2MiR0Wyb8RZHE-aZhRYO8_-k9G2GycTpvc-2GBsP8VHLUKKfAs2B6sW3q3ymU6M0L-cFXkZ9fHkn9ejs-sqZPhMJxtBPBxoUIUQFTgv4VXTSv914f_YkNw-EjuwbgwXMvpyr06EyfImxHoxsZkFYB-qBYHtaMxTnFsZBr6fn8Ha2JqT1hoP7Z5r5wxDu3GQhKkHw".to_string(),
            alg: "RS256".to_string(),
        };

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Twitch.get_config().iss,
                "1".to_string(), 
            ),
            content,
        );

        let data = [
            TWITCH_DATA
        ];

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

            let time_for_vergrth16 = single_vrgrth16(&eph_pubkey, &zk_login_inputs, &all_jwk, verification_key_id, max_epoch);
            println!("time_for_vergrth16 is {time_for_vergrth16}");

            println!("==========================================");
        }
    }

    
}
