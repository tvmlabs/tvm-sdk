#![warn(unreachable_pub)]
#![deny(unused_must_use, missing_debug_implementations)]

use ark_bn254::Bn254;
use ark_bn254::Fr;
use ark_serialize::CanonicalDeserialize;
use derive_more::From;

use crate::executor::zk_stuff::bn254::api::SCALAR_SIZE;
use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::error::ZkCryptoResult;

/// API that takes in serialized inputs
pub mod api;

/// Groth16 SNARK verifier
pub mod verifier;

/// Poseidon hash function over BN254
pub mod poseidon;

/// A field element in the BN254 construction. Thin wrapper around
/// `api::Bn254Fr`.
#[derive(Debug, From)]
pub struct FieldElement(pub(crate) ark_bn254::Fr);

/// A Groth16 proof in the BN254 construction. Thin wrapper around
/// `ark_groth16::Proof::<ark_bn254::Bn254>`.
#[derive(Debug, From)]
pub struct Proof(pub(crate) ark_groth16::Proof<ark_bn254::Bn254>);

/// A Groth16 verifying key in the BN254 construction. Thin wrapper around
/// `ark_groth16::VerifyingKey::<ark_bn254::Bn254>`.
#[derive(Debug, From)]
pub struct VerifyingKey(pub(crate) ark_groth16::VerifyingKey<ark_bn254::Bn254>);

impl Proof {
    /// Deserialize a serialized Groth16 proof using arkworks' canonical serialisation format: https://docs.rs/ark-serialize/latest/ark_serialize/.
    pub fn deserialize(proof_points_as_bytes: &[u8]) -> ZkCryptoResult<Self> {
        ark_groth16::Proof::<Bn254>::deserialize_compressed(proof_points_as_bytes)
            .map_err(|_| ZkCryptoError::InvalidInput)
            .map(Proof)
    }
}

impl FieldElement {
    /// Deserialize 32 bytes into a BN254 field element using little-endian
    /// format.
    pub(crate) fn deserialize(bytes: &[u8]) -> ZkCryptoResult<FieldElement> {
        if bytes.len() != SCALAR_SIZE {
            return Err(ZkCryptoError::InputLengthWrong(bytes.len()));
        }
        Fr::deserialize_compressed(bytes).map_err(|_| ZkCryptoError::InvalidInput).map(FieldElement)
    }

    /// Deserialize a vector of bytes into a vector of BN254 field elements,
    /// assuming that each element is serialized as a chunk of 32 bytes. See
    /// also [`FieldElement::deserialize`].
    pub(crate) fn deserialize_vector(
        field_element_bytes: &[u8],
    ) -> ZkCryptoResult<Vec<FieldElement>> {
        if field_element_bytes.len() % SCALAR_SIZE != 0 {
            return Err(ZkCryptoError::InputLengthWrong(field_element_bytes.len()));
        }
        let mut public_inputs = Vec::new();
        for chunk in field_element_bytes.chunks(SCALAR_SIZE) {
            public_inputs.push(FieldElement::deserialize(chunk)?);
        }
        Ok(public_inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::executor::zk_stuff::error::ZkCryptoError;

    #[test]
    fn field_element_deserialize_validates_length_and_content() {
        assert!(matches!(
            FieldElement::deserialize(&[0u8; 31]),
            Err(ZkCryptoError::InputLengthWrong(31))
        ));
        assert!(matches!(
            FieldElement::deserialize(&[0xff; SCALAR_SIZE]),
            Err(ZkCryptoError::InvalidInput)
        ));
        assert!(FieldElement::deserialize(&[0u8; SCALAR_SIZE]).is_ok());
    }

    #[test]
    fn field_element_vector_requires_scalar_aligned_input() {
        assert!(matches!(
            FieldElement::deserialize_vector(&[0u8; 1]),
            Err(ZkCryptoError::InputLengthWrong(1))
        ));
        assert_eq!(FieldElement::deserialize_vector(&[0u8; SCALAR_SIZE * 2]).unwrap().len(), 2);
    }

    #[test]
    fn proof_deserialize_rejects_invalid_bytes() {
        assert!(matches!(Proof::deserialize(&[0u8; 8]), Err(ZkCryptoError::InvalidInput)));
    }
}
