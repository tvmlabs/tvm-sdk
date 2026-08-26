---
description: Deploy and operate UpdateCustodianMultisigWallet_v2 with TVM CLI
---

# How to deploy a Multisig Wallet

This guide deploys `UpdateCustodianMultisigWallet_v2` version **2.4.0**, compiled with `sold` **0.81.0**.

## Prerequisites

* [tvm-cli 3.0.4 or later](https://github.com/tvmlabs/tvm-sdk/releases); the commands in this guide were verified with version `3.0.4`
* `jq` for reading the generated public key from the key file
* Access to the Acki Nacki Shellnet or Mainnet

{% hint style="warning" %}
TVM SDK v3 uses extended addresses for CLI command address arguments:

`<dapp_id>::<account_id>`

A newly deployed Multisig wallet is self-rooted, so its DApp ID is equal to its account ID. ABI parameters of Solidity type `address` still use the regular `0:<account_id>` format inside JSON arguments.

If a CLI command receives a legacy address, version `3.0.4` reports:

```text
Error: address `0:…` must be in the form `dapp_id::account_id` (two 64-character hex ids, no 0x, no workchain); legacy `0:…`, bare-hex, single-colon and 128-hex forms are no longer accepted
```
{% endhint %}

The guide uses the following placeholders:

* `<MSIG_ACCOUNT_ID>` — the Multisig wallet's 64-character account ID
* `<MSIG_DAPP_ID>` — the Multisig wallet's 64-character DApp ID; it is equal to `<MSIG_ACCOUNT_ID>` for a new self-rooted wallet
* `<MSIG_ADDR>` — the extended Multisig address, `<MSIG_DAPP_ID>::<MSIG_ACCOUNT_ID>`
* `<DEST_ACCOUNT_ID>` — the recipient's 64-character account ID
* `<DEST_DAPP_ID>` — the recipient's 64-character DApp ID
* `<OWNER_PUBLIC_KEY>` — the public key embedded in the TVC by `genaddr --save`, without the `0x` prefix
* `<FIRST_OWNER_PUBLIC_KEY>` — the first current or proposed custodian key; in the two-custodian deployment example, this is `<OWNER_PUBLIC_KEY>`
* `<SECOND_OWNER_PUBLIC_KEY>` — an additional custodian public key without the `0x` prefix
* `<CUSTODIAN_KEYS_FILE>`, `<FIRST_CUSTODIAN_KEYS_FILE>`, and `<NEXT_CUSTODIAN_KEYS_FILE>` — key files belonging to the custodians that call, submit, and confirm a request
* `<NETWORK_ENDPOINT>` — `shellnet.ackinacki.org` or `mainnet.ackinacki.org`

{% hint style="info" %}
The outputs below are representative and may contain additional metadata fields. Addresses, transaction IDs, hashes, balances, timestamps, and producer names will differ. For JSON transaction responses, `"aborted": false` and `"exit_code": 0` indicate successful execution.
{% endhint %}

<a id="create-a-wallet-1"></a>

## Prepare the wallet binary and ABI

Create a working folder:

```bash
mkdir -p ~/wallet
```

Example output:

```text
No output is printed on success.
```

Enter the working folder:

```bash
cd ~/wallet
```

Example output:

```text
No output is printed on success. The current directory is now ~/wallet.
```

Download these files from the `0.81.0_compiled/updatecustodianmultisigwallet_v2` directory:

* [`UpdateCustodianMultisigWallet_v2.abi.json`](https://raw.githubusercontent.com/ackinacki/ackinacki/main/contracts/0.81.0_compiled/updatecustodianmultisigwallet_v2/UpdateCustodianMultisigWallet_v2.abi.json)
* [`UpdateCustodianMultisigWallet_v2.tvc`](https://raw.githubusercontent.com/ackinacki/ackinacki/main/contracts/0.81.0_compiled/updatecustodianmultisigwallet_v2/UpdateCustodianMultisigWallet_v2.tvc)

The contract source is available in [`UpdateCustodianMultisigWallet_v2.sol`](https://github.com/ackinacki/ackinacki/blob/main/contracts/updatecustodianmultisigwallet_v2/UpdateCustodianMultisigWallet_v2.sol).

You can inspect the TVC metadata before using it:

```bash
tvm-cli decode stateinit --tvc UpdateCustodianMultisigWallet_v2.tvc
```

Example output (the `code` and `data` cells are shortened here):

```text
Input arguments:
   input: UpdateCustodianMultisigWallet_v2.tvc
Decoded data:
{
  "split_depth": "None",
  "special": "None",
  "data": "te6ccg...",
  "code": "te6ccg...",
  "code_hash": "cfcaac10d43c8dc062298cb48df097be67cddec52b9cfd558309a7549f01c1f1",
  "data_hash": "580cd7b1a754947447007c7a3001f2a4ce0f2b09666ec941061cd63ae770f0a6",
  "code_depth": "14",
  "data_depth": "3",
  "version": "sol 0.81.0",
  "lib": ""
}
```

The expected compiler version is `sol 0.81.0`, and the expected code hash is:

```text
cfcaac10d43c8dc062298cb48df097be67cddec52b9cfd558309a7549f01c1f1
```

## Configure TVM CLI

The Multisig wallet can be deployed to either Acki Nacki network:

* Shellnet — `shellnet.ackinacki.org`
* Mainnet — `mainnet.ackinacki.org`

Select the network before funding the precomputed address. The funding transaction, deployment, and all subsequent calls must use the same endpoint.

You can save the selected endpoint in the global TVM CLI configuration:

```bash
tvm-cli config -g --url <NETWORK_ENDPOINT>
```

Example output:

```text
Succeeded.
```

Alternatively, leave the global configuration unchanged and pass `-u <NETWORK_ENDPOINT>` immediately after `tvm-cli` in every network command. For example, use `tvm-cli -u <NETWORK_ENDPOINT> -j account ...`, `tvm-cli -u <NETWORK_ENDPOINT> -j deploy ...`, `tvm-cli -u <NETWORK_ENDPOINT> -j call ...`, and `tvm-cli -u <NETWORK_ENDPOINT> -j run ...`. The wallet examples below omit `-u` and assume that the endpoint was saved with `tvm-cli config -g`; the Shellnet-only giver commands keep an explicit `-u shellnet.ackinacki.org` as a safety measure.

{% hint style="danger" %}
Shellnet tokens have no Mainnet value. Do not reuse Shellnet keys for Mainnet wallets.
{% endhint %}

## Generate the key pair and address

Generate a seed phrase and key pair, write the generated public key into the TVC, and calculate the future wallet address:

```bash
tvm-cli genaddr \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --genkey UpdateCustodianMultisigWallet_v2.keys.json \
  --save \
  UpdateCustodianMultisigWallet_v2.tvc
```

Example output:

```text
Input arguments:
     tvc: UpdateCustodianMultisigWallet_v2.tvc
     abi: UpdateCustodianMultisigWallet_v2.abi.json
      wc: None
    keys: UpdateCustodianMultisigWallet_v2.keys.json
init_data: None
is_update_tvc: true
TVC file updated

Seed phrase: "<12-WORD_SEED_PHRASE>"
Raw address: 0:<MSIG_ACCOUNT_ID>
dapp::account: <MSIG_DAPP_ID>::<MSIG_ACCOUNT_ID>
Succeeded
```

For this self-rooted wallet, `<MSIG_DAPP_ID>` and `<MSIG_ACCOUNT_ID>` are the same value. Use the printed `dapp::account` value as `<MSIG_ADDR>` in TVM CLI commands.

The key pair is saved to `UpdateCustodianMultisigWallet_v2.keys.json`. Its `public` field is `<OWNER_PUBLIC_KEY>`:

```bash
jq -r .public UpdateCustodianMultisigWallet_v2.keys.json
```

Example output:

```text
0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
```

{% hint style="danger" %}
Write down the seed phrase and store it securely. Never share the seed phrase or the private key. Anyone who obtains either of them can sign transactions as this custodian.

The TVC contains the public key, not the private key. Back up `UpdateCustodianMultisigWallet_v2.keys.json` securely and do not commit it to a repository.
{% endhint %}

### Generate additional custodian keys

Generate a separate key file for every additional public-key custodian:

```bash
tvm-cli genphrase --dump SecondCustodian.keys.json
```

Example output:

```text
Succeeded.
Seed phrase: <SECOND_12_WORD_SEED_PHRASE>
Keypair successfully saved to SecondCustodian.keys.json.
Succeeded.
Keypair saved to SecondCustodian.keys.json
```

If you already have a securely stored seed phrase, recreate its key file instead:

```bash
tvm-cli getkeypair \
  -o SecondCustodian.keys.json \
  -p '<SECOND_12_WORD_SEED_PHRASE>'
```

Example output:

```text
Input arguments:
key_file: SecondCustodian.keys.json
  phrase: <SECOND_12_WORD_SEED_PHRASE>
Keypair successfully saved to SecondCustodian.keys.json.
Succeeded.
```

Read the additional public key:

```bash
jq -r .public SecondCustodian.keys.json
```

Example output:

```text
abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789
```

Save this value as `<SECOND_OWNER_PUBLIC_KEY>`. Keep each custodian's seed phrase and key file separate; a confirmation must be signed with the key file that corresponds to that custodian.

## Fund the precomputed address

The precomputed address must have enough VMSHELL to pay the deployment fee. The funding method depends on the selected network.

### Shellnet

Use the Shellnet giver as described in the [Shellnet test tokens guide](https://dev.ackinacki.com/readme/get-test-tokens-in-shellnet).

{% hint style="warning" %}
The giver is available only on Shellnet. On Mainnet, SHELL tokens must be purchased through the dedicated pool and then converted into VMSHELL to pay network fees.
{% endhint %}

Before funding, querying the precomputed address normally returns `failed to get account: Not found: Resource not found`; the account becomes visible as `Uninit` only after it receives funds.

Download [`GiverV3.abi.json`](https://raw.githubusercontent.com/ackinacki/ackinacki/main/contracts/giver/GiverV3.abi.json), then choose exactly one of the following funding variants.

#### Exchange with flag 16

Flag `16` converts the SHELL in `ecc[2]` into VMSHELL at the destination. Because send flag `1` is not set, forwarding fees are deducted from the attached `value`.

```bash
tvm-cli -u shellnet.ackinacki.org -j callx \
  --abi GiverV3.abi.json \
  --addr 0000000000000000000000000000000000000000000000000000000000000000::1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrencyWithFlag \
  '{"dest":"0:<MSIG_ACCOUNT_ID>","value":1000000000,"ecc":{"2":1000000000000},"flag":16}'
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "1111111111111111111111111111111111111111111111111111111111111111",
  "dapp_id": "0000000000000000000000000000000000000000000000000000000000000000"
}
```

#### Combine flags 16 and 1

Flag `17` is the numeric combination `16 | 1`. Bit `16` performs the same SHELL-to-VMSHELL exchange, while bit `1` pays forwarding fees separately from the giver's VMSHELL balance, so they are not deducted from the attached `value`. Repository integration tests use this combined variant.

```bash
tvm-cli -u shellnet.ackinacki.org -j callx \
  --abi GiverV3.abi.json \
  --addr 0000000000000000000000000000000000000000000000000000000000000000::1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrencyWithFlag \
  '{"dest":"0:<MSIG_ACCOUNT_ID>","value":1000000000,"ecc":{"2":1000000000000},"flag":17}'
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "1111111111111111111111111111111111111111111111111111111111111111",
  "dapp_id": "0000000000000000000000000000000000000000000000000000000000000000"
}
```

In both variants, `1000000000000` is `1000 SHELL`, expressed in the smallest units. Do not run both commands for the same funding operation.

Check the precomputed account:

```bash
tvm-cli -j account <MSIG_ADDR>
```

Example output:

```json
{
  "acc_type": "Uninit",
  "address": "<MSIG_ACCOUNT_ID>",
  "balance": "1000000000000",
  "code_hash": "null",
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>",
  "ecc_balance": {
    "2": "0"
  }
}
```

Before deployment, its state must be `Uninit`, and its `balance` must be greater than zero. After using either giver variant above, the SHELL has already been converted into VMSHELL, so there must be no positive ECC balance for token `2`. On Shellnet, `tvm-cli 3.0.4` can show the zero entry as `{"2":"0"}` for the funded `Uninit` account and normalize it to `{}` after deployment; both representations mean that no SHELL remains.

### Mainnet

The giver is not available on Mainnet. Purchase SHELL through the dedicated Mainnet pool, fund the precomputed address, and convert enough SHELL into VMSHELL to cover the deployment fee. Before deploying, query the account through `mainnet.ackinacki.org` and verify that its state is `Uninit` and its VMSHELL `balance` is greater than zero.

## Deploy the Multisig wallet

{% hint style="warning" %}
The deployment fee is not fixed. It depends on the size of the contract code and data being deployed, as well as the computation performed by the constructor. Fund the precomputed address with enough VMSHELL to cover the expected deployment fee with a safety margin.
{% endhint %}

The constructor has the following parameters:

* `owners_pubkey` — custodian public keys with the `0x` prefix
* `owners_address` — custodian contract addresses in `0:<account_id>` format
* `reqConfirms` — confirmations required for regular transactions
* `reqConfirmsData` — confirmations required for custodian, code, and balance-configuration updates
* `value` — amount to pass to `gosh.cnvrtshellq`, expressed in nanoVMSHELL (the smallest VMSHELL units; `1 VMSHELL = 1,000,000,000 nanoVMSHELL`)
* `minBalance` — VMSHELL balance below which the wallet converts its own SHELL to refill gas; `0` disables automatic top-up
* `targetBalance` — VMSHELL balance the wallet converts SHELL up to when it tops itself up

`minBalance` and `targetBalance` set the initial gas self-management configuration described in [Manage the wallet's gas balance](#manage-the-wallets-gas-balance). A `targetBalance` lower than `minBalance` is rejected with contract error `126`. The examples below deploy with automatic top-up disabled; it can be enabled later without redeploying.

The two submitted arrays must contain between 1 and 32 entries in total, and every entry must be non-zero. This limit is checked before deduplication, so duplicate entries still consume one of the 32 input slots. Duplicates are ignored when the effective custodian set is built. The contract limits both confirmation thresholds to the number of unique custodians and ensures that the effective `reqConfirmsData` is not lower than the effective `reqConfirms`.

A public-key custodian signs external calls with its key file. An address custodian is a contract listed in `owners_address`; it confirms requests by sending an internal message from that exact address and cannot be represented by an externally signed CLI call. Internally, the wallet hashes `(public key, zero address)` for key custodians and `(zero public key, address)` for address custodians, so the two custodian types use disjoint identities and cannot collide.

The constructor converts up to `value` of the account's remaining SHELL into VMSHELL at a 1:1 smallest-unit ratio. The conversion is quiet: if less SHELL is available, it converts the available amount without failing. Use `value: 0` with either giver variant above because both flag `16` and flag `17` already convert the transferred SHELL. If the account was separately funded with unconverted SHELL, `value: 100000000` requests creation of `100,000,000 nanoVMSHELL` (`0.1 VMSHELL`) by converting the same smallest-unit amount of SHELL, as used by the repository integration tests.

The deployment signature is independent of the custodian arrays. The constructor requires the external deployment message to be signed by the key embedded in the TVC by `genaddr --save`, so `--sign` must use `UpdateCustodianMultisigWallet_v2.keys.json` even if `<OWNER_PUBLIC_KEY>` is not included in `owners_pubkey`. The deployment signer can operate the deployed wallet only if that key is also included as a custodian.

Deploy a single-custodian wallet:

```bash
tvm-cli -j deploy \
  --dst-dapp-id <MSIG_DAPP_ID> \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign UpdateCustodianMultisigWallet_v2.keys.json \
  UpdateCustodianMultisigWallet_v2.tvc \
  '{"owners_pubkey":["0x<OWNER_PUBLIC_KEY>"],"owners_address":[],"reqConfirms":1,"reqConfirmsData":1,"value":0,"minBalance":0,"targetBalance":0}'
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>",
  "deployed_at": "<MSIG_DAPP_ID>::<MSIG_ACCOUNT_ID>"
}
```

Alternatively, deploy a two-custodian, two-confirmation wallet to use the confirmation flows later in this guide. Choose either the single-custodian command above or this command; the same precomputed address can be deployed only once.

```bash
tvm-cli -j deploy \
  --dst-dapp-id <MSIG_DAPP_ID> \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign UpdateCustodianMultisigWallet_v2.keys.json \
  UpdateCustodianMultisigWallet_v2.tvc \
  '{"owners_pubkey":["0x<OWNER_PUBLIC_KEY>","0x<SECOND_OWNER_PUBLIC_KEY>"],"owners_address":[],"reqConfirms":2,"reqConfirmsData":2,"value":0,"minBalance":0,"targetBalance":0}'
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>",
  "deployed_at": "<MSIG_DAPP_ID>::<MSIG_ACCOUNT_ID>"
}
```

In this two-custodian example, use `UpdateCustodianMultisigWallet_v2.keys.json` as `<FIRST_CUSTODIAN_KEYS_FILE>` and `SecondCustodian.keys.json` as `<NEXT_CUSTODIAN_KEYS_FILE>`.

{% hint style="warning" %}
Use the DApp ID printed by `genaddr` in `--dst-dapp-id`. For a new self-rooted wallet it equals `<MSIG_ACCOUNT_ID>`. A Shellnet deployment with a zero DApp ID looks for the account in the root DApp and fails with `code 621: The account doesn't have a state`, because the giver-funded `Uninit` account is in `<MSIG_DAPP_ID>`.

After deployment, continue using the generated `<MSIG_ADDR>` extended address for account queries and contract calls.
{% endhint %}

Check the account again:

```bash
tvm-cli -j account <MSIG_ADDR>
```

Example output:

```json
{
  "acc_type": "Active",
  "address": "<MSIG_ACCOUNT_ID>",
  "balance": "<REMAINING_VMSHELL_BALANCE>",
  "code_hash": "cfcaac10d43c8dc062298cb48df097be67cddec52b9cfd558309a7549f01c1f1",
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>",
  "ecc_balance": {}
}
```

Its state must now be `Active`. Verify the deployed version:

```bash
tvm-cli -j run <MSIG_ADDR> getVersion '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "value0": "2.4.0",
  "value1": "UpdateCustodianMultisigWallet_v2"
}
```

`getVersion` must return `2.4.0` and `UpdateCustodianMultisigWallet_v2`.

Read the configured limits and confirmation thresholds:

```bash
tvm-cli -j run <MSIG_ADDR> getParameters '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "maxQueuedTransactions": "5",
  "maxCustodianCount": "32",
  "expirationTime": "3601",
  "requiredTxnConfirms": "1",
  "requiredDataConfirms": "1"
}
```

Read the effective custodian set:

```bash
tvm-cli -j run <MSIG_ADDR> getCustodians '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "custodians": [
    {
      "owner_pubkey": "0x<OWNER_PUBLIC_KEY>",
      "owner_address": null,
      "index": "0"
    }
  ]
}
```

`getCustodians` iterates a dictionary keyed by the hash of each custodian identity, so array order does not correspond to confirmation-mask bits. Always use each object's explicit `index` field; never infer the index from its position in the returned array.

Read the gas self-management configuration set by the constructor:

```bash
tvm-cli -j run <MSIG_ADDR> getBalanceConfig '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output for a wallet deployed with automatic top-up disabled:

```json
{
  "config": {
    "minBalance": "0",
    "targetBalance": "0"
  }
}
```

## Multisig Wallet API

In the examples below:

* the called wallet uses the extended `<MSIG_ADDR>` form
* `dest` uses Solidity `address` form, `0:<DEST_ACCOUNT_ID>`
* `dapp_id` uses uint256 form, `0x<DEST_DAPP_ID>`
* each call is signed with the key file of the custodian making that call

{% hint style="info" %}
The `dapp_id` argument of `sendTransaction` and `submitTransaction` is stored in queued transactions and emitted in lifecycle events. It does not route the transfer itself.
{% endhint %}

{% hint style="warning" %}
VMSHELL attached to an internal message is credited to the recipient minus fees only when the sender and recipient are in the same DApp. VMSHELL sent between different DApps is burned. Use ECC tokens, such as SHELL, when transferring value across DApps.
{% endhint %}

Pending requests expire after `3601` seconds. A custodian can have at most five pending requests in each request queue.

### Build a payload for a contract call

For a simple value transfer, use an empty string as `payload`. To transfer value and invoke a function on the destination contract, encode its internal message body with the destination contract's ABI:

```bash
tvm-cli body \
  --abi <TARGET_ABI_FILE> \
  <TARGET_METHOD> \
  '<TARGET_METHOD_PARAMS_JSON>'
```

Example output:

```text
Input arguments:
  method: <TARGET_METHOD>
  params: <TARGET_METHOD_PARAMS_JSON>
     abi: <TARGET_ABI_FILE>
  output: None
Message body: <BASE64_ENCODED_PAYLOAD_CELL>
```

Pass the `Message body` value as `payload` to `sendTransaction` or `submitTransaction`.

### Contract error codes

Contract failures use the following exit codes:

In `tvm-cli 3.0.4`, a failed network call is wrapped in a top-level CLI error with code `621`. The contract-specific code from the table below is available as `Error.data.node_error.extensions.details.exit_code` in the JSON response.

| Code | Meaning |
| ---: | --- |
| `100` | The sender is not a custodian, or the deployment message was not signed by the public key embedded in the TVC. |
| `101` | A proposed custodian public key or address is zero. |
| `102` | The requested transfer, custodian update, code update, or balance-configuration update does not exist. |
| `103` | This custodian has already confirmed the request. |
| `108` | `sendTransaction` was called on a wallet with more than one custodian. |
| `113` | This custodian already has five pending requests in that queue. |
| `117` | The submitted custodian arrays contain fewer than 1 or more than 32 entries in total. |
| `123` | `reqConfirms` or `reqConfirmsData` is zero. |
| `124` | `setMaxCleanupOperations` was called with a value below `1`. |
| `125` | The transfer destination is the zero address. |
| `126` | The balance configuration has `targetBalance` below `minBalance`. |

### Send a transaction from a single-custodian wallet

`sendTransaction` is available only when the wallet has exactly one custodian. A `1-of-N` wallet with more than one custodian must use `submitTransaction`, even when `reqConfirms` is `1`.

```solidity
sendTransaction(
    address dest,
    uint128 value,
    mapping(uint32 => varuint32) cc,
    bool bounce,
    uint8 flags,
    TvmCell payload,
    uint256 dapp_id
)
```

Parameters:

* `dest` — recipient account address
* `value` — VMSHELL attached to the internal message; it may be `0`
* `cc` — ECC token amounts keyed by token ID; SHELL uses key `2`
* `bounce` — use `true` for an existing contract when failures should bounce, and `false` for an undeployed account
* `flags` — send-message flags; use `1` for a regular transfer
* `payload` — the `Message body` produced by `tvm-cli body`, or an empty string for a simple transfer
* `dapp_id` — recipient DApp ID recorded in events

If `flags` contains send-all flag `128`, the node ignores the explicit `value` and sweeps the wallet's remaining VMSHELL balance. `TransactionSent.value` reports the balance being swept, not the supplied `value` argument.

Example regular transfer:

```bash
tvm-cli -j call <MSIG_ADDR> sendTransaction \
  '{
    "dest":"0:<DEST_ACCOUNT_ID>",
    "value":1000000000,
    "cc":{"2":5000000000},
    "bounce":false,
    "flags":1,
    "payload":"",
    "dapp_id":"0x<DEST_DAPP_ID>"
  }' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign UpdateCustodianMultisigWallet_v2.keys.json
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": {
    "value0": "0:<DEST_ACCOUNT_ID>"
  },
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

The call executes immediately and returns the destination address.

### Fund an undeployed account

Choose one of the following variants depending on how forwarding fees should be paid.

#### Exchange with flag 16

Use flag `16` to convert the SHELL in `cc[2]` into VMSHELL at the destination. Forwarding fees are deducted from the attached `value`:

```bash
tvm-cli -j call <MSIG_ADDR> sendTransaction \
  '{
    "dest":"0:<DEST_ACCOUNT_ID>",
    "value":1000000000,
    "cc":{"2":5000000000},
    "bounce":false,
    "flags":16,
    "payload":"",
    "dapp_id":"0x<DEST_DAPP_ID>"
  }' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign UpdateCustodianMultisigWallet_v2.keys.json
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": {
    "value0": "0:<DEST_ACCOUNT_ID>"
  },
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

#### Combine flags 16 and 1

Use flag `17` (`16 | 1`) to perform the same conversion and pay forwarding fees separately from the wallet's VMSHELL balance. The attached `value` is not reduced by forwarding fees:

```bash
tvm-cli -j call <MSIG_ADDR> sendTransaction \
  '{
    "dest":"0:<DEST_ACCOUNT_ID>",
    "value":1000000000,
    "cc":{"2":5000000000},
    "bounce":false,
    "flags":17,
    "payload":"",
    "dapp_id":"0x<DEST_DAPP_ID>"
  }' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign UpdateCustodianMultisigWallet_v2.keys.json
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": {
    "value0": "0:<DEST_ACCOUNT_ID>"
  },
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

Both commands convert `5 SHELL` into VMSHELL at the destination. When the undeployed destination is in another DApp, any attached VMSHELL remaining after applicable fees is burned rather than credited; the SHELL in `cc` is what becomes VMSHELL at the destination. Run only one variant for a funding operation.

For a wallet that requires multiple confirmations, use `submitTransaction` instead and pass the selected value through its singular `flag` parameter: `16` for exchange with fees deducted from `value`, or `17` for `16 | 1` with fees paid separately. Then collect the required confirmations.

### Submit a transaction

Any custodian can use `submitTransaction`:

```solidity
submitTransaction(
    address dest,
    uint128 value,
    mapping(uint32 => varuint32) cc,
    bool bounce,
    uint8 flag,
    TvmCell payload,
    uint256 dapp_id
) returns (uint64 transId)
```

Parameters:

* `dest` — recipient account address
* `value` — VMSHELL attached to the internal message; it may be `0`
* `cc` — ECC token amounts keyed by token ID; SHELL uses key `2`
* `bounce` — use `true` for an existing contract when failures should bounce, and `false` for an undeployed account
* `flag` — send-message flag; this parameter is singular, unlike `flags` in `sendTransaction`
* `payload` — the `Message body` produced by `tvm-cli body`, or an empty string for a simple transfer
* `dapp_id` — recipient DApp ID stored with the request and recorded in events

If `flag` contains send-all flag `128`, `submitTransaction` explicitly replaces `value` with `0` before executing or storing the request. The node then sweeps the balance. By contrast, `sendTransaction` passes its original `value` to the node, which ignores it under flag `128`. In both methods, `TransactionSent.value` reports the balance being swept rather than the supplied argument.

The submitting custodian also provides the first confirmation. If the effective `reqConfirms` is `1`, the transfer executes immediately and `transId` is `0`. Otherwise, the method returns the ID of the queued transaction.

```bash
tvm-cli -j call <MSIG_ADDR> submitTransaction \
  '{
    "dest":"0:<DEST_ACCOUNT_ID>",
    "value":1000000000,
    "cc":{"2":5000000000},
    "bounce":false,
    "flag":1,
    "payload":"",
    "dapp_id":"0x<DEST_DAPP_ID>"
  }' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <FIRST_CUSTODIAN_KEYS_FILE>
```

Example output for the single-custodian wallet deployed in this guide:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": {
    "transId": "0"
  },
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

When more than one confirmation is required, `return_value.transId` contains a non-zero `<TRANSACTION_ID>` and the transaction is queued.

You can list non-expired queued transaction IDs with:

```bash
tvm-cli -j run <MSIG_ADDR> getTransactionIds '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output for a wallet with a queued transaction:

```json
{
  "ids": [
    "<TRANSACTION_ID>"
  ]
}
```

### Confirm a transaction

Each additional custodian confirms the queued transaction with its own key file. Use this method only when `submitTransaction` returned a non-zero `transId`; a zero ID means that the transaction already executed.

```solidity
confirmTransaction(uint64 transactionId)
```

```bash
tvm-cli -j call <MSIG_ADDR> confirmTransaction \
  '{"transactionId":<TRANSACTION_ID>}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <NEXT_CUSTODIAN_KEYS_FILE>
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

The transfer executes when the required number of distinct custodian confirmations is collected. For an expired ID, the initial cleanup normally deletes the request and the subsequent lookup fails with contract error `102`. If the cleanup budget is exhausted on earlier expired entries before reaching this ID, the call can instead return `exit_code: 0` while deleting the request without executing it. Verify that the ID is still returned by `getTransactionIds` immediately before confirming.

### Submit a custodian update

Use `submitDataUpdate` to replace the complete custodian set and both confirmation thresholds:

```solidity
submitDataUpdate(
    uint256[] owners_pubkey,
    address[] owners_address,
    uint8 reqConfirms,
    uint8 reqConfirmsData
) returns (uint64 transId)
```

Parameters:

* `owners_pubkey` — the complete replacement list of public-key custodians
* `owners_address` — the complete replacement list of address custodians; each entry is a contract that must confirm with an internal message from that address
* `reqConfirms` — confirmations required for regular transactions under the new set
* `reqConfirmsData` — confirmations required for later custodian, code, and balance-configuration updates under the new set

The same validation rules as the constructor apply: the input arrays contain 1–32 non-zero entries before deduplication. A CLI call signed with a key can act only for a public-key custodian, not for an address custodian.

```bash
tvm-cli -j call <MSIG_ADDR> submitDataUpdate \
  '{
    "owners_pubkey":[
      "0x<FIRST_OWNER_PUBLIC_KEY>",
      "0x<SECOND_OWNER_PUBLIC_KEY>"
    ],
    "owners_address":[],
    "reqConfirms":2,
    "reqConfirmsData":2
  }' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <FIRST_CUSTODIAN_KEYS_FILE>
```

Example output for the single-custodian wallet deployed in this guide:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": {
    "transId": "0"
  },
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

When the current effective `reqConfirmsData` is greater than `1`, `return_value.transId` contains a non-zero `<DATA_UPDATE_ID>` and the update is queued.

The request is confirmed according to the wallet's current `reqConfirmsData`, not the proposed value. If the current effective threshold is `1`, the update applies immediately and `transId` is `0`.

List non-expired queued custodian-update IDs before confirming:

```bash
tvm-cli -j run <MSIG_ADDR> getUpdateDataIds '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "ids": [
    "<DATA_UPDATE_ID>"
  ]
}
```

The data-update queue has no submitted or confirmed events. Monitor its intermediate state by polling `getUpdateDataIds` or `getUpdateDatas`. `CustodiansUpdated` is emitted only when the update is actually applied.

{% hint style="warning" %}
Applying a custodian update clears all pending transfer, custodian-update, code-update, and balance-configuration-update queues. The discarded request counts are emitted in `RequestsDropped`, which reports one counter per queue. Pending requests must be submitted again under the new custodian set. Two operator settings survive the custodian change: the value configured through `setMaxCleanupOperations` and the balance configuration read by `getBalanceConfig`.
{% endhint %}

{% hint style="danger" %}
Verify every replacement key, address, and threshold before submitting or confirming the update. An incorrect custodian set can permanently lock all protected wallet methods because future calls must pass `_findCustodian`. For a queued request, inspect it with `getUpdateData`; after application, immediately run `getCustodians` and `getParameters` and confirm that the effective set and thresholds are exactly the intended values.
{% endhint %}

### Confirm a custodian update

Use this method only when `submitDataUpdate` returned a non-zero `transId`; a zero ID means that the update already applied.

```solidity
confirmDataUpdate(uint64 dataUpdateId)
```

```bash
tvm-cli -j call <MSIG_ADDR> confirmDataUpdate \
  '{"dataUpdateId":<DATA_UPDATE_ID>}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <NEXT_CUSTODIAN_KEYS_FILE>
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

When the last required confirmation applies the update, its ID disappears from the queue. An expired ID normally fails with error `102` after initial cleanup. If the cleanup budget was exhausted before reaching that ID, the call can instead return `exit_code: 0` while only deleting it. Verify the ID with `getUpdateDataIds` immediately before confirming.

### Upgrade the wallet code

Code upgrades use the current effective `reqConfirmsData` threshold. Extract the `code` cell from the new TVC:

```bash
tvm-cli decode stateinit --tvc <NEW_WALLET_TVC>
```

Example output (the cells are shortened here):

```text
Input arguments:
   input: <NEW_WALLET_TVC>
Decoded data:
{
  "split_depth": "None",
  "special": "None",
  "data": "te6ccg...",
  "code": "<BASE64_ENCODED_CODE_CELL>",
  "code_hash": "<NEW_CODE_HASH>",
  "data_hash": "<NEW_DATA_HASH>",
  "version": "sol <COMPILER_VERSION>"
}
```

Inspect the new code and submit it together with the migration-data cell expected by the new version:

```solidity
submitUpdateCode(TvmCell newcode, TvmCell cell) returns (uint64 codeUpdateId)
```

```bash
tvm-cli -j call <MSIG_ADDR> submitUpdateCode \
  '{"newcode":"<BASE64_ENCODED_CODE_CELL>","cell":"<BASE64_ENCODED_MIGRATION_CELL>"}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <FIRST_CUSTODIAN_KEYS_FILE>
```

Example output when the effective `reqConfirmsData` is `1`:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

In this immediate path, `_applyUpdateCode` transfers control to `onCodeUpgrade` and does not return to `submitUpdateCode`, so no `codeUpdateId: "0"` response is produced. This behavior was verified on Shellnet with `tvm-cli 3.0.4` and the artifacts used by this guide.

Example output for a queued code update:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": {
    "codeUpdateId": "<CODE_UPDATE_ID>"
  },
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

When the effective `reqConfirmsData` is greater than `1`, the response contains a non-zero queued ID as shown above.

Use an empty string for `cell` when the target version does not require migration data. The hash of that empty cell is `96a296d224f285c67bee93c30f8a309157f0daa35dc5b87e410b78630a09cfc7`.

Before confirming a queued upgrade, call `getUpdateCodes` as shown in [Inspect pending code updates](#inspect-pending-code-updates). After removing the CLI's `0x` prefix, its `codeHash` must exactly equal the `code_hash` printed by `tvm-cli decode stateinit`. Likewise, verify `cellHash` against the reviewed migration cell; for an empty `cell`, it must equal the empty-cell hash above.

Use `getUpdateCode` to inspect the full queued code and migration cells:

```bash
tvm-cli -j run <MSIG_ADDR> getUpdateCode \
  '{"codeUpdateId":<CODE_UPDATE_ID>}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output (the cells are shortened here):

```json
{
  "codeUpdate": {
    "id": "<CODE_UPDATE_ID>",
    "confirmationsMask": "1",
    "signsRequired": "2",
    "signsReceived": "1",
    "creator": {
      "owner_pubkey": "0x<FIRST_OWNER_PUBLIC_KEY>",
      "owner_address": null,
      "index": "0"
    },
    "newcode": "<BASE64_ENCODED_CODE_CELL>",
    "cell": "<BASE64_ENCODED_MIGRATION_CELL>"
  }
}
```

Confirm only a non-zero ID that is still returned by `getUpdateCodeIds`:

```solidity
confirmUpdateCode(uint64 codeUpdateId)
```

```bash
tvm-cli -j call <MSIG_ADDR> confirmUpdateCode \
  '{"codeUpdateId":<CODE_UPDATE_ID>}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <NEXT_CUSTODIAN_KEYS_FILE>
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

An expired code-update ID normally fails with error `102` after initial cleanup. If the cleanup budget was exhausted before reaching that ID, the call can instead return `exit_code: 0` while deleting it without applying the code. Check `getUpdateCodeIds` immediately before confirming.

{% hint style="danger" %}
Only confirm a code update after verifying its code hash and migration data. An incompatible upgrade can make the wallet unusable.
{% endhint %}

### Manage the wallet's gas balance

The wallet can refill its own gas. At the start of every operation, and once from the constructor, it compares its VMSHELL balance with `minBalance`; if the balance is lower, it converts its own SHELL into VMSHELL until the balance reaches `targetBalance`. The conversion is quiet and bounded by the SHELL the wallet actually holds, so it never fails when there is not enough SHELL. While `minBalance` is `0`, automatic top-up is disabled and the check does nothing.

Read the current configuration at any time:

```bash
tvm-cli -j run <MSIG_ADDR> getBalanceConfig '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output for a wallet that tops up at `1 VMSHELL` and refills to `5 VMSHELL`:

```json
{
  "config": {
    "minBalance": "1000000000",
    "targetBalance": "5000000000"
  }
}
```

Changing the configuration is multisig-protected and uses the current effective `reqConfirmsData` threshold, like a custodian or code update:

```solidity
submitConfigUpdate(uint128 minBalance, uint128 targetBalance) returns (uint64 configUpdateId)
```

Parameters:

* `minBalance` — VMSHELL balance below which the wallet tops itself up, in nanoVMSHELL; `0` disables automatic top-up
* `targetBalance` — VMSHELL balance the wallet converts SHELL up to, in nanoVMSHELL; it must not be lower than `minBalance`, otherwise the call fails with contract error `126`

```bash
tvm-cli -j call <MSIG_ADDR> submitConfigUpdate \
  '{"minBalance":1000000000,"targetBalance":5000000000}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <FIRST_CUSTODIAN_KEYS_FILE>
```

Example output when the effective `reqConfirmsData` is `1` and the configuration is applied immediately:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": {
    "configUpdateId": "0"
  },
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

When the effective `reqConfirmsData` is greater than `1`, `return_value.configUpdateId` contains a non-zero `<CONFIG_UPDATE_ID>` and the change is queued. Unlike the custodian-update queue, this queue is observable through events: `ConfigUpdateSubmitted` and `ConfigUpdateConfirmed` are emitted while the request is pending, and `ConfigUpdateApplied` when it takes effect.

Each additional custodian confirms the queued change with its own key file:

```solidity
confirmConfigUpdate(uint64 configUpdateId)
```

```bash
tvm-cli -j call <MSIG_ADDR> confirmConfigUpdate \
  '{"configUpdateId":<CONFIG_UPDATE_ID>}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <NEXT_CUSTODIAN_KEYS_FILE>
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

An expired ID behaves like the other queues: the call normally fails with error `102` after the initial cleanup, and only when the cleanup budget was exhausted on earlier entries can it return `exit_code: 0` while deleting the request without applying it. Verify the ID with `getConfigUpdateIds` immediately before confirming.

After the change is applied, confirm the effective values with `getBalanceConfig`.

## Configure expired-request cleanup

The wallet removes expired requests in bounded batches so that one transaction does not spend an unpredictable amount of gas on cleanup. `getMaxCleanupOperations` returns the current maximum number of requests removed during one cleanup pass. A newly deployed wallet uses `40`; because the ABI type is `uint256`, TVM CLI displays it as a 64-digit hexadecimal value.

```bash
tvm-cli -j run <MSIG_ADDR> getMaxCleanupOperations '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "maxCleanupOperations": "0x0000000000000000000000000000000000000000000000000000000000000028"
}
```

Any current custodian can change the cleanup budget with `setMaxCleanupOperations`. This method is not multisig-protected: one custodian changes the setting immediately, regardless of `reqConfirms` or `reqConfirmsData`, and no confirmation request is queued. The value must be at least `1`:

```bash
tvm-cli -j call <MSIG_ADDR> setMaxCleanupOperations \
  '{"value":100}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json \
  --sign <CUSTODIAN_KEYS_FILE>
```

Example output:

```json
{
  "message_hash": "<MESSAGE_HASH>",
  "block_hash": "<BLOCK_HASH>",
  "tx_hash": "<TRANSACTION_HASH>",
  "return_value": null,
  "aborted": false,
  "exit_code": 0,
  "account_id": "<MSIG_ACCOUNT_ID>",
  "dapp_id": "<MSIG_DAPP_ID>"
}
```

{% hint style="warning" %}
A higher value removes more expired requests per cleanup pass but can increase the gas used by the operation that triggers cleanup. Change the default only when the wallet operator understands this trade-off.
{% endhint %}

## Check a confirmation mask

Queued requests contain `confirmationsMask`. Each bit corresponds to the explicit custodian `index` returned by `getCustodians`, not to the custodian's position in that array. Use `isConfirmed` to check whether a specific custodian has confirmed a request:

```bash
tvm-cli -j run <MSIG_ADDR> isConfirmed \
  '{"mask":1,"index":0}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "confirmed": true
}
```

For example, mask `1` means that custodian index `0` has confirmed the request. Mask `3` means that indexes `0` and `1` have confirmed it.

## Inspect pending requests

The list methods below return only non-expired requests. A getter that fetches one request by ID can still return an expired entry that has not yet been removed. Before confirming, verify that the ID appears in the corresponding filtered list: `getTransactionIds`, `getUpdateDataIds`, `getUpdateCodeIds`, or `getConfigUpdateIds`. Comparing `signsReceived` with `signsRequired` does not reveal expiration. A confirmation call first performs bounded cleanup: it normally removes the expired target and then fails with error `102`; only when the cleanup budget is consumed by earlier entries can the target survive to the explicit expiry check, which deletes it with `exit_code: 0` without execution.

### Inspect pending transactions

Use `getTransactions` to retrieve all non-expired queued transfers:

```bash
tvm-cli -j run <MSIG_ADDR> getTransactions '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output (the payload is shortened here):

```json
{
  "transactions": [
    {
      "id": "<TRANSACTION_ID>",
      "confirmationsMask": "1",
      "signsRequired": "2",
      "signsReceived": "1",
      "creator": {
        "owner_pubkey": "0x<FIRST_OWNER_PUBLIC_KEY>",
        "owner_address": null,
        "index": "0"
      },
      "dest": "0:<DEST_ACCOUNT_ID>",
      "value": "1000000000",
      "cc": {
        "2": "5000000000"
      },
      "sendFlags": "1",
      "payload": "<BASE64_ENCODED_PAYLOAD_CELL>",
      "bounce": false,
      "dapp_id": "0x<DEST_DAPP_ID>"
    }
  ]
}
```

Use `getTransaction` when you already know the transaction ID:

```bash
tvm-cli -j run <MSIG_ADDR> getTransaction \
  '{"transactionId":<TRANSACTION_ID>}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output (the payload is shortened here):

```json
{
  "trans": {
    "id": "<TRANSACTION_ID>",
    "confirmationsMask": "1",
    "signsRequired": "2",
    "signsReceived": "1",
    "creator": {
      "owner_pubkey": "0x<FIRST_OWNER_PUBLIC_KEY>",
      "owner_address": null,
      "index": "0"
    },
    "dest": "0:<DEST_ACCOUNT_ID>",
    "value": "1000000000",
    "cc": {
      "2": "5000000000"
    },
    "sendFlags": "1",
    "payload": "<BASE64_ENCODED_PAYLOAD_CELL>",
    "bounce": false,
    "dapp_id": "0x<DEST_DAPP_ID>"
  }
}
```

If the transaction does not exist, the getter fails with contract error `102`.

### Inspect pending custodian updates

Use `getUpdateDatas` to retrieve all non-expired custodian-update requests:

```bash
tvm-cli -j run <MSIG_ADDR> getUpdateDatas '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "data": [
    {
      "id": "<DATA_UPDATE_ID>",
      "confirmationsMask": "1",
      "signsRequired": "2",
      "signsReceived": "1",
      "creator": {
        "owner_pubkey": "0x<FIRST_OWNER_PUBLIC_KEY>",
        "owner_address": null,
        "index": "0"
      },
      "owners_pubkey": [
        "0x<FIRST_OWNER_PUBLIC_KEY>",
        "0x<SECOND_OWNER_PUBLIC_KEY>"
      ],
      "owners_address": [],
      "reqConfirms": "2",
      "reqConfirmsData": "2"
    }
  ]
}
```

Use `getUpdateData` when you already know the data-update ID:

```bash
tvm-cli -j run <MSIG_ADDR> getUpdateData \
  '{"updateDataId":<DATA_UPDATE_ID>}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "data": {
    "id": "<DATA_UPDATE_ID>",
    "confirmationsMask": "1",
    "signsRequired": "2",
    "signsReceived": "1",
    "creator": {
      "owner_pubkey": "0x<FIRST_OWNER_PUBLIC_KEY>",
      "owner_address": null,
      "index": "0"
    },
    "owners_pubkey": [
      "0x<FIRST_OWNER_PUBLIC_KEY>",
      "0x<SECOND_OWNER_PUBLIC_KEY>"
    ],
    "owners_address": [],
    "reqConfirms": "2",
    "reqConfirmsData": "2"
  }
}
```

If the custodian update does not exist, the getter fails with contract error `102`.

### Inspect pending code updates

`getUpdateCodes` returns every non-expired code-update request without copying the full code cells. Use the returned hashes to identify and verify each pending upgrade efficiently:

```bash
tvm-cli -j run <MSIG_ADDR> getUpdateCodes '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "codeUpdates": [
    {
      "id": "<CODE_UPDATE_ID>",
      "confirmationsMask": "1",
      "signsRequired": "2",
      "signsReceived": "1",
      "creatorIndex": "0",
      "codeHash": "0x<NEW_CODE_HASH>",
      "cellHash": "0x<MIGRATION_CELL_HASH>"
    }
  ]
}
```

Use `getUpdateCodeIds` when only the pending IDs are needed:

```bash
tvm-cli -j run <MSIG_ADDR> getUpdateCodeIds '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "ids": [
    "<CODE_UPDATE_ID>"
  ]
}
```

Use `getUpdateCode`, described in [Upgrade the wallet code](#upgrade-the-wallet-code), to inspect one request together with its full code and migration cells.

### Inspect pending balance-configuration updates

Use `getConfigUpdates` to retrieve all non-expired balance-configuration updates:

```bash
tvm-cli -j run <MSIG_ADDR> getConfigUpdates '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "configUpdates": [
    {
      "id": "<CONFIG_UPDATE_ID>",
      "confirmationsMask": "1",
      "signsRequired": "2",
      "signsReceived": "1",
      "creator": {
        "owner_pubkey": "0x<FIRST_OWNER_PUBLIC_KEY>",
        "owner_address": null,
        "index": "0"
      },
      "config": {
        "minBalance": "1000000000",
        "targetBalance": "5000000000"
      }
    }
  ]
}
```

Use `getConfigUpdate` when you already know the config-update ID:

```bash
tvm-cli -j run <MSIG_ADDR> getConfigUpdate \
  '{"configUpdateId":<CONFIG_UPDATE_ID>}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "configUpdate": {
    "id": "<CONFIG_UPDATE_ID>",
    "confirmationsMask": "1",
    "signsRequired": "2",
    "signsReceived": "1",
    "creator": {
      "owner_pubkey": "0x<FIRST_OWNER_PUBLIC_KEY>",
      "owner_address": null,
      "index": "0"
    },
    "config": {
      "minBalance": "1000000000",
      "targetBalance": "5000000000"
    }
  }
}
```

If the config update does not exist, the getter fails with contract error `102`.

Use `getConfigUpdateIds` when only the pending IDs are needed:

```bash
tvm-cli -j run <MSIG_ADDR> getConfigUpdateIds '{}' \
  --abi UpdateCustodianMultisigWallet_v2.abi.json
```

Example output:

```json
{
  "ids": [
    "<CONFIG_UPDATE_ID>"
  ]
}
```

## Read-only method reference

The v2 wallet provides the following read-only methods:

* `getVersion` — contract name and version
* `getParameters` — queue, custodian, expiration, and confirmation limits
* `getMaxCleanupOperations` — expired-request cleanup budget
* `getBalanceConfig` — gas auto-top-up thresholds
* `getCustodians` — current custodian set
* `isConfirmed` — confirmation-mask check for a custodian index
* `getTransaction`, `getTransactions`, `getTransactionIds` — transfer queue
* `getUpdateData`, `getUpdateDatas`, `getUpdateDataIds` — custodian-update queue
* `getUpdateCode`, `getUpdateCodes`, `getUpdateCodeIds` — code-update queue
* `getConfigUpdate`, `getConfigUpdates`, `getConfigUpdateIds` — balance-configuration-update queue

## Lifecycle events

`UpdateCustodianMultisigWallet_v2` emits lifecycle events for monitoring:

* `WalletSetup` and `CustodiansUpdated`
* `TransactionSubmitted`, `TransactionConfirmed`, and `TransactionSent`
* `CodeUpdateSubmitted`, `CodeUpdateConfirmed`, and `CodeUpdateApplied`
* `ConfigUpdateSubmitted`, `ConfigUpdateConfirmed`, and `ConfigUpdateApplied`
* `RequestsDropped`
* `FundsReceived`, `UnknownCall`, and `ExecutionFailure`

Each event is emitted to an external destination whose numeric ID is the subscription key:

| Event | Destination ID |
| --- | ---: |
| `TransactionSent` | `1100` |
| `TransactionSubmitted` | `1101` |
| `TransactionConfirmed` | `1102` |
| `FundsReceived` | `1103` |
| `WalletSetup` | `1104` |
| `ExecutionFailure` | `1105` |
| `CustodiansUpdated` | `1108` |
| `CodeUpdateSubmitted` | `1109` |
| `CodeUpdateConfirmed` | `1110` |
| `CodeUpdateApplied` | `1111` |
| `UnknownCall` | `1112` |
| `RequestsDropped` | `1113` |
| `ConfigUpdateSubmitted` | `1114` |
| `ConfigUpdateConfirmed` | `1115` |
| `ConfigUpdateApplied` | `1116` |

IDs `1106` and `1107` are unused. `WalletSetup` is emitted only during deployment, while `CustodiansUpdated` is emitted when the custodian set actually changes.

The custodian-update queue is the only queue without submission and confirmation events. Until `CustodiansUpdated` is emitted on application, monitor it through `getUpdateDataIds`, `getUpdateDatas`, or `getUpdateData`.
