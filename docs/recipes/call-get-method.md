---
description: Call a contract get method with tvm-cli.
status: stable
product: sdk
audience: app-developer
task: call-get-method
last_verified: 2026-06-11
---

# Call a Get Method

## Goal

Read contract state by calling a get method without sending an on-chain transaction.

## Prerequisites

* `tvm-cli` is configured for the target network.
* You know the contract address.
* You have the contract ABI file.
* You know the get method name and arguments.

## Command

```bash
tvm-cli run <YourAddress> timestamp {} --abi helloWorld.abi.json
```

## Expected Result

`tvm-cli` prints the get method result. For the `timestamp` example, the result is the value stored in the contract state.

## Notes

Use `run` for get methods. Use `call` only when you need to send a signed on-chain message that changes state.

## Related Docs

* [Dapp ID Full Guide](../README.md#run-a-getter)
* [Run ABI Get Method](../js-ts-guides/work-with-contracts/run-abi-get-method.md)
