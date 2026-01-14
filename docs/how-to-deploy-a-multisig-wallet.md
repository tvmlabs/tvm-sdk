---
description: Create a Multisig wallet  with TVM CLI
---

# How to deploy a Multisig Wallet

## Prerequisites <a href="#create-a-wallet" id="create-a-wallet"></a>

* [tvm-cli](https://github.com/tvmlabs/tvm-sdk/releases)

## **Prepare wallet binary and ABI** <a href="#create-a-wallet" id="create-a-wallet"></a>

Create a folder:

```
cd ~
mkdir wallet
cd wallet
```

Download the  [UpdateCustodianMultisigWallet.abi.json](https://raw.githubusercontent.com/ackinacki/ackinacki/blob/main/contracts/0.79.3_compiled/updatecustodianmultisigwallet/UpdateCustodianMultisigWallet.abi.json) and [UpdateCustodianMultisigWallet.tvc](https://raw.githubusercontent.com/ackinacki/ackinacki/blob/main/contracts/0.79.3_compiled/updatecustodianmultisigwallet/UpdateCustodianMultisigWallet.tvc) files for your wallet from the `updatecustodianmultisigwallet` [repository](https://github.com/ackinacki/ackinacki/tree/main/contracts/0.79.3_compiled/updatecustodianmultisigwallet) and place them in this folder.&#x20;

{% hint style="info" %}
The contract code can be downloaded from [here](https://github.com/ackinacki/ackinacki/blob/main/contracts/updatecustodianmultisigwallet/UpdateCustodianMultisigWallet.sol)
{% endhint %}

{% hint style="warning" %}
At the moment this wallet is undergoing formal verification. Wallet code may be updated during this process. However, the API of the wallet will remain the same.
{% endhint %}

## Configure CLI tool

In this guide, we will use the test network at [`shellnet.ackinacki.org`](https://shellnet.ackinacki.org).\
We need to specify the blockchain endpoint for deployment:

```
tvm-cli config -g --url shellnet.ackinacki.org
```

## Generate seed phrase, keys and address

In Acki Nacki blockchain, the Multisig wallet address depends on its binary code and initial data, which includes the owner's public key.

You can generate everything with one command:

```

tvm-cli genaddr UpdateCustodianMultisigWallet.tvc --save --genkey UpdateCustodianMultisigWallet.keys.json
```

{% hint style="danger" %}
**Write down your `seed phrase` and store it in a secure location. Never share it with anyone. Avoid storing it in plain text, screenshots, or any other insecure method. If you lose it, you lose access to your assets. Anyone who obtains it will have full access to your assets.**&#x20;

**Additionally, ensure the file containing the `key pair` is saved in a safe place.**
{% endhint %}

{% hint style="info" %}
After this step, the `.tvc` file will be overwritten with the specified keys.
{% endhint %}

The `Raw address` is the future Multisig wallet address. Keys are saved to `updateCustodianMultisigWallet.keys.`&#x20;

Be sure to copy your seed phrase if you need it.

<figure><img src=".gitbook/assets/multisig_create_address (1).jpg" alt=""><figcaption></figcaption></figure>

## **Request Test tokens** <a href="#request-test-tokens-for-future-use" id="request-test-tokens-for-future-use"></a>

[VMSHELL](https://docs.ackinacki.com/glossary#vmshell) tokens are used to pay network fees and are derived from [SHELL](https://docs.ackinacki.com/glossary#shell) tokens.\
On the Mainnet, SHELL tokens are purchased via a special pool and then converted into VMSHELL tokens.

On the test network, you can request test tokens to be sent to your address. Please contact us on [Telegram](https://t.me/+1tWNH2okaPthMWU0) to receive them.

{% hint style="info" %}
If you plan to test your smart contract systems, you can use the provided Multisig wallet to top up contract balances in order to cover gas fees.
{% endhint %}

## Deploy your Multisig wallet

Once you receive the test tokens, check the state of the pre-deployed contract. It should be **`Uninit`**:

```
tvm-cli account <YourAddress>
```

{% hint style="success" %}
The received **VMSHELL** tokens will be displayed in the `balance` field. \
VMSHELL tokens are transferred and stored in (_in_ [_nanotokens_](https://github.com/gosh-sh/TVM-Solidity-Compiler/blob/master/API.md#tvm-units)) units.\
\
The received **SHELL** tokens will be displayed in the `ecc` field under index **2**
{% endhint %}

<figure><img src=".gitbook/assets/Uninit.jpg" alt=""><figcaption></figcaption></figure>

Now you are ready to deploy your Multisig wallet using the following command:

```

tvm-cli deploy --abi UpdateCustodianMultisigWallet.abi.json --sign UpdateCustodianMultisigWallet.keys.json UpdateCustodianMultisigWallet.tvc '{"owners_pubkey":[<PubKeyList>], "owners_address": [], "reqConfirms":<ConfirmsNum>, "reqConfirmsData": <NumConfirms>, "value":<NumTokens>}'
```

The arguments for the constructor must be enclosed in curly brackets: `{<constructor arguments>}`

* **`owners_pubkey`** — an array of custodians’ public keys. Each key must include the **`0x` prefix**
* **`owners_address`** — an array of custodian contract addresses.
* **`reqConfirms`** — the number of signatures required to approve a transaction.
* **`reqConfirmsData`** — the number of confirmations required to approve a change of custodians.
* **`value`** — the amount (_in_ [_nanotokens_](https://github.com/gosh-sh/TVM-Solidity-Compiler/blob/master/API.md#tvm-units)) of **SHELL** tokens you want to exchange for **VMSHELL**. \
  If the exchange is not required, set the parameter `value` to **0**.

In our example, the command will be as follows:

```

tvm-cli deploy --abi UpdateCustodianMultisigWallet.abi.json --sign UpdateCustodianMultisigWallet.keys.json UpdateCustodianMultisigWallet.tvc '{"owners_pubkey":["0x92658a2dee35923cc628b7f5f09e014eeeb7f492dd4dfd2f65cd304a73d2d2f4"], "owners_address": [], "reqConfirms":1, "reqConfirmsData": 1, "value":10000000000}'
```

<figure><img src=".gitbook/assets/deploy.jpg" alt=""><figcaption></figcaption></figure>

Check the contract state again. This time, it should be `Active`

{% hint style="info" %}
The contract deployment fee was deducted from the VMSHELL balance.
{% endhint %}

<figure><img src=".gitbook/assets/active.jpg" alt=""><figcaption></figcaption></figure>

{% hint style="success" %}
During contract deployment, **10** **SHELL** tokens were converted into **10** **VMSHELL** (values are specified in nanotokens).
{% endhint %}

## Multisig Wallet API

In the examples below:

* `<MSIG_ADDR>` — Multisig Wallet address (e.g. `0:7a55...dd45`)
* ABI file: [`UpdateCustodianMultisigWallet.abi.json`](https://raw.githubusercontent.com/ackinacki/ackinacki/blob/main/contracts/0.79.3_compiled/updatecustodianmultisigwallet/UpdateCustodianMultisigWallet.abi.json)
* Signer keys (one of the custodians): `UpdateCustodianMultisigWallet.keys.json` , generated in the [previous step](how-to-deploy-a-multisig-wallet.md#generate-seed-phrase-keys-and-address)

{% hint style="info" %}
The transaction **expiration time** is **1 hour**.
{% endhint %}

### How to Send Tokens From Multisig Wallet

**If the required number of confirmations for transactions is 1,** \
tokens can be sent using the function `sendTransaction`:

```solidity

sendTransaction(
        address dest,
        uint128 value,
        mapping(uint32 => varuint32) cc,
        bool bounce,
        uint8 flags,
        TvmCell payload)
```

**Parameters**

* `dest`  - the transfer target address;
* `value`  - the amount of funds (VMSHELL) to transfer (should be `0`);
* `cc`  - a mapping of ECC token types to the token amounts to be transferred;
* `bounce`  - [bounce flag](https://github.com/gosh-sh/TON-Solidity-Compiler/blob/master/API.md#addresstransfer): (should be `false`);
* `flags`-  [send message flags](https://github.com/gosh-sh/TON-Solidity-Compiler/blob/master/API.md#addresstransfer) (should be `1`);
* `payload` - [tree of cells used as the body](https://github.com/gosh-sh/TON-Solidity-Compiler/blob/master/API.md#addresstransfer) of the outbound internal message (should be an empty string).&#x20;

{% hint style="warning" %}
In this case, the transaction is executed immediately, without creating a request or requiring additional confirmations.
{% endhint %}

Example command:

```bash
tvm-cli call <MSIG_ADDR> sendTransaction '{
  "dest":"0:2672bb98816f2f9088d027f99681b65e05843b19367fe690cb4b5130d04eccf1",
  "value":0,
  "cc":{"2":5000000000},
  "bounce":false,
  "flags":1,
  "payload":""
}'
```

**If confirmation from multiple custodians is required**, \
use the function `submitTransaction`:

```solidity

submitTransaction(
        address dest,
        uint128 value,
        mapping(uint32 => varuint32) cc,
        bool bounce,
        uint8 flag,
        TvmCell payload)
```

**Parameters**

* `dest` — the transfer target address;
* `value` — the amount of funds (VMSHELL) to transfer (should be `0`);
* `cc` — a mapping of ECC token types to the token amounts to be transferred;
* `bounce` — [bounce flag](https://github.com/gosh-sh/TON-Solidity-Compiler/blob/master/API.md#addresstransfer): (should be `false`);
* `flags` — [send message flags](https://github.com/gosh-sh/TON-Solidity-Compiler/blob/master/API.md#addresstransfer) (usually `1`);
* `payload` — [tree of cells used as the body](https://github.com/gosh-sh/TON-Solidity-Compiler/blob/master/API.md#addresstransfer) of the outbound internal message (usually an empty string).

**Return value**

* `transactionId` — identifier of the created multisig transaction.\
  This id is later used in [`confirmTransaction`](how-to-deploy-a-multisig-wallet.md#how-to-confirm-an-already-created-transaction).

{% hint style="warning" %}
The transaction will be executed only after the required number of confirmations is collected.
{% endhint %}

Example command:

```bash
tvm-cli call <MSIG_ADDR> submitTransaction '{
  "dest":"0:2672bb98816f2f9088d027f99681b65e05843b19367fe690cb4b5130d04eccf1",
  "value":0,
  "cc":{"2":5000000000},   # 5 SHELL
  "bounce":false,
  "flags":1,
  "payload":""
}'
```

{% hint style="info" %}
If the required number of confirmations for transactions is `1`, `submitTransaction` behaves like `sendTransaction` and executes immediately.
{% endhint %}

### How to Confirm a Transaction

To do this, use the function `confirmTransaction`

```solidity
confirmTransaction(uint64 transactionId)
```

**Parameters**

* `transactionId` — identifier of the transaction to confirm. \
  You receive it as a result of calling [`submitTransaction`](how-to-deploy-a-multisig-wallet.md#how-to-send-tokens-from-multisig-wallet)

{% hint style="info" %}
If the transaction has already expired, it will be deleted.
{% endhint %}

Example command:

```bash
tvm-cli call <MSIG_ADDR> confirmTransaction '{"transactionId":123456789}'
```

### How to create a request to update Multisig data

To change the list of custodians and the required number of confirmations, use the function `submitDataUpdate`

```solidity
submitDataUpdate(
        uint256[] owners_pubkey,
        address[] owners_address, 
        uint8 reqConfirms,
        uint8 reqConfirmsData)
```

**Parameters**

* `owners_pubkey` — array of custodian public keys;
* `owners_address` — array of custodian addresses;
* `reqConfirms` — required confirmations for regular transactions;
* `reqConfirmsData` — required confirmations for data update operations;

**Return value**

* `dataUpdateId`— identifier of the created multisig transaction.\
  This id is later used in [confirmDataUpdate](how-to-deploy-a-multisig-wallet.md#how-to-create-a-request-to-update-multisig-data)

Example command:

```bash
tvm-cli call <MSIG_ADDR> submitDataUpdate '{
  "owners_pubkey":[
    "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
  ],
  "owners_address":[
    "0:2672bb98816f2f9088d027f99681b65e05843b19367fe690cb4b5130d04eccf1"
  ],
  "reqConfirms":2,
  "reqConfirmsData":2
}'
```

### How to confirm a request to update the Multisig data

To do this, use the function confirmDataUpdate

```solidity
confirmDataUpdate(uint64 dataUpdateId)
```

**Parameters**

* `dataUpdateId` — identifier of the data update request.\
  You receive it as a result of calling [`submitDataUpdate`](how-to-deploy-a-multisig-wallet.md#how-to-create-a-request-to-update-multisig-data)

{% hint style="info" %}
If the request is expired, it will be removed
{% endhint %}

Example command:

```bash
tvm-cli call <MSIG_ADDR> confirmDataUpdate '{"dataUpdateId":987654321}'

```



{% hint style="warning" %}
In order for the recipient's contract to be deployed, its constructor must include a [function for exchange SHELL tokens for VMSHELL](https://github.com/tvmlabs/sdk-examples/blob/main/contracts/helloWorld/helloWorld.sol#L22)
{% endhint %}
