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
use tvm_vm::executor::MVConfig;

use crate::ExecuteParams;

// Mirrors ExecuteParams construction in acki-nacki/node's block builder
// `build_actions.rs`.
pub(crate) enum BuildActionsTraceMode {
    Regular,
    TvmTracing { trace_callback: Arc<tvm_vm::executor::TraceCallback> },
}

#[cfg(feature = "wasmtime")]
pub(crate) struct BuildActionsWasmCache {
    pub wasm_binary_root_path: String,
    pub wasm_hash_whitelist: HashSet<[u8; 32]>,
    pub wasm_engine: wasmtime::Engine,
    pub wasm_component_cache: HashMap<[u8; 32], wasmtime::component::Component>,
}

#[cfg(feature = "wasmtime")]
impl Default for BuildActionsWasmCache {
    fn default() -> Self {
        Self {
            wasm_binary_root_path: "./config/wasm".to_owned(),
            wasm_hash_whitelist: HashSet::new(),
            wasm_engine: wasmtime::Engine::default(),
            wasm_component_cache: HashMap::new(),
        }
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
        #[cfg(feature = "wasmtime")]
        {
            fixture.wasm_cache.wasm_binary_root_path = "./tests/wasm".to_owned();
            fixture.wasm_cache.wasm_hash_whitelist.insert([0xab; 32]);
        }

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
            assert_eq!(params.wasm_binary_root_path, "./tests/wasm");
            assert!(params.wasm_hash_whitelist.contains(&[0xab; 32]));
            assert!(params.wasm_engine.is_some());
            assert!(params.wasm_component_cache.is_empty());
        }
        assert_eq!(params.mvconfig, mvconfig);
        assert_eq!(params.engine_version, semver::Version::new(1, 0, 3));
    }

    #[test]
    fn build_actions_execute_params_contract_covers_tvm_tracing_branch() {
        let callback: Arc<tvm_vm::executor::TraceCallback> = Arc::new(|_, _| {});

        let params = BuildActionsExecuteParamsFixture::tvm_tracing(callback).build();

        assert!(params.debug);
        assert!(params.trace_callback.is_some());
    }
}
