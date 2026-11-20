# Build and Call Your First AVM Contract

For an introduction to the execution model, see [Rust contracts with AVM](https://docs.ackinacki.com/for-developers/smart-contracts#rust-contracts-with-avm).

These instructions describe [SDK 0.6.5](https://github.com/gosh-sh/avm-sdk/releases/tag/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2) and Core-WASM V2. Use its matching builder, local stand and profile bundle. Earlier releases retain their own compilers/profiles; follow the documentation inside each installed release. A [local bundle](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/releases.md#install-a-prepared-local-bundle) can also be verified and installed directly.

## Prerequisites

* Linux on an x86-64-v3 CPU with glibc 2.39 or later for this release's contract builder. Check the installed glibc with `getconf GNU_LIBC_VERSION`.
* Git, Bash, Python 3.11+ (available as `python3`), `jq`, and GitHub CLI (`gh`) with access to public releases.
* Rustup to install the guest compiler and WebAssembly target.

## Install the SDK

From a writable directory, clone the tag for this guide. Run the rest of the commands in the same Bash session from the root of that checkout.

```bash
export RELEASE_TAG=avm-dev-d72c855b2eae-sdk-d89a8e7c92c2
git clone --depth 1 --branch "$RELEASE_TAG" https://github.com/gosh-sh/avm-sdk.git
cd avm-sdk
rustup toolchain install 1.99.1 --profile minimal --target wasm32-unknown-unknown
```

The installer places the selected release under `${AVM_SDK_HOME:-$HOME/.local/share/avm-sdk}` and links its commands into `${AVM_SDK_BIN_DIR:-$HOME/.local/bin}`.

```bash
./scripts/install.sh "$RELEASE_TAG"
export PATH="${AVM_SDK_BIN_DIR:-$HOME/.local/bin}:$PATH"
```

## Initialize The Stand

Every command uses the same state and Wasmtime profile paths. Command output is JSON. `init` generates fresh random developer, package-owner, and user identities for this state. Each deploy below is an external deploy: it creates the first contract of a new DApp, whose DApp id is the account id of the contract.

```bash
export AVM_SDK_HOME="${AVM_SDK_HOME:-$HOME/.local/share/avm-sdk}"
export AVM_STATE="${PWD}/developer.state"
export AVM_PROFILE="${AVM_SDK_HOME}/current/profiles/wasmtime-release-profile.v1"

# Use the same state and profile for every local stand command.
stand() {
  local command="$1"
  shift
  avm-local-stand "$command" --state "$AVM_STATE" --wasmtime-profile "$AVM_PROFILE" "$@"
}

if [[ ! -e "$AVM_STATE" ]]; then
  stand init
fi
```

`init` prints the public identities. The state file also holds their signing keys, so keep it private. When resuming a session, the block above reuses the existing state. You can read its public identities with `stand inspect`. A fresh state has a new package owner and needs a new build of contract A.

## Build A Contract

Each example records its tested stack, ABI arena and peak memory in `avm-memory.json`. Deployment builds use those resources unless explicitly overridden in `avm-contract.toml` or, for the arena, in the export macro. Read [contract memory](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/contract-memory.md) before repeating or extending tests.

Copy the [contract A v1 example](https://github.com/gosh-sh/avm-sdk/tree/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/examples/contract-a-v1) inside this checkout before modifying it so its relative SDK dependency remains valid. The build command compiles the contract twice, verifies reproducibility, validates its AVM metadata, and writes three artifacts under `target/avm`.

```bash
if [[ ! -d examples/my-avm-contract ]]; then
  cp -R examples/contract-a-v1 examples/my-avm-contract
fi
# Keep the supplied lock, or explicitly resolve/review dependencies after edits:
# cargo +1.99.1 generate-lockfile --manifest-path examples/my-avm-contract/Cargo.toml

# Arguments: contract directory and the public key authorized to upgrade it.
build_contract() {
  env RUSTUP_TOOLCHAIN=1.99.1 \
    CARGO="$(rustup which --toolchain 1.99.1 cargo)" \
    RUSTC="$(rustup which --toolchain 1.99.1 rustc)" \
    AVM_EXAMPLE_PACKAGE_OWNER_PUBKEY="$2" \
    avm-contract-build build --manifest-path "$1/Cargo.toml"
}

PACKAGE_OWNER="$(stand inspect | jq -ern 'input.identities.package_owner')" && build_contract examples/my-avm-contract "$PACKAGE_OWNER"
```

The per-command environment selects the guest compiler even when the calling shell or native Cargo test uses Rust 1.98.0. Native runtime tools retain their separate 1.98.0 build identity. The builder rejects a different guest compiler, Cargo version or compiler wrapper before compilation.

Artifacts:

* `contract.wasm`: executable WASM;
* `contract.acab`: canonical ABI bytes;
* `contract.descriptor`: canonical code descriptor.

Contract A keeps its owner key after its 72 contract bytes (`owner_key_offset = 72`) and writes the deploy's `owner_id` there in `init`; only that key signs its external calls. See [owner key and external calls](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/owner-key-and-external-calls.md).

Contract A accepts `AVM_EXAMPLE_PACKAGE_OWNER_PUBKEY` only as 64 lowercase hex digits at compile time. This public key becomes part of the WASM and determines who may request its upgrade. Omitting it retains the fixed source-test owner, which differs from a freshly initialized public stand. Changing the key changes the built code identity; it does not change the ABI/state schema or upgrade an already deployed contract.

## Deploy

Use explicit logical timestamps. The stand does not read wall-clock time. A deployment address is derived from the package, owner, salt, and initialization data. The commands below deploy contract A with the package owner as its owner, so that the same key signs its calls and its upgrade. Use `--quote-address` to derive the address, then fund it and submit the deploy with the same options. An underfunded deploy returns `funding_required`.

```bash
ARTIFACTS=examples/my-avm-contract/target/avm

deploy_options=(
  --wasm "$ARTIFACTS/contract.wasm"
  --acab "$ARTIFACTS/contract.acab"
  --descriptor "$ARTIFACTS/contract.descriptor"
  --salt-hex 646576656c6f706572 --init-json null --owner-id "$PACKAGE_OWNER"
  --block-time 1800000000 --created-at 1800000000 --expire-at 1800003600
  --fuel-fee-shells 1000
)

stand deploy "${deploy_options[@]}" --quote-address > quote.json
CONTRACT_ADDRESS="$(jq -er .address quote.json)"
stand fund-user --address "$CONTRACT_ADDRESS" --shells 100000
stand deploy "${deploy_options[@]}" > deploy.json
```

`deploy.json` contains the receipt for the account created at `CONTRACT_ADDRESS`.

## Call And Inspect

```bash
stand call \
  --address "$CONTRACT_ADDRESS" \
  --method causal.a.version \
  --args-json null \
  --identity package-owner \
  --extra-currencies '' \
  --block-time 1800000010 \
  --created-at 1800000009 \
  --expire-at 1800003609 \
  --fuel-fee-shells 1000 > call.json

jq -r '.status' call.json

stand inspect --account "$CONTRACT_ADDRESS" > account.json

jq -r '.raw_state[32:34]' account.json
```

A successful call prints `ok` at the first check. The second check prints `01`: the `causal.a.version` method of contract A v1 writes its version marker to byte 16 of account state. Each byte occupies two hex digits in `raw_state`.

Method names and JSON arguments must match the selected contract ABI. See each example's `avm-contract.toml`. A call under another identity is rejected with `AccountKeyNotAllowed` and changes nothing.

The `--fuel-fee-shells` value is a reserve of ordinary SHELL for chargeable execution, not a separate currency or account balance. Same-DApp messages can execute without a SHELL charge while their source and destination remain in the same active routing partition. See [FUEL And SHELL](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/fuel-and-fees.md) before choosing reserves for internal messages.

## Upgrade And Replay

Build contract A v2 with the same package-owner key. Use the v1 package ID from `quote.json` to publish v2 in the same package. The v2 publication has its own address to fund.

```bash
build_contract examples/contract-a-v2 "$PACKAGE_OWNER"
V2_ARTIFACTS=examples/contract-a-v2/target/avm
PACKAGE_ID="$(jq -er .package_id quote.json)"
v2_options=(
  --package-id "$PACKAGE_ID"
  --wasm "$V2_ARTIFACTS/contract.wasm"
  --acab "$V2_ARTIFACTS/contract.acab"
  --descriptor "$V2_ARTIFACTS/contract.descriptor"
  --salt-hex 7632 --init-json null --owner-id "$PACKAGE_OWNER"
  --block-time 1800000020 --created-at 1800000020 --expire-at 1800003620
  --fuel-fee-shells 1000
)

stand deploy "${v2_options[@]}" --quote-address > v2-quote.json
V2_ADDRESS="$(jq -er .address v2-quote.json)"
stand fund-user --address "$V2_ADDRESS" --shells 100000
stand publish-version "${v2_options[@]}" > publish.json
```

Inspect the newly published account to read its `selected_code_id`. Apply that selection to the **original contract address**. The hex migration payload below encodes the exact bytes `avm-a-v2-migrate-state!!` required by this example.

```bash
stand inspect --account "$V2_ADDRESS" > v2-account.json
SELECTED_CODE_ID="$(jq -er .selected_code_id v2-account.json)"
stand upgrade --address "$CONTRACT_ADDRESS" --selected-code-id "$SELECTED_CODE_ID" \
  --identity package-owner --migration-payload-hex 61766d2d612d76322d6d6967726174652d73746174652121 \
  --block-time 1800000030 --created-at 1800000030 --expire-at 1800003630 \
  --fuel-fee-shells 1000 > upgrade.json

stand call --address "$CONTRACT_ADDRESS" --method causal.a.version --args-json null \
  --identity package-owner --extra-currencies '' \
  --block-time 1800000040 --created-at 1800000040 --expire-at 1800003640 \
  --fuel-fee-shells 1000 > v2-call.json
jq -r '.raw_state[32:34]' v2-call.json
```

The version marker at the original address now reads `02`.

To verify replay, take a snapshot of the live stand and replay its accepted input journal. The comparison prints `true` when both roots and the cumulative fee match.

```bash
stand inspect > live.json
stand replay > replay.json
jq -es '
  map([.registry_root, .account_state_root, .cumulative_fee_shells])
  | length == 2 and (.[0] | all(. != null)) and .[0] == .[1]
' live.json replay.json
```

The SDK release gate builds real Rust contracts and runs the standard-contract developer stand E2E before testing the packaged commands. The source-level `Wallet10K` workload belongs to the AVM block-performance gate; SDK packaging does not repeat it as a sequential stand loop.

### Use A Network

Contract A embeds its upgrade authority in the WASM at build time. If the network package owner uses a different key, rebuild the contract with `build_contract examples/my-avm-contract "$NETWORK_PACKAGE_OWNER"`, where `NETWORK_PACKAGE_OWNER` is that owner's 64-digit lowercase hex public key.

For a new package, use the resulting artifacts with `avm-local-stand network-deploy`. Pass the matching signing key file with `--package-owner-key` and the same public key with `--owner-id` when quoting the address and submitting the deploy. This makes the package owner the account owner as well, so its key can sign external calls and upgrade requests for this example. Use `network-call --key` with that owner's key file to call the contract. Both commands need the node's HTTP API URL. See [contract deploy](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/contract-deploy.md#deploy-on-a-network) and [owner key and external calls](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/owner-key-and-external-calls.md#call-a-contract-on-a-network). [From the local stand to a network](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/network.md) walks through the steps and says what you can read back from a network.

## Useful Links

* [CLI Reference](https://github.com/gosh-sh/avm-sdk/blob/main/docs/cli-reference.md) — Reference for the contract builder and local stand commands. Covers parameters, output formats, and network commands.
* [Concepts](https://github.com/gosh-sh/avm-sdk/blob/main/docs/concepts.md) — Introduction to core AVM concepts: contracts, accounts, addresses, messages, owner keys, and code packages.
* [Contract Deploy](https://github.com/gosh-sh/avm-sdk/blob/main/docs/contract-deploy.md) — Deploying contracts with new or previously published code. Explains network deployment phases, address funding, and fees.
* [Contract Memory](https://github.com/gosh-sh/avm-sdk/blob/main/docs/contract-memory.md) — Measuring and configuring contract stack, ABI arena, and linear memory. Explains how to generate `avm-memory.json` from tests and validate resources before deployment.
* [Crypto](https://github.com/gosh-sh/avm-sdk/blob/main/docs/crypto.md) — Using BLAKE3, SHA-256, and Ed25519 signature verification in Rust contracts. Covers runtime requirements and operation costs in FUEL.
* [Events](https://github.com/gosh-sh/avm-sdk/blob/main/docs/events.md) — Declaring events in the ABI, emitting them from contracts, and reading them on the local stand. Describes event limits and network read availability.
* [FUEL and Fees](https://github.com/gosh-sh/avm-sdk/blob/main/docs/fuel-and-fees.md) — Explains the difference between the FUEL execution budget and the SHELL currency. Covers paid and free routes, reserves, and charges for computation, memory, and code publication.
* [Getting Started](https://github.com/gosh-sh/avm-sdk/blob/main/docs/getting-started.md) — A walkthrough of installing the SDK and building, deploying, and calling your first contract. Also demonstrates contract upgrades and replay verification.
* [Known Limitations](https://github.com/gosh-sh/avm-sdk/blob/main/docs/known-limitations.md) — Known limitations of the current release and available alternatives. Covers network reads, events, execution, and differences between the local stand and a network.
* [Network](https://github.com/gosh-sh/avm-sdk/blob/main/docs/network.md) — Moving from the local stand to a network node: preparing keys, deriving and funding an address, deploying, and calling contracts. Clarifies which outcomes and data the SDK’s network commands can read.
* [Owner Key and External Calls](https://github.com/gosh-sh/avm-sdk/blob/main/docs/owner-key-and-external-calls.md) — Storing, initializing, and rotating an account’s owner key. Explains external-call authorization and how other users interact with a contract.
* [Release Notes](https://github.com/gosh-sh/avm-sdk/blob/main/docs/release-notes.md) — SDK changes by version. Includes compatibility information, behavior changes, and upgrade requirements.
* [Releases](https://github.com/gosh-sh/avm-sdk/blob/main/docs/releases.md) — Release bundle contents, tool provenance, and artifact integrity checks. Describes how to verify and install a prepared local bundle.
* [Runtime Limits](https://github.com/gosh-sh/avm-sdk/blob/main/docs/runtime-limits.md) — Execution limits, data and message sizes, storage rent, and address derivation rules. Shows how to inspect the local stand’s configuration and understand resource constraints.
* [Statuses and Errors](https://github.com/gosh-sh/avm-sdk/blob/main/docs/statuses-and-errors.md) — Reference for message statuses and common local stand and network errors. Provides causes and remedies for common failures.
* [Tooling Preview](https://github.com/gosh-sh/avm-sdk/blob/main/docs/tooling-preview.md) — SDK tooling features, including Core-WASM V2 builds, constructor data, generated identifiers, and typed dispatch. Also covers ABI and account inspection and reproducible builds.
* [Unsafe](https://github.com/gosh-sh/avm-sdk/blob/main/docs/unsafe.md) — Rationale for the SDK’s internal `unsafe` code. Documents safety invariants and tests for the input buffer and runtime cryptographic calls.
