cd examples/wasm/rust/tls_check

cargo component bindings

cargo +nightly build -Zbuild-std="std,panic_abort" -Zbuild-std-features=panic_immediate_abort --target wasm32-wasip2 --release

cd target/wasm32-wasip2/release

wasm-tools component wit tls_check.wasm

sha256sum tls_check.wasm

new_binary_filename = $(sha256sum tls_check.wasm)

cp tls_check.wasm ../../../../../../../tvm_vm/config/wasm/"$new_binary_filename"

in file tvm-sdk/tvm_vm/src/tests/test_multifactor_tls_wasm_execution.rs find variable hash_str and set it to new_binary_filename

in file tvm-sdk/tvm_vm/benches/benchmarks.rs in functions bench_wasmtls_without_whitelist and bench_wasmtls_with_whitelist set variable hash_str to new_binary_filename
