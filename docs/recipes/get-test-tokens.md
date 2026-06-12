---
description: Get Shellnet test tokens for development and contract deployment.
status: stable
product: dapp-id
audience: app-developer
task: get-test-tokens
last_verified: 2026-06-11
---

# Get Test Tokens

## Goal

Top up a Shellnet address with test tokens so it can pay deployment and transaction fees.

## Prerequisites

* You have a Shellnet address.
* You know which token you need: `SHELL`, `VMSHELL`, `NACKL`, or `USDC`.

## Token Units

| Token | ECC key | Decimals |
| --- | --- | --- |
| `NACKL` | `1` | `9` |
| `SHELL` | `2` | `9` |
| `USDC` | `3` | `6` |

## Expected Result

The target Shellnet address receives test tokens. Use `tvm-cli account <address>` to verify balances.

```bash
tvm-cli account <YourAddress>
```

## Notes

On Shellnet, test token availability depends on the current faucet or support flow. If automated faucet instructions are not available, contact the Acki Nacki team through the support channel referenced in the docs.

## Related Docs

* [Get Test Tokens in Shellnet](../readme/get-test-tokens-in-shellnet.md)
* [How to deploy a Multisig Wallet](../how-to-deploy-a-multisig-wallet.md)
