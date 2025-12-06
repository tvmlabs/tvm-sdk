// Copyright (C) 2021-2023 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.
use std::collections::HashMap;
use std::time::Duration;

use base64ct::Encoding;
use criterion::Criterion;
use criterion::SamplingMode;
use criterion::criterion_group;
use criterion::criterion_main;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::KeyPair;
use fastcrypto::traits::ToFromBytes;
use pprof::criterion::Output;
use pprof::criterion::PProfProfiler;
use tvm_abi::TokenValue;
use tvm_abi::contract::ABI_VERSION_2_4;
use tvm_block::Deserializable;
use tvm_block::StateInit;
use tvm_types::SliceData;
use tvm_vm::executor::Engine;
use tvm_vm::executor::zk_stuff::error::ZkCryptoError;
use tvm_vm::executor::zk_stuff::utils::gen_address_seed;
use tvm_vm::executor::zk_stuff::zk_login::CanonicalSerialize;
use tvm_vm::executor::zk_stuff::zk_login::JWK;
use tvm_vm::executor::zk_stuff::zk_login::JwkId;
use tvm_vm::executor::zk_stuff::zk_login::OIDCProvider;
use tvm_vm::executor::zk_stuff::zk_login::ZkLoginInputs;
use tvm_vm::stack::Stack;
use tvm_vm::stack::StackItem;
use tvm_vm::stack::continuation::ContinuationData;
use tvm_vm::stack::savelist::SaveList;
use tvm_vm::utils::pack_data_to_cell;

static DEFAULT_CAPABILITIES: u64 = 0x572e;

fn read_boc(filename: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut file = std::fs::File::open(filename).unwrap();
    std::io::Read::read_to_end(&mut file, &mut bytes).unwrap();
    bytes
}

fn load_boc(filename: &str) -> tvm_types::Cell {
    let bytes = read_boc(filename);
    tvm_types::read_single_root_boc(bytes).unwrap()
}

fn load_stateinit(filename: &str) -> StateInit {
    StateInit::construct_from_file(filename).unwrap()
}

fn bench_elector_algo_1000_vtors(c: &mut Criterion) {
    let elector_code = load_boc("benches/elector-code.boc");
    let elector_data = load_boc("benches/elector-data.boc");
    let config_data = load_boc("benches/config-data.boc");

    let elector_data_output = load_boc("benches/elector-data-output.boc");
    let elector_actions = load_boc("benches/elector-actions.boc");

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

    let mut stack = Stack::new();
    stack.push(StackItem::int(1000000000));
    stack.push(StackItem::int(0));
    stack.push(StackItem::int(0));
    stack.push(StackItem::int(-2));

    let mut group = c.benchmark_group("flat-sampling");
    group.measurement_time(Duration::from_secs(10));
    group.noise_threshold(0.03);
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("elector-algo-1000-vtors", |b| {
        b.iter(|| {
            let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
                SliceData::load_cell_ref(&elector_code).unwrap(),
                Some(ctrls.clone()),
                Some(stack.clone()),
                None,
                vec![],
            );
            engine.execute().unwrap();
            assert_eq!(engine.gas_used(), 82386791);
            let output = engine.ctrl(4).unwrap().as_cell().unwrap();
            assert_eq!(output, &elector_data_output);
            let actions = engine.ctrl(5).unwrap().as_cell().unwrap();
            assert_eq!(actions, &elector_actions);
        })
    });
    group.finish();
}

fn bench_tiny_loop_200000_iters(c: &mut Criterion) {
    let tiny_code = load_boc("benches/tiny-code.boc");
    let tiny_data = load_boc("benches/tiny-data.boc");

    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::Cell(tiny_data)).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::default(),
        StackItem::None,
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let mut stack = Stack::new();
    stack.push(StackItem::int(1000000000));
    stack.push(StackItem::int(0));
    stack.push(StackItem::int(0));
    stack.push(StackItem::int(-2));

    let mut group = c.benchmark_group("flat-sampling");
    group.measurement_time(Duration::from_secs(10));
    group.noise_threshold(0.03);
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("tiny-loop-200000-iters", |b| {
        b.iter(|| {
            let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
                SliceData::load_cell_ref(&tiny_code).unwrap(),
                Some(ctrls.clone()),
                Some(stack.clone()),
                None,
                vec![],
            );
            engine.execute().unwrap();
            assert_eq!(engine.gas_used(), 34000891);
            // result of computation gets verified within the test itself
        })
    });
}

fn bench_num_bigint(c: &mut Criterion) {
    c.bench_function("num-bigint", |b| {
        b.iter(|| {
            let n = num::BigInt::from(1000000);
            let mut accum = num::BigInt::from(0);
            let mut iter = num::BigInt::from(0);
            loop {
                if iter >= n {
                    break;
                }
                accum += num::BigInt::from(iter.bits());
                iter += 1;
            }
            assert_eq!(num::BigInt::from(18951425), accum);
        })
    });
}

// Note: the gmp-mpfr-based rug crate shows almost the same perf as num-bigint

// fn bench_rug_bigint(c: &mut Criterion) {
//     c.bench_function("rug-bigint", |b| b.iter( || {
//         let n = rug::Integer::from(1000000);
//         let mut accum = rug::Integer::from(0);
//         let mut iter = rug::Integer::from(0);
//         loop {
//             if !(iter < n) {
//                 break;
//             }
//             accum += rug::Integer::from(iter.significant_bits());
//             iter += 1;
//         }
//         assert_eq!(rug::Integer::from(18951425), accum);
//     }));
// }

fn bench_load_boc(c: &mut Criterion) {
    let bytes = read_boc("benches/elector-data.boc");
    c.bench_function("load-boc", |b| {
        b.iter(|| tvm_types::read_single_root_boc(bytes.clone()).unwrap())
    });
}

const MAX_TUPLE_SIZE: usize = 255;

// array = [row1, row2, ...], array.len() <= MAX_TUPLE_SIZE
// row_i = [v1, v2, ...], row_i.len() <= row_size

fn make_array(input: &[i64], row_size: usize) -> StackItem {
    assert!(0 < row_size && row_size <= MAX_TUPLE_SIZE);
    assert!(input.len() <= row_size * MAX_TUPLE_SIZE);
    let mut row = Vec::new();
    let mut rows = Vec::new();
    for (i, &item) in input.iter().enumerate() {
        row.push(StackItem::int(item));
        if (i + 1) % row_size == 0 {
            assert_eq!(row.len(), row_size);
            rows.push(StackItem::tuple(row));
            row = Vec::new();
        }
    }
    if !row.is_empty() {
        rows.push(StackItem::tuple(row));
    }
    StackItem::tuple(rows)
}

fn bench_mergesort_tuple(c: &mut Criterion) {
    let code = load_boc("benches/mergesort/mergesort.boc");
    let code_slice = SliceData::load_cell_ref(&code).unwrap();

    const ROW_SIZE: usize = 32; // size() function in the code
    const COUNT: usize = 1000; // total elements count

    let mut input = Vec::with_capacity(COUNT);
    for i in 0..COUNT {
        input.push((COUNT - i - 1) as i64);
    }
    let array = make_array(&input, ROW_SIZE);
    input.sort();
    let expected = make_array(&input, ROW_SIZE);

    // runvmx mode: +1 = same_c3
    let mut ctrls = SaveList::default();
    ctrls
        .put(3, &mut StackItem::continuation(ContinuationData::with_code(code_slice.clone())))
        .unwrap();

    let mut stack = Stack::new();
    stack.push(StackItem::int(-1));
    stack.push(array);
    // runvmx mode: +2 = push_0
    stack.push(StackItem::int(0));

    c.bench_function("mergesort-tuple", |b| {
        b.iter(|| {
            let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
                code_slice.clone(),
                Some(ctrls.clone()),
                Some(stack.clone()),
                None,
                vec![],
            );
            engine.execute().unwrap();
            assert_eq!(engine.gas_used(), 51_216_096);
            assert_eq!(engine.stack().depth(), 1);
            assert_eq!(engine.stack().get(0), &expected);
        })
    });
}

fn bench_massive_cell_upload(c: &mut Criterion) {
    let stateinit = load_stateinit("benches/massive/cell-upload.tvc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::cell(stateinit.data().unwrap().clone())).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1678299227),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::default(),
        StackItem::None,
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let msg = load_boc("benches/massive/cell-upload-msg.boc");
    let mut body = SliceData::load_cell_ref(&msg).unwrap();
    body.move_by(366).unwrap();

    let mut stack = Stack::new();
    stack.push(StackItem::int(1000000000));
    stack.push(StackItem::int(0));
    stack.push(StackItem::cell(msg));
    stack.push(StackItem::slice(body));
    stack.push(StackItem::int(-1));

    c.bench_function("massive-cell-upload", |b| {
        b.iter(|| {
            let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
                SliceData::load_cell_ref(stateinit.code().unwrap()).unwrap(),
                Some(ctrls.clone()),
                Some(stack.clone()),
                None,
                vec![],
            );
            engine.execute().unwrap();
            assert_eq!(engine.gas_used(), 5479);
        })
    });
}

fn bench_massive_cell_finalize(c: &mut Criterion) {
    let stateinit = load_stateinit("benches/massive/cell-finalize.tvc");
    let mut ctrls = SaveList::default();
    ctrls.put(4, &mut StackItem::cell(stateinit.data().unwrap().clone())).unwrap();
    let params = vec![
        StackItem::int(0x76ef1ea),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(1678296619),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::int(0),
        StackItem::tuple(vec![StackItem::int(1000000000), StackItem::None]),
        StackItem::default(),
        StackItem::None,
        StackItem::None,
        StackItem::int(0),
    ];
    ctrls.put(7, &mut StackItem::tuple(vec![StackItem::tuple(params)])).unwrap();

    let msg = load_boc("benches/massive/cell-finalize-msg.boc");
    let mut body = SliceData::load_cell_ref(&msg).unwrap();
    body.move_by(366).unwrap();

    let mut stack = Stack::new();
    stack.push(StackItem::int(1000000000));
    stack.push(StackItem::int(0));
    stack.push(StackItem::cell(msg));
    stack.push(StackItem::slice(body));
    stack.push(StackItem::int(-1));

    c.bench_function("massive-cell-finalize", |b| {
        b.iter(|| {
            let mut engine = Engine::with_capabilities(DEFAULT_CAPABILITIES).setup_with_libraries(
                SliceData::load_cell_ref(stateinit.code().unwrap()).unwrap(),
                Some(ctrls.clone()),
                Some(stack.clone()),
                None,
                vec![],
            );
            engine.execute().unwrap();
            assert_eq!(engine.gas_used(), 203585);
        })
    });
}

// Run  `cargo bench -p tvm_vm --bench benchmarks wasmadd`
fn bench_wasmadd(c: &mut Criterion) {
    c.bench_function("wasmadd", |b| {
        let mut total_duration = Duration::default();
        b.iter_custom(|iters| {
            // b.iter(|| {
            //+++++ Let's say I want to measure execution time from this point.

            for _i in 0..iters {
                let mut stack = Stack::new();

                let hash_str = "7b7f96a857a4ada292d7c6b1f47940dde33112a2c2bc15b577dff9790edaeef2";
                let hash: Vec<u8> = (0..hash_str.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
                    .collect::<Vec<u8>>();
                let cell = tvm_abi::TokenValue::write_bytes(
                    hash.as_slice(),
                    &tvm_abi::contract::ABI_VERSION_2_4,
                )
                .unwrap()
                .into_cell()
                .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let cell =
                    tvm_abi::TokenValue::write_bytes(&[4u8], &tvm_abi::contract::ABI_VERSION_2_4)
                        .unwrap()
                        .into_cell()
                        .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let cell =
                    tvm_abi::TokenValue::write_bytes(&[3u8], &tvm_abi::contract::ABI_VERSION_2_4)
                        .unwrap()
                        .into_cell()
                        .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let cell =
                    tvm_abi::TokenValue::write_bytes(&[2u8], &tvm_abi::contract::ABI_VERSION_2_4)
                        .unwrap()
                        .into_cell()
                        .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let cell =
                    tvm_abi::TokenValue::write_bytes(&[1u8], &tvm_abi::contract::ABI_VERSION_2_4)
                        .unwrap()
                        .into_cell()
                        .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                // Push args, func name, instance name, then wasm.
                let wasm_func = "add";
                let cell = tvm_vm::utils::pack_data_to_cell(&wasm_func.as_bytes(), &mut 0).unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let wasm_func = "docs:adder/add-interface@0.1.0";
                let cell = tvm_vm::utils::pack_data_to_cell(&wasm_func.as_bytes(), &mut 0).unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let wasm_dict = Vec::<u8>::new();

                let cell = tvm_abi::TokenValue::write_bytes(
                    &wasm_dict.as_slice(),
                    &tvm_abi::contract::ABI_VERSION_2_4,
                )
                .unwrap()
                .into_cell()
                .unwrap();
                // let cell = split_to_chain_of_cells(wasm_dict);
                // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let mut res = Vec::<u8>::with_capacity(3);
                res.push(0xC7);
                res.push(0x3A);
                res.push(0x80);

                let code = SliceData::new(res);

                let mut engine = Engine::with_capabilities(0).setup_with_libraries(
                    code,
                    None,
                    Some(stack),
                    None,
                    vec![],
                );
                engine.wasm_engine_init_cached().unwrap();
                engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned()).unwrap();
                let mut engine = engine.precompile_all_wasm_by_hash().unwrap();

                let start = std::time::Instant::now();
                let _ = engine.execute();
                total_duration += start.elapsed();
            }
            total_duration

            // println!("ress: {:?}", hex::encode(ress));
        })
    });
}

// Run  `cargo bench -p tvm_vm --bench benchmarks wasmadd`
fn bench_wasmadd_no_precompile(c: &mut Criterion) {
    c.bench_function("wasmadd", |b| {
        let mut total_duration = Duration::default();
        b.iter_custom(|iters| {
            // b.iter(|| {
            //+++++ Let's say I want to measure execution time from this point.

            for _i in 0..iters {
                let mut stack = Stack::new();

                let hash_str = "7b7f96a857a4ada292d7c6b1f47940dde33112a2c2bc15b577dff9790edaeef2";
                let hash: Vec<u8> = (0..hash_str.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
                    .collect::<Vec<u8>>();
                let cell = tvm_abi::TokenValue::write_bytes(
                    hash.as_slice(),
                    &tvm_abi::contract::ABI_VERSION_2_4,
                )
                .unwrap()
                .into_cell()
                .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let cell =
                    tvm_abi::TokenValue::write_bytes(&[4u8], &tvm_abi::contract::ABI_VERSION_2_4)
                        .unwrap()
                        .into_cell()
                        .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let cell =
                    tvm_abi::TokenValue::write_bytes(&[3u8], &tvm_abi::contract::ABI_VERSION_2_4)
                        .unwrap()
                        .into_cell()
                        .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let cell =
                    tvm_abi::TokenValue::write_bytes(&[2u8], &tvm_abi::contract::ABI_VERSION_2_4)
                        .unwrap()
                        .into_cell()
                        .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let cell =
                    tvm_abi::TokenValue::write_bytes(&[1u8], &tvm_abi::contract::ABI_VERSION_2_4)
                        .unwrap()
                        .into_cell()
                        .unwrap();
                stack.push(StackItem::cell(cell.clone()));
                // Push args, func name, instance name, then wasm.
                let wasm_func = "add";
                let cell = tvm_vm::utils::pack_data_to_cell(&wasm_func.as_bytes(), &mut 0).unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let wasm_func = "docs:adder/add-interface@0.1.0";
                let cell = tvm_vm::utils::pack_data_to_cell(&wasm_func.as_bytes(), &mut 0).unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let wasm_dict = Vec::<u8>::new();

                let cell = tvm_abi::TokenValue::write_bytes(
                    &wasm_dict.as_slice(),
                    &tvm_abi::contract::ABI_VERSION_2_4,
                )
                .unwrap()
                .into_cell()
                .unwrap();
                // let cell = split_to_chain_of_cells(wasm_dict);
                // let cell = pack_data_to_cell(&wasm_dict, &mut engine).unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let mut res = Vec::<u8>::with_capacity(3);
                res.push(0xC7);
                res.push(0x3A);
                res.push(0x80);

                let code = SliceData::new(res);

                let mut engine = Engine::with_capabilities(0).setup_with_libraries(
                    code,
                    None,
                    Some(stack),
                    None,
                    vec![],
                );
                engine.wasm_engine_init_cached().unwrap();
                engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned()).unwrap();
                // let mut engine = engine.precompile_all_wasm_by_hash().unwrap();

                let start = std::time::Instant::now();
                let _ = engine.execute();
                total_duration += start.elapsed();
            }
            total_duration

            // println!("ress: {:?}", hex::encode(ress));
        })
    });
}

// Run  `cargo bench -p tvm_vm --bench benchmarks wasmtls_without_whitelist`
fn bench_wasmtls_without_whitelist(c: &mut Criterion) {
    c.bench_function("wasmtls_without_whitelist", |b| {
        let mut total_duration = Duration::default();
        b.iter_custom(|iters| {
            // b.iter(|| {
            //+++++ Let's say I want to measure execution time from this point.
            for _i in 0..iters {
                let mut stack = Stack::new();

                let hash_str = "9d8beddc8d81853d6ee2390ed141b69d67bf683afbbf45cf405848a78ae969fc";
                let hash: Vec<u8> = (0..hash_str.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
                    .collect::<Vec<u8>>();
                let cell =
                    TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let tls_data: Vec<u8> = vec![231, 226, 189, 128, 175, 192, 46, 233, 160, 243, 227, 168, 186, 174, 207, 111, 124, 21, 6, 220, 18, 155, 18, 17, 39, 165, 203, 108, 109, 3, 40, 186, 1, 3, 22, 3, 1, 0, 158, 1, 0, 0, 154, 3, 3, 209, 131, 116, 114, 224, 15, 205, 152, 242, 176, 127, 8, 86, 5, 221, 80, 162, 132, 201, 168, 99, 134, 196, 185, 88, 35, 234, 76, 112, 108, 224, 149, 0, 0, 2, 19, 1, 1, 0, 0, 111, 0, 0, 0, 20, 0, 18, 0, 0, 15, 107, 97, 117, 116, 104, 46, 107, 97, 107, 97, 111, 46, 99, 111, 109, 0, 10, 0, 4, 0, 2, 0, 29, 0, 13, 0, 20, 0, 18, 4, 3, 8, 4, 4, 1, 5, 3, 8, 5, 5, 1, 8, 6, 6, 1, 2, 1, 0, 51, 0, 38, 0, 36, 0, 29, 0, 32, 192, 66, 56, 95, 6, 86, 129, 217, 28, 232, 5, 177, 109, 189, 139, 154, 6, 3, 215, 62, 202, 195, 214, 238, 231, 82, 157, 198, 107, 200, 81, 16, 0, 45, 0, 2, 1, 1, 0, 43, 0, 3, 2, 3, 4, 22, 3, 3, 0, 90, 2, 0, 0, 86, 3, 3, 243, 248, 14, 224, 190, 21, 30, 172, 110, 1, 251, 43, 1, 162, 190, 249, 110, 32, 65, 224, 72, 202, 34, 161, 37, 163, 24, 186, 18, 240, 11, 154, 0, 19, 1, 0, 0, 46, 0, 43, 0, 2, 3, 4, 0, 51, 0, 36, 0, 29, 0, 32, 224, 217, 73, 158, 17, 12, 43, 19, 230, 133, 14, 153, 249, 104, 67, 55, 131, 8, 74, 104, 200, 163, 38, 63, 103, 114, 35, 122, 111, 43, 109, 111, 23, 3, 3, 15, 223, 59, 241, 156, 165, 95, 67, 151, 76, 79, 170, 168, 136, 45, 166, 254, 155, 235, 77, 95, 64, 53, 87, 66, 69, 123, 104, 24, 59, 34, 224, 81, 108, 143, 124, 243, 51, 118, 127, 237, 78, 223, 181, 146, 178, 61, 124, 226, 194, 21, 156, 243, 194, 46, 51, 91, 205, 50, 30, 121, 33, 9, 16, 214, 15, 204, 31, 247, 213, 188, 200, 196, 12, 170, 247, 6, 229, 99, 8, 238, 144, 46, 59, 165, 29, 152, 5, 37, 186, 82, 159, 248, 96, 197, 196, 218, 115, 237, 19, 98, 19, 194, 108, 165, 199, 190, 28, 203, 4, 80, 49, 8, 154, 108, 205, 57, 209, 204, 109, 139, 134, 89, 243, 130, 61, 66, 116, 201, 177, 23, 132, 40, 188, 247, 45, 45, 139, 138, 34, 69, 241, 134, 248, 202, 188, 35, 18, 1, 221, 237, 204, 185, 42, 62, 190, 174, 63, 63, 57, 247, 218, 23, 244, 5, 80, 70, 218, 44, 244, 45, 116, 12, 144, 166, 173, 155, 6, 76, 112, 200, 133, 38, 174, 114, 92, 208, 233, 214, 134, 5, 223, 93, 17, 164, 75, 70, 14, 111, 241, 115, 253, 114, 61, 88, 15, 20, 59, 89, 33, 188, 150, 100, 173, 199, 27, 125, 253, 5, 198, 244, 142, 89, 145, 20, 141, 170, 2, 193, 29, 181, 149, 116, 12, 96, 21, 147, 154, 156, 147, 47, 16, 33, 59, 75, 180, 235, 28, 221, 229, 175, 171, 64, 216, 186, 216, 46, 91, 242, 83, 127, 60, 227, 227, 8, 9, 180, 71, 180, 73, 234, 177, 154, 14, 28, 108, 38, 129, 123, 37, 112, 106, 221, 29, 243, 254, 159, 8, 1, 29, 56, 64, 127, 152, 93, 70, 186, 201, 58, 27, 238, 212, 22, 145, 152, 101, 29, 21, 10, 51, 76, 171, 81, 41, 133, 104, 232, 200, 86, 29, 206, 123, 81, 18, 53, 122, 110, 212, 106, 117, 103, 143, 185, 15, 34, 113, 222, 116, 78, 35, 165, 163, 63, 137, 191, 146, 184, 147, 6, 149, 39, 202, 216, 168, 220, 214, 83, 87, 8, 255, 62, 149, 26, 125, 95, 147, 17, 24, 142, 143, 43, 74, 70, 147, 88, 92, 20, 207, 20, 55, 138, 106, 15, 240, 77, 228, 31, 58, 181, 73, 97, 235, 66, 202, 183, 136, 22, 51, 198, 245, 235, 31, 186, 55, 44, 227, 29, 238, 240, 34, 37, 168, 137, 45, 43, 163, 24, 235, 176, 203, 78, 128, 235, 231, 2, 32, 7, 17, 24, 251, 90, 58, 112, 183, 243, 229, 150, 195, 228, 60, 187, 125, 88, 167, 177, 108, 14, 30, 244, 112, 16, 2, 239, 44, 186, 193, 13, 205, 57, 31, 125, 125, 155, 94, 139, 184, 78, 106, 252, 51, 205, 134, 184, 39, 119, 193, 186, 163, 21, 157, 215, 18, 59, 164, 248, 203, 60, 163, 197, 47, 121, 123, 131, 183, 18, 234, 151, 22, 192, 160, 249, 232, 190, 49, 222, 208, 251, 153, 159, 23, 239, 50, 42, 175, 70, 61, 98, 84, 51, 220, 72, 110, 111, 174, 50, 87, 62, 7, 127, 77, 26, 40, 182, 37, 40, 231, 238, 217, 222, 97, 246, 255, 134, 65, 144, 218, 185, 63, 39, 148, 174, 148, 133, 6, 31, 114, 174, 179, 132, 170, 32, 218, 220, 47, 218, 131, 23, 138, 63, 107, 43, 252, 187, 167, 70, 13, 81, 100, 180, 12, 97, 161, 13, 239, 159, 96, 232, 237, 129, 170, 120, 62, 213, 227, 202, 178, 104, 254, 195, 15, 6, 175, 5, 92, 12, 202, 135, 243, 66, 103, 208, 131, 153, 83, 219, 160, 114, 114, 210, 163, 227, 9, 89, 240, 230, 97, 183, 75, 247, 158, 119, 104, 216, 202, 249, 231, 192, 146, 62, 242, 109, 175, 173, 208, 249, 132, 206, 121, 240, 187, 41, 157, 25, 25, 47, 23, 88, 250, 146, 159, 239, 77, 191, 188, 193, 198, 40, 6, 0, 57, 58, 206, 45, 37, 106, 246, 5, 38, 224, 122, 68, 26, 173, 135, 5, 84, 160, 32, 253, 103, 209, 75, 159, 17, 79, 143, 72, 93, 52, 170, 135, 8, 159, 98, 47, 147, 222, 117, 215, 159, 255, 206, 55, 207, 225, 214, 20, 216, 40, 189, 213, 223, 224, 186, 38, 102, 16, 170, 225, 147, 39, 94, 199, 224, 123, 146, 21, 0, 51, 26, 39, 35, 9, 21, 52, 242, 230, 188, 218, 205, 139, 107, 42, 239, 175, 61, 149, 74, 50, 162, 84, 167, 94, 135, 149, 200, 154, 187, 254, 95, 235, 90, 9, 18, 48, 171, 150, 146, 43, 42, 134, 132, 199, 55, 177, 1, 2, 181, 186, 109, 93, 57, 117, 110, 90, 80, 172, 154, 251, 209, 165, 136, 12, 101, 194, 131, 42, 2, 232, 196, 237, 209, 204, 36, 59, 148, 52, 23, 29, 80, 66, 201, 245, 222, 39, 31, 199, 194, 177, 68, 80, 244, 192, 220, 251, 3, 181, 39, 242, 209, 119, 231, 97, 25, 228, 152, 26, 48, 142, 153, 102, 126, 235, 140, 175, 78, 12, 52, 98, 105, 29, 171, 189, 217, 214, 249, 252, 83, 241, 255, 9, 18, 121, 37, 24, 181, 35, 110, 184, 218, 194, 97, 49, 229, 154, 206, 99, 159, 248, 104, 52, 171, 29, 177, 39, 145, 209, 78, 107, 234, 201, 85, 43, 112, 27, 140, 187, 110, 46, 222, 255, 124, 176, 164, 54, 235, 165, 134, 28, 131, 69, 106, 229, 229, 2, 197, 201, 140, 100, 132, 251, 180, 238, 233, 205, 104, 21, 172, 136, 236, 158, 39, 232, 63, 229, 211, 45, 179, 182, 61, 74, 68, 178, 67, 131, 37, 69, 5, 127, 158, 181, 2, 189, 65, 219, 233, 94, 125, 52, 38, 94, 234, 118, 66, 32, 84, 116, 196, 186, 230, 246, 52, 196, 218, 216, 38, 111, 89, 201, 243, 114, 193, 75, 64, 197, 168, 37, 35, 246, 92, 253, 12, 223, 8, 73, 152, 71, 140, 98, 225, 182, 98, 219, 35, 193, 180, 151, 103, 29, 156, 40, 118, 136, 81, 232, 222, 223, 92, 224, 17, 127, 48, 27, 3, 219, 215, 134, 178, 126, 192, 44, 182, 96, 80, 87, 39, 3, 44, 41, 82, 109, 59, 202, 94, 133, 51, 16, 125, 109, 245, 156, 1, 121, 123, 113, 102, 127, 131, 204, 210, 176, 223, 190, 159, 176, 34, 253, 23, 81, 142, 59, 192, 84, 129, 175, 183, 241, 82, 106, 19, 3, 119, 250, 119, 101, 170, 125, 187, 58, 233, 224, 84, 185, 115, 123, 143, 84, 154, 244, 6, 90, 86, 150, 15, 187, 82, 183, 133, 0, 243, 229, 19, 80, 102, 142, 36, 98, 41, 111, 25, 188, 233, 85, 85, 59, 223, 165, 10, 126, 204, 97, 235, 215, 15, 245, 180, 28, 238, 43, 79, 60, 159, 153, 217, 191, 190, 176, 101, 234, 147, 97, 193, 3, 116, 13, 120, 131, 248, 247, 90, 89, 186, 0, 99, 78, 128, 109, 161, 168, 152, 247, 116, 16, 28, 96, 159, 73, 39, 134, 131, 104, 37, 234, 74, 179, 172, 57, 38, 183, 137, 89, 218, 176, 71, 79, 58, 102, 182, 130, 17, 198, 60, 34, 7, 83, 211, 40, 146, 202, 171, 228, 187, 117, 94, 35, 198, 89, 107, 92, 155, 35, 158, 11, 198, 171, 21, 252, 239, 47, 252, 85, 86, 163, 71, 138, 249, 64, 64, 71, 233, 110, 255, 111, 250, 217, 218, 85, 26, 61, 11, 19, 237, 200, 177, 47, 83, 236, 147, 239, 87, 79, 100, 96, 215, 122, 219, 2, 22, 17, 225, 56, 28, 221, 44, 231, 24, 119, 234, 70, 229, 25, 32, 130, 110, 16, 215, 91, 109, 193, 177, 254, 173, 98, 188, 14, 197, 39, 137, 88, 113, 194, 185, 218, 98, 169, 64, 180, 231, 152, 244, 92, 85, 187, 141, 238, 23, 167, 139, 181, 140, 156, 81, 109, 158, 209, 97, 172, 201, 99, 130, 231, 174, 19, 216, 108, 104, 160, 120, 106, 30, 30, 150, 96, 109, 163, 82, 169, 5, 135, 142, 213, 50, 72, 39, 117, 42, 34, 197, 8, 100, 184, 92, 22, 64, 10, 39, 161, 226, 81, 173, 241, 86, 240, 252, 102, 11, 95, 95, 108, 98, 136, 150, 31, 48, 23, 197, 85, 33, 242, 151, 81, 209, 117, 211, 38, 228, 148, 163, 114, 75, 254, 21, 177, 7, 53, 129, 106, 64, 49, 72, 229, 33, 62, 88, 173, 185, 190, 26, 102, 19, 138, 17, 65, 211, 3, 246, 29, 54, 113, 84, 213, 160, 45, 151, 251, 0, 75, 85, 185, 138, 213, 46, 240, 243, 214, 255, 192, 51, 188, 188, 194, 32, 113, 208, 69, 125, 61, 215, 134, 236, 187, 81, 252, 33, 184, 106, 169, 66, 241, 66, 189, 26, 53, 212, 139, 84, 248, 215, 85, 181, 18, 171, 71, 242, 162, 157, 232, 212, 180, 8, 235, 222, 188, 90, 62, 106, 201, 132, 196, 3, 84, 76, 52, 169, 161, 255, 193, 235, 84, 3, 40, 3, 141, 90, 48, 133, 61, 127, 109, 154, 14, 12, 247, 48, 152, 192, 122, 247, 115, 189, 115, 92, 28, 117, 57, 175, 133, 253, 223, 79, 19, 132, 12, 43, 92, 214, 66, 166, 179, 89, 16, 188, 4, 38, 8, 157, 186, 24, 116, 225, 230, 180, 75, 62, 170, 151, 91, 140, 93, 4, 40, 176, 60, 195, 120, 110, 193, 95, 3, 70, 94, 161, 184, 248, 152, 213, 152, 128, 25, 226, 230, 216, 83, 115, 176, 110, 126, 194, 141, 8, 187, 125, 50, 56, 3, 216, 88, 175, 98, 216, 176, 185, 219, 42, 230, 54, 127, 149, 152, 135, 116, 150, 26, 206, 149, 232, 194, 245, 42, 135, 45, 171, 76, 38, 239, 178, 5, 67, 75, 104, 181, 34, 111, 134, 191, 33, 210, 252, 30, 200, 145, 142, 210, 85, 240, 103, 23, 149, 34, 225, 29, 7, 228, 134, 228, 215, 99, 43, 146, 9, 240, 147, 65, 165, 192, 26, 141, 29, 112, 189, 8, 174, 139, 60, 20, 235, 7, 88, 233, 5, 207, 101, 16, 55, 92, 90, 145, 55, 209, 252, 142, 43, 160, 74, 148, 84, 2, 115, 219, 163, 84, 103, 20, 5, 17, 92, 91, 170, 199, 133, 45, 197, 26, 105, 72, 201, 188, 134, 127, 124, 212, 128, 127, 188, 48, 69, 220, 97, 163, 71, 203, 88, 177, 49, 176, 250, 29, 226, 27, 221, 79, 87, 115, 102, 35, 105, 38, 23, 225, 243, 215, 120, 79, 29, 252, 113, 78, 82, 22, 82, 120, 120, 4, 15, 9, 76, 254, 14, 2, 7, 146, 150, 34, 228, 218, 175, 112, 132, 137, 13, 237, 47, 213, 54, 240, 203, 91, 215, 185, 234, 65, 37, 248, 78, 206, 108, 58, 48, 181, 171, 127, 6, 17, 37, 224, 154, 177, 216, 5, 118, 30, 249, 29, 136, 39, 78, 109, 112, 253, 118, 10, 32, 238, 28, 185, 171, 35, 232, 202, 120, 186, 254, 232, 137, 77, 42, 106, 12, 68, 37, 120, 124, 203, 224, 49, 66, 143, 126, 154, 84, 4, 167, 174, 34, 75, 96, 253, 98, 123, 227, 150, 240, 114, 80, 105, 57, 3, 150, 224, 151, 227, 180, 52, 63, 16, 162, 71, 213, 43, 121, 234, 242, 2, 70, 174, 82, 27, 36, 228, 15, 165, 138, 137, 152, 22, 125, 126, 115, 131, 91, 29, 129, 126, 194, 114, 117, 93, 247, 31, 239, 157, 182, 151, 243, 54, 112, 173, 66, 138, 235, 226, 113, 58, 133, 173, 205, 200, 196, 245, 157, 0, 244, 44, 127, 224, 166, 240, 151, 252, 188, 43, 85, 17, 82, 73, 198, 101, 140, 230, 186, 84, 112, 158, 59, 53, 164, 131, 219, 85, 125, 49, 1, 215, 168, 94, 41, 34, 192, 121, 101, 187, 65, 107, 124, 254, 12, 76, 121, 68, 68, 125, 199, 225, 38, 94, 211, 212, 250, 107, 208, 74, 13, 0, 59, 126, 232, 213, 60, 205, 89, 208, 224, 222, 219, 97, 141, 1, 219, 63, 117, 31, 238, 23, 110, 128, 171, 188, 235, 211, 210, 78, 184, 98, 132, 242, 182, 4, 66, 78, 228, 190, 151, 6, 135, 158, 123, 144, 20, 208, 130, 203, 104, 211, 203, 195, 140, 66, 31, 64, 159, 221, 130, 246, 169, 187, 157, 172, 102, 13, 99, 251, 207, 240, 23, 245, 184, 244, 247, 74, 244, 77, 51, 207, 139, 82, 228, 84, 125, 77, 81, 145, 86, 202, 31, 176, 178, 206, 149, 10, 83, 119, 28, 124, 176, 97, 247, 35, 182, 178, 138, 47, 113, 81, 63, 224, 77, 252, 119, 149, 208, 230, 167, 89, 109, 123, 118, 65, 41, 240, 35, 127, 234, 135, 42, 110, 205, 49, 5, 188, 209, 232, 169, 168, 241, 137, 15, 30, 104, 53, 21, 248, 137, 178, 215, 243, 41, 178, 113, 105, 94, 157, 123, 39, 16, 49, 122, 228, 31, 130, 104, 228, 136, 220, 205, 173, 140, 163, 37, 189, 92, 105, 145, 83, 244, 89, 215, 74, 226, 148, 142, 166, 150, 120, 239, 128, 161, 18, 71, 10, 161, 172, 213, 119, 20, 94, 69, 84, 170, 241, 241, 115, 126, 175, 100, 117, 156, 199, 27, 130, 179, 99, 16, 10, 205, 73, 7, 212, 207, 4, 66, 172, 202, 14, 152, 157, 197, 115, 145, 73, 217, 79, 137, 193, 183, 73, 248, 125, 86, 196, 141, 184, 104, 189, 173, 19, 255, 3, 6, 49, 180, 33, 47, 225, 192, 249, 180, 161, 191, 229, 4, 112, 124, 137, 255, 239, 200, 197, 98, 127, 176, 89, 61, 116, 108, 8, 17, 204, 145, 186, 251, 100, 243, 219, 152, 53, 119, 155, 5, 11, 242, 7, 78, 207, 128, 144, 115, 134, 203, 132, 88, 159, 175, 213, 126, 176, 174, 141, 120, 204, 21, 217, 83, 146, 146, 63, 213, 33, 157, 43, 109, 104, 79, 208, 156, 193, 122, 123, 91, 230, 212, 134, 178, 159, 199, 72, 6, 206, 191, 95, 233, 57, 132, 215, 131, 188, 95, 178, 253, 43, 190, 129, 189, 198, 179, 225, 114, 234, 79, 174, 83, 62, 219, 136, 255, 81, 174, 232, 221, 4, 14, 111, 167, 229, 129, 86, 164, 15, 233, 56, 33, 241, 91, 147, 199, 112, 116, 109, 54, 146, 33, 60, 121, 207, 5, 242, 95, 145, 117, 68, 178, 131, 57, 46, 242, 211, 63, 100, 46, 68, 68, 77, 84, 97, 64, 50, 75, 19, 181, 78, 210, 72, 220, 214, 173, 243, 233, 97, 29, 8, 59, 18, 210, 17, 4, 128, 209, 67, 215, 234, 80, 17, 28, 168, 226, 54, 0, 193, 88, 107, 138, 34, 39, 45, 121, 225, 93, 12, 96, 113, 33, 223, 135, 150, 21, 153, 1, 173, 122, 127, 121, 183, 105, 151, 41, 122, 165, 143, 62, 104, 161, 165, 85, 190, 223, 140, 186, 51, 58, 249, 228, 216, 202, 48, 181, 179, 34, 93, 214, 212, 17, 195, 200, 235, 236, 23, 5, 114, 82, 79, 106, 255, 146, 203, 132, 94, 58, 93, 25, 68, 70, 92, 96, 117, 42, 231, 12, 235, 225, 33, 89, 107, 118, 151, 40, 120, 234, 102, 156, 225, 255, 239, 176, 91, 23, 197, 160, 157, 148, 120, 90, 131, 202, 103, 113, 152, 181, 144, 37, 150, 174, 73, 157, 240, 208, 102, 25, 241, 141, 235, 153, 229, 59, 170, 52, 160, 96, 24, 202, 1, 153, 177, 76, 195, 142, 222, 118, 35, 142, 177, 170, 252, 168, 188, 180, 90, 226, 233, 225, 61, 27, 204, 219, 175, 61, 189, 199, 133, 13, 74, 69, 99, 70, 83, 37, 120, 134, 226, 216, 240, 70, 135, 238, 83, 9, 200, 134, 146, 63, 49, 122, 240, 197, 14, 71, 87, 110, 222, 173, 153, 152, 25, 201, 169, 165, 91, 77, 4, 84, 196, 44, 143, 226, 89, 150, 58, 124, 235, 121, 45, 16, 155, 138, 61, 216, 46, 2, 166, 250, 39, 220, 139, 92, 191, 70, 140, 195, 161, 76, 47, 196, 9, 69, 208, 135, 48, 86, 165, 127, 146, 113, 212, 162, 51, 92, 23, 75, 254, 185, 239, 250, 15, 205, 127, 78, 4, 102, 250, 128, 188, 186, 215, 186, 212, 136, 74, 169, 39, 182, 113, 81, 232, 82, 197, 219, 177, 158, 45, 83, 138, 65, 152, 138, 192, 70, 88, 144, 28, 244, 96, 54, 152, 37, 19, 216, 235, 22, 210, 198, 213, 247, 99, 194, 105, 154, 39, 94, 96, 194, 73, 160, 142, 125, 3, 201, 219, 157, 44, 1, 87, 90, 77, 32, 147, 149, 109, 25, 6, 116, 11, 122, 227, 126, 237, 181, 207, 34, 164, 67, 163, 141, 221, 228, 94, 195, 157, 97, 245, 223, 148, 132, 78, 98, 172, 53, 113, 3, 131, 130, 100, 23, 42, 94, 7, 219, 232, 203, 208, 190, 104, 21, 215, 215, 20, 158, 144, 89, 117, 255, 125, 9, 157, 67, 41, 16, 86, 205, 168, 168, 164, 42, 68, 21, 34, 188, 222, 104, 188, 231, 1, 45, 9, 157, 214, 24, 102, 130, 156, 245, 174, 96, 63, 39, 143, 207, 112, 36, 106, 228, 240, 158, 13, 78, 171, 109, 178, 138, 239, 206, 150, 70, 37, 133, 117, 143, 33, 7, 44, 1, 65, 145, 224, 238, 220, 52, 129, 136, 127, 178, 253, 77, 79, 245, 240, 105, 57, 164, 225, 9, 241, 126, 126, 106, 66, 216, 22, 0, 189, 247, 114, 21, 172, 2, 117, 31, 118, 58, 74, 216, 162, 30, 99, 137, 87, 136, 105, 158, 224, 6, 121, 167, 134, 237, 144, 122, 119, 53, 4, 15, 109, 240, 57, 194, 226, 157, 68, 7, 226, 224, 32, 96, 167, 54, 53, 181, 161, 62, 61, 163, 211, 74, 107, 210, 60, 203, 104, 140, 213, 113, 91, 101, 223, 169, 74, 49, 241, 219, 146, 37, 75, 30, 35, 123, 145, 90, 212, 29, 206, 93, 119, 81, 104, 22, 158, 194, 254, 207, 25, 13, 195, 130, 30, 168, 194, 120, 2, 57, 193, 112, 159, 181, 74, 137, 11, 70, 90, 160, 67, 76, 111, 185, 132, 228, 174, 1, 37, 102, 206, 25, 155, 45, 233, 70, 103, 37, 45, 197, 87, 248, 163, 132, 162, 184, 230, 181, 38, 236, 178, 80, 184, 182, 227, 197, 193, 29, 100, 188, 114, 153, 230, 208, 225, 139, 161, 220, 167, 188, 155, 139, 244, 117, 15, 209, 148, 135, 81, 55, 32, 63, 95, 211, 241, 78, 247, 203, 98, 165, 237, 156, 41, 152, 132, 60, 191, 185, 144, 13, 147, 196, 80, 41, 222, 66, 226, 193, 74, 227, 70, 158, 83, 105, 114, 18, 102, 8, 135, 253, 83, 165, 228, 106, 245, 255, 77, 77, 209, 20, 248, 175, 88, 72, 36, 198, 225, 75, 210, 121, 113, 213, 162, 119, 181, 228, 220, 35, 71, 74, 157, 177, 81, 175, 119, 235, 199, 73, 107, 56, 106, 241, 32, 57, 33, 36, 23, 200, 162, 177, 207, 26, 135, 122, 50, 55, 219, 210, 139, 168, 85, 124, 121, 50, 3, 64, 133, 162, 232, 39, 217, 46, 228, 225, 7, 17, 85, 9, 20, 1, 160, 228, 235, 47, 109, 133, 82, 228, 192, 184, 17, 152, 43, 13, 30, 149, 87, 213, 195, 26, 123, 94, 221, 134, 249, 131, 42, 52, 182, 79, 84, 212, 13, 190, 36, 22, 84, 171, 191, 218, 129, 128, 178, 24, 71, 8, 3, 144, 159, 154, 202, 47, 233, 249, 126, 212, 37, 247, 19, 104, 108, 94, 153, 140, 22, 54, 40, 45, 146, 197, 177, 208, 123, 161, 0, 148, 54, 113, 145, 23, 62, 182, 41, 100, 105, 127, 70, 176, 166, 25, 1, 60, 143, 208, 51, 77, 211, 184, 163, 9, 219, 116, 230, 196, 100, 183, 176, 91, 249, 73, 119, 0, 177, 242, 220, 242, 150, 237, 178, 194, 233, 11, 190, 92, 71, 30, 65, 72, 65, 116, 106, 5, 43, 238, 59, 31, 147, 188, 78, 141, 42, 53, 33, 62, 189, 90, 65, 70, 138, 103, 24, 109, 15, 52, 81, 253, 216, 45, 205, 198, 218, 29, 254, 147, 60, 164, 46, 210, 213, 110, 80, 213, 12, 142, 139, 17, 144, 50, 181, 117, 1, 29, 246, 137, 58, 108, 108, 15, 237, 49, 172, 218, 56, 183, 113, 206, 36, 223, 173, 132, 33, 136, 52, 53, 229, 136, 5, 65, 157, 244, 74, 137, 19, 144, 253, 188, 48, 22, 92, 173, 10, 119, 57, 88, 83, 217, 116, 229, 139, 119, 87, 54, 84, 102, 93, 150, 75, 117, 176, 145, 244, 131, 167, 89, 129, 28, 136, 219, 245, 234, 60, 151, 202, 188, 162, 172, 145, 213, 99, 29, 125, 49, 140, 61, 255, 94, 128, 67, 193, 23, 73, 225, 254, 154, 62, 107, 218, 59, 59, 78, 63, 37, 194, 196, 128, 247, 230, 50, 238, 216, 45, 191, 5, 204, 21, 82, 128, 114, 78, 209, 255, 84, 96, 153, 17, 198, 205, 183, 201, 109, 82, 6, 149, 133, 208, 225, 82, 179, 81, 68, 144, 189, 134, 129, 250, 229, 193, 166, 143, 74, 92, 133, 233, 5, 7, 210, 19, 76, 56, 109, 104, 224, 103, 248, 36, 209, 139, 161, 69, 213, 113, 108, 6, 6, 158, 157, 215, 87, 176, 205, 144, 135, 140, 45, 70, 228, 139, 196, 59, 132, 153, 83, 201, 107, 112, 240, 197, 235, 198, 248, 33, 156, 102, 218, 105, 244, 127, 252, 228, 79, 112, 171, 165, 85, 150, 69, 161, 183, 202, 154, 161, 252, 167, 176, 173, 37, 81, 82, 41, 105, 145, 107, 178, 38, 200, 215, 32, 135, 4, 83, 25, 164, 234, 83, 29, 22, 132, 209, 111, 49, 98, 92, 247, 160, 152, 172, 125, 47, 49, 72, 96, 157, 157, 238, 110, 69, 151, 59, 185, 209, 17, 67, 25, 248, 229, 129, 10, 29, 227, 49, 231, 6, 62, 48, 220, 226, 158, 89, 51, 240, 62, 243, 68, 25, 250, 56, 137, 39, 4, 152, 182, 204, 186, 111, 106, 71, 151, 169, 48, 123, 203, 84, 6, 45, 81, 155, 182, 84, 78, 171, 244, 248, 46, 186, 221, 83, 153, 50, 191, 107, 72, 63, 194, 141, 30, 177, 7, 146, 192, 232, 165, 147, 179, 56, 221, 84, 144, 213, 158, 252, 174, 23, 65, 66, 231, 183, 254, 254, 234, 11, 88, 41, 251, 0, 220, 190, 51, 73, 158, 21, 20, 249, 201, 51, 17, 201, 22, 114, 244, 89, 126, 82, 184, 165, 196, 13, 144, 116, 254, 195, 201, 106, 164, 1, 209, 162, 218, 157, 87, 58, 250, 192, 85, 112, 169, 194, 109, 92, 193, 126, 33, 101, 152, 4, 109, 134, 119, 73, 252, 237, 49, 83, 155, 56, 137, 196, 234, 29, 172, 31, 107, 88, 85, 40, 234, 151, 200, 196, 69, 94, 254, 122, 134, 172, 99, 135, 128, 92, 42, 15, 216, 120, 165, 33, 242, 139, 4, 251, 231, 253, 173, 120, 46, 5, 130, 161, 65, 155, 158, 35, 47, 241, 53, 217, 49, 234, 129, 231, 27, 24, 41, 173, 175, 132, 122, 47, 62, 35, 194, 249, 158, 190, 234, 241, 169, 62, 178, 162, 186, 151, 13, 60, 195, 177, 135, 102, 187, 224, 182, 138, 70, 40, 229, 219, 150, 208, 173, 237, 248, 233, 192, 67, 161, 241, 105, 230, 75, 199, 9, 126, 153, 86, 224, 163, 230, 108, 173, 166, 192, 4, 213, 24, 53, 112, 26, 228, 145, 104, 122, 116, 75, 56, 1, 94, 209, 5, 105, 223, 188, 9, 71, 201, 166, 93, 236, 110, 71, 183, 53, 234, 121, 155, 171, 131, 188, 230, 93, 136, 154, 5, 36, 143, 224, 153, 57, 57, 247, 226, 0, 162, 178, 221, 235, 166, 1, 189, 242, 211, 234, 162, 164, 8, 155, 24, 77, 142, 35, 17, 255, 145, 140, 181, 251, 12, 141, 192, 17, 117, 208, 51, 161, 166, 143, 51, 26, 164, 254, 97, 20, 196, 75, 111, 186, 181, 58, 81, 115, 29, 185, 60, 221, 60, 37, 77, 135, 134, 240, 145, 132, 174, 59, 76, 53, 124, 17, 39, 230, 235, 203, 23, 3, 3, 0, 98, 155, 183, 154, 36, 156, 238, 133, 141, 73, 196, 197, 111, 179, 233, 48, 198, 215, 186, 166, 187, 137, 201, 81, 128, 64, 247, 146, 71, 211, 225, 163, 37, 121, 214, 57, 73, 6, 155, 201, 199, 14, 48, 64, 198, 98, 24, 240, 56, 151, 145, 207, 34, 68, 132, 20, 78, 208, 238, 107, 75, 108, 192, 230, 142, 31, 222, 172, 226, 58, 61, 176, 255, 215, 248, 184, 239, 246, 207, 193, 64, 125, 100, 201, 220, 80, 39, 139, 55, 139, 250, 102, 175, 246, 21, 237, 22, 251, 155, 23, 3, 3, 1, 10, 89, 74, 203, 38, 228, 17, 135, 240, 110, 213, 190, 108, 128, 143, 51, 214, 49, 33, 222, 19, 248, 237, 198, 86, 122, 133, 184, 66, 56, 138, 2, 216, 83, 195, 155, 143, 216, 203, 246, 5, 17, 91, 179, 144, 77, 18, 118, 229, 188, 242, 23, 18, 117, 109, 48, 101, 57, 99, 132, 213, 119, 143, 100, 43, 11, 153, 229, 244, 228, 40, 154, 245, 3, 146, 81, 56, 70, 94, 79, 229, 65, 196, 69, 147, 174, 20, 55, 60, 212, 247, 20, 234, 159, 149, 124, 15, 150, 141, 209, 222, 140, 253, 139, 31, 168, 6, 230, 12, 46, 148, 221, 52, 39, 90, 138, 77, 16, 223, 40, 186, 1, 175, 140, 203, 145, 36, 173, 69, 215, 173, 242, 115, 104, 108, 47, 21, 45, 38, 61, 191, 181, 17, 66, 72, 76, 165, 18, 222, 131, 230, 86, 84, 192, 101, 85, 213, 254, 88, 29, 51, 157, 239, 37, 38, 100, 22, 54, 213, 94, 219, 131, 228, 167, 81, 195, 187, 83, 182, 67, 90, 31, 134, 210, 102, 158, 8, 169, 79, 138, 208, 208, 255, 214, 84, 35, 17, 189, 123, 46, 178, 165, 120, 188, 171, 19, 218, 144, 95, 56, 209, 71, 74, 238, 75, 94, 236, 247, 192, 209, 18, 208, 39, 146, 123, 81, 22, 226, 41, 138, 255, 161, 184, 77, 42, 209, 95, 239, 15, 232, 32, 75, 156, 210, 36, 190, 68, 223, 236, 77, 108, 66, 120, 179, 184, 237, 30, 245, 198, 219, 64, 169, 28, 37, 224, 189, 90, 23, 3, 3, 1, 10, 221, 239, 235, 41, 136, 100, 200, 246, 92, 80, 87, 142, 70, 113, 197, 225, 124, 204, 139, 250, 142, 201, 52, 187, 129, 164, 58, 186, 250, 111, 208, 61, 150, 74, 157, 173, 164, 69, 7, 147, 132, 216, 184, 162, 67, 156, 43, 167, 53, 7, 130, 127, 254, 79, 190, 220, 117, 51, 72, 86, 222, 131, 70, 231, 66, 51, 60, 49, 129, 183, 177, 149, 217, 207, 108, 126, 208, 14, 91, 53, 46, 84, 95, 75, 33, 17, 76, 33, 98, 178, 122, 234, 72, 250, 153, 214, 50, 249, 208, 224, 61, 140, 46, 87, 96, 249, 4, 191, 236, 31, 189, 222, 108, 217, 0, 48, 224, 161, 161, 164, 55, 139, 191, 244, 145, 222, 12, 235, 175, 212, 227, 44, 109, 144, 123, 51, 206, 172, 183, 231, 63, 140, 64, 191, 165, 89, 248, 247, 245, 72, 94, 59, 108, 12, 111, 36, 38, 46, 221, 0, 153, 141, 213, 98, 100, 255, 101, 93, 132, 179, 169, 63, 241, 115, 62, 109, 80, 118, 206, 133, 3, 34, 62, 72, 79, 161, 253, 133, 74, 243, 143, 232, 110, 60, 155, 74, 33, 202, 198, 47, 212, 43, 46, 199, 148, 154, 214, 31, 141, 205, 147, 158, 94, 201, 189, 14, 135, 101, 10, 149, 91, 189, 100, 77, 150, 113, 64, 9, 107, 54, 113, 77, 235, 36, 144, 147, 48, 245, 231, 58, 235, 44, 164, 47, 119, 248, 177, 175, 62, 228, 207, 62, 25, 9, 70, 144, 68, 191, 10, 214, 124, 14, 14, 80, 253, 81, 23, 3, 3, 4, 232, 242, 239, 86, 125, 133, 14, 232, 181, 1, 110, 98, 16, 185, 31, 122, 71, 139, 111, 179, 159, 197, 228, 37, 36, 128, 118, 36, 46, 64, 35, 78, 31, 23, 112, 54, 129, 195, 219, 40, 158, 246, 172, 254, 221, 226, 66, 144, 56, 107, 64, 197, 216, 204, 42, 233, 21, 142, 140, 162, 83, 196, 33, 49, 84, 54, 134, 183, 82, 224, 11, 90, 90, 2, 198, 74, 77, 223, 112, 200, 27, 133, 32, 191, 119, 100, 19, 181, 204, 173, 24, 211, 56, 182, 107, 207, 133, 119, 201, 109, 205, 154, 186, 124, 143, 62, 121, 179, 227, 235, 85, 211, 184, 95, 177, 166, 202, 107, 130, 158, 247, 71, 17, 206, 226, 131, 119, 66, 236, 213, 29, 84, 43, 193, 6, 222, 16, 50, 61, 108, 112, 228, 20, 41, 215, 237, 86, 103, 104, 101, 88, 203, 120, 255, 56, 2, 192, 138, 37, 66, 56, 44, 49, 253, 20, 98, 149, 159, 200, 79, 201, 154, 196, 23, 41, 121, 87, 176, 211, 251, 205, 47, 91, 210, 22, 248, 121, 193, 63, 122, 151, 5, 45, 213, 20, 83, 225, 46, 105, 29, 6, 69, 162, 150, 168, 127, 128, 128, 63, 102, 195, 229, 171, 142, 183, 111, 203, 125, 50, 81, 204, 128, 71, 122, 185, 151, 204, 133, 80, 187, 8, 155, 78, 197, 105, 85, 162, 47, 89, 122, 108, 201, 130, 52, 222, 224, 155, 25, 172, 81, 182, 205, 207, 195, 210, 171, 19, 182, 166, 156, 55, 42, 192, 144, 19, 63, 70, 64, 215, 51, 132, 104, 47, 113, 145, 46, 227, 68, 235, 92, 138, 201, 141, 255, 168, 172, 47, 28, 190, 97, 225, 219, 156, 205, 41, 223, 191, 126, 58, 21, 217, 201, 156, 27, 129, 136, 148, 139, 155, 108, 68, 98, 175, 71, 63, 11, 207, 98, 83, 106, 99, 119, 164, 241, 5, 136, 244, 195, 33, 132, 230, 181, 244, 105, 162, 56, 150, 3, 74, 197, 192, 11, 5, 88, 90, 145, 164, 47, 21, 169, 134, 82, 148, 173, 217, 130, 51, 172, 47, 253, 41, 43, 179, 200, 218, 171, 102, 192, 130, 32, 45, 178, 207, 22, 37, 92, 118, 202, 248, 140, 244, 244, 74, 152, 223, 201, 21, 140, 234, 199, 81, 244, 74, 88, 55, 83, 13, 193, 238, 250, 190, 21, 250, 69, 180, 81, 254, 236, 173, 15, 73, 39, 209, 215, 171, 83, 130, 142, 68, 8, 217, 208, 81, 234, 23, 137, 103, 13, 63, 251, 122, 228, 255, 38, 238, 31, 7, 26, 124, 18, 230, 216, 119, 16, 157, 41, 246, 109, 208, 184, 222, 16, 29, 206, 180, 0, 22, 184, 142, 196, 49, 120, 68, 193, 20, 216, 0, 107, 40, 77, 243, 36, 49, 164, 44, 79, 70, 23, 152, 25, 248, 51, 57, 206, 202, 206, 78, 75, 112, 27, 93, 173, 115, 90, 231, 207, 90, 219, 27, 221, 204, 252, 148, 44, 194, 184, 153, 35, 129, 62, 220, 237, 178, 52, 228, 97, 247, 140, 94, 161, 134, 69, 164, 184, 206, 98, 165, 127, 44, 17, 41, 185, 106, 42, 80, 100, 200, 62, 53, 124, 235, 122, 5, 184, 240, 43, 82, 77, 163, 161, 151, 26, 175, 172, 84, 214, 14, 181, 179, 237, 136, 86, 98, 173, 228, 40, 41, 209, 147, 234, 175, 93, 208, 251, 86, 114, 108, 161, 16, 25, 208, 202, 66, 104, 163, 2, 198, 230, 104, 3, 80, 42, 127, 53, 199, 154, 236, 19, 243, 39, 163, 74, 173, 245, 141, 84, 170, 234, 99, 126, 197, 194, 248, 45, 233, 105, 104, 22, 116, 158, 229, 37, 13, 39, 4, 81, 165, 16, 141, 12, 159, 183, 171, 185, 158, 242, 50, 234, 186, 92, 93, 35, 42, 85, 149, 162, 20, 92, 90, 71, 202, 184, 9, 193, 138, 191, 15, 193, 150, 159, 25, 244, 38, 58, 170, 237, 164, 189, 15, 81, 29, 209, 231, 75, 158, 206, 254, 174, 216, 96, 167, 137, 207, 54, 234, 206, 207, 77, 7, 251, 76, 161, 79, 164, 184, 142, 28, 178, 202, 179, 217, 45, 83, 184, 174, 95, 100, 65, 40, 134, 196, 1, 49, 151, 8, 210, 149, 5, 197, 20, 111, 211, 18, 234, 124, 181, 24, 45, 92, 22, 122, 95, 165, 28, 98, 92, 68, 199, 76, 27, 167, 154, 7, 124, 220, 3, 229, 40, 117, 126, 252, 86, 32, 22, 18, 86, 195, 39, 20, 91, 49, 244, 148, 6, 166, 186, 43, 223, 210, 31, 181, 100, 201, 163, 175, 144, 238, 137, 127, 207, 179, 24, 83, 130, 169, 203, 53, 224, 213, 214, 144, 232, 145, 190, 114, 80, 82, 174, 100, 236, 217, 159, 159, 177, 1, 66, 24, 222, 154, 174, 104, 1, 225, 209, 118, 164, 89, 25, 95, 189, 97, 242, 192, 114, 105, 255, 210, 61, 140, 181, 30, 87, 226, 167, 135, 24, 249, 100, 199, 36, 159, 228, 204, 201, 44, 217, 105, 167, 204, 16, 95, 4, 188, 98, 184, 227, 159, 14, 87, 144, 182, 31, 134, 129, 143, 161, 216, 196, 242, 167, 124, 243, 173, 192, 202, 196, 132, 218, 22, 59, 242, 174, 188, 82, 103, 87, 212, 114, 206, 238, 2, 116, 8, 74, 110, 8, 243, 34, 249, 141, 115, 149, 232, 103, 251, 108, 101, 86, 62, 186, 111, 13, 109, 27, 143, 32, 114, 190, 236, 54, 156, 0, 248, 135, 114, 29, 19, 175, 163, 104, 75, 125, 196, 156, 217, 98, 84, 187, 26, 231, 25, 147, 154, 92, 93, 175, 32, 65, 227, 14, 159, 28, 233, 143, 241, 127, 167, 59, 207, 90, 39, 45, 94, 13, 0, 192, 104, 34, 33, 233, 42, 45, 248, 93, 218, 184, 95, 41, 84, 19, 225, 199, 112, 45, 75, 142, 111, 88, 177, 208, 95, 13, 156, 7, 157, 160, 1, 100, 51, 120, 41, 109, 98, 109, 246, 230, 211, 28, 134, 229, 253, 148, 124, 107, 217, 9, 33, 221, 72, 214, 251, 137, 3, 213, 168, 151, 3, 151, 241, 220, 55, 117, 244, 41, 73, 3, 78, 167, 38, 70, 103, 152, 235, 215, 113, 238, 27, 40, 81, 113, 62, 71, 50, 182, 218, 38, 152, 244, 227, 52, 13, 98, 182, 70, 175, 214, 7, 240, 8, 240, 92, 30, 106, 145, 182, 60, 10, 125, 51, 207, 229, 145, 112, 149, 83, 144, 147, 183, 176, 17, 16, 161, 119, 70, 32, 200, 22, 207, 198, 60, 145, 247, 92, 90, 13, 12, 253, 138, 46, 212, 223, 108, 164, 40, 189, 63, 54, 66, 213, 244, 133, 125, 190, 230, 14, 192, 145, 222, 239, 133, 40, 202, 102, 170, 11, 153, 130, 130, 134, 98, 243, 207, 200, 20, 58, 125, 146, 143, 245, 14, 189, 21, 12, 221, 88, 206, 191, 228, 132, 90, 44, 72, 236, 189, 98, 69, 213, 26, 185, 60, 100, 217, 198, 2, 249, 11, 189, 83, 111, 221, 229, 131, 89, 13, 71, 97, 197, 141, 217, 150, 178, 128, 165, 42, 157, 138, 115, 255, 234, 234, 121, 248, 19, 10, 248, 92, 183, 115, 183, 168, 167, 240, 112, 123, 220, 203, 83, 173, 255, 236, 103, 212, 63, 18, 44, 179, 102, 126, 91, 11, 241, 192, 94, 193, 120, 189, 95, 233, 190, 155, 59, 103, 203, 89, 113, 75, 111, 255, 119, 116, 31, 85, 82, 142, 112, 152, 112];
                let cell = TokenValue::write_bytes(&tls_data, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let cert: Vec<u8> = hex::decode("055b308205573082033fa003020102020d0203e5936f31b01349886ba217300d06092a864886f70d01010c05003047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f74205231301e170d3136303632323030303030305a170d3336303632323030303030305a3047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f7420523130820222300d06092a864886f70d01010105000382020f003082020a0282020100b611028b1ee3a1779b3bdcbf943eb795a7403ca1fd82f97d32068271f6f68c7ffbe8dbbc6a2e9797a38c4bf92bf6b1f9ce841db1f9c597deefb9f2a3e9bc12895ea7aa52abf82327cba4b19c63dbd7997ef00a5eeb68a6f4c65a470d4d1033e34eb113a3c8186c4becfc0990df9d6429252307a1b4d23d2e60e0cfd20987bbcd48f04dc2c27a888abbbacf5919d6af8fb007b09e31f182c1c0df2ea66d6c190eb5d87e261a45033db079a49428ad0f7f26e5a808fe96e83c689453ee833a882b159609b2e07a8c2e75d69ceba756648f964f68ae3d97c2848fc0bc40c00b5cbdf687b3356cac18507f84e04ccd92d320e933bc5299af32b529b3252ab448f972e1ca64f7e682108de89dc28a88fa38668afc63f901f978fd7b5c77fa7687faecdfb10e799557b4bd26efd601d1eb160abb8e0bb5c5c58a55abd3acea914b29cc19a432254e2af16544d002ceaace49b4ea9f7c83b0407be743aba76ca38f7d8981fa4ca5ffd58ec3ce4be0b5d8b38e45cf76c0ed402bfd530fb0a7d53b0db18aa203de31adcc77ea6f7b3ed6df912212e6befad832fc1063145172de5dd61693bd296833ef3a66ec078a26df13d757657827de5e491400a2007f9aa821b6a9b195b0a5b90d1611dac76c483c40e07e0d5acd563cd19705b9cb4bed394b9cc43fd255136e24b0d671faf4c1bacced1bf5fe8141d800983d3ac8ae7a98371805950203010001a3423040300e0603551d0f0101ff040403020186300f0603551d130101ff040530030101ff301d0603551d0e04160414e4af2b26711a2b4827852f52662ceff08913713e300d06092a864886f70d01010c050003820201009faa4226db0b9bbeff1e96922e3ea2654a6a98ba22cb7dc13ad8820a06c6f6a5dec04e876679a1f9a6589caaf9b5e660e7e0e8b11e4241330b373dce897015cab524a8cf6bb5d2402198cf2234cf3bc52284e0c50e8a7c5d88e43524ce9b3e1a541e6edbb287a7fcf3fa815514620a59a92205313e82d6eedb5734bc3395d3171be827a28b7b4e261a7a5a64b6d1ac37f1fda0f338ec72f011759dcb34528de6766b17c6df86ab278e492b7566811021a6ea3ef4ae25ff7c15dece8c253fca62700af72f096607c83f1cfcf0db4530df6288c1b50f9dc39f4ade595947c5872236e682a7ed0ab9e207a08d7b7a4a3c71d2e203a11f3207dd1be442ce0c00456180b50b20592978bdf955cb63c53c4cf4b6ffdb6a5f316b999e2cc16b50a4d7e61814bd853f67ab469fa0ff42a73a7f5ccb5db0701d2b34f5d476090ceb784c5905f33342c36115101b774dce228cd485f2457db753eaef405a940a5c205f4e405d622276dfffce61bd8c2378d23702e08eded1113789f6bfed490762ae92ec401aaf1409d9d04eb2a2f7beeeeed8ffdc1a2ddeb83671e2fc79b79425d148735ba135e7b3996775c1193a2b474ed3428efd31c81666dad20c3cdbb38ec9a10d800f7b167714bfffdb0994b293bc205815e9db7143f3de10c300dca82a95b6c2d63f906b76db6cfe8cbcf270350cdc991935dcd7c84663d53671ae57fbb7826ddc").unwrap();

                let cell = TokenValue::write_bytes(&cert, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let kid: Vec<u8> = vec![15, 63, 150, 152, 3, 129, 228, 81, 239, 173, 13, 45, 221, 48, 227, 211];
                let cell = TokenValue::write_bytes(&kid, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let timestamp: Vec<u8> = vec![0, 0, 3, 232];
                let cell = TokenValue::write_bytes(&timestamp, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));
                // Push args, func name, instance name, then wasm.
                let wasm_func = "tlscheck";
                let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut 0).unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let wasm_func = "docs:tlschecker/tls-check-interface@0.1.0";
                let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut 0).unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let wasm_dict = Vec::<u8>::new();

                let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
                    .unwrap()
                    .into_cell()
                    .unwrap();

                stack.push(StackItem::cell(cell.clone()));

                let mut res = Vec::<u8>::with_capacity(3);
                res.push(0xC7);
                res.push(0x3A);
                res.push(0x80);

                let code = SliceData::new(res);

                let mut engine = Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);

                let start = std::time::Instant::now();
                let _ = engine.execute();
                total_duration += start.elapsed();

            }
            total_duration
        })
    });
}

// Run  `cargo bench -p tvm_vm --bench benchmarks wasmtls_with_whitelist`
fn bench_wasmtls_with_whitelist(c: &mut Criterion) {
    c.bench_function("wasmtls_with_whitelist", |b| {
        let mut total_duration = Duration::default();
        b.iter_custom(|iters| {
            // b.iter(|| {
            //+++++ Let's say I want to measure execution time from this point.
            for _i in 0..iters {
                let mut stack = Stack::new();

                let hash_str =
                "9d8beddc8d81853d6ee2390ed141b69d67bf683afbbf45cf405848a78ae969fc";
                let hash: Vec<u8> = (0..hash_str.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
                    .collect::<Vec<u8>>();
                let cell =
                    TokenValue::write_bytes(hash.as_slice(), &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let tls_data: Vec<u8> = vec![231, 226, 189, 128, 175, 192, 46, 233, 160, 243, 227, 168, 186, 174, 207, 111, 124, 21, 6, 220, 18, 155, 18, 17, 39, 165, 203, 108, 109, 3, 40, 186, 1, 3, 22, 3, 1, 0, 158, 1, 0, 0, 154, 3, 3, 209, 131, 116, 114, 224, 15, 205, 152, 242, 176, 127, 8, 86, 5, 221, 80, 162, 132, 201, 168, 99, 134, 196, 185, 88, 35, 234, 76, 112, 108, 224, 149, 0, 0, 2, 19, 1, 1, 0, 0, 111, 0, 0, 0, 20, 0, 18, 0, 0, 15, 107, 97, 117, 116, 104, 46, 107, 97, 107, 97, 111, 46, 99, 111, 109, 0, 10, 0, 4, 0, 2, 0, 29, 0, 13, 0, 20, 0, 18, 4, 3, 8, 4, 4, 1, 5, 3, 8, 5, 5, 1, 8, 6, 6, 1, 2, 1, 0, 51, 0, 38, 0, 36, 0, 29, 0, 32, 192, 66, 56, 95, 6, 86, 129, 217, 28, 232, 5, 177, 109, 189, 139, 154, 6, 3, 215, 62, 202, 195, 214, 238, 231, 82, 157, 198, 107, 200, 81, 16, 0, 45, 0, 2, 1, 1, 0, 43, 0, 3, 2, 3, 4, 22, 3, 3, 0, 90, 2, 0, 0, 86, 3, 3, 243, 248, 14, 224, 190, 21, 30, 172, 110, 1, 251, 43, 1, 162, 190, 249, 110, 32, 65, 224, 72, 202, 34, 161, 37, 163, 24, 186, 18, 240, 11, 154, 0, 19, 1, 0, 0, 46, 0, 43, 0, 2, 3, 4, 0, 51, 0, 36, 0, 29, 0, 32, 224, 217, 73, 158, 17, 12, 43, 19, 230, 133, 14, 153, 249, 104, 67, 55, 131, 8, 74, 104, 200, 163, 38, 63, 103, 114, 35, 122, 111, 43, 109, 111, 23, 3, 3, 15, 223, 59, 241, 156, 165, 95, 67, 151, 76, 79, 170, 168, 136, 45, 166, 254, 155, 235, 77, 95, 64, 53, 87, 66, 69, 123, 104, 24, 59, 34, 224, 81, 108, 143, 124, 243, 51, 118, 127, 237, 78, 223, 181, 146, 178, 61, 124, 226, 194, 21, 156, 243, 194, 46, 51, 91, 205, 50, 30, 121, 33, 9, 16, 214, 15, 204, 31, 247, 213, 188, 200, 196, 12, 170, 247, 6, 229, 99, 8, 238, 144, 46, 59, 165, 29, 152, 5, 37, 186, 82, 159, 248, 96, 197, 196, 218, 115, 237, 19, 98, 19, 194, 108, 165, 199, 190, 28, 203, 4, 80, 49, 8, 154, 108, 205, 57, 209, 204, 109, 139, 134, 89, 243, 130, 61, 66, 116, 201, 177, 23, 132, 40, 188, 247, 45, 45, 139, 138, 34, 69, 241, 134, 248, 202, 188, 35, 18, 1, 221, 237, 204, 185, 42, 62, 190, 174, 63, 63, 57, 247, 218, 23, 244, 5, 80, 70, 218, 44, 244, 45, 116, 12, 144, 166, 173, 155, 6, 76, 112, 200, 133, 38, 174, 114, 92, 208, 233, 214, 134, 5, 223, 93, 17, 164, 75, 70, 14, 111, 241, 115, 253, 114, 61, 88, 15, 20, 59, 89, 33, 188, 150, 100, 173, 199, 27, 125, 253, 5, 198, 244, 142, 89, 145, 20, 141, 170, 2, 193, 29, 181, 149, 116, 12, 96, 21, 147, 154, 156, 147, 47, 16, 33, 59, 75, 180, 235, 28, 221, 229, 175, 171, 64, 216, 186, 216, 46, 91, 242, 83, 127, 60, 227, 227, 8, 9, 180, 71, 180, 73, 234, 177, 154, 14, 28, 108, 38, 129, 123, 37, 112, 106, 221, 29, 243, 254, 159, 8, 1, 29, 56, 64, 127, 152, 93, 70, 186, 201, 58, 27, 238, 212, 22, 145, 152, 101, 29, 21, 10, 51, 76, 171, 81, 41, 133, 104, 232, 200, 86, 29, 206, 123, 81, 18, 53, 122, 110, 212, 106, 117, 103, 143, 185, 15, 34, 113, 222, 116, 78, 35, 165, 163, 63, 137, 191, 146, 184, 147, 6, 149, 39, 202, 216, 168, 220, 214, 83, 87, 8, 255, 62, 149, 26, 125, 95, 147, 17, 24, 142, 143, 43, 74, 70, 147, 88, 92, 20, 207, 20, 55, 138, 106, 15, 240, 77, 228, 31, 58, 181, 73, 97, 235, 66, 202, 183, 136, 22, 51, 198, 245, 235, 31, 186, 55, 44, 227, 29, 238, 240, 34, 37, 168, 137, 45, 43, 163, 24, 235, 176, 203, 78, 128, 235, 231, 2, 32, 7, 17, 24, 251, 90, 58, 112, 183, 243, 229, 150, 195, 228, 60, 187, 125, 88, 167, 177, 108, 14, 30, 244, 112, 16, 2, 239, 44, 186, 193, 13, 205, 57, 31, 125, 125, 155, 94, 139, 184, 78, 106, 252, 51, 205, 134, 184, 39, 119, 193, 186, 163, 21, 157, 215, 18, 59, 164, 248, 203, 60, 163, 197, 47, 121, 123, 131, 183, 18, 234, 151, 22, 192, 160, 249, 232, 190, 49, 222, 208, 251, 153, 159, 23, 239, 50, 42, 175, 70, 61, 98, 84, 51, 220, 72, 110, 111, 174, 50, 87, 62, 7, 127, 77, 26, 40, 182, 37, 40, 231, 238, 217, 222, 97, 246, 255, 134, 65, 144, 218, 185, 63, 39, 148, 174, 148, 133, 6, 31, 114, 174, 179, 132, 170, 32, 218, 220, 47, 218, 131, 23, 138, 63, 107, 43, 252, 187, 167, 70, 13, 81, 100, 180, 12, 97, 161, 13, 239, 159, 96, 232, 237, 129, 170, 120, 62, 213, 227, 202, 178, 104, 254, 195, 15, 6, 175, 5, 92, 12, 202, 135, 243, 66, 103, 208, 131, 153, 83, 219, 160, 114, 114, 210, 163, 227, 9, 89, 240, 230, 97, 183, 75, 247, 158, 119, 104, 216, 202, 249, 231, 192, 146, 62, 242, 109, 175, 173, 208, 249, 132, 206, 121, 240, 187, 41, 157, 25, 25, 47, 23, 88, 250, 146, 159, 239, 77, 191, 188, 193, 198, 40, 6, 0, 57, 58, 206, 45, 37, 106, 246, 5, 38, 224, 122, 68, 26, 173, 135, 5, 84, 160, 32, 253, 103, 209, 75, 159, 17, 79, 143, 72, 93, 52, 170, 135, 8, 159, 98, 47, 147, 222, 117, 215, 159, 255, 206, 55, 207, 225, 214, 20, 216, 40, 189, 213, 223, 224, 186, 38, 102, 16, 170, 225, 147, 39, 94, 199, 224, 123, 146, 21, 0, 51, 26, 39, 35, 9, 21, 52, 242, 230, 188, 218, 205, 139, 107, 42, 239, 175, 61, 149, 74, 50, 162, 84, 167, 94, 135, 149, 200, 154, 187, 254, 95, 235, 90, 9, 18, 48, 171, 150, 146, 43, 42, 134, 132, 199, 55, 177, 1, 2, 181, 186, 109, 93, 57, 117, 110, 90, 80, 172, 154, 251, 209, 165, 136, 12, 101, 194, 131, 42, 2, 232, 196, 237, 209, 204, 36, 59, 148, 52, 23, 29, 80, 66, 201, 245, 222, 39, 31, 199, 194, 177, 68, 80, 244, 192, 220, 251, 3, 181, 39, 242, 209, 119, 231, 97, 25, 228, 152, 26, 48, 142, 153, 102, 126, 235, 140, 175, 78, 12, 52, 98, 105, 29, 171, 189, 217, 214, 249, 252, 83, 241, 255, 9, 18, 121, 37, 24, 181, 35, 110, 184, 218, 194, 97, 49, 229, 154, 206, 99, 159, 248, 104, 52, 171, 29, 177, 39, 145, 209, 78, 107, 234, 201, 85, 43, 112, 27, 140, 187, 110, 46, 222, 255, 124, 176, 164, 54, 235, 165, 134, 28, 131, 69, 106, 229, 229, 2, 197, 201, 140, 100, 132, 251, 180, 238, 233, 205, 104, 21, 172, 136, 236, 158, 39, 232, 63, 229, 211, 45, 179, 182, 61, 74, 68, 178, 67, 131, 37, 69, 5, 127, 158, 181, 2, 189, 65, 219, 233, 94, 125, 52, 38, 94, 234, 118, 66, 32, 84, 116, 196, 186, 230, 246, 52, 196, 218, 216, 38, 111, 89, 201, 243, 114, 193, 75, 64, 197, 168, 37, 35, 246, 92, 253, 12, 223, 8, 73, 152, 71, 140, 98, 225, 182, 98, 219, 35, 193, 180, 151, 103, 29, 156, 40, 118, 136, 81, 232, 222, 223, 92, 224, 17, 127, 48, 27, 3, 219, 215, 134, 178, 126, 192, 44, 182, 96, 80, 87, 39, 3, 44, 41, 82, 109, 59, 202, 94, 133, 51, 16, 125, 109, 245, 156, 1, 121, 123, 113, 102, 127, 131, 204, 210, 176, 223, 190, 159, 176, 34, 253, 23, 81, 142, 59, 192, 84, 129, 175, 183, 241, 82, 106, 19, 3, 119, 250, 119, 101, 170, 125, 187, 58, 233, 224, 84, 185, 115, 123, 143, 84, 154, 244, 6, 90, 86, 150, 15, 187, 82, 183, 133, 0, 243, 229, 19, 80, 102, 142, 36, 98, 41, 111, 25, 188, 233, 85, 85, 59, 223, 165, 10, 126, 204, 97, 235, 215, 15, 245, 180, 28, 238, 43, 79, 60, 159, 153, 217, 191, 190, 176, 101, 234, 147, 97, 193, 3, 116, 13, 120, 131, 248, 247, 90, 89, 186, 0, 99, 78, 128, 109, 161, 168, 152, 247, 116, 16, 28, 96, 159, 73, 39, 134, 131, 104, 37, 234, 74, 179, 172, 57, 38, 183, 137, 89, 218, 176, 71, 79, 58, 102, 182, 130, 17, 198, 60, 34, 7, 83, 211, 40, 146, 202, 171, 228, 187, 117, 94, 35, 198, 89, 107, 92, 155, 35, 158, 11, 198, 171, 21, 252, 239, 47, 252, 85, 86, 163, 71, 138, 249, 64, 64, 71, 233, 110, 255, 111, 250, 217, 218, 85, 26, 61, 11, 19, 237, 200, 177, 47, 83, 236, 147, 239, 87, 79, 100, 96, 215, 122, 219, 2, 22, 17, 225, 56, 28, 221, 44, 231, 24, 119, 234, 70, 229, 25, 32, 130, 110, 16, 215, 91, 109, 193, 177, 254, 173, 98, 188, 14, 197, 39, 137, 88, 113, 194, 185, 218, 98, 169, 64, 180, 231, 152, 244, 92, 85, 187, 141, 238, 23, 167, 139, 181, 140, 156, 81, 109, 158, 209, 97, 172, 201, 99, 130, 231, 174, 19, 216, 108, 104, 160, 120, 106, 30, 30, 150, 96, 109, 163, 82, 169, 5, 135, 142, 213, 50, 72, 39, 117, 42, 34, 197, 8, 100, 184, 92, 22, 64, 10, 39, 161, 226, 81, 173, 241, 86, 240, 252, 102, 11, 95, 95, 108, 98, 136, 150, 31, 48, 23, 197, 85, 33, 242, 151, 81, 209, 117, 211, 38, 228, 148, 163, 114, 75, 254, 21, 177, 7, 53, 129, 106, 64, 49, 72, 229, 33, 62, 88, 173, 185, 190, 26, 102, 19, 138, 17, 65, 211, 3, 246, 29, 54, 113, 84, 213, 160, 45, 151, 251, 0, 75, 85, 185, 138, 213, 46, 240, 243, 214, 255, 192, 51, 188, 188, 194, 32, 113, 208, 69, 125, 61, 215, 134, 236, 187, 81, 252, 33, 184, 106, 169, 66, 241, 66, 189, 26, 53, 212, 139, 84, 248, 215, 85, 181, 18, 171, 71, 242, 162, 157, 232, 212, 180, 8, 235, 222, 188, 90, 62, 106, 201, 132, 196, 3, 84, 76, 52, 169, 161, 255, 193, 235, 84, 3, 40, 3, 141, 90, 48, 133, 61, 127, 109, 154, 14, 12, 247, 48, 152, 192, 122, 247, 115, 189, 115, 92, 28, 117, 57, 175, 133, 253, 223, 79, 19, 132, 12, 43, 92, 214, 66, 166, 179, 89, 16, 188, 4, 38, 8, 157, 186, 24, 116, 225, 230, 180, 75, 62, 170, 151, 91, 140, 93, 4, 40, 176, 60, 195, 120, 110, 193, 95, 3, 70, 94, 161, 184, 248, 152, 213, 152, 128, 25, 226, 230, 216, 83, 115, 176, 110, 126, 194, 141, 8, 187, 125, 50, 56, 3, 216, 88, 175, 98, 216, 176, 185, 219, 42, 230, 54, 127, 149, 152, 135, 116, 150, 26, 206, 149, 232, 194, 245, 42, 135, 45, 171, 76, 38, 239, 178, 5, 67, 75, 104, 181, 34, 111, 134, 191, 33, 210, 252, 30, 200, 145, 142, 210, 85, 240, 103, 23, 149, 34, 225, 29, 7, 228, 134, 228, 215, 99, 43, 146, 9, 240, 147, 65, 165, 192, 26, 141, 29, 112, 189, 8, 174, 139, 60, 20, 235, 7, 88, 233, 5, 207, 101, 16, 55, 92, 90, 145, 55, 209, 252, 142, 43, 160, 74, 148, 84, 2, 115, 219, 163, 84, 103, 20, 5, 17, 92, 91, 170, 199, 133, 45, 197, 26, 105, 72, 201, 188, 134, 127, 124, 212, 128, 127, 188, 48, 69, 220, 97, 163, 71, 203, 88, 177, 49, 176, 250, 29, 226, 27, 221, 79, 87, 115, 102, 35, 105, 38, 23, 225, 243, 215, 120, 79, 29, 252, 113, 78, 82, 22, 82, 120, 120, 4, 15, 9, 76, 254, 14, 2, 7, 146, 150, 34, 228, 218, 175, 112, 132, 137, 13, 237, 47, 213, 54, 240, 203, 91, 215, 185, 234, 65, 37, 248, 78, 206, 108, 58, 48, 181, 171, 127, 6, 17, 37, 224, 154, 177, 216, 5, 118, 30, 249, 29, 136, 39, 78, 109, 112, 253, 118, 10, 32, 238, 28, 185, 171, 35, 232, 202, 120, 186, 254, 232, 137, 77, 42, 106, 12, 68, 37, 120, 124, 203, 224, 49, 66, 143, 126, 154, 84, 4, 167, 174, 34, 75, 96, 253, 98, 123, 227, 150, 240, 114, 80, 105, 57, 3, 150, 224, 151, 227, 180, 52, 63, 16, 162, 71, 213, 43, 121, 234, 242, 2, 70, 174, 82, 27, 36, 228, 15, 165, 138, 137, 152, 22, 125, 126, 115, 131, 91, 29, 129, 126, 194, 114, 117, 93, 247, 31, 239, 157, 182, 151, 243, 54, 112, 173, 66, 138, 235, 226, 113, 58, 133, 173, 205, 200, 196, 245, 157, 0, 244, 44, 127, 224, 166, 240, 151, 252, 188, 43, 85, 17, 82, 73, 198, 101, 140, 230, 186, 84, 112, 158, 59, 53, 164, 131, 219, 85, 125, 49, 1, 215, 168, 94, 41, 34, 192, 121, 101, 187, 65, 107, 124, 254, 12, 76, 121, 68, 68, 125, 199, 225, 38, 94, 211, 212, 250, 107, 208, 74, 13, 0, 59, 126, 232, 213, 60, 205, 89, 208, 224, 222, 219, 97, 141, 1, 219, 63, 117, 31, 238, 23, 110, 128, 171, 188, 235, 211, 210, 78, 184, 98, 132, 242, 182, 4, 66, 78, 228, 190, 151, 6, 135, 158, 123, 144, 20, 208, 130, 203, 104, 211, 203, 195, 140, 66, 31, 64, 159, 221, 130, 246, 169, 187, 157, 172, 102, 13, 99, 251, 207, 240, 23, 245, 184, 244, 247, 74, 244, 77, 51, 207, 139, 82, 228, 84, 125, 77, 81, 145, 86, 202, 31, 176, 178, 206, 149, 10, 83, 119, 28, 124, 176, 97, 247, 35, 182, 178, 138, 47, 113, 81, 63, 224, 77, 252, 119, 149, 208, 230, 167, 89, 109, 123, 118, 65, 41, 240, 35, 127, 234, 135, 42, 110, 205, 49, 5, 188, 209, 232, 169, 168, 241, 137, 15, 30, 104, 53, 21, 248, 137, 178, 215, 243, 41, 178, 113, 105, 94, 157, 123, 39, 16, 49, 122, 228, 31, 130, 104, 228, 136, 220, 205, 173, 140, 163, 37, 189, 92, 105, 145, 83, 244, 89, 215, 74, 226, 148, 142, 166, 150, 120, 239, 128, 161, 18, 71, 10, 161, 172, 213, 119, 20, 94, 69, 84, 170, 241, 241, 115, 126, 175, 100, 117, 156, 199, 27, 130, 179, 99, 16, 10, 205, 73, 7, 212, 207, 4, 66, 172, 202, 14, 152, 157, 197, 115, 145, 73, 217, 79, 137, 193, 183, 73, 248, 125, 86, 196, 141, 184, 104, 189, 173, 19, 255, 3, 6, 49, 180, 33, 47, 225, 192, 249, 180, 161, 191, 229, 4, 112, 124, 137, 255, 239, 200, 197, 98, 127, 176, 89, 61, 116, 108, 8, 17, 204, 145, 186, 251, 100, 243, 219, 152, 53, 119, 155, 5, 11, 242, 7, 78, 207, 128, 144, 115, 134, 203, 132, 88, 159, 175, 213, 126, 176, 174, 141, 120, 204, 21, 217, 83, 146, 146, 63, 213, 33, 157, 43, 109, 104, 79, 208, 156, 193, 122, 123, 91, 230, 212, 134, 178, 159, 199, 72, 6, 206, 191, 95, 233, 57, 132, 215, 131, 188, 95, 178, 253, 43, 190, 129, 189, 198, 179, 225, 114, 234, 79, 174, 83, 62, 219, 136, 255, 81, 174, 232, 221, 4, 14, 111, 167, 229, 129, 86, 164, 15, 233, 56, 33, 241, 91, 147, 199, 112, 116, 109, 54, 146, 33, 60, 121, 207, 5, 242, 95, 145, 117, 68, 178, 131, 57, 46, 242, 211, 63, 100, 46, 68, 68, 77, 84, 97, 64, 50, 75, 19, 181, 78, 210, 72, 220, 214, 173, 243, 233, 97, 29, 8, 59, 18, 210, 17, 4, 128, 209, 67, 215, 234, 80, 17, 28, 168, 226, 54, 0, 193, 88, 107, 138, 34, 39, 45, 121, 225, 93, 12, 96, 113, 33, 223, 135, 150, 21, 153, 1, 173, 122, 127, 121, 183, 105, 151, 41, 122, 165, 143, 62, 104, 161, 165, 85, 190, 223, 140, 186, 51, 58, 249, 228, 216, 202, 48, 181, 179, 34, 93, 214, 212, 17, 195, 200, 235, 236, 23, 5, 114, 82, 79, 106, 255, 146, 203, 132, 94, 58, 93, 25, 68, 70, 92, 96, 117, 42, 231, 12, 235, 225, 33, 89, 107, 118, 151, 40, 120, 234, 102, 156, 225, 255, 239, 176, 91, 23, 197, 160, 157, 148, 120, 90, 131, 202, 103, 113, 152, 181, 144, 37, 150, 174, 73, 157, 240, 208, 102, 25, 241, 141, 235, 153, 229, 59, 170, 52, 160, 96, 24, 202, 1, 153, 177, 76, 195, 142, 222, 118, 35, 142, 177, 170, 252, 168, 188, 180, 90, 226, 233, 225, 61, 27, 204, 219, 175, 61, 189, 199, 133, 13, 74, 69, 99, 70, 83, 37, 120, 134, 226, 216, 240, 70, 135, 238, 83, 9, 200, 134, 146, 63, 49, 122, 240, 197, 14, 71, 87, 110, 222, 173, 153, 152, 25, 201, 169, 165, 91, 77, 4, 84, 196, 44, 143, 226, 89, 150, 58, 124, 235, 121, 45, 16, 155, 138, 61, 216, 46, 2, 166, 250, 39, 220, 139, 92, 191, 70, 140, 195, 161, 76, 47, 196, 9, 69, 208, 135, 48, 86, 165, 127, 146, 113, 212, 162, 51, 92, 23, 75, 254, 185, 239, 250, 15, 205, 127, 78, 4, 102, 250, 128, 188, 186, 215, 186, 212, 136, 74, 169, 39, 182, 113, 81, 232, 82, 197, 219, 177, 158, 45, 83, 138, 65, 152, 138, 192, 70, 88, 144, 28, 244, 96, 54, 152, 37, 19, 216, 235, 22, 210, 198, 213, 247, 99, 194, 105, 154, 39, 94, 96, 194, 73, 160, 142, 125, 3, 201, 219, 157, 44, 1, 87, 90, 77, 32, 147, 149, 109, 25, 6, 116, 11, 122, 227, 126, 237, 181, 207, 34, 164, 67, 163, 141, 221, 228, 94, 195, 157, 97, 245, 223, 148, 132, 78, 98, 172, 53, 113, 3, 131, 130, 100, 23, 42, 94, 7, 219, 232, 203, 208, 190, 104, 21, 215, 215, 20, 158, 144, 89, 117, 255, 125, 9, 157, 67, 41, 16, 86, 205, 168, 168, 164, 42, 68, 21, 34, 188, 222, 104, 188, 231, 1, 45, 9, 157, 214, 24, 102, 130, 156, 245, 174, 96, 63, 39, 143, 207, 112, 36, 106, 228, 240, 158, 13, 78, 171, 109, 178, 138, 239, 206, 150, 70, 37, 133, 117, 143, 33, 7, 44, 1, 65, 145, 224, 238, 220, 52, 129, 136, 127, 178, 253, 77, 79, 245, 240, 105, 57, 164, 225, 9, 241, 126, 126, 106, 66, 216, 22, 0, 189, 247, 114, 21, 172, 2, 117, 31, 118, 58, 74, 216, 162, 30, 99, 137, 87, 136, 105, 158, 224, 6, 121, 167, 134, 237, 144, 122, 119, 53, 4, 15, 109, 240, 57, 194, 226, 157, 68, 7, 226, 224, 32, 96, 167, 54, 53, 181, 161, 62, 61, 163, 211, 74, 107, 210, 60, 203, 104, 140, 213, 113, 91, 101, 223, 169, 74, 49, 241, 219, 146, 37, 75, 30, 35, 123, 145, 90, 212, 29, 206, 93, 119, 81, 104, 22, 158, 194, 254, 207, 25, 13, 195, 130, 30, 168, 194, 120, 2, 57, 193, 112, 159, 181, 74, 137, 11, 70, 90, 160, 67, 76, 111, 185, 132, 228, 174, 1, 37, 102, 206, 25, 155, 45, 233, 70, 103, 37, 45, 197, 87, 248, 163, 132, 162, 184, 230, 181, 38, 236, 178, 80, 184, 182, 227, 197, 193, 29, 100, 188, 114, 153, 230, 208, 225, 139, 161, 220, 167, 188, 155, 139, 244, 117, 15, 209, 148, 135, 81, 55, 32, 63, 95, 211, 241, 78, 247, 203, 98, 165, 237, 156, 41, 152, 132, 60, 191, 185, 144, 13, 147, 196, 80, 41, 222, 66, 226, 193, 74, 227, 70, 158, 83, 105, 114, 18, 102, 8, 135, 253, 83, 165, 228, 106, 245, 255, 77, 77, 209, 20, 248, 175, 88, 72, 36, 198, 225, 75, 210, 121, 113, 213, 162, 119, 181, 228, 220, 35, 71, 74, 157, 177, 81, 175, 119, 235, 199, 73, 107, 56, 106, 241, 32, 57, 33, 36, 23, 200, 162, 177, 207, 26, 135, 122, 50, 55, 219, 210, 139, 168, 85, 124, 121, 50, 3, 64, 133, 162, 232, 39, 217, 46, 228, 225, 7, 17, 85, 9, 20, 1, 160, 228, 235, 47, 109, 133, 82, 228, 192, 184, 17, 152, 43, 13, 30, 149, 87, 213, 195, 26, 123, 94, 221, 134, 249, 131, 42, 52, 182, 79, 84, 212, 13, 190, 36, 22, 84, 171, 191, 218, 129, 128, 178, 24, 71, 8, 3, 144, 159, 154, 202, 47, 233, 249, 126, 212, 37, 247, 19, 104, 108, 94, 153, 140, 22, 54, 40, 45, 146, 197, 177, 208, 123, 161, 0, 148, 54, 113, 145, 23, 62, 182, 41, 100, 105, 127, 70, 176, 166, 25, 1, 60, 143, 208, 51, 77, 211, 184, 163, 9, 219, 116, 230, 196, 100, 183, 176, 91, 249, 73, 119, 0, 177, 242, 220, 242, 150, 237, 178, 194, 233, 11, 190, 92, 71, 30, 65, 72, 65, 116, 106, 5, 43, 238, 59, 31, 147, 188, 78, 141, 42, 53, 33, 62, 189, 90, 65, 70, 138, 103, 24, 109, 15, 52, 81, 253, 216, 45, 205, 198, 218, 29, 254, 147, 60, 164, 46, 210, 213, 110, 80, 213, 12, 142, 139, 17, 144, 50, 181, 117, 1, 29, 246, 137, 58, 108, 108, 15, 237, 49, 172, 218, 56, 183, 113, 206, 36, 223, 173, 132, 33, 136, 52, 53, 229, 136, 5, 65, 157, 244, 74, 137, 19, 144, 253, 188, 48, 22, 92, 173, 10, 119, 57, 88, 83, 217, 116, 229, 139, 119, 87, 54, 84, 102, 93, 150, 75, 117, 176, 145, 244, 131, 167, 89, 129, 28, 136, 219, 245, 234, 60, 151, 202, 188, 162, 172, 145, 213, 99, 29, 125, 49, 140, 61, 255, 94, 128, 67, 193, 23, 73, 225, 254, 154, 62, 107, 218, 59, 59, 78, 63, 37, 194, 196, 128, 247, 230, 50, 238, 216, 45, 191, 5, 204, 21, 82, 128, 114, 78, 209, 255, 84, 96, 153, 17, 198, 205, 183, 201, 109, 82, 6, 149, 133, 208, 225, 82, 179, 81, 68, 144, 189, 134, 129, 250, 229, 193, 166, 143, 74, 92, 133, 233, 5, 7, 210, 19, 76, 56, 109, 104, 224, 103, 248, 36, 209, 139, 161, 69, 213, 113, 108, 6, 6, 158, 157, 215, 87, 176, 205, 144, 135, 140, 45, 70, 228, 139, 196, 59, 132, 153, 83, 201, 107, 112, 240, 197, 235, 198, 248, 33, 156, 102, 218, 105, 244, 127, 252, 228, 79, 112, 171, 165, 85, 150, 69, 161, 183, 202, 154, 161, 252, 167, 176, 173, 37, 81, 82, 41, 105, 145, 107, 178, 38, 200, 215, 32, 135, 4, 83, 25, 164, 234, 83, 29, 22, 132, 209, 111, 49, 98, 92, 247, 160, 152, 172, 125, 47, 49, 72, 96, 157, 157, 238, 110, 69, 151, 59, 185, 209, 17, 67, 25, 248, 229, 129, 10, 29, 227, 49, 231, 6, 62, 48, 220, 226, 158, 89, 51, 240, 62, 243, 68, 25, 250, 56, 137, 39, 4, 152, 182, 204, 186, 111, 106, 71, 151, 169, 48, 123, 203, 84, 6, 45, 81, 155, 182, 84, 78, 171, 244, 248, 46, 186, 221, 83, 153, 50, 191, 107, 72, 63, 194, 141, 30, 177, 7, 146, 192, 232, 165, 147, 179, 56, 221, 84, 144, 213, 158, 252, 174, 23, 65, 66, 231, 183, 254, 254, 234, 11, 88, 41, 251, 0, 220, 190, 51, 73, 158, 21, 20, 249, 201, 51, 17, 201, 22, 114, 244, 89, 126, 82, 184, 165, 196, 13, 144, 116, 254, 195, 201, 106, 164, 1, 209, 162, 218, 157, 87, 58, 250, 192, 85, 112, 169, 194, 109, 92, 193, 126, 33, 101, 152, 4, 109, 134, 119, 73, 252, 237, 49, 83, 155, 56, 137, 196, 234, 29, 172, 31, 107, 88, 85, 40, 234, 151, 200, 196, 69, 94, 254, 122, 134, 172, 99, 135, 128, 92, 42, 15, 216, 120, 165, 33, 242, 139, 4, 251, 231, 253, 173, 120, 46, 5, 130, 161, 65, 155, 158, 35, 47, 241, 53, 217, 49, 234, 129, 231, 27, 24, 41, 173, 175, 132, 122, 47, 62, 35, 194, 249, 158, 190, 234, 241, 169, 62, 178, 162, 186, 151, 13, 60, 195, 177, 135, 102, 187, 224, 182, 138, 70, 40, 229, 219, 150, 208, 173, 237, 248, 233, 192, 67, 161, 241, 105, 230, 75, 199, 9, 126, 153, 86, 224, 163, 230, 108, 173, 166, 192, 4, 213, 24, 53, 112, 26, 228, 145, 104, 122, 116, 75, 56, 1, 94, 209, 5, 105, 223, 188, 9, 71, 201, 166, 93, 236, 110, 71, 183, 53, 234, 121, 155, 171, 131, 188, 230, 93, 136, 154, 5, 36, 143, 224, 153, 57, 57, 247, 226, 0, 162, 178, 221, 235, 166, 1, 189, 242, 211, 234, 162, 164, 8, 155, 24, 77, 142, 35, 17, 255, 145, 140, 181, 251, 12, 141, 192, 17, 117, 208, 51, 161, 166, 143, 51, 26, 164, 254, 97, 20, 196, 75, 111, 186, 181, 58, 81, 115, 29, 185, 60, 221, 60, 37, 77, 135, 134, 240, 145, 132, 174, 59, 76, 53, 124, 17, 39, 230, 235, 203, 23, 3, 3, 0, 98, 155, 183, 154, 36, 156, 238, 133, 141, 73, 196, 197, 111, 179, 233, 48, 198, 215, 186, 166, 187, 137, 201, 81, 128, 64, 247, 146, 71, 211, 225, 163, 37, 121, 214, 57, 73, 6, 155, 201, 199, 14, 48, 64, 198, 98, 24, 240, 56, 151, 145, 207, 34, 68, 132, 20, 78, 208, 238, 107, 75, 108, 192, 230, 142, 31, 222, 172, 226, 58, 61, 176, 255, 215, 248, 184, 239, 246, 207, 193, 64, 125, 100, 201, 220, 80, 39, 139, 55, 139, 250, 102, 175, 246, 21, 237, 22, 251, 155, 23, 3, 3, 1, 10, 89, 74, 203, 38, 228, 17, 135, 240, 110, 213, 190, 108, 128, 143, 51, 214, 49, 33, 222, 19, 248, 237, 198, 86, 122, 133, 184, 66, 56, 138, 2, 216, 83, 195, 155, 143, 216, 203, 246, 5, 17, 91, 179, 144, 77, 18, 118, 229, 188, 242, 23, 18, 117, 109, 48, 101, 57, 99, 132, 213, 119, 143, 100, 43, 11, 153, 229, 244, 228, 40, 154, 245, 3, 146, 81, 56, 70, 94, 79, 229, 65, 196, 69, 147, 174, 20, 55, 60, 212, 247, 20, 234, 159, 149, 124, 15, 150, 141, 209, 222, 140, 253, 139, 31, 168, 6, 230, 12, 46, 148, 221, 52, 39, 90, 138, 77, 16, 223, 40, 186, 1, 175, 140, 203, 145, 36, 173, 69, 215, 173, 242, 115, 104, 108, 47, 21, 45, 38, 61, 191, 181, 17, 66, 72, 76, 165, 18, 222, 131, 230, 86, 84, 192, 101, 85, 213, 254, 88, 29, 51, 157, 239, 37, 38, 100, 22, 54, 213, 94, 219, 131, 228, 167, 81, 195, 187, 83, 182, 67, 90, 31, 134, 210, 102, 158, 8, 169, 79, 138, 208, 208, 255, 214, 84, 35, 17, 189, 123, 46, 178, 165, 120, 188, 171, 19, 218, 144, 95, 56, 209, 71, 74, 238, 75, 94, 236, 247, 192, 209, 18, 208, 39, 146, 123, 81, 22, 226, 41, 138, 255, 161, 184, 77, 42, 209, 95, 239, 15, 232, 32, 75, 156, 210, 36, 190, 68, 223, 236, 77, 108, 66, 120, 179, 184, 237, 30, 245, 198, 219, 64, 169, 28, 37, 224, 189, 90, 23, 3, 3, 1, 10, 221, 239, 235, 41, 136, 100, 200, 246, 92, 80, 87, 142, 70, 113, 197, 225, 124, 204, 139, 250, 142, 201, 52, 187, 129, 164, 58, 186, 250, 111, 208, 61, 150, 74, 157, 173, 164, 69, 7, 147, 132, 216, 184, 162, 67, 156, 43, 167, 53, 7, 130, 127, 254, 79, 190, 220, 117, 51, 72, 86, 222, 131, 70, 231, 66, 51, 60, 49, 129, 183, 177, 149, 217, 207, 108, 126, 208, 14, 91, 53, 46, 84, 95, 75, 33, 17, 76, 33, 98, 178, 122, 234, 72, 250, 153, 214, 50, 249, 208, 224, 61, 140, 46, 87, 96, 249, 4, 191, 236, 31, 189, 222, 108, 217, 0, 48, 224, 161, 161, 164, 55, 139, 191, 244, 145, 222, 12, 235, 175, 212, 227, 44, 109, 144, 123, 51, 206, 172, 183, 231, 63, 140, 64, 191, 165, 89, 248, 247, 245, 72, 94, 59, 108, 12, 111, 36, 38, 46, 221, 0, 153, 141, 213, 98, 100, 255, 101, 93, 132, 179, 169, 63, 241, 115, 62, 109, 80, 118, 206, 133, 3, 34, 62, 72, 79, 161, 253, 133, 74, 243, 143, 232, 110, 60, 155, 74, 33, 202, 198, 47, 212, 43, 46, 199, 148, 154, 214, 31, 141, 205, 147, 158, 94, 201, 189, 14, 135, 101, 10, 149, 91, 189, 100, 77, 150, 113, 64, 9, 107, 54, 113, 77, 235, 36, 144, 147, 48, 245, 231, 58, 235, 44, 164, 47, 119, 248, 177, 175, 62, 228, 207, 62, 25, 9, 70, 144, 68, 191, 10, 214, 124, 14, 14, 80, 253, 81, 23, 3, 3, 4, 232, 242, 239, 86, 125, 133, 14, 232, 181, 1, 110, 98, 16, 185, 31, 122, 71, 139, 111, 179, 159, 197, 228, 37, 36, 128, 118, 36, 46, 64, 35, 78, 31, 23, 112, 54, 129, 195, 219, 40, 158, 246, 172, 254, 221, 226, 66, 144, 56, 107, 64, 197, 216, 204, 42, 233, 21, 142, 140, 162, 83, 196, 33, 49, 84, 54, 134, 183, 82, 224, 11, 90, 90, 2, 198, 74, 77, 223, 112, 200, 27, 133, 32, 191, 119, 100, 19, 181, 204, 173, 24, 211, 56, 182, 107, 207, 133, 119, 201, 109, 205, 154, 186, 124, 143, 62, 121, 179, 227, 235, 85, 211, 184, 95, 177, 166, 202, 107, 130, 158, 247, 71, 17, 206, 226, 131, 119, 66, 236, 213, 29, 84, 43, 193, 6, 222, 16, 50, 61, 108, 112, 228, 20, 41, 215, 237, 86, 103, 104, 101, 88, 203, 120, 255, 56, 2, 192, 138, 37, 66, 56, 44, 49, 253, 20, 98, 149, 159, 200, 79, 201, 154, 196, 23, 41, 121, 87, 176, 211, 251, 205, 47, 91, 210, 22, 248, 121, 193, 63, 122, 151, 5, 45, 213, 20, 83, 225, 46, 105, 29, 6, 69, 162, 150, 168, 127, 128, 128, 63, 102, 195, 229, 171, 142, 183, 111, 203, 125, 50, 81, 204, 128, 71, 122, 185, 151, 204, 133, 80, 187, 8, 155, 78, 197, 105, 85, 162, 47, 89, 122, 108, 201, 130, 52, 222, 224, 155, 25, 172, 81, 182, 205, 207, 195, 210, 171, 19, 182, 166, 156, 55, 42, 192, 144, 19, 63, 70, 64, 215, 51, 132, 104, 47, 113, 145, 46, 227, 68, 235, 92, 138, 201, 141, 255, 168, 172, 47, 28, 190, 97, 225, 219, 156, 205, 41, 223, 191, 126, 58, 21, 217, 201, 156, 27, 129, 136, 148, 139, 155, 108, 68, 98, 175, 71, 63, 11, 207, 98, 83, 106, 99, 119, 164, 241, 5, 136, 244, 195, 33, 132, 230, 181, 244, 105, 162, 56, 150, 3, 74, 197, 192, 11, 5, 88, 90, 145, 164, 47, 21, 169, 134, 82, 148, 173, 217, 130, 51, 172, 47, 253, 41, 43, 179, 200, 218, 171, 102, 192, 130, 32, 45, 178, 207, 22, 37, 92, 118, 202, 248, 140, 244, 244, 74, 152, 223, 201, 21, 140, 234, 199, 81, 244, 74, 88, 55, 83, 13, 193, 238, 250, 190, 21, 250, 69, 180, 81, 254, 236, 173, 15, 73, 39, 209, 215, 171, 83, 130, 142, 68, 8, 217, 208, 81, 234, 23, 137, 103, 13, 63, 251, 122, 228, 255, 38, 238, 31, 7, 26, 124, 18, 230, 216, 119, 16, 157, 41, 246, 109, 208, 184, 222, 16, 29, 206, 180, 0, 22, 184, 142, 196, 49, 120, 68, 193, 20, 216, 0, 107, 40, 77, 243, 36, 49, 164, 44, 79, 70, 23, 152, 25, 248, 51, 57, 206, 202, 206, 78, 75, 112, 27, 93, 173, 115, 90, 231, 207, 90, 219, 27, 221, 204, 252, 148, 44, 194, 184, 153, 35, 129, 62, 220, 237, 178, 52, 228, 97, 247, 140, 94, 161, 134, 69, 164, 184, 206, 98, 165, 127, 44, 17, 41, 185, 106, 42, 80, 100, 200, 62, 53, 124, 235, 122, 5, 184, 240, 43, 82, 77, 163, 161, 151, 26, 175, 172, 84, 214, 14, 181, 179, 237, 136, 86, 98, 173, 228, 40, 41, 209, 147, 234, 175, 93, 208, 251, 86, 114, 108, 161, 16, 25, 208, 202, 66, 104, 163, 2, 198, 230, 104, 3, 80, 42, 127, 53, 199, 154, 236, 19, 243, 39, 163, 74, 173, 245, 141, 84, 170, 234, 99, 126, 197, 194, 248, 45, 233, 105, 104, 22, 116, 158, 229, 37, 13, 39, 4, 81, 165, 16, 141, 12, 159, 183, 171, 185, 158, 242, 50, 234, 186, 92, 93, 35, 42, 85, 149, 162, 20, 92, 90, 71, 202, 184, 9, 193, 138, 191, 15, 193, 150, 159, 25, 244, 38, 58, 170, 237, 164, 189, 15, 81, 29, 209, 231, 75, 158, 206, 254, 174, 216, 96, 167, 137, 207, 54, 234, 206, 207, 77, 7, 251, 76, 161, 79, 164, 184, 142, 28, 178, 202, 179, 217, 45, 83, 184, 174, 95, 100, 65, 40, 134, 196, 1, 49, 151, 8, 210, 149, 5, 197, 20, 111, 211, 18, 234, 124, 181, 24, 45, 92, 22, 122, 95, 165, 28, 98, 92, 68, 199, 76, 27, 167, 154, 7, 124, 220, 3, 229, 40, 117, 126, 252, 86, 32, 22, 18, 86, 195, 39, 20, 91, 49, 244, 148, 6, 166, 186, 43, 223, 210, 31, 181, 100, 201, 163, 175, 144, 238, 137, 127, 207, 179, 24, 83, 130, 169, 203, 53, 224, 213, 214, 144, 232, 145, 190, 114, 80, 82, 174, 100, 236, 217, 159, 159, 177, 1, 66, 24, 222, 154, 174, 104, 1, 225, 209, 118, 164, 89, 25, 95, 189, 97, 242, 192, 114, 105, 255, 210, 61, 140, 181, 30, 87, 226, 167, 135, 24, 249, 100, 199, 36, 159, 228, 204, 201, 44, 217, 105, 167, 204, 16, 95, 4, 188, 98, 184, 227, 159, 14, 87, 144, 182, 31, 134, 129, 143, 161, 216, 196, 242, 167, 124, 243, 173, 192, 202, 196, 132, 218, 22, 59, 242, 174, 188, 82, 103, 87, 212, 114, 206, 238, 2, 116, 8, 74, 110, 8, 243, 34, 249, 141, 115, 149, 232, 103, 251, 108, 101, 86, 62, 186, 111, 13, 109, 27, 143, 32, 114, 190, 236, 54, 156, 0, 248, 135, 114, 29, 19, 175, 163, 104, 75, 125, 196, 156, 217, 98, 84, 187, 26, 231, 25, 147, 154, 92, 93, 175, 32, 65, 227, 14, 159, 28, 233, 143, 241, 127, 167, 59, 207, 90, 39, 45, 94, 13, 0, 192, 104, 34, 33, 233, 42, 45, 248, 93, 218, 184, 95, 41, 84, 19, 225, 199, 112, 45, 75, 142, 111, 88, 177, 208, 95, 13, 156, 7, 157, 160, 1, 100, 51, 120, 41, 109, 98, 109, 246, 230, 211, 28, 134, 229, 253, 148, 124, 107, 217, 9, 33, 221, 72, 214, 251, 137, 3, 213, 168, 151, 3, 151, 241, 220, 55, 117, 244, 41, 73, 3, 78, 167, 38, 70, 103, 152, 235, 215, 113, 238, 27, 40, 81, 113, 62, 71, 50, 182, 218, 38, 152, 244, 227, 52, 13, 98, 182, 70, 175, 214, 7, 240, 8, 240, 92, 30, 106, 145, 182, 60, 10, 125, 51, 207, 229, 145, 112, 149, 83, 144, 147, 183, 176, 17, 16, 161, 119, 70, 32, 200, 22, 207, 198, 60, 145, 247, 92, 90, 13, 12, 253, 138, 46, 212, 223, 108, 164, 40, 189, 63, 54, 66, 213, 244, 133, 125, 190, 230, 14, 192, 145, 222, 239, 133, 40, 202, 102, 170, 11, 153, 130, 130, 134, 98, 243, 207, 200, 20, 58, 125, 146, 143, 245, 14, 189, 21, 12, 221, 88, 206, 191, 228, 132, 90, 44, 72, 236, 189, 98, 69, 213, 26, 185, 60, 100, 217, 198, 2, 249, 11, 189, 83, 111, 221, 229, 131, 89, 13, 71, 97, 197, 141, 217, 150, 178, 128, 165, 42, 157, 138, 115, 255, 234, 234, 121, 248, 19, 10, 248, 92, 183, 115, 183, 168, 167, 240, 112, 123, 220, 203, 83, 173, 255, 236, 103, 212, 63, 18, 44, 179, 102, 126, 91, 11, 241, 192, 94, 193, 120, 189, 95, 233, 190, 155, 59, 103, 203, 89, 113, 75, 111, 255, 119, 116, 31, 85, 82, 142, 112, 152, 112];
                let cell = TokenValue::write_bytes(&tls_data, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let cert: Vec<u8> = hex::decode("055b308205573082033fa003020102020d0203e5936f31b01349886ba217300d06092a864886f70d01010c05003047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f74205231301e170d3136303632323030303030305a170d3336303632323030303030305a3047310b300906035504061302555331223020060355040a1319476f6f676c65205472757374205365727669636573204c4c43311430120603550403130b47545320526f6f7420523130820222300d06092a864886f70d01010105000382020f003082020a0282020100b611028b1ee3a1779b3bdcbf943eb795a7403ca1fd82f97d32068271f6f68c7ffbe8dbbc6a2e9797a38c4bf92bf6b1f9ce841db1f9c597deefb9f2a3e9bc12895ea7aa52abf82327cba4b19c63dbd7997ef00a5eeb68a6f4c65a470d4d1033e34eb113a3c8186c4becfc0990df9d6429252307a1b4d23d2e60e0cfd20987bbcd48f04dc2c27a888abbbacf5919d6af8fb007b09e31f182c1c0df2ea66d6c190eb5d87e261a45033db079a49428ad0f7f26e5a808fe96e83c689453ee833a882b159609b2e07a8c2e75d69ceba756648f964f68ae3d97c2848fc0bc40c00b5cbdf687b3356cac18507f84e04ccd92d320e933bc5299af32b529b3252ab448f972e1ca64f7e682108de89dc28a88fa38668afc63f901f978fd7b5c77fa7687faecdfb10e799557b4bd26efd601d1eb160abb8e0bb5c5c58a55abd3acea914b29cc19a432254e2af16544d002ceaace49b4ea9f7c83b0407be743aba76ca38f7d8981fa4ca5ffd58ec3ce4be0b5d8b38e45cf76c0ed402bfd530fb0a7d53b0db18aa203de31adcc77ea6f7b3ed6df912212e6befad832fc1063145172de5dd61693bd296833ef3a66ec078a26df13d757657827de5e491400a2007f9aa821b6a9b195b0a5b90d1611dac76c483c40e07e0d5acd563cd19705b9cb4bed394b9cc43fd255136e24b0d671faf4c1bacced1bf5fe8141d800983d3ac8ae7a98371805950203010001a3423040300e0603551d0f0101ff040403020186300f0603551d130101ff040530030101ff301d0603551d0e04160414e4af2b26711a2b4827852f52662ceff08913713e300d06092a864886f70d01010c050003820201009faa4226db0b9bbeff1e96922e3ea2654a6a98ba22cb7dc13ad8820a06c6f6a5dec04e876679a1f9a6589caaf9b5e660e7e0e8b11e4241330b373dce897015cab524a8cf6bb5d2402198cf2234cf3bc52284e0c50e8a7c5d88e43524ce9b3e1a541e6edbb287a7fcf3fa815514620a59a92205313e82d6eedb5734bc3395d3171be827a28b7b4e261a7a5a64b6d1ac37f1fda0f338ec72f011759dcb34528de6766b17c6df86ab278e492b7566811021a6ea3ef4ae25ff7c15dece8c253fca62700af72f096607c83f1cfcf0db4530df6288c1b50f9dc39f4ade595947c5872236e682a7ed0ab9e207a08d7b7a4a3c71d2e203a11f3207dd1be442ce0c00456180b50b20592978bdf955cb63c53c4cf4b6ffdb6a5f316b999e2cc16b50a4d7e61814bd853f67ab469fa0ff42a73a7f5ccb5db0701d2b34f5d476090ceb784c5905f33342c36115101b774dce228cd485f2457db753eaef405a940a5c205f4e405d622276dfffce61bd8c2378d23702e08eded1113789f6bfed490762ae92ec401aaf1409d9d04eb2a2f7beeeeed8ffdc1a2ddeb83671e2fc79b79425d148735ba135e7b3996775c1193a2b474ed3428efd31c81666dad20c3cdbb38ec9a10d800f7b167714bfffdb0994b293bc205815e9db7143f3de10c300dca82a95b6c2d63f906b76db6cfe8cbcf270350cdc991935dcd7c84663d53671ae57fbb7826ddc").unwrap();

                let cell = TokenValue::write_bytes(&cert, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let kid: Vec<u8> = vec![15, 63, 150, 152, 3, 129, 228, 81, 239, 173, 13, 45, 221, 48, 227, 211];
                let cell = TokenValue::write_bytes(&kid, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));

                let timestamp: Vec<u8> = vec![0, 0, 3, 232];
                let cell = TokenValue::write_bytes(&timestamp, &ABI_VERSION_2_4).unwrap().into_cell().unwrap();
                stack.push(StackItem::cell(cell.clone()));
                // Push args, func name, instance name, then wasm.
                let wasm_func = "tlscheck";
                let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut 0).unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let wasm_func = "docs:tlschecker/tls-check-interface@0.1.0";
                let cell = pack_data_to_cell(&wasm_func.as_bytes(), &mut 0).unwrap();
                stack.push(StackItem::cell(cell.clone()));
                let wasm_dict = Vec::<u8>::new();

                let cell = TokenValue::write_bytes(&wasm_dict.as_slice(), &ABI_VERSION_2_4)
                    .unwrap()
                    .into_cell()
                    .unwrap();

                stack.push(StackItem::cell(cell.clone()));

                let mut res = Vec::<u8>::with_capacity(3);
                res.push(0xC7);
                res.push(0x3A);
                res.push(0x80);

                let code = SliceData::new(res);

                let mut engine = Engine::with_capabilities(0).setup_with_libraries(
                    code,
                    None,
                    Some(stack),
                    None,
                    vec![],
                );
                engine.wasm_engine_init_cached().unwrap();
                engine.add_wasm_hash_to_whitelist_by_str(hash_str.to_owned()).unwrap();
                let mut engine = engine.precompile_all_wasm_by_hash().unwrap();

                let start = std::time::Instant::now();
                let _ = engine.execute();
                total_duration += start.elapsed();

            }
            total_duration
        })
    });
}

// Run  `cargo bench -p tvm_vm --bench benchmarks poseidon`
fn bench_poseidon(c: &mut Criterion) {
    c.bench_function("poseidon", |b| {
        let mut total_duration = Duration::default();
        b.iter_custom(|iters| {
            // b.iter(|| {
            //+++++ Let's say I want to measure execution time from this point.

            for _i in 0..iters {
                let mut stack = Stack::new();

                // password was 567890 in ascii 535455565748
                let user_pass_salt = "535455565748";
                let secret_key = [222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14];
                let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); //
                let mut eph_pubkey = Vec::new();
                eph_pubkey.extend(ephemeral_kp.public().as_ref());
                //println!("eph_pubkey: {:?}", eph_pubkey);
                //println!("len eph_pubkey: {:?}", eph_pubkey.len());
                //let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
                //println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);
                let zk_seed = gen_address_seed(
                    user_pass_salt,
                    "sub",
                    "112897468626716626103",
                    "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
                )
                .unwrap();
                //println!("zk_seed = {:?}", zk_seed);
                let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
                \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
                \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
                \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
                [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
                \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
                \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
                \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
                \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
                \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
                //let len = proof_and_jwt.bytes().len();
                //println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);
                //println!("proof_and_jwt: {}", proof_and_jwt);

                //let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";

                //println!("iss_and_header_base64details: {}", iss_and_header_base64details);

                let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
                let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";

                let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
                let content: JWK = JWK {
                    kty: "RSA".to_string(),
                    e: "AQAB".to_string(),
                    n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(), // Alina's data
                    alg: "RS256".to_string(),
                };

                let mut all_jwk = HashMap::new();
                all_jwk.insert(
                    JwkId::new(
                        OIDCProvider::Google.get_config().iss,
                        "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(),
                    ),
                content,
                );

                let (iss, kid) =
                    (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
                let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
                    ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
                }).unwrap();

                let max_epoch = 142;

                let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
                    .map_err(|_| {
                    ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
                })
                .unwrap();

                let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

                let mut public_inputs_as_bytes = vec![];
                public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();
                //println!("HERE public_inputs_as_bytes : {:?}", public_inputs_as_bytes);
                //println!("HERE public_inputs_as_bytes len : {:?}", public_inputs_as_bytes.len());
                //println!("====== Start Poseidon ========");

                let index_mod_4 = 1;
                stack.push(StackItem::int(index_mod_4));
                stack.push(StackItem::int(max_epoch));
                stack.push(StackItem::integer(tvm_vm::stack::integer::IntegerData::from_unsigned_bytes_be(&eph_pubkey.clone())));

                let modulus_cell = tvm_vm::utils::pack_data_to_cell(&modulus.clone(), &mut 0).unwrap();
                //println!("modulus_cell = {:?}", modulus_cell);
                stack.push(StackItem::cell(modulus_cell.clone()));

                let iss_base_64_cell = tvm_vm::utils::pack_string_to_cell(&iss_base_64, &mut 0).unwrap();
                //println!("iss_base_64_cell = {:?}", iss_base_64_cell);
                stack.push(StackItem::cell(iss_base_64_cell.clone()));

                let header_base_64_cell = tvm_vm::utils::pack_string_to_cell(&header_base_64, &mut 0).unwrap();
                //println!("header_base_64_cell = {:?}", header_base_64_cell);
                stack.push(StackItem::cell(header_base_64_cell.clone()));

                let zk_seed_cell = tvm_vm::utils::pack_string_to_cell(&zk_seed.clone(), &mut 0).unwrap();
                //println!("zk_seed_cell = {:?}", zk_seed_cell);
                stack.push(StackItem::cell(zk_seed_cell.clone()));

                let mut res = Vec::<u8>::with_capacity(3);
                res.push(0xC7);
                res.push(0x32);
                res.push(0x80);

                let code = SliceData::new(res);

                let mut engine = Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);

                let start = std::time::Instant::now();
                let _ = engine.execute();
                total_duration += start.elapsed();

            }
            total_duration

            // println!("ress: {:?}", hex::encode(ress));
        })
    });
}

// Run  `cargo bench -p tvm_vm --bench benchmarks vergrth16`
fn bench_vergrth16(c: &mut Criterion) {
    c.bench_function("vergrth16", |b| {
        let mut total_duration = Duration::default();
        b.iter_custom(|iters| {
            // b.iter(|| {
            //+++++ Let's say I want to measure execution time from this point.

            for _i in 0..iters {
                let mut stack = Stack::new();

                // password was 567890 in ascii 535455565748
                let user_pass_salt = "535455565748";
                let secret_key = [222, 248, 61, 101, 214, 199, 113, 189, 223, 94, 151, 140, 235, 182, 203, 46, 143, 162, 166, 87, 162, 250, 176, 4, 29, 19, 42, 221, 116, 33, 178, 14];
                let ephemeral_kp = Ed25519KeyPair::from_bytes(&secret_key).unwrap(); //
                let mut eph_pubkey = Vec::new();
                eph_pubkey.extend(ephemeral_kp.public().as_ref());
                //println!("eph_pubkey: {:?}", eph_pubkey);
                //println!("len eph_pubkey: {:?}", eph_pubkey.len());
                //let eph_pubkey_hex_number = "0x".to_owned() + &hex::encode(eph_pubkey.clone());
                //println!("eph_pubkey_hex_number: {:?}", eph_pubkey_hex_number);
                let zk_seed = gen_address_seed(
                    user_pass_salt,
                    "sub",
                    "112897468626716626103",
                    "232624085191-v1tq20fg1kdhhgvat6saj7jf0hd8233r.apps.googleusercontent.com",
                )
                .unwrap();
                //println!("zk_seed = {:?}", zk_seed);
                let proof_and_jwt = "{\"proofPoints\":{\"a\":[\"2352077003566407045854435506409565889408960755152253285189640818725808263237\",\
                \"9548308350778027075240385782578683112366097953461273569343148999989145049123\",\"1\"],\
                \"b\":[[\"2172697685172701179756462481453772004245591587568555358926512547679273443868\",\
                \"11300889616992175665271080883374830731684409375838395487979439153562369168807\"],\
                [\"18769153619672444537277685186545610305405730219274884099876386487766026068190\",\
                \"12892936063156115176399929981646174277274895601746717550262309650970826515227\"],[\"1\",\"0\"]],\
                \"c\":[\"21276833037675249246843718004583052134371270695679878402069223253610209272159\",\
                \"8637596258221986824049981569842218428861929142818091935707054543971817804456\",\"1\"]},\
                \"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\
                \"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
                //let len = proof_and_jwt.bytes().len();
                //println!(" proof_and_jwt_bytes len (in bytes) = {:?}", len);
                //println!("proof_and_jwt: {}", proof_and_jwt);

                //let iss_and_header_base64details = "{\"issBase64Details\":{\"value\":\"yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC\",\"indexMod4\":1},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ\"}";
                //println!("iss_and_header_base64details: {}", iss_and_header_base64details);

                //let header_base_64 = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImEzYjc2MmY4NzFjZGIzYmFlMDA0NGM2NDk2MjJmYzEzOTZlZGEzZTMiLCJ0eXAiOiJKV1QifQ";
                //let iss_base_64 = "yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC";

                let zk_login_inputs = ZkLoginInputs::from_json(&*proof_and_jwt, &*zk_seed.to_string()).unwrap();
                let content: JWK = JWK {
                    kty: "RSA".to_string(),
                    e: "AQAB".to_string(),
                    n: "uBHF-esPKiNlFaAvpdpejD4vpONW9FL0rgLDg1z8Q-x_CiHCvJCpiSehD41zmDOhzXP_fbMMSGpGL7R3duiz01nK5r_YmRw3RXeB0kcS7Z9H8MN6IJcde9MWbqkMabCDduFgdr6gvH0QbTipLB1qJK_oI_IBfRgjk6G0bGrKz3PniQw5TZ92r0u1LM-1XdBIb3aTYTGDW9KlOsrTTuKq0nj-anW5TXhecuxqSveFM4Hwlw7pw34ydBunFjFWDx4VVJqGNSqWCfcERxOulizIFruZIHJGkgunZnB4DF7mCZOttx2dwT9j7s3GfLJf0xoGumqpOMvecuipfTPeIdAzcQ".to_string(), // Alina's data
                    alg: "RS256".to_string(),
                };

                let mut all_jwk = HashMap::new();
                all_jwk.insert(
                    JwkId::new(
                        OIDCProvider::Google.get_config().iss,
                        "a3b762f871cdb3bae0044c649622fc1396eda3e3".to_string(),
                    ),
                content,
                );

                let (iss, kid) =
                    (zk_login_inputs.get_iss().to_string(), zk_login_inputs.get_kid().to_string());
                let jwk = all_jwk.get(&JwkId::new(iss.clone(), kid.clone())).ok_or_else(|| {
                    ZkCryptoError::GeneralError(format!("JWK not found ({} - {})", iss, kid))
                }).unwrap();

                let max_epoch = 142;

                let modulus = base64ct::Base64UrlUnpadded::decode_vec(&jwk.n)
                    .map_err(|_| {
                    ZkCryptoError::GeneralError("Invalid Base64 encoded jwk modulus".to_string())
                })
                .unwrap();

                let public_inputs = &[zk_login_inputs.calculate_all_inputs_hash(&eph_pubkey, &modulus, max_epoch).unwrap()];

                let mut public_inputs_as_bytes = vec![];
                public_inputs.serialize_compressed(&mut public_inputs_as_bytes).unwrap();

                println!("====== Start VERGRTH16 ========");
                let proof = &zk_login_inputs.get_proof().as_arkworks().unwrap();
                let mut proof_as_bytes = vec![];
                proof.serialize_compressed(&mut proof_as_bytes).unwrap();

               let proof_cell = tvm_vm::utils::pack_data_to_cell(&proof_as_bytes, &mut 0).unwrap();
                stack.push(StackItem::cell(proof_cell.clone()));

                let public_inputs_cell = tvm_vm::utils::pack_data_to_cell(&public_inputs_as_bytes.clone(), &mut 0).unwrap();
                stack.push(StackItem::cell(public_inputs_cell.clone()));

                let mut res = Vec::<u8>::with_capacity(3);
                res.push(0xC7);
                res.push(0x31);
                res.push(0x80);

                let code = SliceData::new(res);

                let mut engine = Engine::with_capabilities(0).setup_with_libraries(code, None, Some(stack), None, vec![]);

                let start = std::time::Instant::now();
                let _ = engine.execute();
                total_duration += start.elapsed();

            }
            total_duration

            // println!("ress: {:?}", hex::encode(ress));
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets =
        bench_num_bigint,
        // bench_rug_bigint,
        bench_load_boc,
        bench_elector_algo_1000_vtors,
        bench_tiny_loop_200000_iters,
        bench_mergesort_tuple,
        bench_massive_cell_upload,
        bench_massive_cell_finalize,
        bench_wasmadd,
        bench_wasmadd_no_precompile,
        bench_wasmtls_without_whitelist,
        bench_wasmtls_with_whitelist,
        bench_poseidon,
        bench_vergrth16
);
criterion_main!(benches);
