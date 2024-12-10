use std::str::FromStr;

use num_bigint::BigUint;
use serde::Deserialize;
use serde::Serialize;

use super::zk_login::hash_ascii_str_to_field;
use crate::executor::zk::Bn254Fr;
use crate::executor::zk_stuff::bn254::poseidon::poseidon_zk_login;
use crate::executor::zk_stuff::curve_utils::Bn254FrElement;
use crate::executor::zk_stuff::error::ZkCryptoError;
use fastcrypto::hash::{Blake2b256, HashFunction};

const MAX_KEY_CLAIM_NAME_LENGTH: u8 = 32;
const MAX_KEY_CLAIM_VALUE_LENGTH: u8 = 115;
const MAX_AUD_VALUE_LENGTH: u8 = 145;
const ZK_LOGIN_AUTHENTICATOR_FLAG: u8 = 0x05;

pub fn gen_address_seed(
    salt: &str,
    name: &str,  // i.e. "sub"
    value: &str, // i.e. the sub value
    aud: &str,   // i.e. the client ID
) -> Result<String, ZkCryptoError> {
    let salt_hash = poseidon_zk_login(vec![(&Bn254FrElement::from_str(salt)?).into()])?;
    gen_address_seed_with_salt_hash(&salt_hash.to_string(), name, value, aud)
}

pub fn get_zk_login_address(
    address_seed: &Bn254FrElement,
    iss: &str,
) -> Result<[u8; 32], ZkCryptoError> {
    let mut hasher = Blake2b256::default();
    hasher.update([ZK_LOGIN_AUTHENTICATOR_FLAG]);
    let bytes = iss.as_bytes();
    hasher.update([bytes.len() as u8]);
    hasher.update(bytes);
    hasher.update(address_seed.padded());
    Ok(hasher.finalize().digest)
}

/// Same as [`gen_address_seed`] but takes the poseidon hash of the salt as
/// input instead of the salt.
pub(crate) fn gen_address_seed_with_salt_hash(
    salt_hash: &str,
    name: &str,  // i.e. "sub"
    value: &str, // i.e. the sub value
    aud: &str,   // i.e. the client ID
) -> Result<String, ZkCryptoError> {
    Ok(poseidon_zk_login(vec![
        hash_ascii_str_to_field(name, MAX_KEY_CLAIM_NAME_LENGTH)?,
        hash_ascii_str_to_field(value, MAX_KEY_CLAIM_VALUE_LENGTH)?,
        hash_ascii_str_to_field(aud, MAX_AUD_VALUE_LENGTH)?,
        (&Bn254FrElement::from_str(salt_hash)?).into(),
    ])?
    .to_string())
}

/// Given a 33-byte public key bytes (flag || pk_bytes), returns the two Bn254Fr
/// split at the 128 bit index.
pub fn split_to_two_frs(eph_pk_bytes: &[u8]) -> Result<(Bn254Fr, Bn254Fr), ZkCryptoError> {
    // Split the bytes deterministically such that the first element contains the
    // first 128 bits of the hash, and the second element contains the latter
    // ones.
    let (first_half, second_half) = eph_pk_bytes.split_at(eph_pk_bytes.len() - 16);
    let first_bigint = BigUint::from_bytes_be(first_half);
    // TODO: this is not safe if the buffer is large. Can we use a fixed size array
    // for eph_pk_bytes?
    let second_bigint = BigUint::from_bytes_be(second_half);

    let eph_public_key_0 = Bn254Fr::from(first_bigint);
    let eph_public_key_1 = Bn254Fr::from(second_bigint);
    Ok((eph_public_key_0, eph_public_key_1))
}

/// The response struct for the test issuer JWT token.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestIssuerJWTResponse {
    /// JWT token string.
    pub jwt: String,
}
