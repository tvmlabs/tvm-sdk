# Example contract to execute `runwasm`

1. Compile and deploy following the acki-nacki documentation [here](https://dev.ackinacki.com/dapp-id-full-guide-creation-fees-centralized-replenishment).

2. Compile compatible wasm binary, for example from the [rust `add` example](../rust/add/README.md)) and reencode the binary as a hex string:
`cat target/wasm32-wasip2/release/add.wasm | xxd -p | tr -d '\n' > add.hex`

3. Make a hex-encoded argument that matches the syntax of the wasm binary, for exmaple: `1234` would add 0x12 and 0x34 in the above exmaple.

4. Run the `runWasm` function from [`hellowasm.sol`](hellowasm.sol):
`./tvm-cli run YOUR_CONTRACT_ADDRESS runWasm {"wasmBinary":"'$(cat YOUR_HEX_FILE)'", "wasmFunction":"add", "wasmArgs":"1234" } --abi contracts/helloWorld/helloWorld.abi.json`

5. If everything ran correctly, you should receive "46" as the result.