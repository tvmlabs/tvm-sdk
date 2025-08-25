cd examples/wasm/rust/tls_check

cargo component bindings

cargo +nightly build -Zbuild-std="std,panic_abort" -Zbuild-std-features=panic_immediate_abort --target wasm32-wasip2 --release

cd target/wasm32-wasip2/release


sha256sum tls_check.wasm

wasm-tools component wit tls_check.wasm