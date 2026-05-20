---
description: >-
  This guide explains how to top up an address with test SHELL tokens in the
  `Shellnet` network.
---

# Get test SHELL from Giver in Shellnet

{% hint style="danger" %}
Use this guide only for test `SHELL` tokens in Shellnet.
{% endhint %}

### Requirements

Before you start, make sure you have:

* [`tvm-cli` installed](https://github.com/tvmlabs/tvm-sdk/releases)
* [`GiverV3` ABI file](https://github.com/ackinacki/ackinacki/blob/main/contracts/giver/GiverV3.abi.json)
* Recipient wallet or contract address

{% hint style="info" %}
Right now, in enabled `DEV mode`, you will see the address of your multifactor contract
{% endhint %}

`GiverV3` contract artifacts are available in the repository:

{% embed url="https://github.com/ackinacki/ackinacki/tree/main/contracts/giver" %}

### Get SHELL

Replace `0:348c....66bf` with the recipient address.

The command below sends `1000 SHELL`.

```bash
tvm-cli -j -u shellnet.ackinacki.org callx \
  --abi acki-nacki/contracts/giver/GiverV3.abi.json \
  --addr 0:1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrency \
  '{"dest":"0:348c....66bf","value":1000000000,"ecc":{"2":1000000000000}}'
```

{% hint style="warning" %}
Make sure the recipient address is correct before running the command.
{% endhint %}

### Change the SHELL amount

In the `ecc` parameter, `SHELL` is identified by the `"2"` key.

Use one of the examples below, or set another amount using the same format:

| Amount        | `ecc` value                  |
| ------------- | ---------------------------- |
| `100 SHELL`   | `"ecc":{"2":100000000000}`   |
| `1000 SHELL`  | `"ecc":{"2":1000000000000}`  |
| `50000 SHELL` | `"ecc":{"2":50000000000000}` |

{% hint style="warning" %}
Keep the `value` parameter set to `1000000000`.
{% endhint %}
