use crate::executor::zk_stuff::bn254::FieldElement;
use crate::executor::zk_stuff::bn254::Proof;
use crate::executor::zk_stuff::bn254::VerifyingKey;
use crate::executor::zk_stuff::bn254::verifier::PreparedVerifyingKey;
use crate::executor::zk_stuff::error::ZkCryptoError;

/// Size of scalars in the BN254 construction.
pub const SCALAR_SIZE: usize = 32;

/// Deserialize bytes as an Arkwork representation of a verifying key, and
/// return a vector of the four components of a prepared verified key (see more
/// at [`PreparedVerifyingKey`]).
pub fn prepare_pvk_bytes(vk_bytes: &[u8]) -> Result<Vec<Vec<u8>>, ZkCryptoError> {
    PreparedVerifyingKey::from(&VerifyingKey::deserialize(vk_bytes)?).serialize()
}

/// Verify Groth16 proof using the serialized form of the prepared verifying key
/// (see more at [`crate::bn254::verifier::PreparedVerifyingKey`]), serialized
/// proof public input and serialized proof points.
pub fn verify_groth16_in_bytes(
    vk_gamma_abc_g1_bytes: &[u8],
    alpha_g1_beta_g2_bytes: &[u8],
    gamma_g2_neg_pc_bytes: &[u8],
    delta_g2_neg_pc_bytes: &[u8],
    proof_public_inputs_as_bytes: &[u8],
    proof_points_as_bytes: &[u8],
) -> Result<bool, ZkCryptoError> {
    if proof_public_inputs_as_bytes.len() % SCALAR_SIZE != 0 {
        return Err(ZkCryptoError::InputLengthWrong(SCALAR_SIZE));
    }

    let pvk = PreparedVerifyingKey::deserialize(&[
        vk_gamma_abc_g1_bytes,
        alpha_g1_beta_g2_bytes,
        gamma_g2_neg_pc_bytes,
        delta_g2_neg_pc_bytes,
    ])?;

    verify_groth16(&pvk, proof_public_inputs_as_bytes, proof_points_as_bytes)
}

/// Verify proof with a given verifying key in [struct PreparedVerifyingKey],
/// serialized public inputs and serialized proof points.
pub fn verify_groth16(
    pvk: &PreparedVerifyingKey,
    proof_public_inputs_as_bytes: &[u8],
    proof_points_as_bytes: &[u8],
) -> Result<bool, ZkCryptoError> {
    let proof = Proof::deserialize(proof_points_as_bytes)?;
    let public_inputs = FieldElement::deserialize_vector(proof_public_inputs_as_bytes)?;
    pvk.verify(&public_inputs, &proof)
}
