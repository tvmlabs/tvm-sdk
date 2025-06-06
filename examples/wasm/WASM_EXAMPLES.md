# Executing WASM binaries within a smart contract
With the need to run more and more compute-intensive operations within a contract, a decision was made to provide a way to run complex operations with a lower compute cost. This is achieved by allowing for binary executables meeting the [WASI Preview 2 Component Model Standard](https://component-model.bytecodealliance.org/) to be executed within a Wasmtime runtime environment.

This example project provides all the pieces necessary to build, deploy and run a Wasm executable from within a solidity contract. Work In Progress documentation for the `runwasm` instruction can be found [here](../../tvm_vm/WASM.md)

* [Rust projects that can be compiled into compatible wasm binaries](rust)
* [Solidity contract to run the `runwasm` instruction](contracts)