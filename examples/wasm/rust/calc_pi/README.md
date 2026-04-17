This is a more advanced example, where an existing rust library is made into a wasm component.

To replicate this with your own rust code, you can do the following:
1. Replace the library in (`./src/calc`) with your code.
2. Modify the `wit` file to match your public functions. Remember that only `fn (Vec<u8>) -> Vec<u8> ` is currently supported! You will need to make per-funciton wrappers that convert the byte vectors to your input and output variables yourself later.
3. You can have more than 1 interface and more than 1 function per interface. Keep in mind that the `wit` format forbids using `_` in names; use `-` instead - they will be converted to `_` in rust automatically.
4. Once your `wit` is done, generate a new `bindings.rs` using `cargo component bindings`. You may need to install cargo component for that if not already done.
5. Now edit the `lib.rs` file with the following:
```rust
#[allow(warnings)]
mod bindings;

use bindings::exports::package_name::world_name::interface1_name::Guest as YourInterface1Guest; 

mod your_mod1;
mod your_mod2;

use some_std_or_non_std_crate;

struct YourInterface1Component;

impl YourInterface1Guest for YourInterface1Component {
    fn your_func1_from_wit(kwargs: Vec<u8>) -> Vec<u8> {
        // wrapper around your_func1_from_rust()
        let result = your_func1_from_rust("some operation to go Vec<u8> -> your_inut_type");
        result."some operation to go your_output_type -> Vec<u8>"
    }

    fn your_func2_from_wit(kwargs: Vec<u8>) -> Vec<u8> {
        // wrapper around your_func1_from_rust()
        let result = your_func2_from_rust("some operation to go Vec<u8> -> your_inut_type");
        result."some operation to go your_output_type -> Vec<u8>"
    }

    ...
}

bindings::export!(YourInterface1Component with_types_in bindings);

struct YourInterface2Component;

impl YourInterface2Guest for YourInterface2Component {
    fn your_func1_from_wit(kwargs: Vec<u8>) -> Vec<u8> {
        // wrapper around your_func1_from_rust()
        let result = your_func1_from_rust("some operation to go Vec<u8> -> your_inut_type");
        result."some operation to go your_output_type -> Vec<u8>"
    }

    fn your_func2_from_wit(kwargs: Vec<u8>) -> Vec<u8> {
        // wrapper around your_func1_from_rust()
        let result = your_func2_from_rust("some operation to go Vec<u8> -> your_inut_type");
        result."some operation to go your_output_type -> Vec<u8>"
    }

    ...
}

bindings::export!(YourInterface2Component with_types_in bindings);
```
6. After this you should be able to compile with: 
```sh
cargo +nightly build -Zbuild-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort --target wasm32-wasip2 --release
```

7. You can now call the wasm functions from isnide the smart contract! Use `wasmModule = "package_name:world_name/interface_name@version_from_wit` and `wasmFunc = "func_name` to call the moethod. In case of doubt, you can use `wasm-tools component wit binary.wasm` to print the exports from your wasm file.
8. Keep in mind that the compiler will build in dependecies on certain system interfaces, based on the WASI P2 standard. Currently, `wasi:io`, `wasi:cli` and `wasi:network` are not properly supported, and will likely error! You can use the command above to see if your binary file depends on those. `print!` macros for exmaple should be avoided, as well as any libraries that build networking funcitonality or rely on stdin/out and may bundle those dependencies into the binary.
