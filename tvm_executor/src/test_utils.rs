#[cfg(feature = "wasmtime")]
use std::collections::HashMap;
#[cfg(feature = "wasmtime")]
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::time::Duration;
use std::time::Instant;

use tvm_types::HashmapE;
use tvm_types::UInt256;
#[cfg(feature = "wasmtime")]
use tvm_vm::executor::Engine;
use tvm_vm::executor::MVConfig;

use crate::ExecuteParams;

// Mirrors ExecuteParams construction in acki-nacki/node's block builder
// `build_actions.rs` and its WasmNodeCache from `producer/wasm.rs`.
#[cfg(feature = "wasmtime")]
const NODE_WASM_BINARY_ROOT_PATH: &str = "./config/wasm";
#[cfg(feature = "wasmtime")]
const NODE_WASM_HASH_STRS: [&str; 12] = [
    "c5b3fe1a4fa391e9660a13d55ca2200f9343d5b1d18473ebbee19d8219e3ddc1",
    "7b7f96a857a4ada292d7c6b1f47940dde33112a2c2bc15b577dff9790edaeef2",
    "e88c99c9a1cbbde5bf47839db7685953c3bf266945f3270abb731ed84d58d163",
    "e7adc782c05b67bcda5babaca1deabf80f30ca0e6cf668c89825286c3ce0e560",
    "afbe8c5a02df7d6fa5decd4d48ff0f74ecbd4dae38bb5144328354db6bd95967",
    "25dc3d80d7e4d8f27dfadc9c2faf9cf2d8dea0a9e08a692da2db7e34d74d66e1",
    "d4a067079c3ff4e0b0b6f579ef2d1b9a1d8fc21a0076162503ff46a6e8fca2e5",
    "f6b0cc30d023d266819b16dafa5a6a6ad25b97246bbbca80abac2df974939b87",
    "7670910579bb17bf986de6e318c6f5a8bf7e148b3fb8e0cbf03479fb9eb8c948",
    "b8891b913656ae35d9ffff371f0f03e4f1f869d0e17556a8c273750313884b0a",
    "2d577ca2e693700282d6d778dce8cfcedbada644497e411ec6aed889f5a3d5f4",
    "343268736f6dbb5a075a477fb1146b3c25c114d341b41c142e6609a7d1a90a2c",
];

pub(crate) enum BuildActionsTraceMode {
    Regular,
    TvmTracing { trace_callback: Arc<tvm_vm::executor::TraceCallback> },
}

#[cfg(feature = "wasmtime")]
#[derive(Clone)]
pub(crate) struct BuildActionsWasmCache {
    pub wasm_binary_root_path: String,
    pub wasm_hash_whitelist: HashSet<[u8; 32]>,
    pub wasm_engine: wasmtime::Engine,
    pub wasm_component_cache: HashMap<[u8; 32], wasmtime::component::Component>,
}

#[cfg(feature = "wasmtime")]
impl Default for BuildActionsWasmCache {
    fn default() -> Self {
        Self::with_binary_root_path(NODE_WASM_BINARY_ROOT_PATH)
    }
}

#[cfg(feature = "wasmtime")]
impl BuildActionsWasmCache {
    pub(crate) fn with_binary_root_path(wasm_binary_root_path: impl Into<String>) -> Self {
        let wasm_binary_root_path = wasm_binary_root_path.into();
        let wasm_hash_whitelist = Self::node_wasm_hash_whitelist();
        let wasm_engine = Self::node_wasm_engine();
        let wasm_component_cache = Engine::extern_precompile_all_wasm_from_hash_list(
            wasm_binary_root_path.clone(),
            wasm_engine.clone(),
            wasm_hash_whitelist.clone(),
        );

        Self { wasm_binary_root_path, wasm_hash_whitelist, wasm_engine, wasm_component_cache }
    }

    fn node_wasm_hash_whitelist() -> HashSet<[u8; 32]> {
        NODE_WASM_HASH_STRS.into_iter().map(Self::parse_wasm_hash).collect()
    }

    fn parse_wasm_hash(hash_str: &str) -> [u8; 32] {
        let hash = (0..hash_str.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
            .collect::<Vec<u8>>();

        hash.try_into().unwrap_or_else(|_| panic!("node wasm hash must be 32 bytes: {hash_str}"))
    }

    fn node_wasm_engine() -> wasmtime::Engine {
        Engine::extern_wasm_engine_init().unwrap_or_else(|err| {
            panic!("Could not initialise node-like Wasm engine: {err:?}");
        })
    }
}

pub(crate) struct BuildActionsExecuteParamsFixture {
    pub block_unixtime: u32,
    pub block_lt: u64,
    pub seq_no: u32,
    pub last_tr_lt: Arc<AtomicU64>,
    pub seed_block: UInt256,
    pub trace_mode: BuildActionsTraceMode,
    #[cfg(feature = "signature_with_id")]
    pub signature_id: i32,
    pub vm_execution_is_block_related: Arc<Mutex<bool>>,
    pub block_collation_was_finished: Arc<Mutex<bool>>,
    pub dapp_id: Option<UInt256>,
    pub available_credit: i128,
    pub termination_deadline: Option<Instant>,
    pub execution_timeout: Option<Duration>,
    #[cfg(feature = "wasmtime")]
    pub wasm_cache: BuildActionsWasmCache,
    pub mvconfig: MVConfig,
    pub engine_version: semver::Version,
}

impl Default for BuildActionsExecuteParamsFixture {
    fn default() -> Self {
        Self {
            block_unixtime: 0,
            block_lt: 0,
            seq_no: 0,
            last_tr_lt: Arc::new(AtomicU64::new(0)),
            seed_block: UInt256::default(),
            trace_mode: BuildActionsTraceMode::Regular,
            #[cfg(feature = "signature_with_id")]
            signature_id: 0,
            vm_execution_is_block_related: Arc::new(Mutex::new(false)),
            block_collation_was_finished: Arc::new(Mutex::new(false)),
            dapp_id: None,
            available_credit: 0,
            termination_deadline: None,
            execution_timeout: None,
            #[cfg(feature = "wasmtime")]
            wasm_cache: BuildActionsWasmCache::default(),
            mvconfig: MVConfig::default(),
            engine_version: semver::Version::new(1, 0, 3),
        }
    }
}

impl BuildActionsExecuteParamsFixture {
    pub(crate) fn regular() -> Self {
        Self::default()
    }

    pub(crate) fn tvm_tracing(trace_callback: Arc<tvm_vm::executor::TraceCallback>) -> Self {
        let mut fixture = Self::default();
        fixture.trace_mode = BuildActionsTraceMode::TvmTracing { trace_callback };
        fixture
    }

    pub(crate) fn build(self) -> ExecuteParams {
        let (debug, trace_callback) = match self.trace_mode {
            BuildActionsTraceMode::Regular => (false, None),
            BuildActionsTraceMode::TvmTracing { trace_callback } => (true, Some(trace_callback)),
        };

        ExecuteParams {
            // Intentionally default: acki-nacki build_actions.rs leaves state libs to
            // ExecuteParams::default().
            state_libs: HashmapE::with_bit_len(32),
            block_unixtime: self.block_unixtime,
            block_lt: self.block_lt,
            seq_no: self.seq_no,
            last_tr_lt: self.last_tr_lt,
            seed_block: self.seed_block,
            debug,
            trace_callback,
            // Intentionally default: acki-nacki build_actions.rs does not pass behavior modifiers.
            behavior_modifiers: None,
            // Intentionally default: acki-nacki build_actions.rs does not pass block version here.
            block_version: 0,
            #[cfg(feature = "signature_with_id")]
            signature_id: self.signature_id,
            vm_execution_is_block_related: self.vm_execution_is_block_related,
            block_collation_was_finished: self.block_collation_was_finished,
            dapp_id: self.dapp_id,
            available_credit: self.available_credit,
            termination_deadline: self.termination_deadline,
            execution_timeout: self.execution_timeout,
            #[cfg(feature = "wasmtime")]
            wasm_binary_root_path: self.wasm_cache.wasm_binary_root_path,
            #[cfg(feature = "wasmtime")]
            wasm_hash_whitelist: self.wasm_cache.wasm_hash_whitelist,
            #[cfg(feature = "wasmtime")]
            wasm_engine: Some(self.wasm_cache.wasm_engine),
            #[cfg(feature = "wasmtime")]
            wasm_component_cache: self.wasm_cache.wasm_component_cache,
            mvconfig: self.mvconfig,
            engine_version: self.engine_version,
        }
    }
}

pub(crate) fn build_actions_execute_params() -> ExecuteParams {
    BuildActionsExecuteParamsFixture::regular().build()
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;

    use super::*;

    #[test]
    fn build_actions_execute_params_contract_covers_regular_branch() {
        let last_tr_lt = Arc::new(AtomicU64::new(91));
        let vm_execution_is_block_related = Arc::new(Mutex::new(false));
        let block_collation_was_finished = Arc::new(Mutex::new(false));
        let dapp_id = UInt256::with_array([0x44; 32]);
        let seed_block = UInt256::with_array([0x77; 32]);
        let termination_deadline = Some(Instant::now() + Duration::from_secs(30));
        let execution_timeout = Some(Duration::from_secs(10));
        let mut mvconfig = MVConfig::default();
        mvconfig.set_config(vec![3, 5, 8]);

        let mut fixture = BuildActionsExecuteParamsFixture::regular();
        fixture.block_unixtime = 123;
        fixture.block_lt = 456;
        fixture.seq_no = 7;
        fixture.last_tr_lt = last_tr_lt.clone();
        fixture.seed_block = seed_block.clone();
        #[cfg(feature = "signature_with_id")]
        {
            fixture.signature_id = -239;
        }
        fixture.vm_execution_is_block_related = vm_execution_is_block_related.clone();
        fixture.block_collation_was_finished = block_collation_was_finished.clone();
        fixture.dapp_id = Some(dapp_id.clone());
        fixture.available_credit = 13;
        fixture.termination_deadline = termination_deadline;
        fixture.execution_timeout = execution_timeout;
        fixture.mvconfig = mvconfig.clone();
        fixture.engine_version = semver::Version::new(1, 0, 3);

        let params = fixture.build();

        assert_eq!(params.state_libs, HashmapE::with_bit_len(32));
        assert_eq!(params.block_unixtime, 123);
        assert_eq!(params.block_lt, 456);
        assert_eq!(params.seq_no, 7);
        assert!(Arc::ptr_eq(&params.last_tr_lt, &last_tr_lt));
        assert_eq!(params.last_tr_lt.load(Ordering::Relaxed), 91);
        assert_eq!(params.seed_block, seed_block);
        assert!(!params.debug);
        assert!(params.trace_callback.is_none());
        assert!(params.behavior_modifiers.is_none());
        assert_eq!(params.block_version, 0);
        #[cfg(feature = "signature_with_id")]
        assert_eq!(params.signature_id, -239);
        assert!(Arc::ptr_eq(&params.vm_execution_is_block_related, &vm_execution_is_block_related));
        assert!(Arc::ptr_eq(&params.block_collation_was_finished, &block_collation_was_finished));
        assert_eq!(params.dapp_id, Some(dapp_id));
        assert_eq!(params.available_credit, 13);
        assert_eq!(params.termination_deadline, termination_deadline);
        assert_eq!(params.execution_timeout, execution_timeout);
        #[cfg(feature = "wasmtime")]
        {
            assert_eq!(params.wasm_binary_root_path, NODE_WASM_BINARY_ROOT_PATH);
            assert_eq!(
                params.wasm_hash_whitelist,
                BuildActionsWasmCache::node_wasm_hash_whitelist()
            );
            assert_eq!(params.wasm_hash_whitelist.len(), NODE_WASM_HASH_STRS.len());
            assert!(params.wasm_engine.is_some());
            assert!(
                params
                    .wasm_component_cache
                    .keys()
                    .all(|hash| params.wasm_hash_whitelist.contains(hash))
            );
        }
        assert_eq!(params.mvconfig, mvconfig);
        assert_eq!(params.engine_version, semver::Version::new(1, 0, 3));
    }

    #[cfg(feature = "wasmtime")]
    #[test]
    fn build_actions_wasm_cache_precompiles_available_node_wasm_components() {
        use std::fmt::Write;
        use std::path::Path;

        fn hash_to_hex(hash: &[u8; 32]) -> String {
            let mut s = String::with_capacity(64);
            for byte in hash {
                write!(&mut s, "{byte:02x}").unwrap();
            }
            s
        }

        let wasm_binary_root_path = format!("{}/../tvm_vm/config/wasm", env!("CARGO_MANIFEST_DIR"));
        let cache = BuildActionsWasmCache::with_binary_root_path(wasm_binary_root_path.clone());
        let expected_precompiled = cache
            .wasm_hash_whitelist
            .iter()
            .filter(|hash| {
                let path = Path::new(&wasm_binary_root_path).join(hash_to_hex(hash));
                path.is_file()
            })
            .count();

        assert_eq!(cache.wasm_hash_whitelist, BuildActionsWasmCache::node_wasm_hash_whitelist());
        assert!(expected_precompiled > 0);
        assert_eq!(cache.wasm_component_cache.len(), expected_precompiled);
        assert!(
            cache.wasm_component_cache.keys().all(|hash| cache.wasm_hash_whitelist.contains(hash))
        );
    }

    #[test]
    fn build_actions_execute_params_contract_covers_tvm_tracing_branch() {
        let callback: Arc<tvm_vm::executor::TraceCallback> = Arc::new(|_, _| {});

        let params = BuildActionsExecuteParamsFixture::tvm_tracing(callback).build();

        assert!(params.debug);
        assert!(params.trace_callback.is_some());
    }
}
