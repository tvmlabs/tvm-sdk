---
description: Deploy a contract to Shellnet with tvm-cli.
status: stable
product: sdk
audience: app-developer
task: deploy-contract
last_verified: 2026-06-11
---

# Deploy a Contract

## Goal

Deploy a smart contract and create its Dapp ID.

## Prerequisites

* `tvm-cli` is configured for Shellnet.
* Contract `.tvc` and `.abi.json` files are available.
* A key pair file is available.
* The future contract address has enough tokens for deployment.

## Commands

Generate or set keys and calculate the address:

```bash
tvm-cli genaddr helloWorld.tvc --save --setkey helloWorld.keys.json
```

Check the future account state:

```bash
tvm-cli account <YourAddress>
```

Deploy the contract:

```bash
tvm-cli deploy --abi helloWorld.abi.json --sign helloWorld.keys.json helloWorld.tvc '{"value":10000000000}'
```

Check the deployed account:

```bash
tvm-cli account <YourAddress>
```

## Expected Result

The contract account becomes `Active`. The deployed contract address becomes the Dapp ID for that contract system.

## Common Errors

If deployment fails with `The account doesn't have a state`, verify that the address was calculated from the same `.tvc` and keys that you use for deployment, and that the account has enough funds.

## Related Docs

* [Dapp ID Full Guide](../README.md)
* [Configure TVM CLI](configure-tvm-cli.md)
* [Get Test Tokens](get-test-tokens.md)
