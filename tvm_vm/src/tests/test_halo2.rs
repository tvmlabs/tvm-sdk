use std::collections::HashMap;
use std::env;
use std::io::Cursor;
use std::time::Instant;

use gosh_dark_dex_halo2_circuit::circuit::*;
use gosh_dark_dex_halo2_circuit::poseidon::*;
use gosh_dark_dex_halo2_circuit::proof::*;
use gosh_dark_dex_halo2_circuit::snark_utils::*;
use halo2_base::halo2_proofs::arithmetic::CurveAffine;
use halo2_base::halo2_proofs::halo2curves::bn256::Fr;
use halo2_base::halo2_proofs::halo2curves::secp256k1::Fp;
use halo2_base::halo2_proofs::halo2curves::secp256k1::Fq;
use halo2_base::halo2_proofs::halo2curves::secp256k1::Secp256k1Affine;
use halo2_base::halo2_proofs::plonk::Fixed;
use tvm_types::Cell;
use tvm_types::SliceData;

use crate::executor::engine::Engine;
use crate::executor::test_helper::*;
use crate::executor::zk_halo2::execute_halo2_proof_verification;
use crate::stack::Stack;
use crate::stack::StackItem;
use crate::stack::integer::IntegerData;
use crate::stack::savelist::SaveList;
use crate::utils::pack_data_to_cell;
use crate::utils::pack_string_to_cell;
use crate::utils::unpack_data_from_cell;

const KZG_PARAMS_PATH: &str = "halo2_test_data/kzg_bn254_12.srs";
const PROOF_KEY_PATH: &str = "halo2_test_data/proof_key.bin";
const BREAK_POINTS_PATH: &str = "halo2_test_data/break_points.bin";
const CONFIG_PARAMS_PATH: &str = "halo2_test_data/config_params.bin";

#[test]
fn test() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );

    let k = 12u32;
    let unusable_rows = 9;
    let token_type = 1u64;
    let private_note_sum = 1000u64;
    // let params = gen_srs(k);
    let sk_u_ = Fr::from(23u64);
    let token_type_ = Fr::from(1u64);
    let private_note_sum_ = Fr::from(1000u64);

    let sk_u_commitment = poseidon_hash(&[sk_u_, Fr::zero()]);
    let data_to_hash = [sk_u_commitment, private_note_sum_, token_type_, sk_u_];
    let digest = poseidon_hash(&data_to_hash);
    let digest: [u8; 32] = digest.to_bytes();
    let digest_hex = hex::encode(&digest);

    println!("digest here here: {:?}", digest.clone());
    println!("digest_hex: {:?}", digest_hex);

    let mut pub_inputs_bytes: Vec<u8> = Vec::new();

    pub_inputs_bytes.append(&mut vec![0u8; 24]);
    pub_inputs_bytes.append(&mut private_note_sum.to_be_bytes().to_vec());

    pub_inputs_bytes.append(&mut vec![0u8; 24]);
    pub_inputs_bytes.append(&mut token_type.to_be_bytes().to_vec());

    pub_inputs_bytes.append(&mut digest.to_vec());

    println!("pub_inputs_bytes: {:?}", pub_inputs_bytes);

    let pub_inputs_cell = pack_data_to_cell(&pub_inputs_bytes.clone(), &mut 0).unwrap();

    engine.cc.stack.push(StackItem::cell(pub_inputs_cell.clone()));

    let params = read_kzg_params(KZG_PARAMS_PATH.to_string());
    let proof = generate_dark_dex_proof(
        k,
        unusable_rows,
        &params,
        token_type_,
        private_note_sum_,
        sk_u_,
        sk_u_commitment,
        BREAK_POINTS_PATH.to_string(),
        CONFIG_PARAMS_PATH.to_string(),
        PROOF_KEY_PATH.to_string(),
    )
    .unwrap();

    let proof_cell = pack_data_to_cell(&proof.clone().0, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let start: Instant = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_micros();

    println!("elapsed in microsecond: {:?}", elapsed);

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("res: {:?}", res);
    assert!(res == true);
}

#[test]
fn test_negative() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );

    let k = 12u32;
    let unusable_rows = 9;
    // let params = gen_srs(k);

    let sk_u: u64 = 23;
    let token_type: u64 = 1;
    let token_type_wrong: u64 = 2;
    let private_note_sum: u64 = 1000;

    let sk_u_ = Fr::from(sk_u);
    let token_type_ = Fr::from(token_type);
    let private_note_sum_ = Fr::from(private_note_sum);

    let sk_u_commitment = poseidon_hash(&[sk_u_, Fr::zero()]);
    let data_to_hash = [sk_u_commitment, private_note_sum_, token_type_, sk_u_];
    let digest = poseidon_hash(&data_to_hash);
    let digest: [u8; 32] = digest.to_bytes();
    let digest_hex = hex::encode(&digest);

    println!("digest here here: {:?}", digest.clone());
    println!("digest_hex: {:?}", digest_hex);

    let mut pub_inputs_bytes: Vec<u8> = Vec::new();

    pub_inputs_bytes.append(&mut vec![0u8; 24]);
    pub_inputs_bytes.append(&mut private_note_sum.to_be_bytes().to_vec());

    pub_inputs_bytes.append(&mut vec![0u8; 24]);
    pub_inputs_bytes.append(&mut token_type_wrong.to_be_bytes().to_vec());

    pub_inputs_bytes.append(&mut digest.to_vec());

    println!("pub_inputs_bytes: {:?}", pub_inputs_bytes);

    let pub_inputs_cell = pack_data_to_cell(&pub_inputs_bytes.clone(), &mut 0).unwrap();

    engine.cc.stack.push(StackItem::cell(pub_inputs_cell.clone()));

    let params = read_kzg_params(KZG_PARAMS_PATH.to_string());
    let proof = generate_dark_dex_proof(
        k,
        unusable_rows,
        &params,
        token_type_,
        private_note_sum_,
        sk_u_,
        sk_u_commitment,
        BREAK_POINTS_PATH.to_string(),
        CONFIG_PARAMS_PATH.to_string(),
        PROOF_KEY_PATH.to_string(),
    )
    .unwrap();

    let proof_cell = pack_data_to_cell(&proof.clone().0, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let start: Instant = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_micros();

    println!("elapsed in microsecond: {:?}", elapsed);

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("res: {:?}", res);
    assert!(res == false);
}

#[test]
fn test_bad_proof() {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(elector_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1633458077),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::slice(
            SliceData::from_string(
                "9fe0000000000000000000000000000000000000000000000000000000000000001_",
            )
            .unwrap(),
        ),
        StackItem::cell(config_data.reference(0).unwrap()),
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();
    let stack = Stack::new();
    let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
        SliceData::load_cell_ref(&elector_code).unwrap(),
        Some(ctrls.clone()),
        Some(stack.clone()),
        None,
        vec![],
    );

    let k = 12u32;
    let unusable_rows = 9;
    let token_type = 1u64;
    let private_note_sum = 1000u64;
    // let params = gen_srs(k);
    let sk_u_ = Fr::from(23u64);
    let token_type_ = Fr::from(1u64);
    let private_note_sum_ = Fr::from(1000u64);

    let sk_u_commitment = poseidon_hash(&[sk_u_, Fr::zero()]);
    let data_to_hash = [sk_u_commitment, private_note_sum_, token_type_, sk_u_];
    let digest = poseidon_hash(&data_to_hash);
    let digest: [u8; 32] = digest.to_bytes();
    let digest_hex = hex::encode(&digest);

    println!("digest here here: {:?}", digest.clone());
    println!("digest_hex: {:?}", digest_hex);

    let mut pub_inputs_bytes: Vec<u8> = Vec::new();

    pub_inputs_bytes.append(&mut vec![0u8; 24]);
    pub_inputs_bytes.append(&mut private_note_sum.to_be_bytes().to_vec());

    pub_inputs_bytes.append(&mut vec![0u8; 24]);
    pub_inputs_bytes.append(&mut token_type.to_be_bytes().to_vec());

    pub_inputs_bytes.append(&mut digest.to_vec());

    println!("pub_inputs_bytes: {:?}", pub_inputs_bytes);

    let pub_inputs_cell = pack_data_to_cell(&pub_inputs_bytes.clone(), &mut 0).unwrap();

    engine.cc.stack.push(StackItem::cell(pub_inputs_cell.clone()));

    let params = read_kzg_params(KZG_PARAMS_PATH.to_string());

    let bad_proof: Vec<u8> = hex::decode("ec7ea4642ef84a6d45cbbe4d8f45bec4eb29941d802f961db22c9ca4184728023cdb7dbddb6c053fda7ce2c89779e3825efe4c2a42ac8847bafdc1f848bf390ebc890a541c0b917cbb1e4bf0fa41d4f0e0cb9c4dca05075f5632e0dec099001979bf39880c3b8b4d8a931ac7edf5974cefffb3ead79f2e79c80255f9b839ab58f66caf8e80b0978d7a00e24d3780432503305f4278812784feb5326f3413221968ad8bcacf1b982f070cff939d5ea73affcfc89b434deaa7f3c47d8672b5b00d3561a42e891c9eff1cc0052d56f870ac20ed891fbab53f171e1a7c6b4546200df8955203385c905dd2ff26b1753684e2813cd6eac137e54b8dba13c93be7aa2c5452330b460ae7cb6094f344c89e6b4ef0d09c34fdad58f78d85dedc8bd2102f90b71eba55e389b1f6d60fba9607db38f628f46da38283805bccfe1fea31cf560100000000000000000000000000000000000000000000000000000000000000a0d7fcbc7d4b108fd65f43f6843bdf5da355f1547abe301f587ad8aea305450e5fb36ecdda00b684ea15abee6edbde65ed933b1ed41c4f5c9e7430a0670cc41a35e846f749473158be51f5bbdf26d9e99ef8ad16e312a42e0d62d115f8b830190602ada53419e486de026ea3b9487dc4487c567c749078088a6ad34827c68e5371236a2e20802065343b77357dc46a0430766e83c7186b1e01da3d4d7a493925544a7c491b77374902b4876d42e46c9f3145cf9227d5b30eb55fd8363a61c1071c216baed8090ce2a6128eed2589f772faefc084cbb2467b169c27eddef1442427e914fd7a75cbf4667c8ae9279336de512d7d7a2b87ad227677c8df086a61224de40087e77ffddc92229d6850c5be15405454539c62f09dc683891d14fb6209eab58e8af79272e33223774396c1060def11839660067f378dddbd56d9fb162b2e731089556a083072f17a793075e33b908133b1a256a7636cc49ae7359d8606c5322d1248c6e11523e56b967d5ffb9315478f259cf953c8a923cb5924f84f28ba51235cfdf67a825327bd904a30bc335450df3f4a23a53c3e192daa929e4a0a97cb62e2c302b470eda861de87a00e93672a7b6e5cfd6f5023e9306af2c526285adee045cc7cbbfa26de24de38691865c0d590bc49f0ae2e38aa507e32c2650b7e43b14df86c27bbce9f9513448ce0c05a29d7c299c3f5f4dcc5b7c0faa1061ff50373c9821c3cdc313759a5b7fdb125faea76cf635930f7b76c9a1eb90f0b249bb8729502a36b436e689dbe498b3d2a21d3d817df7861e61f222c250caae9008b5d7aca75fddbe8ad2dd8eb88909cd5cb840022e9d6a952287ceca5751ceb190dce306a125aede0a4fe5f5e07f467377f436952885b6b7393f7a8ec2268a02e488645abb007cb25b0754e5a8e99c7bc7d2c336e67b10a6eda95cc182b6c6a146103f2a5de6d2d196282d0a3f56dfa21bff12d4d62ea0314d680448ac787460f1613803937ba7f08c6f167f2e29323126eb5e06b4df921afa23439ca2a9c621049bf1ad00696778f4f96bc922b12fa5446707744d1b5ac014b1f5b525512092d33c3169b9acccdf94776c52990c08f2a5fc87e2252e6b09b3f8645f391807e13e929c45bd99f5ff43ad9414fd8aaa61f536bf38fd0895e822f7550cd4c04750f5777d45ab5ee0340fdd560de069f938904bd9f78e481c39f80a1570f5d41ea092aed92ae1151e69ce6f8398d590bddfa0e1f19aae207a4b83e86985f7f6ba42c0100000000000000000000000000000000000000000000000000000000000000bfd60e8dab4f5104fe0737738427f1df47c10b9815fa02feb42afb9d300fd20593d530b639c61adcc8e0808e4237c0f944d1136a5adb9e8e68d97f0a15e3b128b0abe780198f5ca28b618f1ec3727229b91fbfa9a65885360b0d352322f2ba02bab000657985afc92b9e5e20d4315746a23f49b38264d18734447eb3a0347d03b9cf4d0b942a91e97d9d64fa29fead022f7754110fa835db254aa1d39fb63719b5862c929e664d05857d08eefd8069f9308fd3de379cb9a7853b17b100d90b226c4e058d3c465b40091f87d33daaa54a7840a6e7e525a8c968e46aae4084d72db041ffb8146e62605e42059b8282fb1473db6178269e0238016b421aeb040a1c241a11f702936953a50eeed01a8d2e426ce666818e6c237edcbdd0f796a0c7106d664c7fe4bd20c0b37ac1b3fa4d1cd35cc6db1be0e78330f017dfbcf9ee3c28221bf454bd56ffcfaeb2bf89fb3cc1a2ae8f0eaa1110c1f332c613ad0bd0ba11498fdea427616bb6edec3eaf98639d5a2480f23c4b9e353d22ecf0cb7b64bd1aaef941bf1162fe21b02108fa58e7b0aa5cd2ab47674059248b81dc53a902b30b34cb8ed8b76ff56b96c7bec08557f5f969ff39dfbc24c854763c4787507f120e3c45bafcdb593b0a5efaeae2ba253ad38131abf383a90efd73d16bd38e67551dc04a4c344bd21bdd0e9cda4a8b478ac527526749bc3207cb64aba7846b53732ff876e30f504e0e41428e26536b596e838ae05a30645bc7c25f83d84c7e7e0a00f2a6ea94bebecb812abeb0a58c65832943229bc7c021b3f6d83139e482975412cc25326659ff7e1b0c0abd8482000c4328c96d9737eb671dcbfc0d59e47d83273db6c70ff9c3f50146ce715277e1c53ea8d271416d60dc0846d54dfbb8d89902304b6bb9945543f2a71aa76c50dcb3a7336f029299899307d978b20223cdd868f7421e8e94920d234dfa787db5405e3cb08d5167a3fa1965e222e073d76c6d0a".to_string()).unwrap();

    let proof_cell = pack_data_to_cell(/* &proof.clone().0 */ &bad_proof, &mut 0).unwrap();
    engine.cc.stack.push(StackItem::cell(proof_cell.clone()));

    let start: Instant = Instant::now();
    let _ = execute_halo2_proof_verification(&mut engine).unwrap();
    let elapsed = start.elapsed().as_micros();

    println!("elapsed in microsecond: {:?}", elapsed);

    let res = engine.cc.stack.get(0).as_bool().unwrap();
    println!("res: {:?}", res);
    assert!(res == false);
}
