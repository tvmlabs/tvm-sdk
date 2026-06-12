---
description: Configure tvm-cli to use the Acki Nacki Shellnet endpoint.
status: stable
product: sdk
audience: app-developer
task: configure-tvm-cli
last_verified: 2026-06-11
---

# Configure TVM CLI

## Goal

Configure `tvm-cli` to send requests to the Acki Nacki Shellnet endpoint.

## Prerequisites

* `tvm-cli` is installed.
* You are using Shellnet for development or testing.

## Command

```bash
tvm-cli config -g --url shellnet.ackinacki.org
```

## Expected Result

Subsequent `tvm-cli` commands use `shellnet.ackinacki.org` as the default endpoint.

## Notes

Use Shellnet for examples, tutorials, and test token flows. Do not use Shellnet addresses or balances as Mainnet data.

## Related Docs

* [Dapp ID Full Guide](../README.md)
* [How to deploy a Multisig Wallet](../how-to-deploy-a-multisig-wallet.md)
* [GraphQL Schema for AI Agents](../graphql/graphql-schema-for-ai-agents.md)
