// Copyright (C) 2019-2023 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use std::borrow::Cow;

use ed25519::Signature;
use ed25519_dalek::Verifier;
use ed25519_dalek::VerifyingKey;
use tvm_block::GlobalCapabilities;
use tvm_types::BuilderData;
use tvm_types::ExceptionCode;
use tvm_types::GasConsumer;
use tvm_types::UInt256;
use tvm_types::error;

use crate::error::TvmError;
use crate::executor::engine::Engine;
use crate::executor::engine::storage::fetch_stack;
use crate::executor::types::Instruction;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::integer::serialization::UnsignedIntegerBigEndianEncoding;
use crate::types::Exception;
use crate::types::Status;

const PUBLIC_KEY_BITS: usize = PUBLIC_KEY_BYTES * 8;
const SIGNATURE_BITS: usize = SIGNATURE_BYTES * 8;
const PUBLIC_KEY_BYTES: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;
const SIGNATURE_BYTES: usize = ed25519_dalek::SIGNATURE_LENGTH;

fn hash_to_uint(bits: impl AsRef<[u8]>) -> IntegerData {
    IntegerData::from_unsigned_bytes_be(bits)
}

/// HASHCU (c – x), computes the representation hash of a Cell c
/// and returns it as a 256-bit unsigned integer x.
pub(super) fn execute_hashcu(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("HASHCU"))?;
    fetch_stack(engine, 1)?;
    let hash_int = hash_to_uint(engine.cmd.var(0).as_cell()?.repr_hash());
    engine.cc.stack.push(StackItem::integer(hash_int));
    Ok(())
}

/// Computes the hash of a Slice s and returns it as a 256-bit unsigned integer
/// x. The result is the same as if an ordinary cell containing only data
/// and references from s had been created and its hash computed by HASHCU.
pub(super) fn execute_hashsu(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("HASHSU"))?;
    fetch_stack(engine, 1)?;
    let builder = engine.cmd.var(0).as_slice()?.as_builder();
    let cell = engine.finalize_cell(builder)?;
    let hash_int = hash_to_uint(cell.repr_hash());
    engine.cc.stack.push(StackItem::integer(hash_int));
    Ok(())
}

// SHA256U ( s – x )
// Computes sha256 of the data bits of Slices.
// If the bit length of s is not divisible by eight, throws a cell underflow
// exception. The hash value is returned as a 256-bit unsigned integer x.
pub(super) fn execute_sha256u(engine: &mut Engine) -> Status {
    engine.load_instruction(Instruction::new("SHA256U"))?;
    fetch_stack(engine, 1)?;
    let slice = engine.cmd.var(0).as_slice()?;
    if slice.remaining_bits() % 8 == 0 {
        let hash = UInt256::calc_file_hash(&slice.get_bytestring(0));
        let hash_int = hash_to_uint(hash);
        engine.cc.stack.push(StackItem::integer(hash_int));
        Ok(())
    } else {
        err!(ExceptionCode::CellUnderflow)
    }
}

enum DataForSignature {
    Hash(BuilderData),
    Slice(Vec<u8>),
}

impl AsRef<[u8]> for DataForSignature {
    fn as_ref(&self) -> &[u8] {
        match self {
            DataForSignature::Hash(hash) => hash.data(),
            DataForSignature::Slice(slice) => slice.as_slice(),
        }
    }
}

fn preprocess_signed_data<'a>(_engine: &Engine, data: &'a [u8]) -> Cow<'a, [u8]> {
    #[cfg(feature = "signature_with_id")]
    if _engine.check_capabilities(GlobalCapabilities::CapSignatureWithId as u64) {
        let mut extended_data = Vec::with_capacity(4 + data.len());
        extended_data.extend_from_slice(&_engine.signature_id().to_be_bytes());
        extended_data.extend_from_slice(data);
        return Cow::Owned(extended_data);
    }
    Cow::Borrowed(data)
}

fn check_signature(engine: &mut Engine, name: &'static str, hash: bool) -> Status {
    engine.load_instruction(Instruction::new(name))?;
    fetch_stack(engine, 3)?;
    let pub_key = engine
        .cmd
        .var(0)
        .as_integer()?
        .as_builder::<UnsignedIntegerBigEndianEncoding>(PUBLIC_KEY_BITS)?;
    engine.cmd.var(1).as_slice()?;
    if hash {
        engine.cmd.var(2).as_integer()?;
    } else {
        engine.cmd.var(2).as_slice()?;
    }
    if engine.cmd.var(1).as_slice()?.remaining_bits() < SIGNATURE_BITS {
        return err!(ExceptionCode::CellUnderflow);
    }
    let data = if hash {
        DataForSignature::Hash(
            engine.cmd.var(2).as_integer()?.as_builder::<UnsignedIntegerBigEndianEncoding>(256)?,
        )
    } else {
        if engine.cmd.var(2).as_slice()?.remaining_bits() % 8 != 0 {
            return err!(ExceptionCode::CellUnderflow);
        }
        DataForSignature::Slice(engine.cmd.var(2).as_slice()?.get_bytestring(0))
    };
    let pub_key = match VerifyingKey::try_from(pub_key.data()) {
        Ok(pub_key) => pub_key,
        Err(err) => {
            if engine.check_capabilities(GlobalCapabilities::CapsTvmBugfixes2022 as u64) {
                engine.cc.stack.push(boolean!(false));
                return Ok(());
            } else {
                return err!(ExceptionCode::FatalError, "cannot load public key {}", err);
            }
        }
    };
    let signature = engine.cmd.var(1).as_slice()?.get_bytestring(0);
    let signature = match Signature::try_from(&signature[..SIGNATURE_BYTES]) {
        Ok(signature) => signature,
        Err(err) =>
        {
            #[allow(clippy::collapsible_else_if)]
            if engine.check_capabilities(GlobalCapabilities::CapsTvmBugfixes2022 as u64) {
                engine.cc.stack.push(boolean!(false));
                return Ok(());
            } else {
                if hash {
                    engine.cc.stack.push(boolean!(false));
                    return Ok(());
                } else {
                    return err!(ExceptionCode::FatalError, "cannot load signature {}", err);
                }
            }
        }
    };
    let data = preprocess_signed_data(engine, data.as_ref());
    #[cfg(feature = "signature_no_check")]
    let result =
        engine.modifiers.chksig_always_succeed || pub_key.verify(&data, &signature).is_ok();
    #[cfg(not(feature = "signature_no_check"))]
    let result = pub_key.verify(&data, &signature).is_ok();

    engine.cc.stack.push(boolean!(result));
    Ok(())
}

// CHKSIGNS (d s k – ?)
// checks whether s is a valid Ed25519-signature of the data portion of Slice d
// using public key k, similarly to CHKSIGNU. If the bit length of Slice d is
// not divisible by eight, throws a cell underflow exception. The verification
// of Ed25519 signatures is the standard one, with sha256 used to reduce d to
// the 256-bit number that is actually signed.
pub(super) fn execute_chksigns(engine: &mut Engine) -> Status {
    check_signature(engine, "CHKSIGNS", false)
}

/// CHKSIGNU (h s k – -1 or 0)
/// checks the Ed25519-signature s (slice) of a hash h (a 256-bit unsigned
/// integer) using public key k (256-bit unsigned integer).
pub(super) fn execute_chksignu(engine: &mut Engine) -> Status {
    check_signature(engine, "CHKSIGNU", true)
}
