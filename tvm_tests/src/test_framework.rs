#![allow(dead_code)]
use std::convert::Into;
use std::sync::Arc;

use tvm_assembler::CompileError;
use tvm_assembler::compile_code;
use tvm_types::BuilderData;
use tvm_types::Cell;
use tvm_types::ExceptionCode;
use tvm_types::HashmapE;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_vm::executor::BehaviorModifiers;
use tvm_vm::executor::Engine;
use tvm_vm::executor::IndexProvider;
use tvm_vm::executor::gas::gas_state::Gas;
use tvm_vm::stack::Stack;
use tvm_vm::stack::savelist::SaveList;
use tvm_vm::types::Exception;
use tvm_vm::error::TvmError;

pub type Bytecode = SliceData;

#[allow(dead_code)]
fn logger_init() {
    if log::log_enabled!(log::Level::Info) {
        return;
    }
    let log_level = log::LevelFilter::Info;
    let encoder_boxed = Box::new(log4rs::encode::pattern::PatternEncoder::new("{m}"));
    let config = {
        let console =
            log4rs::append::console::ConsoleAppender::builder().encoder(encoder_boxed).build();
        log4rs::config::Config::builder()
            .appender(log4rs::config::Appender::builder().build("console", Box::new(console)))
            .build(log4rs::config::Root::builder().appender("console").build(log_level))
            .unwrap()
    };
    log4rs::init_config(config).ok();
}

pub struct TestCaseInputs {
    code: String,
    ctrls: SaveList,
    stack: Stack,
    refs: Vec<Cell>,
    gas: Option<Gas>,
    library: HashmapE,
    behavior_modifiers: Option<BehaviorModifiers>,
    capabilities: u64,
    block_version: u32,
    skip_fift_check: bool,
    index_provider: Option<Arc<dyn IndexProvider>>,
}

impl TestCaseInputs {
    pub fn new(code: String, stack: Stack, refs: Vec<Cell>, capabilities: u64) -> TestCaseInputs {
        logger_init();
        TestCaseInputs {
            code,
            ctrls: SaveList::new(),
            stack,
            refs,
            gas: None,
            library: HashmapE::with_bit_len(256),
            behavior_modifiers: None,
            capabilities,
            block_version: 0,
            skip_fift_check: false,
            index_provider: None,
        }
    }
}

impl From<TestCaseInputs> for TestCase {
    fn from(inputs: TestCaseInputs) -> Self {
        TestCase::new(inputs)
    }
}

pub struct TestCase {
    executor: Option<Engine>,
    compilation_result: std::result::Result<Bytecode, CompileError>,
    execution_result: Result<i32>,
}

impl TestCase {
    fn executor(&self, message: Option<&str>) -> &Engine {
        match self.executor {
            Some(ref exectuor) => exectuor,
            None => {
                let err = self.compilation_result.as_ref().unwrap_err();
                match message {
                    Some(msg) => panic!(
                        "{}No executor was created, because of bytecode compilation error {:?}",
                        msg, err
                    ),
                    None => panic!(
                        "No executor was created, because of bytecode compilation error {:?}",
                        err
                    ),
                }
            }
        }
    }
}

impl TestCase {
    pub(super) fn new(args: TestCaseInputs) -> TestCase {
        match compile_code(&args.code) {
            Ok(bytecode) => {
                let code = if args.refs.is_empty() {
                    bytecode.clone()
                } else if bytecode.remaining_references() + args.refs.len()
                    <= BuilderData::references_capacity()
                {
                    let mut builder = bytecode.as_builder();
                    args.refs.iter().rev().for_each(|reference| {
                        builder.checked_prepend_reference(reference.clone()).unwrap();
                    });
                    SliceData::load_builder(builder).unwrap()
                } else {
                    log::error!(target: "compile", "Cannot use 4 refs with long code");
                    bytecode.clone()
                };
                log::trace!(target: "compile", "code: {}\n", code);
                let mut executor = Engine::with_capabilities(args.capabilities)
                    .setup_with_libraries(
                        code.clone(),
                        Some(args.ctrls.clone()),
                        Some(args.stack.clone()),
                        args.gas.clone(),
                        vec![args.library.clone()],
                    );
                executor.set_block_version(args.block_version);
                if let Some(modifiers) = args.behavior_modifiers {
                    executor.modify_behavior(modifiers);
                }
                if let Some(index_provider) = args.index_provider.clone() {
                    executor.set_index_provider(index_provider)
                }
                let execution_result = executor.execute();

                TestCase {
                    executor: Some(executor),
                    compilation_result: Ok(bytecode),
                    execution_result,
                }
            }
            Err(e) => {
                TestCase { executor: None, compilation_result: Err(e), execution_result: Ok(-1) }
            }
        }
    }
}

pub trait Expects {
    fn expect_stack(self, stack: &Stack) -> TestCase;
    fn expect_stack_extended(self, stack: &Stack, message: Option<&str>) -> TestCase;
    fn expect_success(self) -> TestCase;
    fn expect_success_extended(self, message: Option<&str>) -> TestCase;
    fn expect_failure(self, exception_code: ExceptionCode) -> TestCase;
    fn expect_custom_failure_extended<F : Fn(&Exception) -> bool>(self, op: F, exc_name: &str, message: Option <&str>) -> TestCase;
    fn expect_failure_extended(self, exception_code: ExceptionCode, message: Option <&str>) -> TestCase; 
}

impl<T: Into<TestCase>> Expects for T {
    fn expect_stack(self, stack: &Stack) -> TestCase {
        self.expect_stack_extended(stack, None)
    }

    fn expect_stack_extended(self, stack: &Stack, message: Option<&str>) -> TestCase {
        let test_case: TestCase = self.into();
        let executor = test_case.executor(message);
        match test_case.execution_result {
            Ok(_) => {
                if !executor.eq_stack(stack) {
                    if let Some(msg) = message {
                        log::info!("{}", msg)
                    }
                    log::info!(target: "tvm", "\nExpected stack: \n{}", stack);
                    log::info!(
                        target: "tvm",
                        "\n{}\n",
                        executor.dump_stack("Actual Stack:", false)
                    );
                    panic!("Stack is not expected")
                }
            }
            // TODO this is not quite right: execution may fail but still produce a stack
            Err(ref e) => {
                log::info!(target: "tvm", "\nExpected stack: \n{}", stack);
                // print_failed_detail_extended(&test_case, e, message);
                panic!("Execution error: {:?}", e)
            }
        }
        test_case
    }

    fn expect_success(self) -> TestCase {
        self.expect_success_extended(None)
    }

    fn expect_success_extended(self, message: Option<&str>) -> TestCase {
        let test_case: TestCase = self.into();
        let executor = test_case.executor(message);
        print_stack(&test_case, executor);
        if let Err(ref e) = test_case.execution_result {
            match message {
                None => {
                    // print_failed_detail_extended(&test_case, e, message);
                    panic!("Execution error: {:?}", e);
                }
                Some(msg) => {
                    // print_failed_detail_extended(&test_case, e, message);
                    panic!("{}\nExecution error: {:?}", msg, e);
                }
            }
        }
        test_case
    }

    fn expect_failure(self, exception_code: ExceptionCode) -> TestCase {
        self.expect_failure_extended(exception_code, None)
    }

    fn expect_failure_extended(
        self, 
        exception_code: ExceptionCode, 
        message: Option <&str>
    ) -> TestCase {
       self.expect_custom_failure_extended(
           |e| e.exception_code() != Some(exception_code),
           &format!("{}", exception_code),
           message
       )
    }

    fn expect_custom_failure_extended<F : Fn(&Exception) -> bool>(
        self, 
        op: F, 
        exc_name: &str, 
        message: Option <&str>
    ) -> TestCase {
        let test_case: TestCase = self.into();
        let executor = test_case.executor(message);
        match test_case.execution_result {
            Ok(_) => {
                log::info!(
                    target: "tvm",
                    "Expected failure: {}, however execution succeeded.",
                    exc_name
                );
                print_stack(&test_case, executor);
                match message {
                    None => panic!(
                        "Expected failure: {}, however execution succeeded.", 
                        exc_name
                    ),
                    Some(msg) => panic!(
                        "{}.\nExpected failure: {}, however execution succeeded.", 
                        msg, exc_name
                    )
                }
            }
            Err(ref e) => {
                if let Some(TvmError::TvmExceptionFull(e, msg2)) = e.downcast_ref() {
                    if op(e) {
                        match message {
                            Some(msg) => panic!(
                                "{} - {}\nNon expected exception: {}, expected: {}", 
                                msg2, msg, e, exc_name
                            ),
                            None => panic!(
                                "{}\nNon expected exception: {}, expected: {}", 
                                msg2, e, exc_name
                            )
                        }
                    }
                } else {
                    let code = e.downcast_ref::<ExceptionCode>();
                    match code {
                        Some(code) => {
                            let e = Exception::from(*code);
                            if op(&e) {
                                panic!("Non expected exception: {}, expected: {}", e, exc_name)
                            }
                        }
                        None => {
                            if op(&Exception::from(ExceptionCode::FatalError)) {
                                panic!("Non expected exception: {}, expected: {}", e, exc_name)
                            }
                        }
                    }
                }
            }
        }
        test_case
    }
}

fn print_stack(test_case: &TestCase, executor: &Engine) {
    if test_case.execution_result.is_ok() {
        log::info!(target: "tvm", "Post-execution:\n");
        log::info!(target: "tvm", "{}", executor.dump_stack("Post-execution stack state", false));
        log::info!(target: "tvm", "{}", executor.dump_ctrls(false));
    }
}

pub fn test_case_with_refs(code: &str, references: Vec<Cell>) -> TestCaseInputs {
    TestCaseInputs::new(code.to_string(), Stack::new(), references, 0)
}

pub fn test_case(code: impl ToString) -> TestCaseInputs {
    TestCaseInputs::new(code.to_string(), Stack::new(), vec![], 0)
}
