#[cfg(feature = "wasmtime")]
use std::collections::HashMap;
#[cfg(feature = "wasmtime")]
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

#[cfg(feature = "wasmtime")]
use sha2::Digest;
use tvm_block::Account;
use tvm_block::ConfigParam8;
use tvm_block::ConfigParam18;
use tvm_block::ConfigParam31;
use tvm_block::ConfigParamEnum;
use tvm_block::ConfigParams;
use tvm_block::CurrencyCollection;
use tvm_block::Deserializable;
use tvm_block::GasLimitsPrices;
use tvm_block::GlobalVersion;
use tvm_block::HashUpdate;
use tvm_block::InternalMessageHeader;
use tvm_block::Message;
use tvm_block::MsgAddressInt;
use tvm_block::MsgForwardPrices;
use tvm_block::OutAction;
use tvm_block::OutActions;
use tvm_block::Serializable;
use tvm_block::StateInit;
use tvm_block::StoragePrices;
use tvm_block::Transaction;
use tvm_block::TransactionDescr;
use tvm_executor::BlockchainConfig;
use tvm_executor::ExecuteParams;
use tvm_executor::OrdinaryTransactionExecutor;
use tvm_executor::TransactionExecutor;
use tvm_types::Cell;
use tvm_types::HashmapE;
use tvm_types::UInt256;
use tvm_vm::executor::MVConfig;

#[cfg(feature = "wasmtime")]
const ADDER_WASM_HASH_STR: &str =
    "7b7f96a857a4ada292d7c6b1f47940dde33112a2c2bc15b577dff9790edaeef2";
#[cfg(feature = "wasmtime")]
const CLOCK_WASM_HASH_STR: &str =
    "afbe8c5a02df7d6fa5decd4d48ff0f74ecbd4dae38bb5144328354db6bd95967";

pub struct NodeExecuteParamsFixture {
    pub block_unixtime: u32,
    pub block_lt: u64,
    pub seq_no: u32,
    pub last_tr_lt: Arc<AtomicU64>,
    pub seed_block: UInt256,
    pub trace_callback: Option<Arc<tvm_vm::executor::TraceCallback>>,
    #[cfg(feature = "signature_with_id")]
    pub signature_id: i32,
    pub vm_execution_is_block_related: Arc<Mutex<bool>>,
    pub block_collation_was_finished: Arc<Mutex<bool>>,
    pub dapp_id: Option<UInt256>,
    pub available_credit: i128,
    pub termination_deadline: Option<Instant>,
    pub execution_timeout: Option<Duration>,
    #[cfg(feature = "wasmtime")]
    pub wasm_fixtures: NodeWasmFixtures,
    pub mvconfig: MVConfig,
    pub engine_version: semver::Version,
}

impl Default for NodeExecuteParamsFixture {
    fn default() -> Self {
        Self {
            block_unixtime: 0,
            block_lt: 0,
            seq_no: 0,
            last_tr_lt: Arc::new(AtomicU64::new(0)),
            seed_block: UInt256::default(),
            trace_callback: None,
            #[cfg(feature = "signature_with_id")]
            signature_id: 0,
            vm_execution_is_block_related: Arc::new(Mutex::new(false)),
            block_collation_was_finished: Arc::new(Mutex::new(false)),
            dapp_id: None,
            available_credit: 0,
            termination_deadline: None,
            execution_timeout: None,
            #[cfg(feature = "wasmtime")]
            wasm_fixtures: NodeWasmFixtures::default(),
            mvconfig: MVConfig::default(),
            engine_version: semver::Version::new(1, 0, 3),
        }
    }
}

impl NodeExecuteParamsFixture {
    pub fn build(self) -> ExecuteParams {
        ExecuteParams {
            state_libs: HashmapE::with_bit_len(32),
            block_unixtime: self.block_unixtime,
            block_lt: self.block_lt,
            seq_no: self.seq_no,
            last_tr_lt: self.last_tr_lt,
            seed_block: self.seed_block,
            debug: self.trace_callback.is_some(),
            trace_callback: self.trace_callback,
            behavior_modifiers: None,
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
            wasm_binary_root_path: self.wasm_fixtures.binary_root_path,
            #[cfg(feature = "wasmtime")]
            wasm_hash_whitelist: self.wasm_fixtures.whitelist,
            #[cfg(feature = "wasmtime")]
            wasm_engine: Some(self.wasm_fixtures.engine),
            #[cfg(feature = "wasmtime")]
            wasm_component_cache: self.wasm_fixtures.component_cache,
            mvconfig: self.mvconfig,
            engine_version: self.engine_version,
        }
    }
}

#[cfg(feature = "wasmtime")]
#[derive(Clone)]
pub struct NodeWasmFixtures {
    pub binary_root_path: String,
    pub whitelist: HashSet<[u8; 32]>,
    pub engine: wasmtime::Engine,
    pub component_cache: HashMap<[u8; 32], wasmtime::component::Component>,
}

#[cfg(feature = "wasmtime")]
impl Default for NodeWasmFixtures {
    fn default() -> Self {
        Self::with_hashes([ADDER_WASM_HASH_STR, CLOCK_WASM_HASH_STR])
    }
}

#[cfg(feature = "wasmtime")]
impl NodeWasmFixtures {
    pub fn with_hashes<const N: usize>(hashes: [&str; N]) -> Self {
        let binary_root_path = format!("{}/../tvm_vm/config/wasm", env!("CARGO_MANIFEST_DIR"));
        let whitelist = hashes.into_iter().map(parse_wasm_hash).collect::<HashSet<_>>();

        for hash in &whitelist {
            assert_hash_named_binary_matches(&binary_root_path, hash);
        }

        let engine = tvm_vm::executor::Engine::extern_wasm_engine_init()
            .unwrap_or_else(|err| panic!("failed to initialize node-like wasm engine: {err:?}"));
        let component_cache = tvm_vm::executor::Engine::extern_precompile_all_wasm_from_hash_list(
            binary_root_path.clone(),
            engine.clone(),
            whitelist.clone(),
        );

        assert_eq!(component_cache.len(), whitelist.len());
        Self { binary_root_path, whitelist, engine, component_cache }
    }
}

#[cfg(all(feature = "gosh", feature = "wasmtime"))]
impl NodeWasmFixtures {
    pub fn without_hash(hash: [u8; 32]) -> Self {
        let mut fixtures = Self::default();
        fixtures.whitelist.remove(&hash);
        fixtures.component_cache.remove(&hash);
        fixtures
    }

    pub fn adder_call(&self, lhs: u8, rhs: u8) -> WasmCallFixture {
        WasmCallFixture::new(
            parse_wasm_hash(ADDER_WASM_HASH_STR),
            &[lhs, rhs],
            "docs:adder/add-interface@0.1.0",
            "add",
            &[lhs + rhs],
        )
    }

    pub fn clock_call(&self, block_time: u32) -> WasmCallFixture {
        let mut expected = Vec::new();
        expected.extend_from_slice(&(block_time as u64).to_be_bytes());
        expected.extend_from_slice(&0u64.to_be_bytes());
        WasmCallFixture::new(
            parse_wasm_hash(CLOCK_WASM_HASH_STR),
            &1u32.to_be_bytes(),
            "gosh:determinism/test-interface@0.1.0",
            "test",
            &expected,
        )
    }
}

#[cfg(all(feature = "gosh", feature = "wasmtime"))]
pub struct WasmCallFixture {
    pub hash: [u8; 32],
    pub hash_cell: Cell,
    pub args_cell: Cell,
    pub interface_cell: Cell,
    pub function_cell: Cell,
    pub empty_executable_cell: Cell,
    pub expected_result_cell: Cell,
}

#[cfg(all(feature = "gosh", feature = "wasmtime"))]
impl WasmCallFixture {
    fn new(
        hash: [u8; 32],
        args: &[u8],
        interface: &str,
        function: &str,
        expected_result: &[u8],
    ) -> Self {
        Self {
            hash,
            hash_cell: abi_bytes_cell(&hash),
            args_cell: abi_bytes_cell(args),
            interface_cell: tvm_vm::utils::pack_data_to_cell(interface.as_bytes(), &mut 0).unwrap(),
            function_cell: tvm_vm::utils::pack_data_to_cell(function.as_bytes(), &mut 0).unwrap(),
            empty_executable_cell: abi_bytes_cell(&[]),
            expected_result_cell: abi_bytes_cell(expected_result),
        }
    }
}

pub struct ExecutionHarness {
    executor: OrdinaryTransactionExecutor,
}

impl Default for ExecutionHarness {
    fn default() -> Self {
        Self { executor: OrdinaryTransactionExecutor::new(executor_config()) }
    }
}

impl ExecutionHarness {
    pub fn run(
        &self,
        code: Cell,
        message: Message,
        params_fixture: NodeExecuteParamsFixture,
    ) -> tvm_types::Result<ExecutionObservation> {
        let address = message.dst().expect("inbound test message must have destination");
        let original_account = active_account_with_code(address.clone(), code);
        let original_balance = original_account.balance_checked();
        let mut account_root = original_account.serialize()?;
        let old_account_hash = account_root.repr_hash();
        let last_tr_lt = params_fixture.last_tr_lt.clone();
        let vm_execution_is_block_related = params_fixture.vm_execution_is_block_related.clone();
        let block_collation_was_finished = params_fixture.block_collation_was_finished.clone();

        let (transaction, minted_shell) = self.executor.execute_with_libs_and_params(
            Some(&message),
            &mut account_root,
            params_fixture.build(),
        )?;

        Ok(MockBlockSink::consume(
            transaction,
            account_root,
            minted_shell,
            last_tr_lt,
            vm_execution_is_block_related,
            block_collation_was_finished,
            old_account_hash,
            original_balance,
        ))
    }
}

pub struct MockBlockSink;

impl MockBlockSink {
    fn consume(
        transaction: Transaction,
        account_root: Cell,
        minted_shell: i128,
        last_tr_lt: Arc<AtomicU64>,
        vm_execution_is_block_related: Arc<Mutex<bool>>,
        block_collation_was_finished: Arc<Mutex<bool>>,
        old_account_hash: UInt256,
        original_balance: CurrencyCollection,
    ) -> ExecutionObservation {
        let out_messages = collect_out_messages(&transaction);
        let state_update = transaction.read_state_update().unwrap_or_else(|_| HashUpdate {
            old_hash: UInt256::default(),
            new_hash: UInt256::default(),
        });
        let account = Account::construct_from_cell(account_root.clone()).unwrap();
        ExecutionObservation {
            transaction,
            account,
            account_root,
            out_messages,
            minted_shell,
            last_tr_lt: last_tr_lt.load(Ordering::Relaxed),
            vm_execution_is_block_related: *vm_execution_is_block_related.lock().unwrap(),
            block_collation_was_finished: *block_collation_was_finished.lock().unwrap(),
            old_account_hash,
            new_account_hash: state_update.new_hash.clone(),
            state_update,
            original_balance,
        }
    }
}

pub struct ExecutionObservation {
    pub transaction: Transaction,
    pub account: Account,
    pub account_root: Cell,
    pub out_messages: Vec<Message>,
    pub minted_shell: i128,
    pub last_tr_lt: u64,
    pub vm_execution_is_block_related: bool,
    pub block_collation_was_finished: bool,
    pub old_account_hash: UInt256,
    pub new_account_hash: UInt256,
    pub state_update: HashUpdate,
    pub original_balance: CurrencyCollection,
}

impl ExecutionObservation {
    pub fn ordinary_description(&self) -> tvm_block::TransactionDescrOrdinary {
        match self.transaction.read_description().unwrap() {
            TransactionDescr::Ordinary(description) => description,
            _ => panic!("unexpected transaction description"),
        }
    }
}

pub fn address(byte: u8) -> MsgAddressInt {
    MsgAddressInt::with_standart(None, 0, UInt256::with_array([byte; 32]).into()).unwrap()
}

pub fn masterchain_address(byte: u8) -> MsgAddressInt {
    MsgAddressInt::with_standart(None, -1, UInt256::with_array([byte; 32]).into()).unwrap()
}

pub fn internal_message(src: u8, dst: u8, value: u64) -> Message {
    Message::with_int_header(InternalMessageHeader::with_addresses(
        address(src),
        address(dst),
        CurrencyCollection::with_grams(value),
    ))
}

pub fn outbound_internal_message(dst: u8, value: u64) -> Message {
    let mut header = InternalMessageHeader::new();
    header.ihr_disabled = true;
    header.set_dst(masterchain_address(dst));
    header.value = CurrencyCollection::with_grams(value);
    Message::with_int_header(header)
}

pub fn compile_code(code: &str) -> Cell {
    tvm_assembler::compile_code_to_cell(code).unwrap()
}

pub fn push_ref_cell_asm(cell: &Cell) -> String {
    let mut code = String::from("PUSHREF {\n");
    if cell.bit_length() > 0 {
        code.push_str("  .BLOB x");
        code.push_str(&cell.to_hex_string(true));
        code.push('\n');
    }
    for index in 0..cell.references_count() {
        append_cell_asm(&cell.reference(index).unwrap(), &mut code, "  ");
    }
    code.push_str("}\n");
    code
}

pub fn action_cell(actions: OutActions) -> Cell {
    actions.serialize().unwrap()
}

pub fn send_action(dst: u8, value: u64) -> OutAction {
    OutAction::new_send(
        tvm_block::SENDMSG_PAY_FEE_SEPARATELY,
        outbound_internal_message(dst, value),
    )
}

fn active_account_with_code(address: MsgAddressInt, code: Cell) -> Account {
    let mut state_init = StateInit::default();
    state_init.set_code(code);
    state_init.set_data(Cell::default());
    Account::active_by_init_code_hash(
        address,
        CurrencyCollection::with_grams(1_000_000_000),
        0,
        state_init,
        false,
    )
    .unwrap()
}

fn append_cell_asm(cell: &Cell, code: &mut String, indent: &str) {
    code.push_str(indent);
    code.push_str(".CELL {\n");
    let inner_indent = format!("{indent}  ");
    if cell.bit_length() > 0 {
        code.push_str(&inner_indent);
        code.push_str(".BLOB x");
        code.push_str(&cell.to_hex_string(true));
        code.push('\n');
    }
    for index in 0..cell.references_count() {
        append_cell_asm(&cell.reference(index).unwrap(), code, &inner_indent);
    }
    code.push_str(indent);
    code.push_str("}\n");
}

fn collect_out_messages(tx: &Transaction) -> Vec<Message> {
    let mut messages = Vec::new();
    tx.iterate_out_msgs(|msg| {
        messages.push(msg);
        Ok(true)
    })
    .unwrap();
    messages
}

fn storage_prices() -> ConfigParam18 {
    let mut prices = ConfigParam18::default();
    prices
        .insert(&StoragePrices {
            utime_since: 1,
            bit_price_ps: 2,
            cell_price_ps: 4,
            mc_bit_price_ps: 8,
            mc_cell_price_ps: 16,
        })
        .unwrap();
    prices
}

fn gas_prices() -> GasLimitsPrices {
    GasLimitsPrices {
        gas_price: 65_536,
        gas_limit: 1_000_000,
        special_gas_limit: 1_000_000,
        gas_credit: 10_000,
        block_gas_limit: 1_000_000,
        freeze_due_limit: 5,
        delete_due_limit: 8,
        max_gas_threshold: 1_000_000_000,
        flat_gas_limit: 10,
        flat_gas_price: 10,
    }
}

fn executor_config() -> BlockchainConfig {
    let mut config =
        ConfigParams { config_addr: UInt256::with_array([0x55; 32]), ..ConfigParams::default() };
    config
        .set_config(ConfigParamEnum::ConfigParam8(ConfigParam8 {
            global_version: GlobalVersion { version: 42, capabilities: 0x572e },
        }))
        .unwrap();
    config.set_config(ConfigParamEnum::ConfigParam18(storage_prices())).unwrap();
    config.set_config(ConfigParamEnum::ConfigParam20(gas_prices())).unwrap();
    config.set_config(ConfigParamEnum::ConfigParam21(gas_prices())).unwrap();
    config.set_config(ConfigParamEnum::ConfigParam24(MsgForwardPrices::default())).unwrap();
    config.set_config(ConfigParamEnum::ConfigParam25(MsgForwardPrices::default())).unwrap();
    config.set_config(ConfigParamEnum::ConfigParam31(ConfigParam31::new())).unwrap();
    BlockchainConfig::with_config(config).unwrap()
}

#[cfg(all(feature = "gosh", feature = "wasmtime"))]
pub fn wasm_validation_code(call: &WasmCallFixture, actions: OutActions) -> String {
    format!(
        "{}{}{}{}{}RUNWASM\nHASHCU\n{}HASHCU\nEQUAL\nTHROWIFNOT 901\n{}POP c5\n",
        push_ref_cell_asm(&call.hash_cell),
        push_ref_cell_asm(&call.args_cell),
        push_ref_cell_asm(&call.function_cell),
        push_ref_cell_asm(&call.interface_cell),
        push_ref_cell_asm(&call.empty_executable_cell),
        push_ref_cell_asm(&call.expected_result_cell),
        push_ref_cell_asm(&action_cell(actions)),
    )
}

#[cfg(all(feature = "gosh", feature = "wasmtime"))]
fn abi_bytes_cell(bytes: &[u8]) -> Cell {
    tvm_abi::TokenValue::write_bytes(bytes, &tvm_abi::contract::ABI_VERSION_2_4)
        .unwrap()
        .into_cell()
        .unwrap()
}

#[cfg(feature = "wasmtime")]
fn parse_wasm_hash(hash_str: &str) -> [u8; 32] {
    let hash = (0..hash_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hash_str[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    hash.try_into().unwrap_or_else(|_| panic!("wasm hash must be 32 bytes: {hash_str}"))
}

#[cfg(feature = "wasmtime")]
fn hash_to_hex(hash: &[u8; 32]) -> String {
    let mut hex = String::with_capacity(64);
    for byte in hash {
        use std::fmt::Write;
        write!(&mut hex, "{byte:02x}").unwrap();
    }
    hex
}

#[cfg(feature = "wasmtime")]
fn assert_hash_named_binary_matches(binary_root_path: &str, hash: &[u8; 32]) {
    let expected = hash_to_hex(hash);
    let bytes = std::fs::read(format!("{binary_root_path}/{expected}")).unwrap();
    let actual = sha2::Sha256::digest(bytes);
    assert_eq!(actual.as_slice(), hash, "wasm binary hash mismatch for {expected}");
}
