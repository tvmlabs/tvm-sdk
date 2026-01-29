use std::cmp::Ordering;

use ark_bn254::Fr;
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use byte_slice_cast::AsByteSlice;
use ff::PrimeField as OtherPrimeField;
use neptune::Poseidon;
use neptune::poseidon::HashMode::OptimizedStatic;

use crate::executor::zk_stuff::FrRepr;
use crate::executor::zk_stuff::bn254::poseidon::constants::*;
use crate::executor::zk_stuff::error::ZkCryptoError;
use crate::executor::zk_stuff::error::ZkCryptoError::InputTooLong;
use crate::executor::zk_stuff::error::ZkCryptoError::InvalidInput;
use crate::executor::zk_stuff::error::ZkCryptoResult;

/// The output of the Poseidon hash function is a field element in BN254 which
/// is 254 bits long, so we need 32 bytes to represent it as an integer.
pub const FIELD_ELEMENT_SIZE_IN_BYTES: usize = 32;

/// The degree of the Merkle tree used to hash multiple elements.
pub const MERKLE_TREE_DEGREE: usize = 16;

mod constants;

/// Define a macro to calculate the poseidon hash of a vector of inputs using
/// the neptune library.
macro_rules! define_poseidon_hash {
    ($inputs:expr, $poseidon_constants:expr) => {{
        let mut poseidon = Poseidon::new(&$poseidon_constants);
        poseidon.reset();
        for input in $inputs.iter() {
            poseidon.input(bn254_to_fr(*input)).expect("The number of inputs must be aligned with the constants");
        }
        poseidon.hash_in_mode(OptimizedStatic);

        // Neptune returns the state element with index 1 but we want the first element to be aligned
        // with poseidon-rs and circomlib's implementation which returns the 0'th element.
        //
        // See:
        //  * https://github.com/lurk-lab/neptune/blob/b7a9db1fc6ce096aff52b903f7d228eddea6d4e3/src/poseidon.rs#L698
        //  * https://github.com/arnaucube/poseidon-rs/blob/f4ba1f7c32905cd2ae5a71e7568564bb150a9862/src/lib.rs#L116
        //  * https://github.com/iden3/circomlib/blob/cff5ab6288b55ef23602221694a6a38a0239dcc0/circuits/poseidon.circom#L207
        poseidon.elements[0]
    }};
}

/// Poseidon hash function over BN254. The input vector cannot be empty and must
/// contain at most 16 elements, otherwise an error is returned.
pub fn poseidon(inputs: Vec<Fr>) -> Result<Fr, ZkCryptoError> {
    if inputs.is_empty() || inputs.len() > 16 {
        return Err(ZkCryptoError::InputLengthWrong(inputs.len()));
    }

    // Instances of Poseidon and PoseidonConstants from neptune have different types
    // depending on the number of inputs, so unfortunately we need to use a
    // macro here.
    let result = match inputs.len() {
        1 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U1),
        2 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U2),
        3 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U3),
        4 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U4),
        5 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U5),
        6 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U6),
        7 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U7),
        8 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U8),
        9 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U9),
        10 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U10),
        11 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U11),
        12 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U12),
        13 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U13),
        14 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U14),
        15 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U15),
        16 => define_poseidon_hash!(inputs, POSEIDON_CONSTANTS_U16),
        _ => return Err(InvalidInput),
    };
    Ok(fr_to_bn254fr(result))
}

/// Calculate the poseidon hash of the field element inputs. If there are no
/// inputs, return an error. If input length is <= 16, calculate H(inputs), if
/// it is <= 32, calculate H(H(inputs[0..16]), H(inputs[16..])), otherwise
/// return an error.
///
/// This functions must be equivalent with the one found in the zk_login
/// circuit.
pub(crate) fn poseidon_zk_login(inputs: Vec<Fr>) -> ZkCryptoResult<Fr> {
    if inputs.is_empty() || inputs.len() > 32 {
        return Err(ZkCryptoError::InputLengthWrong(inputs.len()));
    }
    poseidon_merkle_tree(inputs)
}

/// Calculate the poseidon hash of the field element inputs. If the input length
/// is <= 16, calculate H(inputs), otherwise chunk the inputs into groups of 16,
/// hash them and input the results recursively.
pub fn poseidon_merkle_tree(inputs: Vec<Fr>) -> Result<Fr, ZkCryptoError> {
    if inputs.len() <= MERKLE_TREE_DEGREE {
        poseidon(inputs)
    } else {
        poseidon_merkle_tree(
            inputs
                .chunks(MERKLE_TREE_DEGREE)
                .map(|chunk| poseidon(chunk.to_vec()))
                .collect::<ZkCryptoResult<Vec<_>>>()?,
        )
    }
}

/// Calculate the poseidon hash of an array of inputs. Each input is interpreted
/// as a BN254 field element assuming a little-endian encoding. The field
/// elements are then hashed using the poseidon hash function
/// ([poseidon_merkle_tree]) and the result is serialized as a little-endian
/// integer (32 bytes).
///
/// If one of the inputs is in non-canonical form, e.g. it represents an integer
/// greater than the field size or is longer than 32 bytes, an error is
/// returned.
pub fn poseidon_bytes(
    inputs: &Vec<Vec<u8>>,
) -> Result<[u8; FIELD_ELEMENT_SIZE_IN_BYTES], ZkCryptoError> {
    let mut field_elements = Vec::new();
    for input in inputs {
        field_elements.push(canonical_le_bytes_to_field_element(input)?);
    }
    let output_as_field_element = poseidon_merkle_tree(field_elements)?;
    Ok(field_element_to_canonical_le_bytes(&output_as_field_element))
}

pub fn poseidon_bytes_flat(
    input_data: &Vec<u8>,
) -> Result<[u8; FIELD_ELEMENT_SIZE_IN_BYTES], ZkCryptoError> {
    if input_data.len()%FIELD_ELEMENT_SIZE_IN_BYTES != 0 {
        return Err(InputTooLong(input_data.len()));
    }
    let mut inputs_groupped: Vec<Vec<u8>> = Vec::new();

    let field_elements = input_data.len()/FIELD_ELEMENT_SIZE_IN_BYTES;
    for i in 0..field_elements {
        let buffer = &input_data[i*FIELD_ELEMENT_SIZE_IN_BYTES..(i+1)*FIELD_ELEMENT_SIZE_IN_BYTES];
        inputs_groupped.push(buffer.to_vec());
    }
    poseidon_bytes(&inputs_groupped)
}

/// Given a binary representation of a BN254 field element as an integer in
/// little-endian encoding, this function returns the corresponding field
/// element. If the field element is not canonical (is larger than the field
/// size as an integer), an `FastCryptoError::InvalidInput` is returned.
///
/// If more than 32 bytes is given, an `FastCryptoError::InputTooLong` is
/// returned.
fn canonical_le_bytes_to_field_element(bytes: &[u8]) -> Result<Fr, ZkCryptoError> {
    match bytes.len().cmp(&FIELD_ELEMENT_SIZE_IN_BYTES) {
        Ordering::Less => Ok(Fr::from_le_bytes_mod_order(bytes)),
        Ordering::Equal => {
            let field_element = Fr::from_le_bytes_mod_order(bytes);
            // Unfortunately, there doesn't seem to be a nice way to check if a modular
            // reduction happened without doing the extra work of serializing
            // the field element again.
            let reduced_bytes = field_element.into_bigint().to_bytes_le();
            if reduced_bytes != bytes {
                return Err(InvalidInput);
            }
            Ok(field_element)
        }
        Ordering::Greater => Err(InputTooLong(bytes.len())),
    }
}

/// Convert a BN254 field element to a byte array as the little-endian
/// representation of the underlying canonical integer representation of the
/// element.
fn field_element_to_canonical_le_bytes(field_element: &Fr) -> [u8; FIELD_ELEMENT_SIZE_IN_BYTES] {
    let bytes = field_element.into_bigint().to_bytes_le();
    <[u8; FIELD_ELEMENT_SIZE_IN_BYTES]>::try_from(bytes)
        .expect("The result is guaranteed to be 32 bytes")
}

/// Convert an ff field element to an arkworks-ff field element.
fn fr_to_bn254fr(fr: crate::executor::zk_stuff::Fr) -> Fr {
    // We use big-endian as in the definition of the BN254 prime field (see
    // fastcrypto-zkp/src/lib.rs).
    Fr::from_be_bytes_mod_order(fr.to_repr().as_byte_slice())
}

/// Convert an arkworks-ff field element to an ff field element.
fn bn254_to_fr(fr: Fr) -> crate::executor::zk_stuff::Fr {
    let mut bytes = [0u8; 32];
    // We use big-endian as in the definition of the BN254 prime field (see
    // fastcrypto-zkp/src/lib.rs).
    bytes.clone_from_slice(&fr.into_bigint().to_bytes_be());
    crate::executor::zk_stuff::Fr::from_repr_vartime(FrRepr(bytes))
        .expect("The bytes of fr are guaranteed to be canonical here")
}

#[test]
fn test_poseidon_bytes_flat() {
    let input_bytes = [0u8; 32];
    let hash = poseidon_bytes_flat(&input_bytes.to_vec()).unwrap();
    println!(" bytes of Poseidon hash from zeroes = {:?}", hash);
    

    let etalon_res: Vec<u8> = hex::decode("0b63a53787021a4a962a452c2921b3663aff1ffd8d5510540f8e659e782956f1").unwrap();
    //assert(true);
    assert!(hash == etalon_res);
}
