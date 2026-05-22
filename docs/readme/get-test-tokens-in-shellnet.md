---
description: >-
  This guide explains how to top up an address with test tokens in the
  `Shellnet` network
---

# Get Test Tokens in Shellnet

{% hint style="danger" %}
**for the `Shellnet` network only**
{% endhint %}

You can receive test tokens in two ways:\
\* by using the giver \
\* by requesting them from us through our [Telegram channel](https://t.me/tvmlabs)

### Requirements

Before you start, make sure you have:

* [`tvm-cli` installed](https://github.com/tvmlabs/tvm-sdk/releases)
* [`GiverV3` ABI file](https://github.com/ackinacki/ackinacki/blob/main/contracts/giver/GiverV3.abi.json)
* Recipient wallet or contract address

{% hint style="info" %}
Right now, in enabled `DEV mode`, you will see the address of your Multifactor contract
{% endhint %}

`GiverV3` contract artifacts are available in the repository Acki Nacki

{% embed url="https://github.com/ackinacki/ackinacki" %}

### Test Tokens

Token amounts in `ecc` are specified in the smallest units.

<table><thead><tr><th width="95.13330078125">Token</th><th width="123.699951171875">`ecc` key</th><th width="110.29998779296875">Decimals</th><th>Example</th></tr></thead><tbody><tr><td><code>NACKL</code></td><td>1</td><td><code>9</code></td><td><code>100 NACKL</code> = <code>100,000,000,000</code></td></tr><tr><td><code>SHELL</code></td><td>2</td><td><code>9</code></td><td><code>100 SHELL</code> = <code>100,000,000,000</code></td></tr><tr><td><code>USDC</code></td><td>3</td><td><code>6</code></td><td><code>100 USDC</code> = <code>100,000,000</code></td></tr></tbody></table>

{% hint style="warning" %}
* Make sure the recipient address is correct before running the command
* Keep the `value` parameter set to `1000000000`
{% endhint %}

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

### Get NACKL

Replace `0:348c....66bf` with the recipient address.

The command below sends `100 NACKL`.

```bash
tvm-cli -j -u shellnet.ackinacki.org callx \
  --abi acki-nacki/contracts/giver/GiverV3.abi.json \
  --addr 0:1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrency \
  '{"dest":"0:348c....66bf","value":1000000000,"ecc":{"1":100000000000}}'
```

### Get USDC

Replace `0:348c....66bf` with the recipient address.

The command below sends `5000 USDC`.

```bash
tvm-cli -j -u shellnet.ackinacki.org callx \
  --abi acki-nacki/contracts/giver/GiverV3.abi.json \
  --addr 0:1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrency \
  '{"dest":"0:348c....66bf","value":1000000000,"ecc":{"3":5000000000}}'
```

### Get multiple tokens

You can request several test tokens in one command by adding multiple keys to `ecc`.

The command below sends `1000 NACKL`, `50000 SHELL`, and `5000 USDC`.

```bash
tvm-cli -j -u shellnet.ackinacki.org callx \
  --abi acki-nacki/contracts/giver/GiverV3.abi.json \
  --addr 0:1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrency \
  '{"dest":"0:348c....66bf","value":1000000000,"ecc":{"1":1000000000000,"2":50000000000000,"3":5000000000}}'
```

### Get VMSHELL to a Precomputed Address

Use this option to send test SHELL tokens to an address where the contract has not yet been deployed.

{% hint style="info" %}
The SHELL tokens will be converted into VMSHELL at the destination address.
{% endhint %}

These tokens will be used to pay for the deployment of that contract.

Replace `0:348c....66bf` with the recipient address.

The command below sends `1000 SHELL`, which will be credited as `1000 VMSHELL`.

```bash
tvm-cli -j -u shellnet.ackinacki.org callx \
  --abi acki-nacki/contracts/giver/GiverV3.abi.json \
  --addr 0:1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrencyWithFlag \
  '{"dest":"0:348c....66bf","value":1000000000,"ecc":{"2":1000000000000},"flag":16}'
```

