// Copyright (C) 2026 Pruvendo (bridge integration team).
//
// Constants shared by the `ZKHALO2VERIFYWITHVK` opcode. Only the three
// KZG points the SHPLONK verifier actually consumes are embedded here —
// the full SRS blob is never needed for verification.
//
// The opcode integration tests
// (`tvm_vm/src/tests/test_halo2_with_vk.rs`) load the proof / VK /
// instances fixture from `tvm_vm/halo2_test_data/fallback_*.bin` at
// runtime; nothing test-only is embedded into this file.
//
// ## Provenance (production-grade, BN254)
//
// All three points are sourced from the **Hermez Perpetual Powers of Tau**
// ceremony output (`powersOfTau28_hez_final.ptau`, K=20 slice). That is
// the standard multi-party BN254 KZG trusted setup with 80+ public
// contributions — the same SRS used by snarkjs, iden3 and Polygon zkEVM.
//
// Conversion path: `powersOfTau28_hez_final_20.ptau` (Polygon zkEVM
// mirror, SHA-256 `159d3f93…`) → `han0110/halo2-kzg-srs` raw halo2
// canonical SRS (SHA-256 `80394564…`, `same_ratio` validated) → bytes
// extracted at the canonical offsets `[4..68]`, `[134217732..860]`,
// `[134217860..988]`.
//
// `KZG_G0_BYTES` and `KZG_G2_BYTES` are the curve generators G1/G2 —
// these match the values from any well-formed BN254 SRS (test or
// production). Only `KZG_S_G2_BYTES` (= `[s] · G2`, where `s` is the
// ceremony's shared trapdoor) is ceremony-specific.

/// G1 generator point `g[0]` from the KZG SRS, 64 bytes (BN256 G1Affine
/// uncompressed). Shared verifier-only material; together with `KZG_G2_BYTES`
/// and `KZG_S_G2_BYTES` it is enough for `verify_proof::<…, VerifierSHPLONK,
/// …>(…)` at any `k`.
///
/// Curve constant (G1 generator), identical across all BN254 KZG SRS.
/// SHA-256: `35c3fc84f9f41294e5a54237f24a94f76fcc8608689981ea2ca7152d8e394332`.
pub(crate) const KZG_G0_BYTES: [u8; 64] = [
    157, 13, 143, 197, 141, 67, 93, 211, 61, 11, 199, 245, 40, 235, 120, 10, 44, 70, 121, 120, 111,
    163, 110, 102, 47, 223, 7, 154, 193, 119, 10, 14, 58, 27, 30, 139, 27, 135, 186, 166, 123, 22,
    142, 235, 81, 214, 241, 20, 88, 140, 242, 240, 222, 70, 221, 204, 94, 190, 15, 52, 131, 239,
    20, 28,
];

/// G2 generator from the KZG SRS, 128 bytes (BN256 G2Affine uncompressed).
///
/// Curve constant (G2 generator), identical across all BN254 KZG SRS.
/// SHA-256: `4835edb12ea98bcbbbfaefe1ce63453167b3f1dc66e23fb8c7e5c4ed288966a9`.
pub(crate) const KZG_G2_BYTES: [u8; 128] = [
    38, 32, 188, 2, 209, 181, 131, 142, 114, 1, 123, 73, 53, 25, 235, 220, 223, 26, 129, 151, 71,
    38, 184, 251, 59, 80, 150, 175, 65, 56, 87, 25, 64, 97, 76, 168, 125, 115, 180, 175, 196, 216,
    2, 88, 90, 221, 67, 96, 134, 47, 160, 82, 252, 80, 233, 9, 107, 123, 234, 58, 131, 240, 254,
    20, 246, 233, 107, 136, 157, 250, 157, 97, 120, 155, 158, 245, 151, 210, 127, 254, 254, 125,
    27, 35, 98, 26, 158, 255, 6, 66, 158, 174, 235, 126, 253, 40, 238, 86, 24, 199, 86, 91, 9, 100,
    187, 60, 125, 50, 34, 249, 87, 220, 118, 16, 53, 51, 190, 53, 249, 85, 130, 100, 253, 147, 230,
    160, 164, 13,
];

/// `[s] · G2` from the KZG SRS (toxic-waste-scaled G2), 128 bytes.
///
/// **Ceremony-specific.** Sourced from the Hermez Perpetual Powers of
/// Tau ceremony (BN254, K=20 slice of `powersOfTau28_hez_final.ptau`).
/// SHA-256: `f9f0416d47fc9128e4fcac130f1ea9a0cd0c015dfe9c53a6b81476099b080979`.
pub(crate) const KZG_S_G2_BYTES: [u8; 128] = [
    146, 143, 175, 179, 208, 204, 61, 215, 22, 199, 254, 90, 54, 185, 141, 10, 219, 108, 103, 2,
    251, 231, 149, 55, 125, 126, 14, 251, 159, 31, 33, 6, 54, 216, 149, 107, 43, 132, 49, 141, 221,
    99, 155, 38, 126, 180, 50, 149, 239, 220, 11, 16, 15, 186, 198, 106, 82, 105, 151, 149, 183,
    45, 32, 9, 105, 170, 202, 180, 88, 37, 29, 4, 128, 1, 190, 150, 175, 237, 242, 120, 78, 60,
    141, 127, 0, 15, 196, 236, 102, 139, 237, 187, 190, 229, 17, 1, 45, 83, 141, 248, 233, 227,
    185, 78, 7, 105, 177, 147, 7, 15, 27, 139, 126, 59, 254, 173, 204, 133, 136, 15, 211, 86, 179,
    190, 89, 92, 105, 0,
];

