// use derive_more::From;
use ff::PrimeField;

// pub mod alphabet;
pub mod curve_utils;
// pub mod encodings;
pub mod bn254;
pub mod error;
pub mod jwt_utils;
pub mod utils;
pub mod zk_login;

/// Definition of the BN254 prime field.
#[derive(PrimeField)]
#[PrimeFieldModulus = "21888242871839275222246405745257275088548364400416034343698204186575808495617"]
#[PrimeFieldGenerator = "5"]
#[PrimeFieldReprEndianness = "big"]
pub struct Fr([u64; 4]);

// #[derive(Debug, From)]
// pub struct VerifyingKey(pub(crate)
// ark_groth16::VerifyingKey<ark_bn254::Bn254>);
