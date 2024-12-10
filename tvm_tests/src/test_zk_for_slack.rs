#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
  
    use std::time::Instant;

    use ark_std::rand::{rngs::StdRng, SeedableRng};
    use num_bigint::BigUint;

    use base64::decode;
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
    
    use tvm_types::Cell;

    use tvm_vm::executor::zk_stuff::error::ZkCryptoError;

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
        pub typ: String,
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
        pub at_hash: String,
        pub auth_time: u32,
        pub nonce_supported: bool,
    }

    /*
    {
  "iss": "https://slack.com",
  "sub": "U03MR0T0QMU",
  "aud": "2426087588661.5742457039348",
  "exp": 1698165680,
  "iat": 1698165380,
  "auth_time": 1698165380,
  "nonce": "hTPpgF7XAKbW37rEUS6pEVZqmoI",
  "at_hash": "ZlEhq6eEblPPZ5UZ9vdf0w",
  "https://slack.com/team_id": "T02CJ2KHAKF",
  "https://slack.com/user_id": "U03MR0T0QMU"
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
        let parsed_token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6Im1CMk1BeUtTbjU1NWlzZDBFYmRoS3g2bmt5QWk5eExxOHJ2Q0ViX25PeVkifQ.eyJpc3MiOiJodHRwczpcL1wvc2xhY2suY29tIiwic3ViIjoiVTAzTVIwVDBRTVUiLCJhdWQiOiIyNDI2MDg3NTg4NjYxLjU3NDI0NTcwMzkzNDgiLCJleHAiOjE2OTgxNjU2ODAsImlhdCI6MTY5ODE2NTM4MCwiYXV0aF90aW1lIjoxNjk4MTY1MzgwLCJub25jZSI6ImhUUHBnRjdYQUtiVzM3ckVVUzZwRVZacW1vSSIsImF0X2hhc2giOiJabEVocTZlRWJsUFBaNVVaOXZkZjB3IiwiaHR0cHM6XC9cL3NsYWNrLmNvbVwvdGVhbV9pZCI6IlQwMkNKMktIQUtGIiwiaHR0cHM6XC9cL3NsYWNrLmNvbVwvdXNlcl9pZCI6IlUwM01SMFQwUU1VIn0.GzkVxav70jC5TAKffNi2bZoRjtT2kDBr5oY_dJpbIoDsFP6IGRQ8181y1aoSpeJAi0bhjdB-h9wFsJOo6eY3rWh5om3z3cA4zm4qOCjSHCup90s80LP4emw_oZRQ_Wj8Q0F4YTkrDLW4CYJZYn0kMo7efM9ChT8henKQP-Yz2n_-8VzrT2uudv7hRLyGKvgf0xGvDcs_UVbOKR_lFXLaksSPJgTEx48cLHA979e8aH68Zv7b4sWv4D1qUEAu4YuJkXQ573023zq5QDpUki0qSow2gaqxdNUW2XOSxqV9ImZcsXqea769kP2rJvNgNnur4hO6wB7I_ImXsIn70aU-lQ";
        let content: JWK = JWK {
            kty: "RSA".to_string(),
            e: "AQAB".to_string(),
            n: "zQqzXfb677bpMKw0idKC5WkVLyqk04PWMsWYJDKqMUUuu_PmzdsvXBfHU7tcZiNoHDuVvGDqjqnkLPEzjXnaZY0DDDHvJKS0JI8fkxIfV1kNy3DkpQMMhgAwnftUiSXgb5clypOmotAEm59gHPYjK9JHBWoHS14NYEYZv9NVy0EkjauyYDSTz589aiKU5lA-cePG93JnqLw8A82kfTlrJ1IIJo2isyBGANr0YzR-d3b_5EvP7ivU7Ph2v5JcEUHeiLSRzIzP3PuyVFrPH659Deh-UAsDFOyJbIcimg9ITnk5_45sb_Xcd_UN6h5I7TGOAFaJN4oi4aaGD4elNi_K1Q".to_string(),
            alg: "RS256".to_string(),};

        let mut all_jwk = HashMap::new();
        all_jwk.insert(
            JwkId::new(
                OIDCProvider::Slack.get_config().iss,
                "mB2MAyKSn555isd0EbdhKx6nkyAi9xLq8rvCEb_nOyY".to_string(), 
            ),
            content,
        );

        println!("all_jwk = {:?}", all_jwk);

        let max_epoch = 10;
        let jwt_randomness = "100681567828351849884072155819400689117";
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

       /* let jwt_data_2 = decode(jwt_data_vector[1]).expect("Base64 decoding failed");
        let jwt_string_2 = String::from_utf8(jwt_data_2).expect("UTF-8 conversion failed");
        println!("jwt_string_2 is {:?}", jwt_string_2); 


        let jwt_data_decoded2: JwtDataDecodedPart2 =
                serde_json::from_str(&jwt_string_2).unwrap();

        println!("aud: {:?}", jwt_data_decoded2.aud);
        println!("sub: {:?}", jwt_data_decoded2.sub);*/

        let aud = "2426087588661.5742457039348";
        let sub = "U03MR0T0QMU";


        let zk_seed = gen_address_seed(
            user_pass_salt,
            "sub",
            sub, //jwt_data_decoded2.sub.as_str(), 
            aud//jwt_data_decoded2.aud.as_str(), 
        )
        .unwrap();

        let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"19508631872749825068935870018265784117861865573424350889432961628986943943460\",\"18527802882269200969230761895509672328185268965794317503430195232491748946581\",\"1\"],\"b\":[[\"20579276344330402551860570961554219653348757957690057944990921282263019700869\",\"19296082404156926707387231232024313141715665581104526286540316969201773351101\"],[\"1746349363416478114435793546674051879256513185944258245304452776141252804784\",\"13940376417492856527814056827766821569072133985880717537912726615146421205536\"],[\"1\",\"0\"]],\"c\":[\"10291620914291411809151828500778488795852641171652698023219955362740804074082\",\"559262222428023349342750782944557360772770760306984913080130511014557070193\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczpcL1wvc2xhY2suY29tIiw\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6Im1CMk1BeUtTbjU1NWlzZDBFYmRoS3g2bmt5QWk5eExxOHJ2Q0ViX25PeVkifQ\"}";
        
        let zk_login_inputs =
                ZkLoginInputs::from_json(&*proof_and_jwt.to_string(), &*zk_seed.to_string())
                    .unwrap();


        let time_for_vergrth16 = single_vrgrth16(&eph_pubkey, &zk_login_inputs, &all_jwk);
                    println!("time_for_vergrth16 is {time_for_vergrth16}");
        
        
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
    }

 
}
