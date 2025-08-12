cd examples\wasm\rust\tls_check

cargo component bindings

cargo +nightly build -Zbuild-std="std,panic_abort" -Zbuild-std-features=panic_immediate_abort --target wasm32-wasip2 --release

cd examples/wasm/rust/tls_check/target/wasm32-wasip2/release
cd examples\wasm\rust\tls_check\target\wasm32-wasip2\release


sha256sum /Users/alinat/ZKLOGIN/WASMM/tvm-sdk/examples/wasm/rust/tls_check/target/wasm32-wasip2/release/tls_check.wasm

wasm-tools component wit tls_check.wasm