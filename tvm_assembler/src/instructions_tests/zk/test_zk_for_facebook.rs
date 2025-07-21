#[allow(dead_code)]
#[cfg(test)]
//#[cfg(feature = "gosh")]
mod tests {
    use std::collections::HashMap;
    use tvm_vm::stack::Stack;
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
    use tvm_vm::stack::integer::IntegerData;
use tvm_vm::stack::StackItem;
use tvm_vm::int;

    use crate::instructions_tests::zk::test_helper::JwtData;
    use crate::instructions_tests::zk::test_helper::secret_key_from_integer_map;
    use crate::instructions_tests::zk::test_helper::single_vrgrth16;

    pub const FACEBOOK_DATA: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImIzNjQwYjliODY2YTdlM2EzNjc2MThjYWY0OWJlMjBjODVjMDA2NDcifQ.eyJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIsImF1ZCI6IjQ2NTcyNDU5NjEyMjAyOSIsInN1YiI6IjM5NDQwNDYyNjU4MzY4MzciLCJpYXQiOjE3MzI4MDkyODQsImV4cCI6MTczMjgxMjg4NCwianRpIjoiTjd0QS5iZGFhMWFhYjQxN2QyZjcxNmU0YWFjOGFjNjIxOTFhNDhlZDVmZjkzODI3YzA3MWVlMTVlODk2ZTYzNWM0NzRjIiwibm9uY2UiOiJweG9zSF9vSFoweTM2OUM4c3k0d3lJME55X2ciLCJnaXZlbl9uYW1lIjoiXHUwNDEwXHUwNDNiXHUwNDM4XHUwNDNkXHUwNDMwIiwiZmFtaWx5X25hbWUiOiJcdTA0MTBcdTA0M2JcdTA0MzhcdTA0M2RcdTA0M2VcdTA0MzJcdTA0M2RcdTA0MzAiLCJuYW1lIjoiXHUwNDEwXHUwNDNiXHUwNDM4XHUwNDNkXHUwNDMwIFx1MDQxMFx1MDQzYlx1MDQzOFx1MDQzZFx1MDQzZVx1MDQzMlx1MDQzZFx1MDQzMCIsInBpY3R1cmUiOiJodHRwczpcL1wvcGxhdGZvcm0tbG9va2FzaWRlLmZic2J4LmNvbVwvcGxhdGZvcm1cL3Byb2ZpbGVwaWNcLz9hc2lkPTM5NDQwNDYyNjU4MzY4MzcmaGVpZ2h0PTEwMCZ3aWR0aD0xMDAmZXh0PTE3MzU0MDEyODUmaGFzaD1BYllHREI3TXFxY1Zqb3RmTUdsbzBCdDgifQ.kBJ5Xc4b39z_Sbn5Ysua5sMPvVKroj5fDdVB59yHDetYe0QjMPwiJt9PPkx8Dqzva0ZcYAuxz-pVHo6PH2kPkpxChvS5X0gv3RTXPFAN--agBFOK-9zGdHsYarAlB7wiZ2mG6hdWwulLFGyhVTLTQGXW71qD-Qo41nWxPZpkOmL97YuTyQWwzpGL9yNROMbm9FzFRFco21GfjO4bFkUKqU2Cudj6Go79uBmyfC6GXxOVKQBUTjc53bgiRxsnCB_XJVWdaCLqDZgVVw-0j-vtV1Fiw6_Kw4PhW4OOLe-ObYImUG39Yn-iY9u5PgnkwOemmvqwTWLzoGPbE1npdeMzzw\",\"user_pass_to_int_format\":\"49505152\",\"zk_addr\":\"0x007b9ad7b4c4301c6fcff30dff0a60d2bc047a84f9f884a53294a60646d75052\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":185,\"1\":22,\"2\":229,\"3\":110,\"4\":165,\"5\":51,\"6\":175,\"7\":131,\"8\":216,\"9\":53,\"10\":31,\"11\":136,\"12\":53,\"13\":52,\"14\":38,\"15\":121,\"16\":138,\"17\":185,\"18\":235,\"19\":94,\"20\":226,\"21\":190,\"22\":36,\"23\":73,\"24\":202,\"25\":251,\"26\":213,\"27\":73,\"28\":131,\"29\":238,\"30\":110,\"31\":18},\"secret_key\":{\"0\":177,\"1\":88,\"2\":130,\"3\":121,\"4\":56,\"5\":71,\"6\":208,\"7\":157,\"8\":236,\"9\":16,\"10\":70,\"11\":56,\"12\":164,\"13\":92,\"14\":157,\"15\":65,\"16\":111,\"17\":114,\"18\":123,\"19\":25,\"20\":239,\"21\":238,\"22\":151,\"23\":60,\"24\":65,\"25\":59,\"26\":70,\"27\":25,\"28\":63,\"29\":110,\"30\":113,\"31\":216,\"32\":185,\"33\":22,\"34\":229,\"35\":110,\"36\":165,\"37\":51,\"38\":175,\"39\":131,\"40\":216,\"41\":53,\"42\":31,\"43\":136,\"44\":53,\"45\":52,\"46\":38,\"47\":121,\"48\":138,\"49\":185,\"50\":235,\"51\":94,\"52\":226,\"53\":190,\"54\":36,\"55\":73,\"56\":202,\"57\":251,\"58\":213,\"59\":73,\"60\":131,\"61\":238,\"62\":110,\"63\":18}}},\"maxEpoch\":142,\"extended_ephemeral_public_key\":\"ALkW5W6lM6+D2DUfiDU0JnmKuete4r4kScr71UmD7m4S\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"7269633675479506664200010144613270138485890407861566483377692676852557562218\",\"13587998474892985687301676263522084547365602899882348938501733685489981566914\",\"1\"],\"b\":[[\"642240896264072090180719588055219671717749387061086854151891869266642845510\",\"12175247506924153356664510876830077939237267768131586808029992829056946471625\"],[\"15218532967355660749775646424437448577155434269216505254311069415565134887309\",\"2372202838541023723807866940956654762916701825047820674030555190958854060965\"],[\"1\",\"0\"]],\"c\":[\"2898608196136591814205076818985925390711890021831207435260124321751664465794\",\"13971063586547807697292502962370796575248643312588599300730318814751096354553\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIs\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImIzNjQwYjliODY2YTdlM2EzNjc2MThjYWY0OWJlMjBjODVjMDA2NDcifQ\"}}";


    pub const FACEBOOK_DATA_2: &str = "{\"jwt\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImQ4N2QyNDc0ODk2ZjIxM2VlNTJlNjA2OWFlMGRkMTU1MzM0MGEwOGMifQ.eyJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIsImF1ZCI6IjQ2NTcyNDU5NjEyMjAyOSIsInN1YiI6IjM5NDQwNDYyNjU4MzY4MzciLCJpYXQiOjE3NTI2MTUzODQsImV4cCI6MTc1MjYxODk4NCwianRpIjoiUERHcy5iY2JhNzI2ZWRlMGNhYzUzNTJjOWQ5ZmJlYTczZDEwNmViYmEzOWUzMDJmYjZkZTMzZWQ1OTlhMTE5MWU2M2U1Iiwibm9uY2UiOiJDNEY1UGdGVEg4amRLQ3dZUkxDWDJQc1BiZlUiLCJnaXZlbl9uYW1lIjoiXHUwNDEwXHUwNDNiXHUwNDM4XHUwNDNkXHUwNDMwIiwiZmFtaWx5X25hbWUiOiJcdTA0MTBcdTA0M2JcdTA0MzhcdTA0M2RcdTA0M2VcdTA0MzJcdTA0M2RcdTA0MzAiLCJuYW1lIjoiXHUwNDEwXHUwNDNiXHUwNDM4XHUwNDNkXHUwNDMwIFx1MDQxMFx1MDQzYlx1MDQzOFx1MDQzZFx1MDQzZVx1MDQzMlx1MDQzZFx1MDQzMCIsInBpY3R1cmUiOiJodHRwczpcL1wvcGxhdGZvcm0tbG9va2FzaWRlLmZic2J4LmNvbVwvcGxhdGZvcm1cL3Byb2ZpbGVwaWNcLz9hc2lkPTM5NDQwNDYyNjU4MzY4MzcmaGVpZ2h0PTEwMCZ3aWR0aD0xMDAmZXh0PTE3NTUyMDczODUmaGFzaD1BVDhjYXFRLVdleWlTWFBfR2NDcWRIYmsifQ.itdC4hXpVQaq9N_9blDiiKYwecGfPXEm3tXYSbwRQ2AiMrkoolziQ5qd4K0a3l2-hsnniD2qRdskWqZsCYjKP6KUE0UitWwHZYyypaScnxPZZHyCJmllCuFYKwTcYgNvK_QdXh_1lr6l2g6D5UXxyh5JFDuu5zcYQHmHHT3z4hIElTjRvAf3Abs2IQfycMwIVvcEdeJe1eSKABcwVv7qrphr9Bf6EazjpBLtlFwodpEyjRgHniuoBHPu6IM9VZuRCIaGrWLJSLeHXRIVqkP3Of2Tskj2EN9xibgPpZITdfuIRvthA2154Zbk2853Xl1SQg7SF1w4rEjPhl1mFRyYig\",\"user_pass_to_int_format\":\"535455565748\",\"zk_addr\":\"0x8fe042d5f16bc3e9e7fe2e282d56c42f018e65fea745c35a85d485c9d5e29969\",\"ephemeral_key_pair\":{\"keypair\":{\"public_key\":{\"0\":192,\"1\":32,\"2\":198,\"3\":100,\"4\":183,\"5\":233,\"6\":4,\"7\":41,\"8\":196,\"9\":119,\"10\":87,\"11\":131,\"12\":27,\"13\":78,\"14\":207,\"15\":175,\"16\":3,\"17\":17,\"18\":223,\"19\":203,\"20\":33,\"21\":0,\"22\":25,\"23\":92,\"24\":55,\"25\":90,\"26\":41,\"27\":110,\"28\":183,\"29\":209,\"30\":125,\"31\":203},\"secret_key\":{\"0\":138,\"1\":164,\"2\":19,\"3\":76,\"4\":65,\"5\":242,\"6\":197,\"7\":243,\"8\":232,\"9\":69,\"10\":52,\"11\":25,\"12\":92,\"13\":42,\"14\":78,\"15\":176,\"16\":149,\"17\":41,\"18\":23,\"19\":237,\"20\":203,\"21\":74,\"22\":29,\"23\":135,\"24\":103,\"25\":236,\"26\":105,\"27\":73,\"28\":213,\"29\":237,\"30\":233,\"31\":173,\"32\":192,\"33\":32,\"34\":198,\"35\":100,\"36\":183,\"37\":233,\"38\":4,\"39\":41,\"40\":196,\"41\":119,\"42\":87,\"43\":131,\"44\":27,\"45\":78,\"46\":207,\"47\":175,\"48\":3,\"49\":17,\"50\":223,\"51\":203,\"52\":33,\"53\":0,\"54\":25,\"55\":92,\"56\":55,\"57\":90,\"58\":41,\"59\":110,\"60\":183,\"61\":209,\"62\":125,\"63\":203}}},\"maxEpoch\":1837114018,\"extended_ephemeral_public_key\":\"AMAgxmS36QQpxHdXgxtOz68DEd/LIQAZXDdaKW630X3L\",\"zk_proofs\":{\"proofPoints\":{\"a\":[\"2462569216429571312007124289082630220401530723684912514166683366599699515539\",\"7883821542718525444091301469123358819780258365694589397889572410033752768278\",\"1\"],\"b\":[[\"2351917491525896894596397339414693508942272445003022943327429885171724593513\",\"15954576869770155190826948931450206948983828571722169741523592666702018090523\"],[\"2375679854915147062110128367485439044173775008177572852094708591857305802863\",\"3858921924155309032950196640309813006096991762314983759261692814930796775298\"],[\"1\",\"0\"]],\"c\":[\"20427193921456089297737019108797399317941127534506254906097503006873270177037\",\"5082645918023219178920606941558600931735655771354722424867057533628742920487\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczpcL1wvd3d3LmZhY2Vib29rLmNvbSIs\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6ImQ4N2QyNDc0ODk2ZjIxM2VlNTJlNjA2OWFlMGRkMTU1MzM0MGEwOGMifQ\"}}";

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
        pub jti: String,
        pub given_name: String,
        pub family_name: String,
        pub name: String,
        pub picture: String,
    }

  

    //#[ignore]
    #[test]
    //#[cfg(feature = "gosh")]
    fn test_vrgrth16() {
        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "orUzBcoyhkBP6cS3XLTBz1OvYdCwA-1YPQbNSH5MB1j4TkfuT7goo8MLaVtdI02f2I1T7kfCjliHGuAXryRCTGNJF6bZf9b4jUBxeo7rK0uZLj7R0KtjydCOAxJETslfQP-xm7M2cEbijeB_2CA6PCZjKx2OMXVSKNNHKEZABrEFK-LxpTj_B8Z--VqCB7tZ69GHjb22BspD3COpK3hUkqOu_JPxew_kaYLbtcRAe4omEnNyJGd3XjD5mkxMADygI_OPxSB8EcmdPHhwmgUf1CKcji_bML-dEzMCDrJO-ph_Qh8UJsjUCeinXKoU44X_5KY-Wo9WC3gHKEt7gJ3Qdw".to_string(), // Alina's data
            alg: "RS256".to_string(),
        };

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Facebook.get_config().iss,
                "b3640b9b866a7e3a367618caf49be20c85c00647".to_string(),
            ),
            content,
        );

        let data = [FACEBOOK_DATA];

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

            let verification_key_id: u32 = 0;
            let max_epoch = 142;

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
