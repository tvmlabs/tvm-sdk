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

pub const VK_LEN: usize = 296;

pub const INSECURE_VK_SERIALIZED: [u8; VK_LEN] = [226, 242, 109, 190, 162, 153, 245, 34, 59, 100, 108, 177, 251, 51, 234, 219, 5, 157, 148, 7, 85, 157, 116, 65, 223, 217, 2, 227, 167, 154, 77, 45, 171, 183, 61, 193, 127, 188, 19, 2, 30, 36, 113, 224, 192, 139, 214, 125, 132, 1, 245, 43, 115, 214, 208, 116, 131, 121, 76, 173, 71, 120, 24, 14, 12, 6, 243, 59, 188, 76, 121, 169, 202, 222, 242, 83, 166, 128, 132, 211, 130, 241, 119, 136, 248, 133, 201, 175, 209, 118, 247, 203, 47, 3, 103, 137, 237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94, 102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151, 18, 231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146, 147, 147, 142, 25, 237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94, 102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151, 18, 231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146, 147, 147, 142, 25, 2, 0, 0, 0, 0, 0, 0, 0, 188, 109, 65, 14, 59, 194, 107, 53, 43, 136, 71, 184, 217, 252, 205, 146, 137, 248, 166, 82, 243, 30, 4, 205, 71, 203, 158, 80, 49, 134, 196, 45, 104, 33, 147, 62, 5, 214, 248, 224, 214, 11, 163, 236, 65, 113, 21, 154, 124, 161, 149, 238, 58, 248, 236, 80, 209, 30, 86, 217, 167, 170, 27, 129];

pub const GLOBAL_VK_SERIALIZED: [u8; VK_LEN] = [153, 95, 236, 192, 209, 69, 53, 46, 44, 142, 22, 242, 149, 93, 223, 124, 146, 8, 25, 154, 53, 214, 241, 163, 103, 180, 152, 36, 31, 126, 153, 47, 210, 17, 210, 0, 61, 234, 165, 49, 25, 21, 84, 112, 182, 96, 83, 184, 146, 79, 162, 25, 16, 177, 167, 181, 115, 149, 186, 207, 43, 183, 151, 14, 151, 70, 109, 244, 206, 238, 26, 171, 24, 142, 154, 116, 90, 195, 28, 13, 228, 122, 128, 98, 173, 245, 60, 45, 111, 162, 108, 94, 56, 8, 213, 163, 237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94, 102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151, 18, 231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146, 147, 147, 142, 25, 2, 2, 242, 114, 244, 198, 146, 132, 120, 207, 247, 34, 5, 178, 202, 159, 101, 165, 84, 196, 10, 110, 110, 69, 231, 94, 93, 59, 233, 242, 148, 42, 36, 120, 3, 59, 238, 165, 13, 209, 130, 26, 71, 16, 75, 84, 248, 56, 180, 54, 248, 216, 82, 139, 80, 199, 60, 205, 239, 244, 145, 222, 123, 133, 2, 0, 0, 0, 0, 0, 0, 0, 60, 87, 71, 146, 228, 115, 61, 79, 87, 24, 48, 121, 168, 233, 18, 143, 162, 218, 225, 6, 231, 94, 56, 204, 94, 100, 111, 57, 67, 236, 141, 131, 103, 200, 250, 33, 200, 246, 96, 233, 240, 248, 141, 250, 199, 97, 59, 203, 165, 97, 216, 37, 33, 45, 71, 252, 147, 94, 249, 48, 34, 162, 169, 45];

pub const MY_TEST_VK_1_SERIALIZED: [u8; VK_LEN] = [197, 4, 58, 66, 35, 79, 42, 146, 231, 7, 232, 227, 177, 50, 124, 221, 242, 199, 161, 192, 52, 46, 131, 80, 165, 85, 178, 200, 10, 193, 142, 163, 247, 111, 107, 63, 62, 194, 77, 89, 129, 26, 60, 23, 189, 135, 127, 50, 63, 34, 75, 204, 168, 248, 186, 41, 23, 107, 149, 232, 143, 208, 131, 15, 143, 217, 239, 13, 173, 52, 55, 113, 156, 224, 20, 141, 200, 144, 146, 56, 121, 35, 101, 142, 106, 162, 17, 23, 202, 208, 226, 233, 243, 5, 51, 21, 237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94, 102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151, 18, 231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146, 147, 147, 142, 25, 237, 246, 146, 217, 92, 189, 222, 70, 221, 218, 94, 247, 212, 34, 67, 103, 121, 68, 92, 94, 102, 0, 106, 66, 118, 30, 31, 18, 239, 222, 0, 24, 194, 18, 243, 174, 183, 133, 228, 151, 18, 231, 169, 53, 51, 73, 170, 241, 37, 93, 251, 49, 183, 191, 96, 114, 58, 72, 13, 146, 147, 147, 142, 25, 2, 0, 0, 0, 0, 0, 0, 0, 0, 236, 6, 195, 27, 115, 127, 86, 133, 110, 193, 169, 191, 53, 44, 41, 81, 95, 73, 177, 222, 78, 205, 228, 54, 49, 213, 76, 52, 69, 0, 154, 164, 179, 134, 188, 139, 110, 45, 141, 171, 202, 250, 228, 197, 1, 36, 56, 243, 107, 135, 51, 152, 144, 183, 171, 50, 183, 98, 73, 242, 76, 5, 150];

fn insecure_pvk() -> VerifyingKey<Bn254> {
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

    vk
}

fn global_pvk() -> VerifyingKey<Bn254> {
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

    VerifyingKey {
        alpha_g1: vk_alpha_1,
        beta_g2: vk_beta_2,
        gamma_g2: vk_gamma_2,
        delta_g2: vk_delta_2,
        gamma_abc_g1: vk_gamma_abc_g1,
    }
}

fn my_test_pvk_1() -> VerifyingKey<Bn254> {
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

    VerifyingKey {
        alpha_g1: vk_alpha_1,
        beta_g2: vk_beta_2,
        gamma_g2: vk_gamma_2,
        delta_g2: vk_delta_2,
        gamma_abc_g1: vk_gamma_abc_g1,
    }
}

pub(crate) fn execute_vergrth16(engine: &mut Engine) -> Status {
    engine.load_instruction(crate::executor::types::Instruction::new("VERGRTH16"))?;
    engine.try_use_gas(Gas::vergrth16_price())?;
    fetch_stack(engine, 2)?;

    let public_inputs_slice = SliceData::load_cell_ref(engine.cmd.var(0).as_cell()?)?;
    let public_inputs_as_bytes = unpack_data_from_cell(public_inputs_slice, engine)?;

    let proof_slice = SliceData::load_cell_ref(engine.cmd.var(1).as_cell()?)?;
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

    /*let vk_bytes = if vk_index == 0 {
        INSECURE_VK_SERIALIZED
    } else if vk_index == 1 {
        GLOBAL_VK_SERIALIZED
    } else {
        MY_TEST_VK_1_SERIALIZED
    };*/

    let vk_bytes = engine.get_vergrth16_verififcation_key_serialized();

    let vk_deserialized: VerifyingKey<Bn254> = match ark_groth16::VerifyingKey::<Bn254>::deserialize_compressed(vk_bytes.as_slice()){
        Ok(vk) => vk,
        Err(err) => {
            return err!(ExceptionCode::FatalError, "Incorrect verification key {}", err);
        }
    };

    let vk: PreparedVerifyingKey<Bn254> = PreparedVerifyingKey::from(vk_deserialized);

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

#[cfg(test)]
mod tests {
    pub use ark_bn254::Bn254;
    use ark_groth16::VerifyingKey;
    use ark_serialize::CanonicalDeserialize;
    use ark_serialize::CanonicalSerialize;
    use crate::executor::zk::global_pvk;
    use crate::executor::zk::insecure_pvk;
    use crate::executor::zk::my_test_pvk_1;
    
    #[test]
    fn test_serialization_deserialization_global_pvk() {
        let vk: VerifyingKey<Bn254> = global_pvk();

        let mut bytes = Vec::new();
        vk.serialize_compressed(&mut bytes).unwrap();
        println!("vk serialized: {:?}", bytes.clone());
        println!("vk serialized len: {:?}", bytes.len());

        let vk_deserialized: VerifyingKey<Bn254> = match ark_groth16::VerifyingKey::<Bn254>::deserialize_compressed(bytes.as_slice()) {
            Ok(res) => res,
            Err(err) => {
                println!("err: {:?}", err);
                assert!(false);
                return;
            }
        };
        println!("global_pvk_deserialized: {:?}", vk);
        assert_eq!(vk, vk_deserialized);
    }

    #[test]
    fn test_serialization_deserialization_insecure_pvk() {
        let vk: VerifyingKey<Bn254> = insecure_pvk();

        let mut bytes = Vec::new();
        vk.serialize_compressed(&mut bytes).unwrap();
        println!("vk serialized: {:?}", bytes.clone());
        println!("vk serialized len: {:?}", bytes.len());

        let vk_deserialized: VerifyingKey<Bn254> = match ark_groth16::VerifyingKey::<Bn254>::deserialize_compressed(bytes.as_slice()) {
            Ok(res) => res,
            Err(err) => {
                println!("err: {:?}", err);
                assert!(false);
                return;
            }
        };
        println!("insecure_pvk_deserialized: {:?}", vk);
        assert_eq!(vk, vk_deserialized);
    }

     #[test]
    fn test_serialization_deserialization_my_test_pvk_1() {
        let vk: VerifyingKey<Bn254> = my_test_pvk_1();

        let mut bytes = Vec::new();
        vk.serialize_compressed(&mut bytes).unwrap();
        println!("vk serialized: {:?}", bytes.clone());
        println!("vk serialized len: {:?}", bytes.len());

        let vk_deserialized: VerifyingKey<Bn254> = match ark_groth16::VerifyingKey::<Bn254>::deserialize_compressed(bytes.as_slice()) {
            Ok(res) => res,
            Err(err) => {
                println!("err: {:?}", err);
                assert!(false);
                return;
            }
        };
        println!("my_test_pvk_1_deserialized: {:?}", vk);
        assert_eq!(vk, vk_deserialized);
    }

}
