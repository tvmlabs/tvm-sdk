use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;
use std::str::FromStr;

pub use ark_bn254::Bn254;
pub use ark_bn254::Fr as Bn254Fr;
pub use ark_ff::ToConstraintField;
use ark_ff::Zero;
use ark_groth16::Proof;
pub use ark_serialize::CanonicalDeserialize;
pub use ark_serialize::CanonicalSerialize;
use itertools::Itertools;
use num_bigint::BigUint;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use super::utils::split_to_two_frs;
use crate::executor::zk_stuff::bn254::poseidon::poseidon_zk_login;
use crate::executor::zk_stuff::curve_utils::g1_affine_from_str_projective;
use crate::executor::zk_stuff::curve_utils::g2_affine_from_str_projective;
use crate::executor::zk_stuff::curve_utils::Bn254FrElement;
use crate::executor::zk_stuff::curve_utils::CircomG1;
use crate::executor::zk_stuff::curve_utils::CircomG2;
use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::error::ZkCryptoResult;
use crate::executor::zk_stuff::jwt_utils::JWTHeader;

pub const MAX_HEADER_LEN: u8 = 248;
pub const PACK_WIDTH: u8 = 248;
pub const ISS: &str = "iss";
pub const BASE64_URL_CHARSET: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
pub const MAX_EXT_ISS_LEN: u8 = 165;
pub const MAX_ISS_LEN_B64: u8 = 4 * (1 + MAX_EXT_ISS_LEN / 3);

/// Key to identify a JWK, consists of iss and kid.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct JwkId {
    /// iss string that identifies the OIDC provider.
    pub iss: String,
    /// kid string that identifies the JWK.
    pub kid: String,
}

impl JwkId {
    /// Create a new JwkId.
    pub fn new(iss: String, kid: String) -> Self {
        Self { iss, kid }
    }
}

/// The provider config consists of iss string and jwk endpoint.
#[derive(Debug)]
pub struct ProviderConfig {
    /// iss string that identifies the OIDC provider.
    pub iss: String,
    /// The JWK url string for the given provider.
    pub jwk_endpoint: String,
}

impl ProviderConfig {
    /// Create a new provider config.
    pub fn new(iss: &str, jwk_endpoint: &str) -> Self {
        Self { iss: iss.to_string(), jwk_endpoint: jwk_endpoint.to_string() }
    }
}

/// Supported OIDC providers.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum OIDCProvider {
    /// See https://accounts.google.com/.well-known/openid-configuration
    Google,
    /// See https://id.twitch.tv/oauth2/.well-known/openid-configuration
    Twitch,
    /// See https://www.facebook.com/.well-known/openid-configuration/
    Facebook,
    /// See https://kauth.kakao.com/.well-known/openid-configuration
    Kakao,
    /// See https://appleid.apple.com/.well-known/openid-configuration
    Apple,
    /// See https://slack.com/.well-known/openid-configuration
    Slack,
    /// This is a test issuer maintained by Mysten that will return a JWT
    /// non-interactively.
    TestIssuer,
}

impl FromStr for OIDCProvider {
    type Err = ZkCryptoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Google" => Ok(Self::Google),
            "Twitch" => Ok(Self::Twitch),
            "Facebook" => Ok(Self::Facebook),
            "Kakao" => Ok(Self::Kakao),
            "Apple" => Ok(Self::Apple),
            "Slack" => Ok(Self::Slack),
            "TestIssuer" => Ok(Self::TestIssuer),
            _ => Err(ZkCryptoError::InvalidInput),
        }
    }
}

impl ToString for OIDCProvider {
    fn to_string(&self) -> String {
        match self {
            Self::Google => "Google".to_string(),
            Self::Twitch => "Twitch".to_string(),
            Self::Facebook => "Facebook".to_string(),
            Self::Kakao => "Kakao".to_string(),
            Self::Apple => "Apple".to_string(),
            Self::Slack => "Slack".to_string(),
            Self::TestIssuer => "TestIssuer".to_string(),
        }
    }
}

impl OIDCProvider {
    /// Returns the provider config consisting of iss and jwk endpoint.
    pub fn get_config(&self) -> ProviderConfig {
        match self {
            OIDCProvider::Google => ProviderConfig::new(
                "https://accounts.google.com",
                "https://www.googleapis.com/oauth2/v2/certs",
            ),
            OIDCProvider::Twitch => ProviderConfig::new(
                "https://id.twitch.tv/oauth2",
                "https://id.twitch.tv/oauth2/keys",
            ),
            OIDCProvider::Facebook => ProviderConfig::new(
                "https://www.facebook.com",
                "https://www.facebook.com/.well-known/oauth/openid/jwks/",
            ),
            OIDCProvider::Kakao => ProviderConfig::new(
                "https://kauth.kakao.com",
                "https://kauth.kakao.com/.well-known/jwks.json",
            ),
            OIDCProvider::Apple => ProviderConfig::new(
                "https://appleid.apple.com",
                "https://appleid.apple.com/auth/keys",
            ),
            OIDCProvider::Slack => {
                ProviderConfig::new("https://slack.com", "https://slack.com/openid/connect/keys")
            }
            OIDCProvider::TestIssuer => ProviderConfig::new(
                "https://oauth.sui.io",
                "https://jwt-tester.mystenlabs.com/.well-known/jwks.json",
            ),
        }
    }

    /// Returns the OIDCProvider for the given iss string.
    pub fn from_iss(iss: &str) -> Result<Self, ZkCryptoError> {
        match iss {
            "https://accounts.google.com" => Ok(Self::Google),
            "https://id.twitch.tv/oauth2" => Ok(Self::Twitch),
            "https://www.facebook.com" => Ok(Self::Facebook),
            "https://kauth.kakao.com" => Ok(Self::Kakao),
            "https://appleid.apple.com" => Ok(Self::Apple),
            "https://slack.com" => Ok(Self::Slack),
            "https://oauth.sui.io" => Ok(Self::TestIssuer),
            _ => Err(ZkCryptoError::InvalidInput),
        }
    }
}

/// Struct that contains info for a JWK. A list of them for different kids can
/// be retrieved from the JWK endpoint (e.g. <https://www.googleapis.com/oauth2/v3/certs>).
/// The JWK is used to verify the JWT token.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub struct JWK {
    /// Key type parameter, https://datatracker.ietf.org/doc/html/rfc7517#section-4.1
    pub kty: String,
    /// RSA public exponent, https://datatracker.ietf.org/doc/html/rfc7517#section-9.3
    pub e: String,
    /// RSA modulus, https://datatracker.ietf.org/doc/html/rfc7517#section-9.3
    pub n: String,
    /// Algorithm parameter, https://datatracker.ietf.org/doc/html/rfc7517#section-4.4
    pub alg: String,
}

/// Reader struct to parse all fields in a JWK from JSON.
#[derive(Debug, Serialize, Deserialize)]
pub struct JWKReader {
    e: String,
    n: String,
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    my_use: Option<String>,
    kid: String,
    kty: String,
    alg: String,
}

impl JWK {
    /// Parse JWK from the reader struct.
    pub fn from_reader(reader: JWKReader) -> ZkCryptoResult<Self> {
        let trimmed_e = trim(reader.e);
        if reader.alg != "RS256" || reader.kty != "RSA" || trimmed_e != "AQAB" {
            return Err(ZkCryptoError::InvalidInput);
        }
        Ok(Self { kty: reader.kty, e: trimmed_e, n: trim(reader.n), alg: reader.alg })
    }
}

/// Trim trailing '=' so that it is considered a valid base64 url encoding
/// string by base64ct library.
fn trim(str: String) -> String {
    str.trim_end_matches('=').to_owned()
}

// /// Fetch JWKs from the given provider and return a list of JwkId -> JWK.
// pub async fn fetch_jwks(
// provider: &OIDCProvider,
// client: &Client,
// ) -> Result<Vec<(JwkId, JWK)>, ZkCryptoError> {
// let response = client
// .get(provider.get_config().jwk_endpoint)
// .send()
// .await
// .map_err(|e| {
// ZkCryptoError::GeneralError(format!(
// "Failed to get JWK {:?} {:?}",
// e.to_string(),
// provider
// ))
// })?;
// let bytes = response.bytes().await.map_err(|e| {
// ZkCryptoError::GeneralError(format!(
// "Failed to get bytes {:?} {:?}",
// e.to_string(),
// provider
// ))
// })?;
// parse_jwks(&bytes, provider)
// }

/// Parse the JWK bytes received from the given provider and return a list of
/// JwkId -> JWK.
pub fn parse_jwks(
    json_bytes: &[u8],
    provider: &OIDCProvider,
) -> Result<Vec<(JwkId, JWK)>, ZkCryptoError> {
    let json_str = String::from_utf8_lossy(json_bytes);
    let parsed_list: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&json_str);
    if let Ok(parsed_list) = parsed_list {
        if let Some(keys) = parsed_list["keys"].as_array() {
            let mut ret = Vec::new();
            for k in keys {
                let parsed: JWKReader = serde_json::from_value(k.clone())
                    .map_err(|_| ZkCryptoError::GeneralError("Parse error".to_string()))?;

                ret.push((
                    JwkId::new(provider.get_config().iss, parsed.kid.clone()),
                    JWK::from_reader(parsed)?,
                ));
            }
            return Ok(ret);
        }
    }
    Err(ZkCryptoError::GeneralError("Invalid JWK response".to_string()))
}

/// A claim consists of value and index_mod_4.
#[derive(Debug, Clone, PartialEq, Eq, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub value: String,
    pub index_mod_4: u8,
}

/// A structed of parsed JWT details, consists of kid, header, iss.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct JWTDetails {
    kid: String,
    header: String,
    iss: String,
}

impl JWTDetails {
    /// Read in the Claim and header string. Parse and validate kid, header, iss
    /// as JWT details.
    pub fn new(header_base64: &str, claim: &Claim) -> Result<Self, ZkCryptoError> {
        let header = JWTHeader::new(header_base64)?;
        let ext_claim = decode_base64_url(&claim.value, &claim.index_mod_4)?;
        Ok(JWTDetails {
            kid: header.kid,
            header: header_base64.to_string(),
            iss: verify_extended_claim(&ext_claim, ISS)?,
        })
    }
}

/// All inputs required for the zk login proof verification and other public
/// inputs.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZkLoginInputs {
    proof_points: ZkLoginProof,
    iss_base64_details: Claim,
    header_base64: String,
    address_seed: Bn254FrElement,
    #[serde(skip)]
    jwt_details: JWTDetails,
}

/// The reader struct for the proving service response.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZkLoginInputsReader {
    proof_points: ZkLoginProof,
    iss_base64_details: Claim,
    header_base64: String,
    #[serde(skip)]
    jwt_details: JWTDetails,
}

impl ZkLoginInputs {
    /// Parse the proving service response and pass in the address seed.
    /// Initialize the jwt details struct.
    pub fn from_json(value: &str, address_seed: &str) -> Result<Self, ZkCryptoError> {
        let reader: ZkLoginInputsReader =
            serde_json::from_str(value).map_err(|_| ZkCryptoError::InvalidInput)?;
        Self::from_reader(reader, address_seed)
    }

    /// Initialize ZkLoginInputs from the
    pub fn from_reader(
        reader: ZkLoginInputsReader,
        address_seed: &str,
    ) -> Result<Self, ZkCryptoError> {
        ZkLoginInputs {
            proof_points: reader.proof_points,
            iss_base64_details: reader.iss_base64_details,
            header_base64: reader.header_base64,
            address_seed: Bn254FrElement::from_str(address_seed)
                .map_err(|_| ZkCryptoError::InvalidInput)?,
            jwt_details: reader.jwt_details,
        }
        .init()
    }

    /// Initialize JWTDetails by parsing header_base64 and iss_base64_details.
    pub fn init(&mut self) -> Result<Self, ZkCryptoError> {
        self.jwt_details = JWTDetails::new(&self.header_base64, &self.iss_base64_details)?;
        Ok(self.to_owned())
    }

    /// Get the parsed kid string.
    pub fn get_kid(&self) -> &str {
        &self.jwt_details.kid
    }

    /// Get the parsed iss string.
    pub fn get_iss(&self) -> &str {
        &self.jwt_details.iss
    }

    /// Get the zk login proof.
    pub fn get_proof(&self) -> &ZkLoginProof {
        &self.proof_points
    }

    /// Get the address seed string.
    pub fn get_address_seed(&self) -> &Bn254FrElement {
        &self.address_seed
    }

    /// Calculate the poseidon hash from selected fields from inputs, along with
    /// the ephemeral pubkey.
    pub fn calculate_all_inputs_hash(
        &self,
        eph_pk_bytes: &[u8],
        modulus: &[u8],
        max_epoch: u64,
    ) -> Result<Bn254Fr, ZkCryptoError> {
        if self.header_base64.len() > MAX_HEADER_LEN as usize {
            return Err(ZkCryptoError::GeneralError("Header too long".to_string()));
        }

        let addr_seed = (&self.address_seed).into();
        let (first, second) = split_to_two_frs(eph_pk_bytes)?;

        let max_epoch_f = (&Bn254FrElement::from_str(&max_epoch.to_string())?).into();
        let index_mod_4_f =
            (&Bn254FrElement::from_str(&self.iss_base64_details.index_mod_4.to_string())?).into();

        let iss_base64_f =
            hash_ascii_str_to_field(&self.iss_base64_details.value, MAX_ISS_LEN_B64)?;
        let header_f = hash_ascii_str_to_field(&self.header_base64, MAX_HEADER_LEN)?;
        let modulus_f = hash_to_field(&[BigUint::from_bytes_be(modulus)], 2048, PACK_WIDTH)?;
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
}
/// The struct for zk login proof.
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct ZkLoginProof {
    a: CircomG1,
    b: CircomG2,
    c: CircomG1,
}

impl ZkLoginProof {
    /// Parse the proof from a json string.
    pub fn from_json(value: &str) -> Result<Self, ZkCryptoError> {
        let proof: ZkLoginProof =
            serde_json::from_str(value).map_err(|_| ZkCryptoError::InvalidProof)?;
        Ok(proof)
    }

    /// Convert the Circom G1/G2/GT to arkworks G1/G2/GT
    pub fn as_arkworks(&self) -> Result<Proof<Bn254>, ZkCryptoError> {
        return Ok(Proof {
            a: g1_affine_from_str_projective(&self.a)?,
            b: g2_affine_from_str_projective(&self.b)?,
            c: g1_affine_from_str_projective(&self.c)?,
        });
    }
}

/// Parse the extended claim json value to its claim value, using the expected
/// claim key.
fn verify_extended_claim(
    extended_claim: &str,
    expected_key: &str,
) -> Result<String, ZkCryptoError> {
    // Last character of each extracted_claim must be '}' or ','
    if !(extended_claim.ends_with('}') || extended_claim.ends_with(',')) {
        return Err(ZkCryptoError::GeneralError("Invalid extended claim".to_string()));
    }

    let json_str = format!("{{{}}}", &extended_claim[..extended_claim.len() - 1]);
    let json: Value = serde_json::from_str(&json_str).map_err(|_| ZkCryptoError::InvalidInput)?;
    let value = json
        .as_object()
        .ok_or(ZkCryptoError::InvalidInput)?
        .get(expected_key)
        .ok_or(ZkCryptoError::InvalidInput)?
        .as_str()
        .ok_or(ZkCryptoError::InvalidInput)?;
    Ok(value.to_string())
}

/// Parse the base64 string, add paddings based on offset, and convert to a
/// bytearray.
fn decode_base64_url(s: &str, i: &u8) -> Result<String, ZkCryptoError> {
    if s.len() < 2 {
        return Err(ZkCryptoError::GeneralError("Base64 string smaller than 2".to_string()));
    }
    let mut bits = base64_to_bitarray(s)?;
    match i {
        0 => {}
        1 => {
            bits.drain(..2);
        }
        2 => {
            bits.drain(..4);
        }
        _ => {
            return Err(ZkCryptoError::GeneralError("Invalid first_char_offset".to_string()));
        }
    }

    let last_char_offset = (i + s.len() as u8 - 1) % 4;
    match last_char_offset {
        3 => {}
        2 => {
            bits.drain(bits.len() - 2..);
        }
        1 => {
            bits.drain(bits.len() - 4..);
        }
        _ => {
            return Err(ZkCryptoError::GeneralError("Invalid last_char_offset".to_string()));
        }
    }

    if bits.len() % 8 != 0 {
        return Err(ZkCryptoError::GeneralError("Invalid bits length".to_string()));
    }

    Ok(std::str::from_utf8(&bitarray_to_bytearray(&bits)?)
        .map_err(|_| ZkCryptoError::GeneralError("Invalid UTF8 string".to_string()))?
        .to_owned())
}

/// Map a base64 string to a bit array by taking each char's index and covert it
/// to binary form with one bit per u8 element in the output. Returns
/// [ZkCryptoError::InvalidInput] if one of the characters is not in the base64
/// charset.
fn base64_to_bitarray(input: &str) -> ZkCryptoResult<Vec<u8>> {
    input
        .chars()
        .map(|c| {
            BASE64_URL_CHARSET
                .find(c)
                .map(|index| index as u8)
                .map(|index| (0..6).rev().map(move |i| index >> i & 1))
                .ok_or(ZkCryptoError::InvalidInput)
        })
        .flatten_ok()
        .collect()
}

/// Convert a bitarray (each bit is represented by a u8) to a byte array by
/// taking each 8 bits as a byte in big-endian format.
fn bitarray_to_bytearray(bits: &[u8]) -> ZkCryptoResult<Vec<u8>> {
    if bits.len() % 8 != 0 {
        return Err(ZkCryptoError::InvalidInput);
    }
    Ok(bits
        .chunks(8)
        .map(|chunk| {
            let mut byte = 0u8;
            for (i, bit) in chunk.iter().rev().enumerate() {
                byte |= bit << i;
            }
            byte
        })
        .collect())
}

/// Pads a stream of bytes and maps it to a field element
pub fn hash_ascii_str_to_field(str: &str, max_size: u8) -> Result<Bn254Fr, ZkCryptoError> {
    let str_padded = str_to_padded_char_codes(str, max_size)?;
    hash_to_field(&str_padded, 8, PACK_WIDTH)
}

fn str_to_padded_char_codes(str: &str, max_len: u8) -> Result<Vec<BigUint>, ZkCryptoError> {
    let arr: Vec<BigUint> = str.chars().map(|c| BigUint::from_slice(&([c as u32]))).collect();
    pad_with_zeroes(arr, max_len)
}

fn pad_with_zeroes(in_arr: Vec<BigUint>, out_count: u8) -> Result<Vec<BigUint>, ZkCryptoError> {
    if in_arr.len() > out_count as usize {
        return Err(ZkCryptoError::GeneralError("in_arr too long".to_string()));
    }
    let mut padded = in_arr;
    padded.resize(out_count as usize, BigUint::zero());
    Ok(padded)
}

/// Maps a stream of bigints to a single field element. First we convert the
/// base from inWidth to packWidth. Then we compute the poseidon hash of the
/// "packed" input. input is the input vector containing equal-width big ints.
/// inWidth is the width of each input element.
pub fn hash_to_field(
    input: &[BigUint],
    in_width: u16,
    pack_width: u8,
) -> Result<Bn254Fr, ZkCryptoError> {
    let packed = convert_base(input, in_width, pack_width)?;
    poseidon_zk_login(packed)
}

/// Helper function to pack field elements from big ints.
fn convert_base(
    in_arr: &[BigUint],
    in_width: u16,
    out_width: u8,
) -> Result<Vec<Bn254Fr>, ZkCryptoError> {
    if out_width == 0 {
        return Err(ZkCryptoError::InvalidInput);
    }
    let bits = big_int_array_to_bits(in_arr, in_width as usize)?;
    let mut packed: Vec<Bn254Fr> = bits
        .rchunks(out_width as usize)
        .map(|chunk| Bn254Fr::from(BigUint::from_radix_be(chunk, 2).unwrap()))
        .collect();
    packed.reverse();
    match packed.len() != (in_arr.len() * in_width as usize).div_ceil(out_width as usize) {
        true => Err(ZkCryptoError::InvalidInput),
        false => Ok(packed),
    }
}

/// Convert a big int array to a bit array with 0 paddings.
fn big_int_array_to_bits(integers: &[BigUint], intended_size: usize) -> ZkCryptoResult<Vec<u8>> {
    integers
        .iter()
        .map(|integer| {
            let bits = integer.to_radix_be(2);
            match bits.len().cmp(&intended_size) {
                Less => {
                    let extra_bits = intended_size - bits.len();
                    let mut padded = vec![0; extra_bits];
                    padded.extend(bits);
                    Ok(padded)
                }
                Equal => Ok(bits),
                Greater => Err(ZkCryptoError::InvalidInput),
            }
        })
        .flatten_ok()
        .collect()
}
