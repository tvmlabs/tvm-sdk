use crate::executor::Engine;
use crate::types::Status;


use std::borrow::Cow;
use std::fmt::Error;

use tvm_block::GlobalCapabilities;
use tvm_types::error;
use tvm_types::BuilderData;
use tvm_types::ExceptionCode;
use tvm_types::GasConsumer;
use tvm_types::UInt256;
use tvm_types::SliceData;

use super::zk_stuff::curve_utils::*;
use base64ct::{Base64UrlUnpadded,Encoding};
use super::zk_stuff::error::*;
use super::zk_stuff::zk_login::*;

use std::str::FromStr;
use std::time::Instant;

pub use ark_bn254::{Bn254, Fr as Bn254Fr};
use ark_bn254::Fr;
use ark_groth16::{Groth16, PreparedVerifyingKey, Proof, VerifyingKey};
use ark_snark::SNARK;
use im::HashMap;

use crate::error::TvmError;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::stack::integer::serialization::UnsignedIntegerBigEndianEncoding;
use crate::stack::integer::IntegerData;
use crate::stack::StackItem;
use crate::types::Exception;
use crate::utils::bytes_to_string;
use crate::utils::pack_data_to_cell;
use crate::utils::unpack_data_from_cell;
use crate::executor::zk_stuff::bn254::FieldElement;
use once_cell::sync::Lazy;



/**
    Here there are Groth16 verification keys taken from Mysten sui for now fo tests
    todo: will be replaced by our keys later
    todo: move all key data to json config file (?), use hash as id
**/

/////////////////////////////////////////////////////////////////////////////////////////////////////////
static GLOBAL_VERIFYING_KEY: Lazy<PreparedVerifyingKey<Bn254>> = Lazy::new(global_pvk);

/// Corresponding to proofs generated from prover-dev. Used in devnet/testnet.
static INSECURE_VERIFYING_KEY: Lazy<PreparedVerifyingKey<Bn254>> = Lazy::new(insecure_pvk);

static ZKP_VERIFYING_KEYS: Lazy<HashMap<u32, PreparedVerifyingKey<Bn254>>> = Lazy::new(keys);

//todo: will contain our keys later, key ould be a hash of verification key
fn keys() -> HashMap<u32, PreparedVerifyingKey<Bn254>> {
    let mut h = HashMap::new();
    h.insert(0, insecure_pvk());
    h.insert(1, global_pvk());
    h
}

/// Load a fixed verifying key from zkLogin.vkey output. This is based on a local setup and should not use in production.
fn insecure_pvk() -> PreparedVerifyingKey<Bn254> {
    // Convert the Circom G1/G2/GT to arkworks G1/G2/GT
    let vk_alpha_1 = g1_affine_from_str_projective(&vec![
        Bn254FqElement::from_str(
            "20491192805390485299153009773594534940189261866228447918068658471970481763042",
        )
            .unwrap(),
        Bn254FqElement::from_str(
            "9383485363053290200918347156157836566562967994039712273449902621266178545958",
        )
            .unwrap(),
        Bn254FqElement::from_str("1").unwrap(),
    ])
        .unwrap();
    let vk_beta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElement::from_str(
                "6375614351688725206403948262868962793625744043794305715222011528459656738731",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "4252822878758300859123897981450591353533073413197771768651442665752259397132",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str(
                "10505242626370262277552901082094356697409835680220590971873171140371331206856",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "21847035105528745403288232691147584728191162732299865338377159692350059136679",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str("1").unwrap(),
            Bn254FqElement::from_str("0").unwrap(),
        ],
    ])
        .unwrap();
    let vk_gamma_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElement::from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str("1").unwrap(),
            Bn254FqElement::from_str("0").unwrap(),
        ],
    ])
        .unwrap();
    let vk_delta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElement::from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str("1").unwrap(),
            Bn254FqElement::from_str("0").unwrap(),
        ],
    ])
        .unwrap();

    // Create a vector of G1Affine elements from the IC
    let mut vk_gamma_abc_g1 = Vec::new();
    for e in [
        vec![
            Bn254FqElement::from_str(
                "20701306374481714853949730154526815782802808896228594855451770849676897643964",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "2766989084754673216772682210231588284954002353414778477810174100808747060165",
            )
                .unwrap(),
            Bn254FqElement::from_str("1").unwrap(),
        ],
        vec![
            Bn254FqElement::from_str(
                "501195541410525737371980194958674422793469475773065719916327137354779402600",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "13527631693157515024233848630878973193664410306029731429350155106228769355415",
            )
                .unwrap(),
            Bn254FqElement::from_str("1").unwrap(),
        ],
    ] {
        let g1 = g1_affine_from_str_projective(&e).unwrap();
        vk_gamma_abc_g1.push(g1);
    }

    let vk = VerifyingKey {
        alpha_g1: vk_alpha_1,
        beta_g2: vk_beta_2,
        gamma_g2: vk_gamma_2,
        delta_g2: vk_delta_2,
        gamma_abc_g1: vk_gamma_abc_g1,
    };

    // Convert the verifying key into the prepared form.
    PreparedVerifyingKey::from(vk)
}

/// Load a fixed verifying key from zkLogin.vkey output. This is based on a local setup and should not use in production.
fn global_pvk() -> PreparedVerifyingKey<Bn254> {
    // Convert the Circom G1/G2/GT to arkworks G1/G2/GT
    let vk_alpha_1 = g1_affine_from_str_projective(&vec![
        Bn254FqElement::from_str(
            "21529901943976716921335152104180790524318946701278905588288070441048877064089",
        )
            .unwrap(),
        Bn254FqElement::from_str(
            "7775817982019986089115946956794180159548389285968353014325286374017358010641",
        )
            .unwrap(),
        Bn254FqElement::from_str("1").unwrap(),
    ])
        .unwrap();
    let vk_beta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElement::from_str(
                "6600437987682835329040464538375790690815756241121776438004683031791078085074",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "16207344858883952201936462217289725998755030546200154201671892670464461194903",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str(
                "17943105074568074607580970189766801116106680981075272363121544016828311544390",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "18339640667362802607939727433487930605412455701857832124655129852540230493587",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str("1").unwrap(),
            Bn254FqElement::from_str("0").unwrap(),
        ],
    ])
        .unwrap();
    let vk_gamma_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElement::from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str("1").unwrap(),
            Bn254FqElement::from_str("0").unwrap(),
        ],
    ])
        .unwrap();
    let vk_delta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElement::from_str(
                "19260309516619721648285279557078789954438346514188902804737557357941293711874",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "2480422554560175324649200374556411861037961022026590718777465211464278308900",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str(
                "14489104692423540990601374549557603533921811847080812036788172274404299703364",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "12564378633583954025611992187142343628816140907276948128970903673042690269191",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElement::from_str("1").unwrap(),
            Bn254FqElement::from_str("0").unwrap(),
        ],
    ])
        .unwrap();

    // Create a vector of G1Affine elements from the IC
    let mut vk_gamma_abc_g1 = Vec::new();
    for e in [
        vec![
            Bn254FqElement::from_str(
                "1607694606386445293170795095076356565829000940041894770459712091642365695804",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "18066827569413962196795937356879694709963206118612267170825707780758040578649",
            )
                .unwrap(),
            Bn254FqElement::from_str("1").unwrap(),
        ],
        vec![
            Bn254FqElement::from_str(
                "20653794344898475822834426774542692225449366952113790098812854265588083247207",
            )
                .unwrap(),
            Bn254FqElement::from_str(
                "3296759704176575765409730962060698204792513807296274014163938591826372646699",
            )
                .unwrap(),
            Bn254FqElement::from_str("1").unwrap(),
        ],
    ] {
        let g1 = g1_affine_from_str_projective(&e).unwrap();
        vk_gamma_abc_g1.push(g1);
    }

    let vk = VerifyingKey {
        alpha_g1: vk_alpha_1,
        beta_g2: vk_beta_2,
        gamma_g2: vk_gamma_2,
        delta_g2: vk_delta_2,
        gamma_abc_g1: vk_gamma_abc_g1,
    };

    // Convert the verifying key into the prepared form.
    PreparedVerifyingKey::from(vk)
}
///////////////////////////////////

//this vrgrth16 works only for zklogin stuff for now, later it will be reworked for universal purpose
pub(crate) fn execute_vrgrth16(engine: &mut Engine) -> Status {
    let start = Instant::now();
    engine.load_instruction(crate::executor::types::Instruction::new("VERGRTH16"))?;
    fetch_stack(engine, 3);

    let vk_index = engine.cmd.var(0).as_small_integer().unwrap() as u32;
    println!("from vrgrth16 vk_index: {:?}", vk_index);


    let public_inputs_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let public_inputs_as_bytes = unpack_data_from_cell(public_inputs_slice, engine)?;
    println!("from vrgrth16 value public_inputs_as_bytes: {:?}", public_inputs_as_bytes);

    let proof_slice = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let proof_as_bytes = unpack_data_from_cell(proof_slice, engine)?;
    println!("from vrgrth16 value proof_as_bytes: {:?}", proof_as_bytes);

    let proof = crate::executor::zk_stuff::bn254::Proof::deserialize(&proof_as_bytes)?;
    let public_inputs = FieldElement::deserialize_vector(&public_inputs_as_bytes)?;
    let x: Vec<Fr> = public_inputs.iter().map(|x| x.0).collect();

    let vk = ZKP_VERIFYING_KEYS.get(&vk_index).unwrap();//&GLOBAL_VERIFYING_KEY;
    println!("vk data = {:?}", vk.alpha_g1_beta_g2.to_string());
    //todo: add alternative for elliptic curve (may be we need bls curve also?), read from stack curve id
    let res = Groth16::<Bn254>::verify_with_processed_vk(&vk, &x, &proof.0)
        .map_err(|e| ZkCryptoError::GeneralError(e.to_string()));

    let duration = start.elapsed();

    println!("Time elapsed by vergrth16 is: {:?}", duration);


    let succes = res.is_ok();
    println!("res: {:?}", res);
    let res = if (succes) {
        boolean!(res.unwrap())
    }
    else {
        boolean!(false)
    };
    println!("res: {:?}", res);

    engine.cc.stack.push(res);

    Ok(())
}



//this vrgrth16 works only for zklogin stuff for now, later it will be reworked for universal purpose
// pub(crate) fn execute_vrgrth16(engine: &mut Engine) -> Status {
//     engine.load_instruction(crate::executor::types::Instruction::new("VERGRTH16"))?;
//     crate::executor::engine::storage::fetch_stack(engine, 3)?;
//     let slice1 = engine.cmd.var(0).as_slice()?;
//     let slice2 = engine.cmd.var(1).as_slice()?;
//
//
//
//     let eph_pubkey = slice1.get_bytestring(0);
//     let zk_seed = slice2.as_hex_string();
//
//     let content: JWK = JWK {
//         kty: "RSA".to_string(),
//         e: "AQAB".to_string(),
//         n: "oUriU8GqbRw-avcMn95DGW1cpZR1IoM6L7krfrWvLSSCcSX6Ig117o25Yk7QWBiJpaPV0FbP7Y5-DmThZ3SaF0AXW-3BsKPEXfFfeKVc6vBqk3t5mKlNEowjdvNTSzoOXO5UIHwsXaxiJlbMRalaFEUm-2CKgmXl1ss_yGh1OHkfnBiGsfQUndKoHiZuDzBMGw8Sf67am_Ok-4FShK0NuR3-q33aB_3Z7obC71dejSLWFOEcKUVCaw6DGVuLog3x506h1QQ1r0FXKOQxnmqrRgpoHqGSouuG35oZve1vgCU4vLZ6EAgBAbC0KL35I7_0wUDSMpiAvf7iZxzJVbspkQ".to_string(),
//         alg: "RS256".to_string(),
//     };
//
//
//
//     //third argument handling
//     let s = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
//
//     let data = unpack_data_from_cell(s, engine)?;
//     let value_str = String::from_utf8(data).expect("Found invalid UTF-8");
//
//     println!("from vrgrth16 value: {:?}", value_str);
//     println!("from vrgrth16 eph_pubkey: {:?}", eph_pubkey.clone());
//     println!("from vrgrth16 address_seed: {:?}", zk_seed);
//
//
//     // Get a proof from endpoint and serialize it.
//     // let zk_login_inputs = ZkLoginInputs::from_json(
//     //   "{\"proofPoints\":{\"a\":[\"8247215875293406890829839156897863742504615191361518281091302475904551111016\",\"6872980335748205979379321982220498484242209225765686471076081944034292159666\",\"1\"],\"b\":[[\"21419680064642047510915171723230639588631899775315750803416713283740137406807\",\"21566716915562037737681888858382287035712341650647439119820808127161946325890\"],[\"17867714710686394159919998503724240212517838710399045289784307078087926404555\",\"21812769875502013113255155836896615164559280911997219958031852239645061854221\"],[\"1\",\"0\"]],\"c\":[\"7530826803702928198368421787278524256623871560746240215547076095911132653214\",\"16244547936249959771862454850485726883972969173921727256151991751860694123976\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\"}",
//     // &zk_seed.to_string()).unwrap();/**/
//
//     let zk_login_inputs = ZkLoginInputs::from_json(
//         &*value_str, &*zk_seed.to_string()).unwrap();
//
//     let mut all_jwk = ImHashMap::new();
//     all_jwk.insert(
//         JwkId::new(
//             OIDCProvider::Google.get_config().iss,
//             "6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
//         ),
//         content,
//     );
//
//     /*let res = verify_zk_login(
//         &zk_login_inputs,
//         10,
//         &eph_pubkey,
//         &all_jwk,
//         &ZkLoginEnv::Prod,
//     );*/
//
//     //===============================
//     let (iss, kid) = (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
//     let jwk = all_jwk
//         .get(&JwkId::new(iss.clone(), kid.clone()))
//         .ok_or_else(|| {
//             ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
//         })?;
//
//     let max_epoch = 10;
//
//     // Decode modulus to bytes.
//     let modulus = Base64UrlUnpadded::decode_vec(&jwk.n).map_err(|_| {
//         ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
//     })?;
//
//     let vk = &GLOBAL_VERIFYING_KEY;
//     //let vk = match usage {
//     //ZkLoginEnv::Prod => &GLOBAL_VERIFYING_KEY,
//     //ZkLoginEnv::Test => &INSECURE_VERIFYING_KEY,
//     //};
//
//     let public_inputs = &zk_login_inputs.get_proof().as_arkworks()?;
//     let proof = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch)?];
//
//
//     /*
//     pub fn verify(
//         &self,
//         public_inputs: &[FieldElement],
//         proof: &Proof,
//     ) -> Result<bool, FastCryptoError> {
//         let x: Vec<Fr> = public_inputs.iter().map(|x| x.0).collect();
//         Groth16::<Bn254>::verify_with_processed_vk(&self.into(), &x, &proof.0)
//             .map_err(|e| FastCryptoError::GeneralError(e.to_string()))
//     }
//      */
//
//     //let proof = Proof::deserialize(proof_points_as_bytes)?;
//     //     let public_inputs = FieldElement::deserialize_vector(proof_public_inputs_as_bytes)?;
//
//     let res = Groth16::<Bn254>::verify_with_processed_vk(&vk, proof, &public_inputs)
//         .map_err(|e| ZkCryptoError::GeneralError(e.to_string()));
//
//     println!("res: {:?}", res.is_ok());
//
//     engine.cc.stack.push(boolean!(res.is_ok()));
//
//     // assert!(res.is_ok());
//
//     //assert!(false);
//
//     /* let sprout_vk = {
//              use bellman::groth16::{prepare_verifying_key, VerifyingKey};
//             // let sprout_vk_bytes = include_bytes!("sprout-groth16.vk");
//          let sprout_vk_bytes = include_bytes!("sprout-groth16.vk");
//              let vk = VerifyingKey::<Bls12>::read(&sprout_vk_bytes[..])
//                  .expect("should be able to parse Sprout verification key");
//              println!("vk.alpha_g1 : {:?}", vk.alpha_g1);
//              prepare_verifying_key(&vk)
//      };*/
//
//
//     Ok(())
// }

//this vrgrth16 works only for zklogin stuff for now, later it will be reworked for universal purpose
// pub(crate) fn execute_vrgrth16(engine: &mut Engine) -> Status {
//     engine.load_instruction(crate::executor::types::Instruction::new("VERGRTH16"))?;
//     crate::executor::engine::storage::fetch_stack(engine, 3)?;
//     let slice1 = engine.cmd.var(0).as_slice()?;
//     let slice2 = engine.cmd.var(1).as_slice()?;
//     let slice3 = engine.cmd.var(2).as_slice()?;
//
//     let eph_pubkey = slice1.get_bytestring(0);
//     let zk_seed = slice2.as_hex_string();
//     let mut value_node = slice3.clone().into_cell();
//     let mut value_str = "".to_string();
//
//     println!("slice 3 : {:?}", slice3.to_string());
//
//     let content: JWK = JWK {
//         kty: "RSA".to_string(),
//         e: "AQAB".to_string(),
//         n: "oUriU8GqbRw-avcMn95DGW1cpZR1IoM6L7krfrWvLSSCcSX6Ig117o25Yk7QWBiJpaPV0FbP7Y5-DmThZ3SaF0AXW-3BsKPEXfFfeKVc6vBqk3t5mKlNEowjdvNTSzoOXO5UIHwsXaxiJlbMRalaFEUm-2CKgmXl1ss_yGh1OHkfnBiGsfQUndKoHiZuDzBMGw8Sf67am_Ok-4FShK0NuR3-q33aB_3Z7obC71dejSLWFOEcKUVCaw6DGVuLog3x506h1QQ1r0FXKOQxnmqrRgpoHqGSouuG35oZve1vgCU4vLZ6EAgBAbC0KL35I7_0wUDSMpiAvf7iZxzJVbspkQ".to_string(),
//         alg: "RS256".to_string(),
//     };
//
//     loop {
//         println!("@@@node.depths(): {:?}", value_node.depths());
//         let data = value_node.cell_data().data();
//         let rr = String::from_utf8(data.to_vec()).expect("Found invalid UTF-8");
//         value_str = value_str + &*rr;
//         if (value_node.depth(0) == 0) {
//             break;
//         }
//         value_node = value_node.reference(0).unwrap();
//     }
//
//
//     println!("from vrgrth16 value: {:?}", value_str);
//     println!("from vrgrth16 eph_pubkey: {:?}", eph_pubkey.clone());
//     println!("from vrgrth16 address_seed: {:?}", zk_seed);
//
//
//     // Get a proof from endpoint and serialize it.
//     // let zk_login_inputs = ZkLoginInputs::from_json(
//     //   "{\"proofPoints\":{\"a\":[\"8247215875293406890829839156897863742504615191361518281091302475904551111016\",\"6872980335748205979379321982220498484242209225765686471076081944034292159666\",\"1\"],\"b\":[[\"21419680064642047510915171723230639588631899775315750803416713283740137406807\",\"21566716915562037737681888858382287035712341650647439119820808127161946325890\"],[\"17867714710686394159919998503724240212517838710399045289784307078087926404555\",\"21812769875502013113255155836896615164559280911997219958031852239645061854221\"],[\"1\",\"0\"]],\"c\":[\"7530826803702928198368421787278524256623871560746240215547076095911132653214\",\"16244547936249959771862454850485726883972969173921727256151991751860694123976\",\"1\"]},\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6IjZmNzI1NDEwMWY1NmU0MWNmMzVjOTkyNmRlODRhMmQ1NTJiNGM2ZjEiLCJ0eXAiOiJKV1QifQ\"}",
//     // &zk_seed.to_string()).unwrap();/**/
//
//     let zk_login_inputs = ZkLoginInputs::from_json(
//         &*value_str, &*zk_seed.to_string()).unwrap();
//
//
//     let mut map = ImHashMap::new();
//     map.insert(
//         JwkId::new(
//             OIDCProvider::Google.get_config().iss,
//             "6f7254101f56e41cf35c9926de84a2d552b4c6f1".to_string(),
//         ),
//         content,
//     );
//
//
//     let res = verify_zk_login(
//         &zk_login_inputs,
//         10,
//         &eph_pubkey,
//         &map,
//         &ZkLoginEnv::Prod,
//     );/**/
//
//     println!("res: {:?}", res.is_ok());
//
//     engine.cc.stack.push(boolean!(res.is_ok()));
//
//     // assert!(res.is_ok());
//
//     //assert!(false);
//
//     /* let sprout_vk = {
//              use bellman::groth16::{prepare_verifying_key, VerifyingKey};
//             // let sprout_vk_bytes = include_bytes!("sprout-groth16.vk");
//          let sprout_vk_bytes = include_bytes!("sprout-groth16.vk");
//              let vk = VerifyingKey::<Bls12>::read(&sprout_vk_bytes[..])
//                  .expect("should be able to parse Sprout verification key");
//              println!("vk.alpha_g1 : {:?}", vk.alpha_g1);
//              prepare_verifying_key(&vk)
//      };*/
//
//
//     Ok(())
// }






