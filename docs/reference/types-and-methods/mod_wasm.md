# Instruction runwasm
An instruction allowing for execution of arbitrary WASM code.

## Working principle (docs to be finalised)
The runwasm instruction allows arbitrary pre-compiled wasm code to be executed directly by the node.

## Argument Syntax
2 arguments should be encoded as a series of bytes inside of separate bags of cells:

wasmArgs: argument(s) for the wasm code to execute. Currently, it is up to the wasm binary deveoper to handle parsing raw bytes into more complex types; this must be done inside the wasm code.

wasmBinary: compiled wasm executable. can be provided in .wasm binary or .wat text format

2 arguments should be passed as cell-encoded strings:

wasmFunction: name of the function to call from the wasm binary.

wasmModule: name of the module containing wasmFunction. in case the target function is not inside a module, pass an empty or invalid string.

## Compiling the target wasm binary

Input and output from the executed wasm function should take the form of `list<u8>`. You can double-check this by generating a .wit file from your wasm binary. Here's an example wit file describing a compatible function:
```js
package docs:adder@0.1.0 {
  interface calculator {
    add: func(kwargs: list<u8>) -> list<u8>;
  }
}
```
To execute the above function, we could set:
wasmBinary = <bytes of the wasm binary>
wasmArgs = [u8, u8] or [u8, u8, u8, u8, u8, u8, u8, u8] if adding 2 * i64 for exmaple 
wasmFunction = "add"
wasmModule = "docs:adder/calculator@0.1.0" - note that the interface name needs to be included

The supported target as of June 2025 is any `wasm32-wasip2` binary utilising the _component_ model only, with a set of WASI modules provided by the executing node. This is done to minimise the binary size while allowing the use of more complex language features.
Make sure that your binary is compiled following the [WASM Component model](https://component-model.bytecodealliance.org/)

Currently provided [WASI interfaces/features](https://wasi.dev/interfaces) are:
```json
"wasi:cli/exit"
"wasi:cli/environment"
"wasi:cli/stdin"
"wasi:cli/stdout"
"wasi:cli/stderr"
"wasi:cli/terminal_input"
"wasi:cli/terminal_output"
"wasi:cli/terminal_stdin"
"wasi:cli/terminal_stdout"
"wasi:cli/terminal_stderr"
"wasi:filesystem/types"
"wasi:filesystem/preopens"
"wasi:clocks/wall_clock"
"wasi:clocks/monotonic_clock"
"wasi:random/random"
"wasi:random/insecure"
"wasi:random/insecure_seed"
```

Some standard WASI features are not available, which is likely to cause errors with binaries containing complex error handlers and traceback info.

To minimise extra WASI dependencies in your wasm binary, try to remove as much panic/crash handling logic as possible (it is not needed in this context anyway).
For example with the Rust compiler:
`cargo +nightly build -Zbuild-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort --target wasm32-wasip2 --release` will replace all panic handlers with wasm's `unreachable` instruction. Some more info [here](https://github.com/rust-lang/rust/issues/133235).
As an aded bonus, this can greatly reduce the resulting binary size, along with some [other compiler settings](https://github.com/johnthagen/min-sized-rust).


## Example Solidity contract

```solidity
    function runWasm (bytes wasmBinary, string wasmModule, string wasmFunction, bytes wasmArgs) public pure returns (bytes) {
        
        tvm.accept(); 
        getTokens();

        TvmCell wasmResultCell = gosh.runwasm(abi.encode(wasmArgs), abi.encode(wasmFunction), abi.encode(wasmModule), abi.encode(wasmBinary));


        bytes wasmResult = abi.decode(wasmResultCell, bytes);

        return wasmResult;
    }
```