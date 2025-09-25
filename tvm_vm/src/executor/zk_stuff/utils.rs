use std::str::FromStr;

use base64ct::Encoding;
use fastcrypto::hash::Blake2b256;
use fastcrypto::hash::HashFunction;
use fastcrypto::rsa::Base64UrlUnpadded;
use num_bigint::BigUint;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

use super::zk_login::hash_ascii_str_to_field;
use crate::executor::zk::Bn254Fr;
use crate::executor::zk_stuff::bn254::poseidon::poseidon_zk_login;
use crate::executor::zk_stuff::curve_utils::Bn254FrElement;
use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::zk_login::ZkLoginInputsReader;

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

/// Call the prover backend to get the zkLogin inputs based on jwt_token,
/// max_epoch, jwt_randomness, eph_pubkey and salt.
pub async fn get_proof(
    jwt_token: &str,
    max_epoch: u64,
    jwt_randomness: &str,
    eph_pubkey: &str,
    salt: &str,
    prover_url: &str,
) -> Result<ZkLoginInputsReader, ZkCryptoError> {
    let body = json!({
    "jwt": jwt_token,
    "extendedEphemeralPublicKey": eph_pubkey,
    "maxEpoch": max_epoch,
    "jwtRandomness": jwt_randomness,
    "salt": salt,
    "keyClaimName": "sub",
    });
    let client = Client::new();
    let response = client
        .post(prover_url.to_string())
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|err| {
		println!("{err:?}");
		ZkCryptoError::InvalidInput
	})?;
    let full_bytes = response.bytes().await
        .map_err(|err| {
		println!("{err:?}");
		ZkCryptoError::InvalidInput
	})?;

    let get_proof_response: ZkLoginInputsReader =
        serde_json::from_slice(&full_bytes)
        .map_err(|err| {
		println!("{err:?}");
		ZkCryptoError::InvalidInput
	})?;

    Ok(get_proof_response)
}

/// Calculate the nonce for the given parameters. Nonce is defined as the
/// Base64Url encoded of the poseidon hash of 4 inputs: first half of
/// eph_pk_bytes in BigInt, second half of eph_pk_bytes in BigInt, max_epoch and
/// jwt_randomness.
pub fn get_nonce(
    eph_pk_bytes: &[u8],
    max_epoch: u64,
    jwt_randomness: &str,
) -> Result<String, ZkCryptoError> {
    let (first, second) = split_to_two_frs(eph_pk_bytes)?;

    let max_epoch = Bn254Fr::from_str(&max_epoch.to_string())
        .expect("max_epoch.to_string is always non empty string without trailing zeros");
    let jwt_randomness =
        Bn254Fr::from_str(jwt_randomness).map_err(|_| ZkCryptoError::InvalidInput)?;

    let hash = poseidon_zk_login([first, second, max_epoch, jwt_randomness].to_vec())
        .expect("inputs is not too long");
    let data = BigUint::from(hash).to_bytes_be();
    let truncated = &data[data.len() - 20..];
    let mut buf = vec![0; Base64UrlUnpadded::encoded_len(truncated)];
    Ok(Base64UrlUnpadded::encode(truncated, &mut buf).unwrap().to_string())
}
