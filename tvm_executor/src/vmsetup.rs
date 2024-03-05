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

use std::sync::Arc;
use std::sync::Mutex;

use tvm_block::GlobalCapabilities;
use tvm_types::Cell;
use tvm_types::HashmapE;
use tvm_types::Result;
use tvm_types::SliceData;
use tvm_vm::executor::gas::gas_state::Gas;
use tvm_vm::executor::Engine;
use tvm_vm::smart_contract_info::SmartContractInfo;
use tvm_vm::stack::savelist::SaveList;
use tvm_vm::stack::Stack;
use tvm_vm::stack::StackItem;

use crate::BlockchainConfig;

pub struct VMSetupContext {
    pub capabilities: u64,
    pub block_version: u32,
    #[cfg(feature = "signature_with_id")]
    pub signature_id: i32,
}

/// Builder for virtual machine engine. Initialises registers,
/// stack and code of VM engine. Returns initialized instance of TVM.
pub struct VMSetup {
    vm: Engine,
    code: SliceData,
    ctrls: SaveList,
    stack: Option<Stack>,
    gas: Option<Gas>,
    libraries: Vec<HashmapE>,
    ctx: VMSetupContext,
    vm_execution_is_block_related: Arc<Mutex<bool>>,
    block_collation_was_finished: Arc<Mutex<bool>>,
}

impl VMSetup {
    pub fn set_block_related_flags(
        mut self,
        vm_execution_is_block_related: Arc<Mutex<bool>>,
        block_collation_was_finished: Arc<Mutex<bool>>,
    ) -> VMSetup {
        self.vm_execution_is_block_related = vm_execution_is_block_related;
        self.block_collation_was_finished = block_collation_was_finished;
        self
    }

    /// Creates new instance of VMSetup with contract code.
    /// Initializes some registers of TVM with predefined values.
    pub fn with_context(code: SliceData, ctx: VMSetupContext) -> Self {
        VMSetup {
            vm: Engine::with_capabilities(ctx.capabilities),
            code,
            ctrls: SaveList::new(),
            stack: None,
            gas: Some(Gas::empty()),
            libraries: vec![],
            ctx,
            vm_execution_is_block_related: Arc::new(Mutex::new(false)),
            block_collation_was_finished: Arc::new(Mutex::new(false)),
        }
    }

    pub fn set_smart_contract_info(mut self, sci: SmartContractInfo) -> Result<VMSetup> {
        debug_assert_ne!(sci.capabilities, 0);
        let mut sci = sci.into_temp_data_item();
        self.ctrls.put(7, &mut sci)?;
        Ok(self)
    }

    /// Sets SmartContractInfo for TVM register c7
    #[deprecated]
    pub fn set_contract_info_with_config(
        self,
        mut sci: SmartContractInfo,
        config: &BlockchainConfig,
    ) -> Result<VMSetup> {
        sci.capabilities |= config.raw_config().capabilities();
        self.set_smart_contract_info(sci)
    }

    /// Sets SmartContractInfo for TVM register c7
    #[deprecated]
    pub fn set_contract_info(
        self,
        mut sci: SmartContractInfo,
        with_init_code_hash: bool,
    ) -> Result<VMSetup> {
        if with_init_code_hash {
            sci.capabilities |= GlobalCapabilities::CapInitCodeHash as u64;
        }
        self.set_smart_contract_info(sci)
    }

    /// Sets persistent data for contract in register c4
    pub fn set_data(mut self, data: Cell) -> Result<VMSetup> {
        self.ctrls.put(4, &mut StackItem::Cell(data))?;
        Ok(self)
    }

    /// Sets initial stack for TVM
    pub fn set_stack(mut self, stack: Stack) -> VMSetup {
        self.stack = Some(stack);
        self
    }

    /// Sets gas for TVM
    pub fn set_gas(mut self, gas: Gas) -> VMSetup {
        self.gas = Some(gas);
        self
    }

    /// Sets libraries for TVM
    pub fn set_libraries(mut self, libraries: Vec<HashmapE>) -> VMSetup {
        self.libraries = libraries;
        self
    }

    /// Sets trace flag to TVM for printing stack and commands
    pub fn set_debug(mut self, enable: bool) -> VMSetup {
        if enable {
            self.vm.set_trace(Engine::TRACE_ALL);
        } else {
            self.vm.set_trace(0);
        }
        self
    }

    /// Creates new instance of TVM with defined stack, registers and code.
    pub fn create(self) -> Engine {
        if cfg!(debug_assertions) {
            // account balance is duplicated in stack and in c7 - so check
            let balance_in_smc =
                self.ctrls.get(7).unwrap().as_tuple().unwrap()[0].as_tuple().unwrap()[7]
                    .as_tuple()
                    .unwrap()[0]
                    .as_integer()
                    .unwrap();
            let stack_depth = self.stack.as_ref().unwrap().depth();
            let balance_in_stack =
                self.stack.as_ref().unwrap().get(stack_depth - 1).as_integer().unwrap();
            assert_eq!(balance_in_smc, balance_in_stack);
        }
        let mut vm = self.vm.setup_with_libraries(
            self.code,
            Some(self.ctrls),
            self.stack,
            self.gas,
            self.libraries,
        );
        vm.set_block_version(self.ctx.block_version);
        #[cfg(feature = "signature_with_id")]
        vm.set_signature_id(self.ctx.signature_id);
        vm.set_block_related_flags(
            self.vm_execution_is_block_related,
            self.block_collation_was_finished,
        );
        vm
    }
}
