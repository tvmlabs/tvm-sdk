#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
  
    use std::time::Instant;


    use base64::decode;
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

    use serde::Deserialize;
    use serde_derive::Serialize;
    
    use tvm_types::Cell;

    use tvm_vm::executor::zk_stuff::error::ZkCryptoError;

    use tvm_vm::utils::pack_data_to_cell;


    use crate::test_framework::Expects;
    use crate::test_framework::test_case_with_refs;


    //pub const FACEBOOK_DATA: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImIzNjQwYjliODY2YTdlM2EzNjc2MThjYWY0OWJlMjBjODVjMDA2NDcifQ.eyJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIsImF1ZCI6IjQ2NTcyNDU5NjEyMjAyOSIsInN1YiI6IjM5NDQwNDYyNjU4MzY4MzciLCJpYXQiOjE3MzI4MDkyODQsImV4cCI6MTczMjgxMjg4NCwianRpIjoiTjd0QS5iZGFhMWFhYjQxN2QyZjcxNmU0YWFjOGFjNjIxOTFhNDhlZDVmZjkzODI3YzA3MWVlMTVlODk2ZTYzNWM0NzRjIiwibm9uY2UiOiJweG9zSF9vSFoweTM2OUM4c3k0d3lJME55X2ciLCJnaXZlbl9uYW1lIjoiXHUwNDEwXHUwNDNiXHUwNDM4XHUwNDNkXHUwNDMwIiwiZmFtaWx5X25hbWUiOiJcdTA0MTBcdTA0M2JcdTA0MzhcdTA0M2RcdTA0M2VcdTA0MzJcdTA0M2RcdTA0MzAiLCJuYW1lIjoiXHUwNDEwXHUwNDNiXHUwNDM4XHUwNDNkXHUwNDMwIFx1MDQxMFx1MDQzYlx1MDQzOFx1MDQzZFx1MDQzZVx1MDQzMlx1MDQzZFx1MDQzMCIsInBpY3R1cmUiOiJodHRwczpcL1wvcGxhdGZvcm0tbG9va2FzaWRlLmZic2J4LmNvbVwvcGxhdGZvcm1cL3Byb2ZpbGVwaWNcLz9hc2lkPTM5NDQwNDYyNjU4MzY4MzcmaGVpZ2h0PTEwMCZ3aWR0aD0xMDAmZXh0PTE3MzU0MDEyODUmaGFzaD1BYllHREI3TXFxY1Zqb3RmTUdsbzBCdDgifQ.kBJ5Xc4b39z_Sbn5Ysua5sMPvVKroj5fDdVB59yHDetYe0QjMPwiJt9PPkx8Dqzva0ZcYAuxz-pVHo6PH2kPkpxChvS5X0gv3RTXPFAN--agBFOK-9zGdHsYarAlB7wiZ2mG6hdWwulLFGyhVTLTQGXW71qD-Qo41nWxPZpkOmL97YuTyQWwzpGL9yNROMbm9FzFRFco21GfjO4bFkUKqU2Cudj6Go79uBmyfC6GXxOVKQBUTjc53bgiRxsnCB_XJVWdaCLqDZgVVw-0j-vtV1Fiw6_Kw4PhW4OOLe-ObYImUG39Yn-iY9u5PgnkwOemmvqwTWLzoGPbE1npdeMzzw\",\"user_pass_to_int_format\":\"49505152\",\"zk_addr\":\"0x007b9ad7b4c4301c6fcff30dff0a60d2bc047a84f9f884a53294a60646d75052\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":185,\"1\":22,\"2\":229,\"3\":110,\"4\":165,\"5\":51,\"6\":175,\"7\":131,\"8\":216,\"9\":53,\"10\":31,\"11\":136,\"12\":53,\"13\":52,\"14\":38,\"15\":121,\"16\":138,\"17\":185,\"18\":235,\"19\":94,\"20\":226,\"21\":190,\"22\":36,\"23\":73,\"24\":202,\"25\":251,\"26\":213,\"27\":73,\"28\":131,\"29\":238,\"30\":110,\"31\":18},\"secret_key\":{\"0\":177,\"1\":88,\"2\":130,\"3\":121,\"4\":56,\"5\":71,\"6\":208,\"7\":157,\"8\":236,\"9\":16,\"10\":70,\"11\":56,\"12\":164,\"13\":92,\"14\":157,\"15\":65,\"16\":111,\"17\":114,\"18\":123,\"19\":25,\"20\":239,\"21\":238,\"22\":151,\"23\":60,\"24\":65,\"25\":59,\"26\":70,\"27\":25,\"28\":63,\"29\":110,\"30\":113,\"31\":216,\"32\":185,\"33\":22,\"34\":229,\"35\":110,\"36\":165,\"37\":51,\"38\":175,\"39\":131,\"40\":216,\"41\":53,\"42\":31,\"43\":136,\"44\":53,\"45\":52,\"46\":38,\"47\":121,\"48\":138,\"49\":185,\"50\":235,\"51\":94,\"52\":226,\"53\":190,\"54\":36,\"55\":73,\"56\":202,\"57\":251,\"58\":213,\"59\":73,\"60\":131,\"61\":238,\"62\":110,\"63\":18}}},\"maxEpoch\":142,\"extended_ephemeral_public_key\":\"ALkW5W6lM6+D2DUfiDU0JnmKuete4r4kScr71UmD7m4S\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"7269633675479506664200010144613270138485890407861566483377692676852557562218\",\"13587998474892985687301676263522084547365602899882348938501733685489981566914\",\"1\"],\"b\":[[\"642240896264072090180719588055219671717749387061086854151891869266642845510\",\"12175247506924153356664510876830077939237267768131586808029992829056946471625\"],[\"15218532967355660749775646424437448577155434269216505254311069415565134887309\",\"2372202838541023723807866940956654762916701825047820674030555190958854060965\"],[\"1\",\"0\"]],\"c\":[\"2898608196136591814205076818985925390711890021831207435260124321751664465794\",\"13971063586547807697292502962370796575248643312588599300730318814751096354553\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIs\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImIzNjQwYjliODY2YTdlM2EzNjc2MThjYWY0OWJlMjBjODVjMDA2NDcifQ\"}}";
    //proof in above test data was created by external mysten proover with their test key

    pub const TWITCH_DATA: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ.eyJhdWQiOiI2cm05Z254cXZvNDJvcHJmbnF4OGI3aHB0cWtmbjkiLCJleHAiOjE3MzMyMTI3MjgsImlhdCI6MTczMzIxMTgyOCwiaXNzIjoiaHR0cHM6Ly9pZC50d2l0Y2gudHYvb2F1dGgyIiwic3ViIjoiMTIxMDM3MTI4NCIsImF6cCI6IjZybTlnbnhxdm80Mm9wcmZucXg4YjdocHRxa2ZuOSIsIm5vbmNlIjoidXo4bGYzRFhQaWI3T3pEVHVtQnRPZEg0Uk5ZIiwicHJlZmVycmVkX3VzZXJuYW1lIjoiYWxpbmF0OTUifQ.NyHeUJRbME6fKKlMf8EGpZLbuu6egmKfWPueiLRuGjuzHaJJClzatI9-Xf526KpnWS7cKt10rhbZx6VOV7tjRxb3ARzZu52jY1CWyBvBgfqct5XJN67kE0GybBNXo40EwhCKRuS3pNjNHsrP9G5fdNYDZRG6d3hVvvD9rsTS6Rtxl-uUz-JI_1C2pomUM9qNPcXQhCy-WFvq1fJ2AZi9qE5ZeQBVB9LhW0tWA0oF0ho5pggS4H4_wX2qx1_b_WMBfDW8oYybuAfuW9Uf4J7rgze9pePIkktna4ZIoiMDhFc7TUKQiW8k-z9RJ0oqF1NenSDqruRC7RaVGjpqJnsYdw\",\"user_pass_to_int_format\":\"49505152\",\"zk_addr\":\"0x54570f826deb9cca5498c43fb92b63a175a929214ce9a849d5ae7b29d20f3b5b\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":67,\"1\":151,\"2\":191,\"3\":17,\"4\":230,\"5\":112,\"6\":244,\"7\":134,\"8\":200,\"9\":230,\"10\":47,\"11\":135,\"12\":250,\"13\":196,\"14\":155,\"15\":97,\"16\":34,\"17\":196,\"18\":158,\"19\":38,\"20\":32,\"21\":184,\"22\":255,\"23\":219,\"24\":229,\"25\":253,\"26\":9,\"27\":39,\"28\":153,\"29\":110,\"30\":29,\"31\":244},\"secret_key\":{\"0\":118,\"1\":234,\"2\":132,\"3\":235,\"4\":35,\"5\":86,\"6\":14,\"7\":125,\"8\":235,\"9\":218,\"10\":61,\"11\":111,\"12\":125,\"13\":207,\"14\":12,\"15\":70,\"16\":116,\"17\":99,\"18\":48,\"19\":180,\"20\":122,\"21\":120,\"22\":169,\"23\":150,\"24\":41,\"25\":77,\"26\":68,\"27\":112,\"28\":98,\"29\":164,\"30\":80,\"31\":226,\"32\":67,\"33\":151,\"34\":191,\"35\":17,\"36\":230,\"37\":112,\"38\":244,\"39\":134,\"40\":200,\"41\":230,\"42\":47,\"43\":135,\"44\":250,\"45\":196,\"46\":155,\"47\":97,\"48\":34,\"49\":196,\"50\":158,\"51\":38,\"52\":32,\"53\":184,\"54\":255,\"55\":219,\"56\":229,\"57\":253,\"58\":9,\"59\":39,\"60\":153,\"61\":110,\"62\":29,\"63\":244}}},\"maxEpoch\":142,\"extended_ephemeral_public_key\":\"ALkW5W6lM6+D2DUfiDU0JnmKuete4r4kScr71UmD7m4S\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"5512737220132661764763433666303745333622817838157246990491883457014772558790\",\"7741191131945326076605655541116327963387701820619204753669967744720810301194\",\"1\"],\"b\":[[\"975939824369696756004366458915319869031560554018320503553850551436103240713\",\"3318646493849785946054690844048321975990817279130676318569276416356932186331\"],[\"20363373817397831189796445832313036107992446943102660272248262372612972428627\",\"10352628036168685925901531114370646835780137031944433752597548025858647162549\"],[\"1\",\"0\"]],\"c\":[\"16764924754241855698364440343184497893704852638018863579795588761000070680187\",\"12850393671928953693043671631466186518426175933356485428915985566432485937123\",\"1\"]},\"issBase64Details\":{\"value\":\"wiaXNzIjoiaHR0cHM6Ly9pZC50d2l0Y2gudHYvb2F1dGgyIiw\",\"indexMod4\":2},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ\"}}";
    
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
        pub aud: String,
        pub sub: String,
        pub nonce: String,
        pub iat: u32,
        pub exp: u32,
        pub azp: String,
        pub preferred_username: String,
    }

    /*
    {
  "aud": "6rm9gnxqvo42oprfnqx8b7hptqkfn9",
  "exp": 1733212728,
  "iat": 1733211828,
  "iss": "https://id.twitch.tv/oauth2",
  "sub": "1210371284",
  "azp": "6rm9gnxqvo42oprfnqx8b7hptqkfn9",
  "nonce": "uz8lf3DXPib7OzDTumBtOdH4RNY",
  "preferred_username": "alinat95"
}
  
    */

    fn secret_key_from_integer_map(key_data: HashMap<String, u8>) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        for i in 0..=31 {
            if let Some(value) = key_data.get(&i.to_string()) {
                vec.push(value.clone());
            }
        }
        return vec;
    }

    #[ignore]
    #[test]
    fn test_vrgrth16() {
        // todo: later n must be extracted from 3d part of jwt

        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "6lq9MQ-q6hcxr7kOUp-tHlHtdcDsVLwVIw13iXUCvuDOeCi0VSuxCCUY6UmMjy53dX00ih2E4Y4UvlrmmurK0eG26b-HMNNAvCGsVXHU3RcRhVoHDaOwHwU72j7bpHn9XbP3Q3jebX6KIfNbei2MiR0Wyb8RZHE-aZhRYO8_-k9G2GycTpvc-2GBsP8VHLUKKfAs2B6sW3q3ymU6M0L-cFXkZ9fHkn9ejs-sqZPhMJxtBPBxoUIUQFTgv4VXTSv914f_YkNw-EjuwbgwXMvpyr06EyfImxHoxsZkFYB-qBYHtaMxTnFsZBr6fn8Ha2JqT1hoP7Z5r5wxDu3GQhKkHw".to_string(),
            alg: "RS256".to_string(),
        };

        /*
        {"keys":[{"alg":"RS256","e":"AQAB","kid":"1","kty":"RSA","n":"6lq9MQ-q6hcxr7kOUp-tHlHtdcDsVLwVIw13iXUCvuDOeCi0VSuxCCUY6UmMjy53dX00ih2E4Y4UvlrmmurK0eG26b-HMNNAvCGsVXHU3RcRhVoHDaOwHwU72j7bpHn9XbP3Q3jebX6KIfNbei2MiR0Wyb8RZHE-aZhRYO8_-k9G2GycTpvc-2GBsP8VHLUKKfAs2B6sW3q3ymU6M0L-cFXkZ9fHkn9ejs-sqZPhMJxtBPBxoUIUQFTgv4VXTSv914f_YkNw-EjuwbgwXMvpyr06EyfImxHoxsZkFYB-qBYHtaMxTnFsZBr6fn8Ha2JqT1hoP7Z5r5wxDu3GQhKkHw","use":"sig"}]}
        */

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Twitch.get_config().iss,
                "1".to_string(), 
            ),
            content,
        );

        // let sui_data = [SUI_DATA_FROM_REACT_1];
        let sui_data = [
            TWITCH_DATA
        ];

        let mut sum_ratio: u128 = 0;

        for i in 0..sui_data.len() {
            println!("====================== Iter@ is {i} =========================");
            // parse
            let jwt_data: JwtData = serde_json::from_str(&sui_data[i]).unwrap();
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

            let time_for_vergrth16 = single_vrgrth16(&eph_pubkey, &zk_login_inputs, &all_jwk);
            println!("time_for_vergrth16 is {time_for_vergrth16}");

            println!("==========================================");
        }
    }

    fn prepare_proof_and_public_key_cells_for_stack(
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

        let max_epoch = 142; // data from the react test

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
    }

    
}
