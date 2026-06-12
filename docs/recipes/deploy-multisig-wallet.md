---
description: Deploy an Acki Nacki multisig wallet with tvm-cli.
status: stable
product: sdk
audience: app-developer
task: deploy-multisig-wallet
last_verified: 2026-06-11
---

# Deploy a Multisig Wallet

## Goal

Create and deploy a multisig wallet that can hold funds and send transactions.

## Prerequisites

* `tvm-cli` is installed.
* `UpdateCustodianMultisigWallet.abi.json` is available.
* `UpdateCustodianMultisigWallet.tvc` is available.
* The future wallet address has been topped up with test tokens on Shellnet.

## Commands

Create a work directory:

```bash
cd ~
mkdir wallet
cd wallet
```

Configure Shellnet:

```bash
tvm-cli config -g --url shellnet.ackinacki.org
```

Generate keys and the future wallet address:

```bash
tvm-cli genaddr UpdateCustodianMultisigWallet.tvc --save --genkey UpdateCustodianMultisigWallet.keys.json
```

Check that the funded future address is `Uninit`:

```bash
tvm-cli account <YourAddress>
```

Deploy the multisig wallet:

```bash
tvm-cli deploy --abi UpdateCustodianMultisigWallet.abi.json --sign UpdateCustodianMultisigWallet.keys.json UpdateCustodianMultisigWallet.tvc '{"owners_pubkey":[<PubKeyList>], "owners_address": [], "reqConfirms":<ConfirmsNum>, "reqConfirmsData":<NumConfirms>, "value":<NumTokens>}'
```

## Expected Result

The account state changes from `Uninit` to `Active`.

```bash
tvm-cli account <YourAddress>
```

## Common Errors

If deployment fails because the account has no state or balance, verify that the future wallet address was funded before deployment.

## Related Docs

* [How to deploy a Multisig Wallet](../how-to-deploy-a-multisig-wallet.md)
* [Get Test Tokens](get-test-tokens.md)
