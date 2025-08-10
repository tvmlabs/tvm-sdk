# Example wasm component to add 2 bytes together

This example follows the [WASI Preview 2 Component Model](https://component-model.bytecodealliance.org/)

To compile the code, simply go to the [project directory](.) and run:
`cargo +nightly build -Zbuild-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort --target wasm32-wasip2 --release`
Note that you might need to install the `wasm32-wasip2` target via `rustup` first. 

The above compiler flags are explained in part [here](https://github.com/rust-lang/rust/issues/133235) and are necessary (for now) to avoid import of excessive WASI interfaces for traceback generation on code panic.