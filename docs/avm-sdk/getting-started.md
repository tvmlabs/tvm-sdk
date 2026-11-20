# Build and Call Your First AVM Contract

For an introduction to the execution model, see [Rust contracts with AVM](https://docs.ackinacki.com/for-developers/smart-contracts#rust-contracts-with-avm).

These instructions describe SDK 0.6.5 and Core-WASM V2. Use its matching
builder, local stand and profile bundle. Earlier releases retain their own
compilers/profiles; follow the documentation inside each installed release.
A [local bundle](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/releases.md#install-a-prepared-local-bundle) can also be
verified and installed directly.

## Prerequisites

- Linux on an x86-64-v3 CPU with glibc 2.39 or later for this release's
  contract builder. Check the installed glibc with `getconf GNU_LIBC_VERSION`.
- Git, Bash, Python 3.11+ (available as `python3`), `jq`, and GitHub CLI
  (`gh`) with access to public releases.
- Rustup to install the guest compiler and WebAssembly target.

## Install the SDK

From a writable directory, clone the tag for this guide. Run the rest of the
commands in the same Bash session from the root of that checkout.

```bash
export RELEASE_TAG=avm-dev-d72c855b2eae-sdk-d89a8e7c92c2
git clone --depth 1 --branch "$RELEASE_TAG" https://github.com/gosh-sh/avm-sdk.git
cd avm-sdk
rustup toolchain install 1.99.1 --profile minimal --target wasm32-unknown-unknown
```

The installer places the selected release under
`${AVM_SDK_HOME:-$HOME/.local/share/avm-sdk}` and links its commands into
`${AVM_SDK_BIN_DIR:-$HOME/.local/bin}`.

```bash
./scripts/install.sh "$RELEASE_TAG"
export PATH="${AVM_SDK_BIN_DIR:-$HOME/.local/bin}:$PATH"
```

## Initialize The Stand

Every command uses the same state and Wasmtime profile paths. Command output is
JSON. `init` generates fresh random developer, package-owner, and user identities
for this state. Each deploy below is an external deploy: it creates the first
contract of a new DApp, whose DApp id is the account id of the contract.

```bash
export AVM_SDK_HOME="${AVM_SDK_HOME:-$HOME/.local/share/avm-sdk}"
export AVM_STATE="${PWD}/developer.state"
export AVM_PROFILE="${AVM_SDK_HOME}/current/profiles/wasmtime-release-profile.v1"

avm-local-stand init \
  --state "$AVM_STATE" \
  --wasmtime-profile "$AVM_PROFILE" > init.json
```

`init.json` returns public identities. The owner-private state file also retains
their signing keys; keep it private. Reinitializing a fresh state creates new
identities, so rebuild contract A for its new package owner when testing upgrades.

## Build A Contract

Each example records its tested stack, ABI arena and peak memory in
`avm-memory.json`. Deployment builds use those resources unless explicitly
overridden in `avm-contract.toml` or, for the arena, in the export macro.
Read [contract memory](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/contract-memory.md) before repeating or extending tests.

Copy the [contract A v1 example](https://github.com/gosh-sh/avm-sdk/tree/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/examples/contract-a-v1) inside this checkout
before modifying it so its relative SDK dependency remains valid. The build
command compiles the contract twice, verifies reproducibility, validates its
AVM metadata, and writes three artifacts under `target/avm`.

```bash
cp -R examples/contract-a-v1 examples/my-avm-contract
# Bind the example's upgrade authority to this stand's public package-owner key.
export AVM_EXAMPLE_PACKAGE_OWNER_PUBKEY="$(jq -er .identities.package_owner init.json)"
# Keep the supplied lock, or explicitly resolve/review dependencies after edits:
# cargo +1.99.1 generate-lockfile --manifest-path examples/my-avm-contract/Cargo.toml
env RUSTUP_TOOLCHAIN=1.99.1 \
  CARGO="$(rustup which --toolchain 1.99.1 cargo)" \
  RUSTC="$(rustup which --toolchain 1.99.1 rustc)" \
  avm-contract-build build \
  --manifest-path examples/my-avm-contract/Cargo.toml
```

The per-command environment selects the guest compiler even when the calling
shell or native Cargo test uses Rust 1.98.0. Native runtime tools retain their
separate 1.98.0 build identity. The builder rejects a different guest compiler,
Cargo version or compiler wrapper before compilation.

Artifacts:

- `contract.wasm`: executable WASM;
- `contract.acab`: canonical ABI bytes;
- `contract.descriptor`: canonical code descriptor.

Contract A keeps its owner key after its 72 contract bytes
(`owner_key_offset = 72`) and writes the deploy's `owner_id` there in `init`;
only that key signs its external calls. See
[owner key and external calls](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/owner-key-and-external-calls.md).

Contract A accepts `AVM_EXAMPLE_PACKAGE_OWNER_PUBKEY` only as 64 lowercase hex
digits at compile time. This public key becomes part of the WASM and determines
who may request its upgrade. Omitting it retains the fixed source-test owner,
which differs from a freshly initialized public stand. Changing the key changes
the built code identity; it does not change the ABI/state schema or upgrade an
already deployed contract.

## Deploy

Use explicit logical timestamps. The stand does not read wall-clock time. A
deployment address is derived from the package, owner, salt, and initialization
data. The commands below deploy contract A with the package owner as its owner,
so that the same key signs its calls and its upgrade. The first unfunded attempt is intentionally side-effect free and returns
that exact address as a structured precondition.

```bash
ARTIFACTS=examples/my-avm-contract/target/avm

set +e
avm-local-stand deploy \
  --state "$AVM_STATE" \
  --wasmtime-profile "$AVM_PROFILE" \
  --wasm "$ARTIFACTS/contract.wasm" \
  --acab "$ARTIFACTS/contract.acab" \
  --descriptor "$ARTIFACTS/contract.descriptor" \
  --salt-hex 646576656c6f706572 \
  --init-json null \
  --owner-id "$AVM_EXAMPLE_PACKAGE_OWNER_PUBKEY" \
  --block-time 1800000000 \
  --created-at 1800000000 \
  --expire-at 1800003600 \
  --fuel-fee-shells 1000 > funding-required.json
deploy_status=$?
set -e

test "$deploy_status" -eq 2
test "$(jq -r .error funding-required.json)" = funding_required
CONTRACT_ADDRESS="$(jq -r .address funding-required.json)"

avm-local-stand fund-user \
  --state "$AVM_STATE" \
  --wasmtime-profile "$AVM_PROFILE" \
  --address "$CONTRACT_ADDRESS" \
  --shells 100000

avm-local-stand deploy \
  --state "$AVM_STATE" \
  --wasmtime-profile "$AVM_PROFILE" \
  --wasm "$ARTIFACTS/contract.wasm" \
  --acab "$ARTIFACTS/contract.acab" \
  --descriptor "$ARTIFACTS/contract.descriptor" \
  --salt-hex 646576656c6f706572 \
  --init-json null \
  --owner-id "$AVM_EXAMPLE_PACKAGE_OWNER_PUBKEY" \
  --block-time 1800000000 \
  --created-at 1800000000 \
  --expire-at 1800003600 \
  --fuel-fee-shells 1000 > deploy.json
```

The deploy result contains the canonical receipt. Repeating the same derivation
after funding creates the account at `CONTRACT_ADDRESS`.

## Call And Inspect

```bash
avm-local-stand call \
  --state "$AVM_STATE" \
  --wasmtime-profile "$AVM_PROFILE" \
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

avm-local-stand inspect \
  --state "$AVM_STATE" \
  --wasmtime-profile "$AVM_PROFILE" \
  --account "$CONTRACT_ADDRESS" > account.json

jq -r '.raw_state[32:34]' account.json
```

A successful call prints `ok` at the first check. The second check prints
`01`: the `causal.a.version` method of contract A v1 writes its version
marker to byte 16 of account state. Each byte occupies two hex digits in
`raw_state`.

Method names and JSON arguments must match the selected contract ABI. See each
example's `avm-contract.toml`. A call under another identity is rejected with
`AccountKeyNotAllowed` and changes nothing.

The `--fuel-fee-shells` value is a reserve of ordinary SHELL for chargeable
execution, not a separate currency or account balance. Same-DApp messages can
execute without a SHELL charge while their source and destination remain in the
same active routing partition. See [FUEL And SHELL](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/fuel-and-fees.md) before
choosing reserves for internal messages.

## Use A Network

The same artifacts deploy to a network node with `avm-local-stand
network-deploy`, and the owner key calls the contract with `network-call`.
Both sign with key files and need the node's HTTP API URL; see
[contract deploy](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/contract-deploy.md#deploy-on-a-network) and
[owner key and external calls](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/owner-key-and-external-calls.md#call-a-contract-on-a-network).
[From the local stand to a network](https://github.com/gosh-sh/avm-sdk/blob/avm-dev-d72c855b2eae-sdk-d89a8e7c92c2/docs/network.md) walks through the steps and
says what you can read back from a network.

## Upgrade And Replay

Build a new version, publish it with `publish-version`, then call `upgrade`
with the returned `selected_code_id`. `upgrade` requires the target account,
identity, explicit timestamps, fee, and a hex migration payload.

After any workflow, replay the accepted input journal and compare the returned
roots and cumulative fees:

```bash
avm-local-stand replay \
  --state "$AVM_STATE" \
  --wasmtime-profile "$AVM_PROFILE"
```

The SDK release gate builds real Rust contracts and runs the standard-contract
developer stand E2E before testing the packaged commands. The source-level
`Wallet10K` workload belongs to the AVM block-performance gate; SDK packaging
does not repeat it as a sequential stand loop.
