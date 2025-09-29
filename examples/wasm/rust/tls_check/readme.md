cd examples/wasm/rust/tls_check

cargo component bindings

cargo +nightly build -Zbuild-std="std,panic_abort" -Zbuild-std-features=panic_immediate_abort --target wasm32-wasip2 --release

cd target/wasm32-wasip2/release

wasm-tools component wit tls_check.wasm

sha256sum tls_check.wasm

new_binary_filename = $(sha256sum tls_check.wasm)

cp tls_check.wasm ../../../../../../../tvm_vm/config/wasm/"$new_binary_filename"
