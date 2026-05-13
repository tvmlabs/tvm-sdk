// Copyright (C) 2026 GOSH. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

//! Unit tests for the `VERGRTH16WITHVK` opcode (`0xC7 0x49`).
//!
//! These tests construct a self-contained Groth16 proof in-process — without
//! relying on any external fixture file — so that they exercise the opcode end
//! to end:
//!
//!  1. A trivial R1CS circuit with `n_public` field-element public inputs is
//!     defined (`a * b = c`, with `c` the public input).
//!  2. `ark-groth16` runs `circuit_specific_setup`, then `prove`, then locally
//!     `verify` (sanity check that the proof is well-formed before pushing it
//!     into the VM).
//!  3. VK, public inputs, and proof are canonical-compressed and pushed onto
//!     the VM stack.
//!  4. The 3-byte program `0xC7 0x49 0x80` executes the new opcode and we
//!     check the boolean result on top of the stack.
//!
//! Negative cases flip individual bits / fields and confirm that the opcode
//! returns `false` rather than throwing.

use ark_bn254::Bn254;
use ark_bn254::Fr;
use ark_ec::AffineRepr;
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use ark_ff::UniformRand;
use ark_groth16::Groth16;
use ark_relations::lc;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::ConstraintSystemRef;
use ark_relations::r1cs::SynthesisError;
use ark_serialize::CanonicalSerialize;
use ark_snark::SNARK;
use ark_std::rand::SeedableRng;
use ark_std::rand::rngs::StdRng;
use tvm_types::SliceData;

use crate::executor::engine::Engine;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::utils::pack_data_to_cell;

/// Minimal R1CS gadget: assigns `a` and `b` as witnesses and proves that the
/// public input `c` equals `a * b`.
#[derive(Clone, Copy)]
struct MulCircuit {
    a: Option<Fr>,
    b: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for MulCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.new_input_variable(|| {
            let a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(a * b)
        })?;
        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)
    }
}

/// Convenience wrapper around `ark_groth16::Groth16::<Bn254>::*` that returns
/// the canonical-compressed bytes the new opcode expects.
fn build_fixture(seed: u64) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut rng = StdRng::seed_from_u64(seed);
    let a = Fr::rand(&mut rng);
    let b = Fr::rand(&mut rng);
    let c = a * b;

    let circuit = MulCircuit { a: Some(a), b: Some(b) };
    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, &mut rng).unwrap();
    let proof = Groth16::<Bn254>::prove(&pk, circuit, &mut rng).unwrap();

    // Sanity: the proof must verify with the same toolchain we wrap in the VM.
    let pvk = Groth16::<Bn254>::process_vk(&vk).unwrap();
    assert!(Groth16::<Bn254>::verify_with_processed_vk(&pvk, &[c], &proof).unwrap());

    let mut vk_bytes = Vec::new();
    vk.serialize_compressed(&mut vk_bytes).unwrap();

    let mut pi_bytes = Vec::new();
    c.serialize_compressed(&mut pi_bytes).unwrap();

    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();

    (vk_bytes, pi_bytes, proof_bytes)
}

/// Push `(proof, public_inputs, vk)` onto a fresh stack — top is `vk` because
/// the opcode reads it from `var(0)`.
fn make_stack(vk: &[u8], pi: &[u8], proof: &[u8]) -> Stack {
    let mut stack = Stack::new();
    let proof_cell = pack_data_to_cell(proof, &mut 0).unwrap();
    stack.push(StackItem::cell(proof_cell));
    let pi_cell = pack_data_to_cell(pi, &mut 0).unwrap();
    stack.push(StackItem::cell(pi_cell));
    let vk_cell = pack_data_to_cell(vk, &mut 0).unwrap();
    stack.push(StackItem::cell(vk_cell));
    stack
}

/// Build the 3-byte program `0xC7 0x49 0x80`, set up an engine, run it, and
/// return the top stack item as a Rust `bool`.
fn run_opcode(stack: Stack) -> bool {
    let code = SliceData::new(vec![0xC7, 0x49, 0x80]);
    let mut engine =
        Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);
    engine.execute().expect("opcode threw");
    let res = engine.cc.stack.get(0).as_integer().unwrap().clone();
    // `boolean!(true)` is encoded as `IntegerData::minus_one()` and `false`
    // as `IntegerData::zero()` in the VM, so check the sign bit through
    // `is_zero` for portability.
    !res.is_zero()
}

#[test]
fn vergrth16_with_vk_accepts_valid_proof() {
    let (vk, pi, proof) = build_fixture(0xC0FFEE);
    assert!(run_opcode(make_stack(&vk, &pi, &proof)));
}

#[test]
fn vergrth16_with_vk_rejects_flipped_proof_bit() {
    let (vk, pi, mut proof) = build_fixture(0xDEADBEEF);
    // Flip a bit deep inside the proof — guaranteed to break the pairing check
    // but not the byte-format checks.
    proof[10] ^= 0x01;
    assert!(!run_opcode(make_stack(&vk, &pi, &proof)));
}

#[test]
fn vergrth16_with_vk_rejects_wrong_public_input() {
    let (vk, _pi, proof) = build_fixture(0xBADC0DE);
    // Swap in a different valid public input (c' = 1) that does not match the
    // witness used to make the proof.
    let mut wrong_pi = Vec::new();
    Fr::ONE.serialize_compressed(&mut wrong_pi).unwrap();
    assert!(!run_opcode(make_stack(&vk, &wrong_pi, &proof)));
}

#[test]
fn vergrth16_with_vk_rejects_swapped_vk() {
    let (_vk_a, pi, proof) = build_fixture(0x11111111);
    let (vk_b, _, _) = build_fixture(0x22222222);
    // VK from a different setup — must reject without panicking.
    assert!(!run_opcode(make_stack(&vk_b, &pi, &proof)));
}

#[test]
fn vergrth16_with_vk_rejects_mismatched_input_count() {
    let (vk, pi_one, proof) = build_fixture(0xFEEDFACE);
    // Append a second field element; this circuit only has one public input,
    // so the opcode must return `false` rather than crashing.
    let mut pi_two = pi_one;
    let mut extra = Vec::new();
    Fr::ONE.serialize_compressed(&mut extra).unwrap();
    pi_two.extend_from_slice(&extra);
    assert!(!run_opcode(make_stack(&vk, &pi_two, &proof)));
}

/// Defensive smoke check that BN254 pairing is well-defined in this build, so
/// that a failure of the opcode tests can be attributed to opcode wiring
/// rather than to a broken curve dependency.
#[test]
fn bn254_pairing_sanity() {
    use ark_bn254::G1Affine;
    use ark_bn254::G2Affine;
    let p1: G1Affine = G1Affine::generator();
    let p2: G2Affine = G2Affine::generator();
    let _ = Bn254::pairing(p1, p2);
}
