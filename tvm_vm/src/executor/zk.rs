use crate::executor::Engine;
use crate::types::Status;

use ark_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use num_bigint::BigUint;
use schemars::JsonSchema;
use derive_more::From;
use serde::Serialize;
use serde::Deserialize;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Error;

use tvm_block::GlobalCapabilities;
use tvm_types::error;
use tvm_types::BuilderData;
use tvm_types::ExceptionCode;
use tvm_types::GasConsumer;
use tvm_types::UInt256;
use tvm_types::SliceData;

use base64ct::{Base64UrlUnpadded,Encoding};

use std::str::FromStr;
use std::time::Instant;

pub use ark_bn254::{Bn254, Fr as Bn254Fr};
use ark_groth16::{Groth16, PreparedVerifyingKey, Proof, VerifyingKey};
use ark_snark::SNARK;


use crate::error::TvmError;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::stack::integer::serialization::UnsignedIntegerBigEndianEncoding;
use crate::stack::integer::IntegerData;
use crate::stack::StackItem;
use crate::types::Exception;
use crate::utils::{bytes_to_string, unpack_string_from_cell};
use crate::utils::pack_data_to_cell;
use crate::utils::unpack_data_from_cell;
//use once_cell::sync::Lazy;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use serde_json::Value;
use thiserror::Error;
use crate::executor::zk_stuff::bn254::poseidon::poseidon_zk_login;
use crate::executor::zk_stuff::curve_utils::Bn254FrElement;
use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::utils::split_to_two_frs;
use crate::executor::zk_stuff::zk_login::{Claim, hash_ascii_str_to_field, hash_to_field, MAX_HEADER_LEN, MAX_ISS_LEN_B64, PACK_WIDTH};
use crate::stack::StackItem::Cell;


pub type ZkCryptoResult<T> = Result<T, ZkCryptoError>;

/// Size of scalars in the BN254 construction.
pub const SCALAR_SIZE: usize = 32;

#[derive(Debug, From)]
pub struct FieldElementWrapper(pub(crate) ark_bn254::Fr);

impl FieldElementWrapper {
    /// Deserialize 32 bytes into a BN254 field element using little-endian format.
    pub(crate) fn deserialize(bytes: &[u8]) -> ZkCryptoResult<FieldElementWrapper> {
        if bytes.len() != SCALAR_SIZE {
            return Err(ZkCryptoError::InputLengthWrong(bytes.len()));
        }
        Fr::deserialize_compressed(bytes)
            .map_err(|_| ZkCryptoError::InvalidInput)
            .map(FieldElementWrapper)
    }

    /// Deserialize a vector of bytes into a vector of BN254 field elements, assuming that each element
    /// is serialized as a chunk of 32 bytes. See also [`FieldElement::deserialize`].
    pub(crate) fn deserialize_vector(
        field_element_bytes: &[u8],
    ) -> ZkCryptoResult<Vec<FieldElementWrapper>> {
        if field_element_bytes.len() % SCALAR_SIZE != 0 {
            return Err(ZkCryptoError::InputLengthWrong(field_element_bytes.len()));
        }
        let mut public_inputs = Vec::new();
        for chunk in field_element_bytes.chunks(SCALAR_SIZE) {
            public_inputs.push(FieldElementWrapper::deserialize(chunk)?);
        }
        Ok(public_inputs)
    }
}


pub type CircomG1 = Vec<Bn254FqElementWrapper>;

/// A G2 point in BN254 serialized as a vector of three vectors each being a vector of two strings
/// which are the canonical decimal representation of the coefficients of the projective coordinates
/// in Fq2.
pub type CircomG2 = Vec<Vec<Bn254FqElementWrapper>>;

/// A struct that stores a Bn254 Fq field element as 32 bytes.
#[derive(Debug, Clone, JsonSchema, Eq, PartialEq)]
pub struct Bn254FqElementWrapper(#[schemars(with = "String")] [u8; 32]);

impl std::str::FromStr for Bn254FqElementWrapper {
    type Err = ZkCryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let big_int = Fq::from_str(s).map_err(|_| ZkCryptoError::InvalidInput)?;
        let be_bytes = big_int.into_bigint().to_bytes_be();
        be_bytes
            .try_into()
            .map_err(|_| ZkCryptoError::InvalidInput)
            .map(Bn254FqElementWrapper)
    }
}

impl std::fmt::Display for Bn254FqElementWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let big_int = BigUint::from_bytes_be(&self.0);
        let radix10 = big_int.to_string();
        f.write_str(&radix10)
    }
}

// Bn254FqElement's serialized format is as a radix10 encoded string
impl Serialize for Bn254FqElementWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bn254FqElementWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let s = std::borrow::Cow::<'de, str>::deserialize(deserializer)?;
        std::str::FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// A struct that stores a Bn254 Fr field element as 32 bytes.
#[derive(Debug, Clone, JsonSchema, Eq, PartialEq)]
pub struct Bn254FrElementWrapper(#[schemars(with = "String")] [u8; 32]);

impl Bn254FrElementWrapper {
    /// Returns the unpadded version of the field element. This returns with leading zeros removed.
    pub fn unpadded(&self) -> &[u8] {
        let mut buf = self.0.as_slice();

        while !buf.is_empty() && buf[0] == 0 {
            buf = &buf[1..];
        }

        // If the value is '0' then just return a slice of length 1 of the final byte
        if buf.is_empty() {
            &self.0[31..]
        } else {
            buf
        }
    }

    /// Returns the padded version of the field element. This returns with leading zeros preserved to 32 bytes.
    pub fn padded(&self) -> &[u8] {
        &self.0
    }
}
impl std::str::FromStr for Bn254FrElementWrapper {
    type Err = ZkCryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let big_int = Fr::from_str(s).map_err(|_| ZkCryptoError::InvalidInput)?;
        let be_bytes = big_int.into_bigint().to_bytes_be();
        be_bytes
            .try_into()
            .map_err(|_| ZkCryptoError::InvalidInput)
            .map(Bn254FrElementWrapper)
    }
}

impl std::fmt::Display for Bn254FrElementWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let big_int = BigUint::from_bytes_be(&self.0);
        let radix10 = big_int.to_string();
        f.write_str(&radix10)
    }
}

// Bn254FrElement's serialized format is as a radix10 encoded string
impl Serialize for Bn254FrElementWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bn254FrElementWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let s = std::borrow::Cow::<'de, str>::deserialize(deserializer)?;
        std::str::FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Convert Bn254FqElement type to arkworks' Fq.
impl From<&Bn254FqElementWrapper> for Fq {
    fn from(f: &Bn254FqElementWrapper) -> Self {
        Fq::from_be_bytes_mod_order(&f.0)
    }
}

/// Convert Bn254FrElement type to arkworks' Fr.
impl From<&Bn254FrElementWrapper> for Fr {
    fn from(f: &Bn254FrElementWrapper) -> Self {
        Fr::from_be_bytes_mod_order(&f.0)
    }
}

/// Deserialize a G1 projective point in BN254 serialized as a vector of three strings into an affine
/// G1 point in arkworks format. Return an error if the input is not a vector of three strings or if
/// any of the strings cannot be parsed as a field element.
pub(crate) fn g1_affine_from_str_projective(s: &CircomG1) -> Result<G1Affine, ZkCryptoError> {
    if s.len() != 3 {
        return Err(ZkCryptoError::InvalidInput);
    }

    let g1: G1Affine =
        G1Projective::new_unchecked((&s[0]).into(), (&s[1]).into(), (&s[2]).into()).into();

    if !g1.is_on_curve() || !g1.is_in_correct_subgroup_assuming_on_curve() {
        return Err(ZkCryptoError::InvalidInput);
    }

    Ok(g1)
}

/// Deserialize a G2 projective point from the BN254 construction serialized as a vector of three
/// vectors each being a vector of two strings into an affine G2 point in arkworks format. Return an
/// error if the input is not a vector of the right format or if any of the strings cannot be parsed
/// as a field element.
pub(crate) fn g2_affine_from_str_projective(s: &CircomG2) -> Result<G2Affine, ZkCryptoError> {
    if s.len() != 3 || s[0].len() != 2 || s[1].len() != 2 || s[2].len() != 2 {
        return Err(ZkCryptoError::InvalidInput);
    }

    let g2: G2Affine = G2Projective::new_unchecked(
        Fq2::new((&s[0][0]).into(), (&s[0][1]).into()),
        Fq2::new((&s[1][0]).into(), (&s[1][1]).into()),
        Fq2::new((&s[2][0]).into(), (&s[2][1]).into()),
    )
        .into();

    if !g2.is_on_curve() || !g2.is_in_correct_subgroup_assuming_on_curve() {
        return Err(ZkCryptoError::InvalidInput);
    }

    Ok(g2)
}


/// A Groth16 proof in the BN254 construction. Thin wrapper around `ark_groth16::Proof::<ark_bn254::Bn254>`.
#[derive(Debug, From)]
pub struct ProofWrapper(pub(crate) ark_groth16::Proof<ark_bn254::Bn254>);

impl ProofWrapper {
    /// Deserialize a serialized Groth16 proof using arkworks' canonical serialisation format: https://docs.rs/ark-serialize/latest/ark_serialize/.
    pub fn deserialize(proof_points_as_bytes: &[u8]) -> ZkCryptoResult<Self> {
        ark_groth16::Proof::<Bn254>::deserialize_compressed(proof_points_as_bytes)
            .map_err(|_| ZkCryptoError::InvalidInput)
            .map(ProofWrapper)
    }
}


/**
    Here there are third party zk login Groth16 verification keys taken  for now for tests
    todo: will be replaced by our keys later
    todo: move all key data to json config file (?), use hash as id
**/

/////////////////////////////////////////////////////////////////////////////////////////////////////////
//static GLOBAL_VERIFYING_KEY: Lazy<PreparedVerifyingKey<Bn254>> = Lazy::new(global_pvk);

/// Corresponding to proofs generated from prover-dev. Used in devnet/testnet.
//static INSECURE_VERIFYING_KEY: Lazy<PreparedVerifyingKey<Bn254>> = Lazy::new(insecure_pvk);

//static ZKP_VERIFYING_KEYS: Lazy<HashMap<u32, PreparedVerifyingKey<Bn254>>> = Lazy::new(keys);

//todo: will contain our keys later, key ould be a hash of verification key
/*fn keys() -> HashMap<u32, PreparedVerifyingKey<Bn254>> {
    let mut h = HashMap::new();
    h.insert(0, insecure_pvk());
    h.insert(1, global_pvk());
    h
}*/

/// Load a fixed verifying key from zkLogin.vkey output. This is based on a local setup and should not use in production.
fn insecure_pvk() -> PreparedVerifyingKey<Bn254> {
    // Convert the Circom G1/G2/GT to arkworks G1/G2/GT
    let vk_alpha_1 = g1_affine_from_str_projective(&vec![
        Bn254FqElementWrapper::from_str(
            "20491192805390485299153009773594534940189261866228447918068658471970481763042",
        )
            .unwrap(),
        Bn254FqElementWrapper::from_str(
            "9383485363053290200918347156157836566562967994039712273449902621266178545958",
        )
            .unwrap(),
        Bn254FqElementWrapper::from_str("1").unwrap(),
    ])
        .unwrap();
    let vk_beta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElementWrapper::from_str(
                "6375614351688725206403948262868962793625744043794305715222011528459656738731",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "4252822878758300859123897981450591353533073413197771768651442665752259397132",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "10505242626370262277552901082094356697409835680220590971873171140371331206856",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "21847035105528745403288232691147584728191162732299865338377159692350059136679",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str("1").unwrap(),
            Bn254FqElementWrapper::from_str("0").unwrap(),
        ],
    ])
        .unwrap();
    let vk_gamma_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElementWrapper::from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str("1").unwrap(),
            Bn254FqElementWrapper::from_str("0").unwrap(),
        ],
    ])
        .unwrap();
    let vk_delta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElementWrapper::from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str("1").unwrap(),
            Bn254FqElementWrapper::from_str("0").unwrap(),
        ],
    ])
        .unwrap();

    // Create a vector of G1Affine elements from the IC
    let mut vk_gamma_abc_g1 = Vec::new();
    for e in [
        vec![
            Bn254FqElementWrapper::from_str(
                "20701306374481714853949730154526815782802808896228594855451770849676897643964",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "2766989084754673216772682210231588284954002353414778477810174100808747060165",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str("1").unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "501195541410525737371980194958674422793469475773065719916327137354779402600",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "13527631693157515024233848630878973193664410306029731429350155106228769355415",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str("1").unwrap(),
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
        Bn254FqElementWrapper::from_str(
            "21529901943976716921335152104180790524318946701278905588288070441048877064089",
        )
            .unwrap(),
        Bn254FqElementWrapper::from_str(
            "7775817982019986089115946956794180159548389285968353014325286374017358010641",
        )
            .unwrap(),
        Bn254FqElementWrapper::from_str("1").unwrap(),
    ])
        .unwrap();
    let vk_beta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElementWrapper::from_str(
                "6600437987682835329040464538375790690815756241121776438004683031791078085074",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "16207344858883952201936462217289725998755030546200154201671892670464461194903",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "17943105074568074607580970189766801116106680981075272363121544016828311544390",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "18339640667362802607939727433487930605412455701857832124655129852540230493587",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str("1").unwrap(),
            Bn254FqElementWrapper::from_str("0").unwrap(),
        ],
    ])
        .unwrap();
    let vk_gamma_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElementWrapper::from_str(
                "10857046999023057135944570762232829481370756359578518086990519993285655852781",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "11559732032986387107991004021392285783925812861821192530917403151452391805634",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "8495653923123431417604973247489272438418190587263600148770280649306958101930",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "4082367875863433681332203403145435568316851327593401208105741076214120093531",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str("1").unwrap(),
            Bn254FqElementWrapper::from_str("0").unwrap(),
        ],
    ])
        .unwrap();
    let vk_delta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElementWrapper::from_str(
                "19260309516619721648285279557078789954438346514188902804737557357941293711874",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "2480422554560175324649200374556411861037961022026590718777465211464278308900",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "14489104692423540990601374549557603533921811847080812036788172274404299703364",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "12564378633583954025611992187142343628816140907276948128970903673042690269191",
            )
                .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str("1").unwrap(),
            Bn254FqElementWrapper::from_str("0").unwrap(),
        ],
    ])
        .unwrap();

    // Create a vector of G1Affine elements from the IC
    let mut vk_gamma_abc_g1 = Vec::new();
    for e in [
        vec![
            Bn254FqElementWrapper::from_str(
                "1607694606386445293170795095076356565829000940041894770459712091642365695804",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "18066827569413962196795937356879694709963206118612267170825707780758040578649",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str("1").unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "20653794344898475822834426774542692225449366952113790098812854265588083247207",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str(
                "3296759704176575765409730962060698204792513807296274014163938591826372646699",
            )
                .unwrap(),
            Bn254FqElementWrapper::from_str("1").unwrap(),
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

pub(crate) fn execute_vergrth16(engine: &mut Engine) -> Status {
    let start = Instant::now();
    engine.load_instruction(crate::executor::types::Instruction::new("VERGRTH16"))?;
    fetch_stack(engine, 3);

    let vk_index = engine.cmd.var(0).as_small_integer().unwrap() as u32;
    println!("from vergrth16 vk_index: {:?}", vk_index);


    let public_inputs_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let public_inputs_as_bytes = unpack_data_from_cell(public_inputs_slice, engine)?;
    println!("from vergrth16 value public_inputs_as_bytes: {:?}", public_inputs_as_bytes);

    let proof_slice = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let proof_as_bytes = unpack_data_from_cell(proof_slice, engine)?;
    println!("from vergrth16 value proof_as_bytes: {:?}", proof_as_bytes);

    let proof = ProofWrapper::deserialize(&proof_as_bytes)?;
    let public_inputs = FieldElementWrapper::deserialize_vector(&public_inputs_as_bytes)?;
    let x: Vec<Fr> = public_inputs.iter().map(|x| x.0).collect();

    let vk = if (vk_index == 0) {
        insecure_pvk()
    } else {
        global_pvk()
    };

        //ZKP_VERIFYING_KEYS.get(&vk_index).unwrap();//&GLOBAL_VERIFYING_KEY;
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

pub(crate) fn execute_poseidon_zk_login(engine: &mut Engine) -> Status {
    engine.load_instruction(crate::executor::types::Instruction::new("POSEIDON"))?;
    fetch_stack(engine, 4);

   /**/ let zkaddr_slice = SliceData::load_cell_ref(engine.cmd.var(0).as_cell()?)?;
    let zkaddr = unpack_string_from_cell(zkaddr_slice, engine)?;
    println!("from poseidon value zkaddr: {:?}", zkaddr);

    let epk_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let epk_as_bytes = unpack_data_from_cell(epk_slice, engine)?;
    println!("from poseidon value epk_as_bytes: {:?}", hex::encode(epk_as_bytes.clone()));

    let header_and_iss_base64_slice = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let header_and_iss_base64 = unpack_string_from_cell(header_and_iss_base64_slice, engine)?;
    println!("from poseidon value header_and_iss_base64: {:?}", header_and_iss_base64);

    let modulus_slice = SliceData::load_cell_ref(engine.cmd.var(3).as_cell()?)?;
    let modulus = unpack_data_from_cell(modulus_slice, engine)?;
    println!("from poseidon value modulus: {:?}",modulus);/**/

    let max_epoch = 10; //todo: read from stack later

    let public_inputs = calculate_poseidon_hash(
        &*zkaddr,
        &*header_and_iss_base64,
        &epk_as_bytes,
        &modulus,
        max_epoch
    ).unwrap();


    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
    println!("from poseidon public_inputs_as_bytes : {:?}", public_inputs_as_bytes.clone());
    //println!("from poseidon public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0).unwrap();

    engine.cc.stack.push(Cell(public_inputs_cell));/**/

    Ok(())

}


pub fn calculate_poseidon_hash(
        address_seed: &str,
        header_and_iss_base64: &str,
        eph_pk_bytes: &[u8],
        modulus: &[u8],
        max_epoch: u64,

    ) -> Result<Bn254Fr, ZkCryptoError>/**/ {
        /*if header_base64.len() > MAX_HEADER_LEN as usize {
            return Err(ZkCryptoError::GeneralError("Header too long".to_string()));
        }*/

        let address_seed = Bn254FrElement::from_str(address_seed).unwrap();
        let addr_seed = (&address_seed).into();

        let (first, second) = split_to_two_frs(eph_pk_bytes).unwrap();

        let max_epoch_f = (&Bn254FrElement::from_str(&max_epoch.to_string()).unwrap()).into();

        let v: Value = serde_json::from_str(header_and_iss_base64).unwrap();

        let header_base64 = v["headerBase64"].as_str().unwrap();
        println!("header_base64 {}", header_base64);

        let issBase64Details = v["issBase64Details"].as_object().unwrap();
        println!("issBase64Details {:?}", issBase64Details);

        let index_mod_4 = issBase64Details["indexMod4"].as_i64().unwrap().to_string();

        println!("index_mod_4 {:?}", index_mod_4);

        let iss_base64_details_value = issBase64Details["value"].as_str().unwrap();

        println!("iss_base64_details_value {:?}", iss_base64_details_value);



        let index_mod_4_f =
            (&Bn254FrElement::from_str(&index_mod_4).unwrap()).into();

        let iss_base64_f =
            hash_ascii_str_to_field(&iss_base64_details_value, MAX_ISS_LEN_B64).unwrap();
        let header_f = hash_ascii_str_to_field(&header_base64, MAX_HEADER_LEN).unwrap();
        let modulus_f = hash_to_field(&[BigUint::from_bytes_be(modulus)], 2048, PACK_WIDTH).unwrap();

        poseidon_zk_login(vec![
            first,
            second,
            addr_seed,
            max_epoch_f,
            iss_base64_f,
            index_mod_4_f,
            header_f,
            modulus_f,
        ])
}





