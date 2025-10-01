use std::str::FromStr;

pub use ark_bn254::Bn254;
use ark_bn254::Fq;
use ark_bn254::Fq2;
use ark_bn254::Fr;
pub use ark_bn254::Fr as Bn254Fr;
use ark_bn254::G1Affine;
use ark_bn254::G1Projective;
use ark_bn254::G2Affine;
use ark_bn254::G2Projective;
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use ark_groth16::Groth16;
use ark_groth16::PreparedVerifyingKey;
use ark_groth16::VerifyingKey;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use ark_snark::SNARK;
use derive_more::From;
use num_bigint::BigUint;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tvm_types::ExceptionCode;
use tvm_types::SliceData;
use tvm_types::error;

use crate::error::TvmError;
use crate::executor::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::gas::gas_state::Gas;
use crate::executor::zk_stuff::bn254::poseidon::poseidon_zk_login;
use crate::executor::zk_stuff::curve_utils::Bn254FrElement;
use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::utils::split_to_two_frs;
use crate::executor::zk_stuff::zk_login::MAX_HEADER_LEN;
use crate::executor::zk_stuff::zk_login::MAX_ISS_LEN_B64;
use crate::executor::zk_stuff::zk_login::PACK_WIDTH;
use crate::executor::zk_stuff::zk_login::hash_ascii_str_to_field;
use crate::executor::zk_stuff::zk_login::hash_to_field;
use crate::stack::StackItem;
use crate::stack::StackItem::Cell;
use crate::stack::integer::IntegerData;
use crate::stack::integer::serialization::UnsignedIntegerBigEndianEncoding;
use crate::types::Exception;
use crate::types::Status;
use crate::utils::pack_data_to_cell;
use crate::utils::unpack_data_from_cell;
use crate::utils::unpack_string_from_cell;

pub const POSEIDON_ZK_LOGIN_GAS_PRICE: i64 = 356;
pub const VERGRTH16_GAS_PRICE: i64 = 2380;

pub type ZkCryptoResult<T> = Result<T, ZkCryptoError>;

/// Size of scalars in the BN254 construction.
pub const SCALAR_SIZE: usize = 32;

const PUBLIC_KEY_BITS: usize = PUBLIC_KEY_BYTES * 8;
const PUBLIC_KEY_BYTES: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;

#[derive(Debug, From)]
pub struct FieldElementWrapper(pub(crate) ark_bn254::Fr);

impl FieldElementWrapper {
    /// Deserialize 32 bytes into a BN254 field element using little-endian
    /// format.
    pub(crate) fn deserialize(bytes: &[u8]) -> ZkCryptoResult<FieldElementWrapper> {
        if bytes.len() != SCALAR_SIZE {
            return Err(ZkCryptoError::InputLengthWrong(bytes.len()));
        }
        Fr::deserialize_compressed(bytes)
            .map_err(|_| ZkCryptoError::InvalidInput)
            .map(FieldElementWrapper)
    }

    /// Deserialize a vector of bytes into a vector of BN254 field elements,
    /// assuming that each element is serialized as a chunk of 32 bytes. See
    /// also [`FieldElement::deserialize`].
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

/// A G2 point in BN254 serialized as a vector of three vectors each being a
/// vector of two strings which are the canonical decimal representation of the
/// coefficients of the projective coordinates in Fq2.
pub type CircomG2 = Vec<Vec<Bn254FqElementWrapper>>;

/// A struct that stores a Bn254 Fq field element as 32 bytes.
#[derive(Debug, Clone, JsonSchema, Eq, PartialEq)]
pub struct Bn254FqElementWrapper(#[schemars(with = "String")] [u8; 32]);

impl std::str::FromStr for Bn254FqElementWrapper {
    type Err = ZkCryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let big_int = Fq::from_str(s).map_err(|_| ZkCryptoError::InvalidInput)?;
        let be_bytes = big_int.into_bigint().to_bytes_be();
        be_bytes.try_into().map_err(|_| ZkCryptoError::InvalidInput).map(Bn254FqElementWrapper)
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
    /// Returns the unpadded version of the field element. This returns with
    /// leading zeros removed.
    pub fn unpadded(&self) -> &[u8] {
        let mut buf = self.0.as_slice();

        while !buf.is_empty() && buf[0] == 0 {
            buf = &buf[1..];
        }

        // If the value is '0' then just return a slice of length 1 of the final byte
        if buf.is_empty() { &self.0[31..] } else { buf }
    }

    /// Returns the padded version of the field element. This returns with
    /// leading zeros preserved to 32 bytes.
    pub fn padded(&self) -> &[u8] {
        &self.0
    }
}
impl std::str::FromStr for Bn254FrElementWrapper {
    type Err = ZkCryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let big_int = Fr::from_str(s).map_err(|_| ZkCryptoError::InvalidInput)?;
        let be_bytes = big_int.into_bigint().to_bytes_be();
        be_bytes.try_into().map_err(|_| ZkCryptoError::InvalidInput).map(Bn254FrElementWrapper)
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

/// Deserialize a G1 projective point in BN254 serialized as a vector of three
/// strings into an affine G1 point in arkworks format. Return an error if the
/// input is not a vector of three strings or if any of the strings cannot be
/// parsed as a field element.
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

/// Deserialize a G2 projective point from the BN254 construction serialized as
/// a vector of three vectors each being a vector of two strings into an affine
/// G2 point in arkworks format. Return an error if the input is not a vector of
/// the right format or if any of the strings cannot be parsed as a field
/// element.
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

/// A Groth16 proof in the BN254 construction. Thin wrapper around
/// `ark_groth16::Proof::<ark_bn254::Bn254>`.
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

fn my_test_pvk_1() -> PreparedVerifyingKey<Bn254> {
    // Convert the Circom G1/G2/GT to arkworks G1/G2/GT
    let vk_alpha_1 = g1_affine_from_str_projective(&vec![
        Bn254FqElementWrapper::from_str(
            "16083174311393072332126484955039141051820368387551336007741432494536231879877",
        )
        .unwrap(),
        Bn254FqElementWrapper::from_str(
            "11995344593741129498206341608147577676708407993917230939676252851997423446210",
        )
        .unwrap(),
        Bn254FqElementWrapper::from_str("1").unwrap(),
    ])
    .unwrap();

    let vk_beta_2 = g2_affine_from_str_projective(&vec![
        vec![
            Bn254FqElementWrapper::from_str(
                "7017589137241388812217334676878160715759313595646525247042913539379033763831",
            )
            .unwrap(),
            Bn254FqElementWrapper::from_str(
                "9588720105182136304988839277158105754318461657916765428451866781594135026063",
            )
            .unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "2484424409632768920146683103978991861859052149379216050446911519906662584090",
            )
            .unwrap(),
            Bn254FqElementWrapper::from_str(
                "3390288516800701266276631045627865236740814264026178914799455551851945389106",
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
                "11760611693671517707466601638901224388668992590928868758649168369215563295744",
            )
            .unwrap(),
            Bn254FqElementWrapper::from_str(
                "15842561259007247784907604255150260908812200067246900457940460682994649597353",
            )
            .unwrap(),
            Bn254FqElementWrapper::from_str("1").unwrap(),
        ],
        vec![
            Bn254FqElementWrapper::from_str(
                "9960247968913608540350443520882802417817484595360267448450266543686043480996",
            )
            .unwrap(),
            Bn254FqElementWrapper::from_str(
                "11040490439713280236989540698814598402024610465375008410116396264618122562865",
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

pub(crate) fn execute_vergrth16(engine: &mut Engine) -> Status {
    engine.load_instruction(crate::executor::types::Instruction::new("VERGRTH16"))?;
    engine.try_use_gas(Gas::vergrth16_price())?;
    fetch_stack(engine, 3)?;

    let vk_index = engine.cmd.var(0).as_small_integer()? as u32;

    let public_inputs_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let public_inputs_as_bytes = unpack_data_from_cell(public_inputs_slice, engine)?;

    let proof_slice = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let proof_as_bytes = unpack_data_from_cell(proof_slice, engine)?;

    let public_inputs = match FieldElementWrapper::deserialize_vector(&public_inputs_as_bytes) {
        Ok(public_inputs) => public_inputs,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect public inputs {}", err);
        }
    };

    let proof = match ProofWrapper::deserialize(&proof_as_bytes) {
        Ok(proof) => proof,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect proof {}", err);
        }
    };

    let x: Vec<Fr> = public_inputs.iter().map(|x| x.0).collect();

    /*let vk = if vk_index == 0 {
        insecure_pvk()
    } else if vk_index == 1 {
        global_pvk()
    } else {
        my_test_pvk_1()
    };*/

    let vk = global_pvk();

    // todo: add alternative for elliptic curve (BLS), read from stack curve id

    let res = Groth16::<Bn254>::verify_with_processed_vk(&vk, &x, &proof.0)
        .map_err(|e| ZkCryptoError::GeneralError(e.to_string()));

    let succes = res.is_ok();
    let res = if succes { boolean!(res?) } else { boolean!(false) };
    engine.cc.stack.push(res);

    Ok(())
}

fn pop(barry: &[u8]) -> &[u8; 8] {
    barry.try_into().expect("slice with incorrect length")
}

pub(crate) fn execute_poseidon_zk_login(engine: &mut Engine) -> Status {
    engine.load_instruction(crate::executor::types::Instruction::new("POSEIDON"))?;
    engine.try_use_gas(Gas::poseidon_zk_login_price())?;
    fetch_stack(engine, 7)?;

    let zkaddr_slice = SliceData::load_cell_ref(engine.cmd.var(0).as_cell()?)?;
    let zkaddr = unpack_string_from_cell(zkaddr_slice, engine)?;

    let header_base_64_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
    let header_base_64 = unpack_string_from_cell(header_base_64_slice, engine)?;

    let iss_base_64_slice = SliceData::load_cell_ref(engine.cmd.var(2).as_cell()?)?;
    let iss_base_64 = unpack_string_from_cell(iss_base_64_slice, engine)?;

    let modulus_slice = SliceData::load_cell_ref(engine.cmd.var(3).as_cell()?)?;
    let modulus = unpack_data_from_cell(modulus_slice, engine)?;

    let eph_pub_key = engine
        .cmd
        .var(4)
        .as_integer()?
        .as_builder::<UnsignedIntegerBigEndianEncoding>(PUBLIC_KEY_BITS)?;

    let eph_pub_key_bytes = eph_pub_key.data();

    let max_epoch_ =
        engine.cmd.var(5).as_integer()?.as_builder::<UnsignedIntegerBigEndianEncoding>(64)?;

    let index_mod_4 = engine.cmd.var(6).as_integer()?.into(0..=255)?.to_string();

    let max_epoch_bytes = pop(max_epoch_.data());

    let max_epoch = u64::from_be_bytes(*max_epoch_bytes);

    /////////

    let address_seed = match Bn254FrElement::from_str(&*zkaddr) {
        Ok(address_seed) => address_seed,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect address seed {}", err);
        }
    };
    let addr_seed = (&address_seed).into();

    let (first, second) = match split_to_two_frs(&eph_pub_key_bytes) {
        Ok((first, second)) => (first, second),
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect ephemeral public key {}", err);
        }
    };

    let max_epoch_f = match Bn254FrElement::from_str(&max_epoch.to_string()) {
        Ok(max_epoch_f) => max_epoch_f,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect max_epoch {}", err);
        }
    };
    let max_epoch_f = (&max_epoch_f).into();

    let index_mod_4_f = match Bn254FrElement::from_str(&index_mod_4) {
        Ok(index_mod_4_f) => index_mod_4_f,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect index_mod_4 {}", err);
        }
    };
    let index_mod_4_f = (&index_mod_4_f).into();

    let iss_base64_f = match hash_ascii_str_to_field(&iss_base_64, MAX_ISS_LEN_B64) {
        Ok(iss_base64_f) => iss_base64_f,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect iss_base64 {}", err);
        }
    };

    let header_f = match hash_ascii_str_to_field(&header_base_64, MAX_HEADER_LEN) {
        Ok(header_f) => header_f,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect header {}", err);
        }
    };

    let modulus_f = match hash_to_field(&[BigUint::from_bytes_be(&modulus)], 2048, PACK_WIDTH) {
        Ok(modulus_f) => modulus_f,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect modulus {}", err);
        }
    };

    let public_inputs = match poseidon_zk_login(vec![
        first,
        second,
        addr_seed,
        max_epoch_f,
        iss_base64_f,
        index_mod_4_f,
        header_f,
        modulus_f,
    ]) {
        Ok(public_inputs) => public_inputs,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "poseidon computation issue {}", err);
        }
    };

    let mut public_inputs_as_bytes = vec![];
    public_inputs.serialize_compressed(&mut public_inputs_as_bytes)?;

    let public_inputs_cell = pack_data_to_cell(&public_inputs_as_bytes, &mut 0)?;
    engine.cc.stack.push(Cell(public_inputs_cell));

    Ok(())
}
