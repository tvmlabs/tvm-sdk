# Dapp ID Full Guide: creation, fees, centralized replenishment

## **What will you learn from this guide?**

* How to create your [Dapp ID](https://docs.ackinacki.com/glossary#dapp-id)
* How to interact with contracts inside a Dapp
* How to interact with contracts of other Dapps
* How to use the centralized replenishment mechanism for the Dapp ID contracts

## **Prerequisites** <a href="#prerequisites" id="prerequisites"></a>

* [TVM-Solidity-Compiler](https://github.com/gosh-sh/TVM-Solidity-Compiler)
* [TVM-CLI and Multisig Wallet](how-to-deploy-a-multisig-wallet.md)

{% hint style="info" %}
This walkthrough was validated end to end on Shellnet on 2026-08-31 with `tvm-cli` 3.0.4,
`sold` 0.81.0, `sdk-examples` commit `43505225cd138a9a6243cfd62c18202bb1739d34`, and
`ackinacki` commit `0f2ada4777a1f4b1b6e9374fca42b8168f216b22`.
{% endhint %}

## Configure CLI tool

In this guide, we will use the test network at [`shellnet.ackinacki.org`](https://shellnet.ackinacki.org).\
We need to specify the blockchain endpoint for deployment:

```bash
tvm-cli config -g --url shellnet.ackinacki.org
```

The command prints `Succeeded.` followed by the complete saved configuration. Only the relevant
part is shown here:

```text
Succeeded.
{
  "url": "shellnet.ackinacki.org",
  ...
}
```

Verify the saved endpoint:

```bash
tvm-cli config -g --list
```

The JSON output must contain:

```json
{
  "url": "shellnet.ackinacki.org"
}
```

The walkthrough below uses Shellnet. To deploy on Mainnet, select `mainnet.ackinacki.org` before funding the precomputed address and use that endpoint for deployment and all subsequent calls.

## Create your first Dapp ID

There are two ways to deploy a contract, and the message type determines its Dapp ID:

| Deployment message | Dapp ID result | Deployment funding |
| --- | --- | --- |
| External message | The contract starts a new Dapp ID and becomes its first, self-rooted contract. Its `dapp_id` is equal to its `account_id`. | Send SHELL to the self-rooted precomputed address with exchange flag `16` before deployment. |
| Internal message | The new contract joins the Dapp ID carried by the internal deployment message. Normally this is the sender's Dapp ID. | Include enough VMSHELL in the deployment message as `initialBalance`; separate pre-funding is not required. |

If your Dapp consists of multiple contracts, deploy the first contract with an external message and deploy the other contracts from that root contract or its children with internal messages.

{% hint style="warning" %}
An **external** deployment message carries no value. Therefore, before an external deployment, the
precomputed address must already have a positive VMSHELL balance. Fund it with SHELL and use
exchange flag `16` (or a combined flag that includes bit `16`) to convert the SHELL into VMSHELL at
the destination. An internal deployment carries its own `initialBalance` and needs no separate
pre-funding transaction.
{% endhint %}

{% hint style="info" %}
**A contract deployed by an external message is self-rooted and is addressed as `<account_id>::<account_id>`.** Multisig wallets are deployed this way, so a wallet is always addressed under its own Dapp ID — not under the Dapp ID of the contracts it funds.\
A contract deployed by an internal message is addressed as `<sender_dapp_id>::<account_id>`.

An active account has one canonical Dapp ID. After an internal deployment, the network records the
Dapp ID from the deployment message and keeps a redirect from the self-rooted address. See
[Troubleshooting](./#can-t-find-account-in-shard-state-when-reading-an-account).
{% endhint %}

In this guide, we will use the [`helloWorld`](https://github.com/tvmlabs/sdk-examples/blob/main/contracts/helloWorld/helloWorld.sol) contract to demonstrate the features of a Dapp ID.

```solidity
pragma tvm-solidity >=0.76.1;
pragma AbiHeader expire;
pragma AbiHeader pubkey;

interface IHelloWorld {
    function touch() external;
}


// This class describes your smart contract.
contract helloWorld {
    // A contract can have instance variables.
    // In this example instance variable `timestamp` is used to store the time of `constructor` or `touch`
    // function call
    uint32 public timestamp;

    // The contract can have a `constructor` – a function that is called when the contract is deployed to the blockchain.
    // Parameter `value` represents the number of SHELL tokens to be converted to VMSHELL to pay the transaction fee.
    // In this example, the constructor stores the current timestamp in an instance variable.
    // All contracts need to call `tvm.accept()` for a successful deployment.
    constructor(uint64 value) {
        // Call the VM command to convert SHELL tokens to VMSHELL tokens to pay the transaction fee.
        gosh.cnvrtshellq(value);

        // Ensure that the contract's public key is set.
        require(tvm.pubkey() != 0, 101);

        // The current smart contract agrees to buy some gas to complete the
        // current transaction. This action is required to process external
        // messages, which carry no value (and therefore no gas).
        tvm.accept();

        // Set the instance variable to the current block timestamp.
        timestamp = block.timestamp;
    }

    // Converts SHELL to VMSHELL for payment of transaction fees
    // Parameter `value`- the amount of SHELL tokens that will be exchanged 1-to-1 into VMSHELL tokens.
    function exchangeToken(uint64 value) public pure {
        tvm.accept();
        getTokens();
        gosh.cnvrtshellq(value);
    }

    // Returns a static message, "helloWorld".
    // This function serves as a basic example of returning a fixed string in Solidity.
    function renderHelloWorld () public pure returns (string) {
        return 'helloWorld';
    }

    // Updates the `timestamp` variable with the current blockchain time.
    // We will use this function to modify the data in the contract.
    // Called by an external message.
    function touch() external {
        // Informs the TVM that we accept this message.
        tvm.accept();
        getTokens();
        // Update the timestamp variable with the current block timestamp.
        timestamp = block.timestamp;
    }

    // Used to call the touch method of a contract via an internal message.
    // Parameter 'addr' - the address of the contract where the 'touch' will be invoked.
    function callExtTouch(address addr) public view {
        // Each function that accepts an external message must check that
        // the message is correctly signed.
        require(msg.pubkey() == tvm.pubkey(), 102);
        tvm.accept();
        getTokens();
        IHelloWorld(addr).touch();
    }

    // Sends VMSHELL to another contract with the same Dapp ID.
    // Parameter `dest` - the target address within the same Dapp ID to receive the transfer.
    // Parameter `amount` - the amount of VMSHELL tokens to transfer.
    // Parameter `bounce` - Bounce flag. Set true if need to transfer funds to existing account;
    // set false to create new account.
    function sendVMShell(address dest, uint128 amount, bool bounce) public view {
        require(msg.pubkey() == tvm.pubkey(), 102);
        tvm.accept();
        getTokens();
        // Enables a transfer with arbitrary settings
        dest.transfer(varuint16(amount), bounce, 0);
    }

    // Allows transferring SHELL tokens within the same Dapp ID and to other Dapp IDs.
    // Parameter `dest` - the target address to receive the transfer.
    // Parameter `value`- the amount of SHELL tokens to transfer.
    function sendShell(address dest, uint128 value) public view {
        require(msg.pubkey() == tvm.pubkey(), 102);
        tvm.accept();
        getTokens();

        TvmCell payload;
        mapping(uint32 => varuint32) cc;
        cc[2] = varuint32(value);
        // Executes transfer to target address
        dest.transfer(0, true, 1, payload, cc);
    }

    // Forwards an arbitrary internal message from this contract.
    // This method does not call getTokens because it is used to deploy
    // DappConfig before centralized replenishment is available.
    function sendTransaction(
        address dest,
        uint128 value,
        mapping(uint32 => varuint32) cc,
        bool bounce,
        uint8 flags,
        TvmCell payload
    ) public view {
        require(msg.pubkey() == tvm.pubkey(), 102);
        tvm.accept();
        dest.transfer(varuint16(value), bounce, flags, payload, cc);
    }

    // Deploys a new contract within its Dapp.
    // The address of the new contract is calculated as a hash of its initial state.
    // The owner's public key is part of the initial state.
    // Parameter `stateInit` - the contract code plus data.
    // Parameter `initialBalance` - the amount of funds to transfer.
    // Parameter `payload` - a tree of cells used as the body of the outbound internal message.
    function deployNewContract(
        TvmCell stateInit,
        uint128 initialBalance,
        TvmCell payload
    )
        public view
    {
        require(msg.pubkey() == tvm.pubkey(), 102);
        // Runtime function to deploy contract with prepared msg body for constructor call.
        tvm.accept();
        getTokens();
        address addr = address.makeAddrStd(0, tvm.hash(stateInit));
        addr.transfer({stateInit: stateInit, body: payload, value: varuint16(initialBalance)});
    }

    // Checks the contract balance
    // and if it is below the specified limit, mints VMSHELL.
    // The amounts are specified in nanotokens.
    // Used to enable automatic balance replenishment.
    function getTokens() private pure {
        if (address(this).balance > 100000000000) {     // 100 VMSHELL
            return;
        }
        // The quiet variant does not abort the transaction if DappConfig has
        // not been deployed yet or does not have enough available credit.
        gosh.mintshellq(100000000000);                  // 100 VMSHELL
    }

}


```

{% hint style="warning" %}
This is a teaching contract, not a production access-control design. `touch()` and
`exchangeToken()` intentionally remain callable without an owner check. Production contracts must
authenticate every state-changing operation that should not be public and must limit who can spend
the Dapp's SHELL credit.
{% endhint %}

### **Prepare contract source code**

Let's create a folder for our project and clone the [repository](https://github.com/tvmlabs/sdk-examples/tree/main) with examples into it:

<pre><code>cd ~
mkdir helloWorld
<strong>cd helloWorld
</strong>git clone https://github.com/tvmlabs/sdk-examples.git
git clone https://github.com/ackinacki/ackinacki.git
export ACKI_NACKI="$PWD/ackinacki"

</code></pre>

and copy the `contracts` folder from there:

```bash
cp -r sdk-examples/contracts .
cd contracts/helloWorld
```

#### Prepare the contract for signed calls and internal-message forwarding <a href="#prepare-contract-for-signed-calls-and-internal-message-forwarding" id="prepare-contract-for-signed-calls-and-internal-message-forwarding"></a>

Current `sdk-examples/main` already contains the required changes. Verify the cloned
`helloWorld.sol` against the snippets below before compiling. If you use an older checkout and a
snippet is missing, add it manually.

First, the source must include the public-key ABI header immediately after
`pragma AbiHeader expire;`. Without it, a deployment signed with `--sign` fails with
`Error 621: The account has an invalid state`:

```solidity
pragma AbiHeader pubkey;
```

The contract must also include the following forwarding method:

```solidity
function sendTransaction(
    address dest,
    uint128 value,
    mapping(uint32 => varuint32) cc,
    bool bounce,
    uint8 flags,
    TvmCell payload
) public view {
    require(msg.pubkey() == tvm.pubkey(), 102);
    tvm.accept();
    dest.transfer(varuint16(value), bounce, flags, payload, cc);
}
```

The public-key check restricts forwarding to signed calls from the key embedded in this contract. The method intentionally does not call `getTokens()`: it is needed to deploy `DappConfig` before centralized replenishment is available.

`deployNewContract` must use the same public-key check and be declared `public view`. Without this
check, anyone can make the contract spend its balance to deploy an arbitrary `stateInit`:

```solidity
function deployNewContract(
    TvmCell stateInit,
    uint128 initialBalance,
    TvmCell payload
) public view {
    require(msg.pubkey() == tvm.pubkey(), 102);
    tvm.accept();
    getTokens();
    address addr = address.makeAddrStd(0, tvm.hash(stateInit));
    addr.transfer({stateInit: stateInit, body: payload, value: varuint16(initialBalance)});
}
```

Finally, verify that `getTokens()` uses the quiet replenishment instruction:

```solidity
gosh.mintshellq(100000000000);
```

The `q` variant lets ordinary calls continue when `DappConfig` has not been deployed yet or has
insufficient credit. The complete source at the beginning of this guide contains all the required
code.

### **Compile**

Compile the contract `helloWorld` using [TVM Solidity compiler 0.81.0](https://github.com/gosh-sh/TVM-Solidity-Compiler/releases/tag/gosh_0.81.0):

```bash
sold --version
```

Expected output:

```text
sold 0.81.0+commit.c5780830.mod.Linux.g++
```

The commit and platform suffix may differ, but the output must start with `sold 0.81.0`.

{% hint style="warning" %}
Stop here if any other version is printed. Both older versions (for example, `sold 0.79.3`) and
newer versions, including release candidates, do not satisfy this guide. A compiler that is too new
may fail with `TypeError: At least one modifier "internalMsg", "crossDappMsg" or "externalMsg" must
be used.` Install the 0.81.0 release linked above or invoke its downloaded binary by its full path.
{% endhint %}

```bash
sold --tvm-version gosh helloWorld.sol
```

The compiler produces no output on success. Verify the artifacts:

```bash
ls -l helloWorld.tvc helloWorld.abi.json
```

Example output:

```text
-rw-r--r-- 1 user user 1871 Aug 27 16:00 helloWorld.abi.json
-rw-r--r-- 1 user user 1163 Aug 27 16:00 helloWorld.tvc
```

TVM binary code of your contract is stored into `helloWorld.tvc` file.

Confirm that the generated ABI contains the forwarding method:

```bash
tvm-cli body sendTransaction '{"dest":"0:9999999999999999999999999999999999999999999999999999999999999999","value":10000000,"cc":{"2":100000000000},"bounce":false,"flags":1,"payload":"te6ccgEBAQEABwAACVumOBNA"}' --abi helloWorld.abi.json
```

Expected output ends with:

```text
Message body: te6ccgEBAwEATwACbRMQgmqAEzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMgAAAAAAAAAAAAAAAAExLQEAwCAQAJW6Y4E0AAFaAAAAACKLpDt0AE
```

An `Invalid name: sendTransaction` error means that `helloWorld.sol` was compiled without the forwarding method.

### **Fund the precomputed address**

Every contract must have enough VMSHELL at its precomputed address to pay the deployment fee. In this first example, `helloWorld` will be deployed with an external message and its precomputed address will be self-rooted.

To do this, we first need to determine its address. Let's start by generating a **seed phrase** and **keys** for your contract:

<pre><code><strong>tvm-cli genphrase --dump helloWorld.keys.json
</strong></code></pre>

{% hint style="info" %}
**Seed phrase** is printed to stdout.\
**Key pair** will be generated and saved to the file **`helloWorld.keys.json`**.
{% endhint %}


{% hint style="danger" %}
**Write your Seed Phrase down and store it somewhere safe, and never share it with anyone. Avoid storing it in plain text or screenshots, or any other non-secure way. If you lose it, you will not be able to recover it from your Key Pair. If you lose both Seed Phrase and Key Pair you lose access to your assets. Anyone who gets it, gets full access to your assets.**\
**Also, save the file with a pair of keys in a safe place.**
{% endhint %}

Now let's generate the **contract address** using the keys obtained earlier:

```bash
tvm-cli -j genaddr helloWorld.tvc --save --abi helloWorld.abi.json --setkey helloWorld.keys.json
```

{% hint style="info" %}
After this step, the `.tvc` file will be overwritten with the specified keys.
{% endhint %}

The command prints two JSON fields: `raw_address` as `0:<account id>` and `dapp_account` as
`<account id>::<account id>`.

Example output from the Shellnet run used throughout this guide:

```json
{
  "raw_address": "0:2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "dapp_account": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca::2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```

Your hashes will be different. All later example outputs use this tested address for consistency.


{% hint style="info" %}
**Save both values** - you will need them to deploy your contract and to work with it. Save the raw account ID as `<HELLOWORLD_ACCOUNT_ID>`. Because this contract is self-rooted, `<HELLOWORLD_DAPP_ID>` has the same value.\
We will refer to the `dapp::account` value as **`<YourAddress>`** below: `<HELLOWORLD_DAPP_ID>::<HELLOWORLD_ACCOUNT_ID>`. That is the form every
`tvm-cli` command takes. Inside ABI arguments, such as `dest`, the `0:<account id>` form is used
instead.
{% endhint %}

Send SHELL to the raw address `0:<HELLOWORLD_ACCOUNT_ID>` with exchange flag `16`. The flag converts the SHELL into VMSHELL at the destination before the contract is deployed. Flag `1` by itself does not perform this conversion.

#### Shellnet

The public Shellnet giver does not require a key. Replace `<HELLOWORLD_ACCOUNT_ID>` and run:

```bash
tvm-cli -j callx \
  --abi "$ACKI_NACKI/contracts/giver/GiverV3.abi.json" \
  --addr 0000000000000000000000000000000000000000000000000000000000000000::1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrencyWithFlag \
  '{"dest":"0:<HELLOWORLD_ACCOUNT_ID>","value":1000000000,"ecc":{"2":1000000000000},"flag":16}'
```

This sends 1000 SHELL and converts it into 1000 VMSHELL at the undeployed destination. The
`ecc["2"]` field is the SHELL amount. The separate `value` field is VMSHELL attached to the
delivery message and is used during message processing; do not add it to `ecc["2"]` when
calculating the converted token amount. A successful response has `"aborted": false` and
`"exit_code": 0`.

Example output, shortened to the fields you need to check:

{% hint style="info" %}
All shortened outputs in this guide intentionally include only the fields relevant to the current
step. Different examples may therefore show different subsets of the complete CLI response.
{% endhint %}

```json
{
  "message_hash": "aebef9f548ccd4e502e2879c7398c145e7f2bf4c70bb243d0e9ac1772f67efc8",
  "tx_hash": "a31f89bedfb2494e736fc0b3a956d7c625bc69ca12d53b96437cea6c24dd9b01",
  "aborted": false,
  "exit_code": 0
}
```

Alternatively:

* transfer SHELL from a previously deployed and funded Multisig Wallet as described in [Fund an undeployed account](how-to-deploy-a-multisig-wallet.md#fund-an-undeployed-account).

See [Get Test Tokens in Shellnet](https://dev.ackinacki.com/readme/get-test-tokens-in-shellnet) for other giver methods and token types.

#### Mainnet

The Shellnet giver is not available on Mainnet. First obtain SHELL as described in [Buying SHELL](https://docs.ackinacki.com/for-users/buy-sell-shell/buying-shell). Then send it to the precomputed address with exchange flag `16` using either:

* a previously deployed and funded [Multisig Wallet](how-to-deploy-a-multisig-wallet.md#fund-an-undeployed-account);
* [Acki Nacki Wallet](https://ackinacki.com/wallet) with Developer Mode enabled.

{% hint style="warning" %}
Funding, deployment, and subsequent calls must use the same network. Shellnet tokens have no Mainnet value.
{% endhint %}

{% hint style="info" %}
Within a Dapp ID, you can transfer both ECC tokens (for example, SHELL) and VMSHELL.\
**For contracts of other Dapp IDs, only ECC tokens can be transferred.**
{% endhint %}

Check the state of the pre-deployed contract. It should be `Uninit`:

```bash
tvm-cli -j account <YourAddress>
```

Expected fields before deployment:

```json
{
  "acc_type": "Uninit",
  "balance": "1000000000000",
  "code_hash": "null",
  "account_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "ecc_balance": {
    "2": "0"
  }
}
```

Verify that its VMSHELL balance is greater than zero before deployment.

### Deploy

This external deployment creates a new Dapp ID with `helloWorld` as its first contract. The deployment fee is paid from the VMSHELL already credited to the precomputed address.

{% hint style="info" %}
The `helloWorld` constructor calls `gosh.cnvrtshellq(value)`. The `q` suffix makes the conversion quiet: if no SHELL remains because flag `16` already converted it into VMSHELL, the instruction does not fail. Pass `0` for `value` in this flow.
{% endhint %}

Let's deploy `helloWorld` and create our first Dapp ID with this command. Use the account ID printed by `genaddr` as the destination Dapp ID:

```bash
tvm-cli -j deploy \
  --dst-dapp-id <HELLOWORLD_DAPP_ID> \
  --abi helloWorld.abi.json \
  --sign helloWorld.keys.json \
  helloWorld.tvc \
  '{"value":0}'
```

Expected output, shortened:

```json
{
  "message_hash": "1f30b07af25d49f2ace0d4857236217777941546dd2a3fa120ab1dc0cd42a10d",
  "tx_hash": "6b08eb14a08f2e29a5b579b41acb4b49f43dc7089dfc27cde434afc632dff5fc",
  "aborted": false,
  "exit_code": 0,
  "account_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "deployed_at": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca::2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```


Check the contract state again:

```bash
tvm-cli -j account <YourAddress>
```

This time, `acc_type` must be `Active`, `code_hash` must be non-null, and `dapp_id` must equal
`<HELLOWORLD_DAPP_ID>`:

```json
{
  "acc_type": "Active",
  "code_hash": "7c35a48547d67095bf4f3f3e572ba539d724b3818e1f96f459516cdcd1ac6c32",
  "account_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```


**View contract information with Explorer**

Go to [testnet Acki Nacki explorer](https://shellnet.ackinacki.org) and enter `<YourAddress>` in the search bar.\
Open your account page. You will need it later to see its transactions and messages, that we will produce in the next steps.


**Explore contract information with GraphQL**

Go to [GraphQL playground](https://shellnet.ackinacki.org/graphql).

Enter the information in the left pane and click the "Run" button (replace the contract's address with the one you obtained in the previous steps).

```graphql
query {
  blockchain {
    account(
      account_id: "<account id>"
      dapp_id: "<dapp id>"
    ) {
      info {
        acc_type_name
        dapp_id
        balance
        code
        code_hash
        data
      }
    }
  }
}
```

Both arguments are bare 64-character hex ids, without `0:` and without `0x`: the two halves of
`<YourAddress>`, which for a self-rooted contract are the same value.

Example response, with large BOC fields shortened for readability:

```json
{
  "data": {
    "blockchain": {
      "account": {
        "info": {
          "acc_type_name": "Active",
          "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
          "balance": "0xe67b482a08",
          "code": "te6ccgEC...",
          "code_hash": "7c35a48547d67095bf4f3f3e572ba539d724b3818e1f96f459516cdcd1ac6c32",
          "data": "te6ccgEBAQ..."
        }
      }
    }
  }
}
```

At the time of this Shellnet validation, GraphQL returned `balance` as a hexadecimal string:
`0xe67b482a08` equals `989910805000` in decimal. The same balance is printed as a decimal string by
`tvm-cli account`; do not compare the two textual representations directly.

{% hint style="info" %}
The `dapp_id` field will contain the identifier of your decentralized contract system on the Acki Nacki blockchain.
{% endhint %}

The playground shows the same response in its right pane:


{% hint style="info" %}
**You can specify any other fields in the result section that are available in GraphQL Schema.**\
Use the control in the upper-left corner of the screen to open the API documentation.
{% endhint %}

## **Run a getter**

The `helloWorld` contract features a get-method: `timestamp`. Let's call it and check the result:

```bash
tvm-cli -j run <YourAddress> timestamp {} --abi helloWorld.abi.json
```

Example output:

```json
{
  "timestamp": "1788186678",
  "state_timestamp": 1788186702176
}
```

`timestamp` is the value stored by the contract. `state_timestamp` is node metadata and is not the
value returned by the getter.


## Call a method on-chain

The helloWorld contract has a `touch` method. Let’s run it on-chain using the `call` command:

```bash
tvm-cli -j call <YourAddress> touch {} --abi helloWorld.abi.json --sign helloWorld.keys.json
```

Expected output, shortened:

```json
{
  "message_hash": "c332ecc60d976c4915d6a4f627cbdb86d2316a228e46bca80011023713a26007",
  "tx_hash": "404eff7a4618c5c3b9392af63b11f71b25158dbe125fb46e819a7b847a0a1644",
  "aborted": false,
  "exit_code": 0
}
```


Call the get-method `timestamp` again to verify that the timestamp has been updated:

```bash
tvm-cli -j run <YourAddress> timestamp {} --abi helloWorld.abi.json
```

```json
{
  "timestamp": "1788186723",
  "state_timestamp": 1788186726938
}
```


## Add another contract to your Dapp ID

{% hint style="warning" %}
To add a contract to the Dapp ID system, it must be deployed via an internal message through the root contract of the Dapp ID, which in our case is `helloWorld`.
{% endhint %}

In our case, this can be done using the following function:

```solidity
function deployNewContract(
        TvmCell stateInit,
        uint128 initialBalance,
        TvmCell payload
    )
```

* `stateInit` - the contract code plus data (tvc in base64);
* `initialBalance` - the amount of funds to transfer;
* `payload` - a tree of cells used as the body of the outbound internal message;

Let’s add another contract to our Dapp ID. For this, we’ll use a copy of the `helloWorld` contract and name it `helloUniverse`:

```bash
cp helloWorld.tvc helloUniverse.tvc
cp helloWorld.abi.json helloUniverse.abi.json
```

Now, let’s calculate the address of the `helloUniverse` contract using the existing key pair.

```bash
tvm-cli -j genaddr helloUniverse.tvc --save --abi helloUniverse.abi.json --setkey helloWorld.keys.json
```

And we get the same address as the `helloWorld` contract.

The output confirms that both TVCs currently contain the same public key:

```json
{
  "raw_address": "0:2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "dapp_account": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca::2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```


To avoid this, it’s essential to use a different key pair.\
Let’s generate a new seed phrase with a fresh pair of keys:

```bash
tvm-cli genphrase --dump helloUniverse.keys.json
```


Let’s calculate the address and prepare the TVC file for the new contract:

```bash
tvm-cli -j genaddr helloUniverse.tvc --save --abi helloUniverse.abi.json --setkey helloUniverse.keys.json
```

Example output:

```json
{
  "raw_address": "0:d59d3b5c6ffd2dfe8c84930920b1f2eb78e600b04ff466b59a6ee3077ac33d91",
  "dapp_account": "d59d3b5c6ffd2dfe8c84930920b1f2eb78e600b04ff466b59a6ee3077ac33d91::d59d3b5c6ffd2dfe8c84930920b1f2eb78e600b04ff466b59a6ee3077ac33d91"
}
```


Save the new raw account ID as `<HELLOUNIVERSE_ACCOUNT_ID>`. Its final address after internal
deployment will be `<HELLOWORLD_DAPP_ID>::<HELLOUNIVERSE_ACCOUNT_ID>`.

Do not fund this address separately. Unlike an external deployment, the internal deployment message
can carry both `stateInit` and VMSHELL. The `initialBalance` argument used below supplies 10 VMSHELL
to the new account and is enough for this example.

To deploy the new contract, prepare its `stateInit` and constructor message body.

To obtain the `stateInit`, execute the following command:

Since the result can be quite large, let’s save this value in a variable: `HW_STATE_INIT`.

```bash
HW_STATE_INIT=$(base64 < helloUniverse.tvc | tr -d '\n')
```

This form works with both GNU and BSD/macOS `base64`.

Let’s generate the constructor message body for the internal deployment:

```bash
tvm-cli body constructor '{"value":0}' --abi helloUniverse.abi.json
```

Expected output ends with:

```text
Message body: te6ccgEBAQEADgAAGAAAAAEAAAAAAAAAAA==
```


We’ll need to place the `Message body` field value into the deployment payload.

Now we can call `deployNewContract` function.

In our case, the command will be as follows:

```bash
tvm-cli -j call <YourAddress> deployNewContract '{"stateInit":"'$HW_STATE_INIT'", "initialBalance":10000000000, "payload":"te6ccgEBAQEADgAAGAAAAAEAAAAAAAAAAA=="}' --abi helloWorld.abi.json --sign helloWorld.keys.json
```

Expected output, shortened:

```json
{
  "message_hash": "85f09d663e886625ce2614da96d659b07fedaddb45c008749075ca8dcc776796",
  "tx_hash": "1c395c10bcf32371d4d94e2864a4e392c51fee9b06f0b314bbfd273150a8993b",
  "aborted": false,
  "exit_code": 0
}
```

This deploys the new contract inside the existing Dapp ID through an internal message.

Check the contract state using its final Dapp ID address:

```bash
tvm-cli -j account <HELLOWORLD_DAPP_ID>::<HELLOUNIVERSE_ACCOUNT_ID>
```

It must now be `Active`, and the returned `dapp_id` must equal `<HELLOWORLD_DAPP_ID>`.

```json
{
  "acc_type": "Active",
  "balance": "9986062000",
  "account_id": "d59d3b5c6ffd2dfe8c84930920b1f2eb78e600b04ff466b59a6ee3077ac33d91",
  "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```

The exact balance depends on execution fees, but it must be positive and lower than the
`100000000000` nanoVMSHELL threshold used later in the replenishment example.


{% hint style="success" %}
Note that the `helloUniverse` contract shares **the same Dapp ID** as the `helloWorld` contract.
{% endhint %}

## Call a contract inside Dapp ID

To transfer SHELL within the same Dapp ID, use the function `sendShell`:

```solidity
function sendShell(address dest, uint128 value)
```

* `dest` - the target address to receive the transfer;
* `value` - the amount of SHELL tokens to transfer.

To transfer VMSHELL within the same Dapp ID, use the function `sendVMShell`:

```solidity
function sendVMShell(address dest, uint128 amount, bool bounce)
```

* `dest` - the target address to receive the transfer;
* `amount` - the amount of VMSHELL tokens to transfer.
* `bounce` - [bounce flag](https://github.com/gosh-sh/TON-Solidity-Compiler/blob/master/API.md#addresstransfer): use `true` for an existing active recipient and `false` when intentionally creating an account.

Let's call the `touch` function in `helloUniverse` through the `helloWorld` contract.\
But first, let's check the value of the `timestamp` variable in the `helloUniverse` contract.

```bash
tvm-cli -j run <HELLOWORLD_DAPP_ID>::<HELLOUNIVERSE_ACCOUNT_ID> timestamp {} --abi helloUniverse.abi.json
```

Example output before the call:

```json
{
  "timestamp": "1788186791",
  "state_timestamp": 1788186817401
}
```


To call the `touch` function in `helloUniverse`, we’ll invoke the `callExtTouch` method in `helloWorld`.

```solidity
function callExtTouch(address addr)
```

* `addr` - is the address of the contract in which the method is called.

In our case, the command will be as follows:

```bash
tvm-cli -j call <YourAddress> callExtTouch \
  '{"addr":"0:<HELLOUNIVERSE_ACCOUNT_ID>"}' \
  --abi helloWorld.abi.json \
  --sign helloWorld.keys.json
```

Expected output, shortened:

```json
{
  "message_hash": "4de2898a68b50bfca0286f8eac6ba421e79f69caaf6673bc877982b82fec7f28",
  "tx_hash": "979046b17e04481c4ea24fcf4ecc195e80f7d77eb7eddc38758b335f6187f612",
  "aborted": false,
  "exit_code": 0
}
```


Then, let's check if the `timestamp` has changed in the `helloUniverse` contract:

```bash
tvm-cli -j run <HELLOWORLD_DAPP_ID>::<HELLOUNIVERSE_ACCOUNT_ID> timestamp {} --abi helloUniverse.abi.json
```

```json
{
  "timestamp": "1788186840",
  "state_timestamp": 1788186843483
}
```


Output: The timestamp has changed.

{% hint style="info" %}
The fee distribution for message transfers within a single Dapp ID is described in the "[Fees](./#fees)" section.
{% endhint %}

## Call a contract from another Dapp ID

Deploy `helloWorld2` with an external message, just like the first root contract:

```bash
cp helloWorld.tvc helloWorld2.tvc
cp helloWorld.abi.json helloWorld2.abi.json
tvm-cli genphrase --dump helloWorld2.keys.json
tvm-cli -j genaddr helloWorld2.tvc --save --abi helloWorld2.abi.json --setkey helloWorld2.keys.json
```

Example address output:

```json
{
  "raw_address": "0:8e020a66a5b3a698030fb13fa3658a02cfdbbb7f4ba3c1198140d7cfb1e25c7d",
  "dapp_account": "8e020a66a5b3a698030fb13fa3658a02cfdbbb7f4ba3c1198140d7cfb1e25c7d::8e020a66a5b3a698030fb13fa3658a02cfdbbb7f4ba3c1198140d7cfb1e25c7d"
}
```

Save the account ID as both `<HELLOWORLD2_ACCOUNT_ID>` and `<HELLOWORLD2_DAPP_ID>`. Fund the raw
address through the Shellnet giver with `sendCurrencyWithFlag` and flag `16`:

```bash
tvm-cli -j callx \
  --abi "$ACKI_NACKI/contracts/giver/GiverV3.abi.json" \
  --addr 0000000000000000000000000000000000000000000000000000000000000000::1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrencyWithFlag \
  '{"dest":"0:<HELLOWORLD2_ACCOUNT_ID>","value":1000000000,"ecc":{"2":1000000000000},"flag":16}'
```

Check that the giver response contains `"aborted": false` and `"exit_code": 0`, then deploy the
contract:

```bash
tvm-cli -j deploy \
  --dst-dapp-id <HELLOWORLD2_DAPP_ID> \
  --abi helloWorld2.abi.json \
  --sign helloWorld2.keys.json \
  helloWorld2.tvc \
  '{"value":0}'
```

Expected output, shortened:

```json
{
  "message_hash": "10965e556a25e565c3253fdd3d4e1dee8b50e7a4f8ecf5656e968adad620f6c5",
  "tx_hash": "93f399dd60e4d796a579e621eff9f3fd37779e2529fa69b4fbea21d2dbf2392a",
  "aborted": false,
  "exit_code": 0,
  "account_id": "8e020a66a5b3a698030fb13fa3658a02cfdbbb7f4ba3c1198140d7cfb1e25c7d",
  "dapp_id": "8e020a66a5b3a698030fb13fa3658a02cfdbbb7f4ba3c1198140d7cfb1e25c7d"
}
```

The `helloWorld` and `helloWorld2` contracts are deployed with different Dapp IDs.


Let’s check the current `timestamp` in the `helloWorld2` contract:

```bash
tvm-cli -j run <HELLOWORLD2_DAPP_ID>::<HELLOWORLD2_ACCOUNT_ID> timestamp {} --abi helloWorld2.abi.json
```

Example output before the call:

```json
{
  "timestamp": "1788187006",
  "state_timestamp": 1788187037289
}
```


To call the `touch` function in `helloWorld2`, we’ll invoke the `callExtTouch` method in `helloWorld`.

```solidity
function callExtTouch(address addr)
```

* `addr` - is the address of the contract in which the method is called.

In our case, the command will be as follows:

```bash
tvm-cli -j call <YourAddress> callExtTouch \
  '{"addr":"0:<HELLOWORLD2_ACCOUNT_ID>"}' \
  --abi helloWorld.abi.json \
  --sign helloWorld.keys.json

```

Expected output, shortened:

```json
{
  "message_hash": "f026931d47b3f7f08a9ee03c3484d5c14ff2160e648f7cdf90f2d40f1eeb4859",
  "tx_hash": "28d1060cfd879b3c87c8a8f5935d79502a287cda5510a48d26f07b98f55199e0",
  "aborted": false,
  "exit_code": 0
}
```

{% hint style="info" %}
If the message is sent to a different Dapp ID, all VMSHELL tokens in `msg.value` are set to zero.
{% endhint %}


Then, let's check if the `timestamp` has changed in the `helloWorld2` contract:

```bash
tvm-cli -j run <HELLOWORLD2_DAPP_ID>::<HELLOWORLD2_ACCOUNT_ID> timestamp {} --abi helloWorld2.abi.json
```

```json
{
  "timestamp": "1788187040",
  "state_timestamp": 1788187044223
}
```


Output: The timestamp has changed.

{% hint style="info" %}
The fee distribution for message transfers between different Dapp IDs is described in the "[Fees](./#fees)" section.
{% endhint %}

## Centralized replenishment of contracts within a Dapp ID

Contracts grouped under one Dapp ID can replenish their VMSHELL balances from shared SHELL credit. The credit is stored in one `DappConfig` contract per Dapp ID and is consumed through the quiet TVM instruction `gosh.mintshellq`.

How it works:

During block assembly, the Block Keeper records calls to `gosh.mintshellq`. The requested amount is credited to the calling contract as VMSHELL and deducted from `DappConfig.data.available_balance`, which is denominated in nanoSHELL. A small credit remainder can remain after minting, so compare values before and after instead of asserting an exact final zero.

To ensure the system functions correctly and resources are managed automatically, follow these steps:

#### **Step 1: Deploying the DappConfig contract**

The `DappConfig` contract stores the amount of SHELL credit available for minting VMSHELL within a specific Dapp ID. Its address is deterministic, so only one `DappConfig` can exist for a given Dapp ID. `DappConfig` contracts do not have an owner, and anyone can fund them. For an explanation of how `DappConfig` replenishes Dapp account balances before fees are charged, see [Fee Payment Process](https://docs.ackinacki.com/tokenomics/fee-system#fee-payment-process).

**Actions to Perform:**

1. To deploy the `DappConfig` contract, you need to know the Dapp ID. You can obtain it as follows:

```bash
tvm-cli -j account <YourAddress>
```

Read the `dapp_id` field. In the tested example it is:

```json
{
  "account_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```


2.  To deploy a `DappConfig` contract, call `deployNewConfigCustom` through the self-rooted
    `helloWorld` contract. In this example its `account_id` is equal to the required Dapp ID.

    Use the six-argument `sendTransaction` forwarding method that you [verified in
    `helloWorld`](./#prepare-contract-for-signed-calls-and-internal-message-forwarding) before
    compiling it. Do not use a Multisig wallet or a child contract for this call.
    `deployNewConfigCustom` passes `msg.sender.value` as the new configuration's Dapp ID, so the
    caller's **account ID** must be the Dapp ID. That condition is true for the self-rooted
    `helloWorld` contract, but not for its children or for an unrelated wallet.

    That is, you first need to generate the message body by running the following command:<br>

    ```bash
    tvm-cli body deployNewConfigCustom '{"authorityAddress":null}' --abi "$ACKI_NACKI/contracts/0.79.3_compiled/dappconfig/DappRoot.abi.json"

    ```

    \
    \* abi [DappRoot](https://github.com/ackinacki/ackinacki/blob/main/contracts/0.79.3_compiled/dappconfig/DappRoot.abi.json)\
    \
    As a result, you will get:<br>

    ```
    Input arguments:
      method: deployNewConfigCustom
      params: {"authorityAddress": null}
         abi: /path/to/ackinacki/contracts/0.79.3_compiled/dappconfig/DappRoot.abi.json
      output: None
    Message body: te6ccgEBAQEABwAACVumOBNA
    ```

    \
    Place the **Message body** value into the `payload` argument of the HelloWorld `sendTransaction` method and set the recipient to the [DappRoot contract](https://github.com/ackinacki/ackinacki/tree/main/contracts/dappconfig).

    Specify the amount of SHELL that becomes the initial `available_balance` credit. `DappRoot`
    burns the received SHELL and passes the same numeric amount into the new `DappConfig`; the
    credit is not added to the account's VMSHELL balance.

{% hint style="info" %}
`DappRoot` is a system contract that manages `DappConfig` contracts, including their deployment and the calculation of the `DappConfig` address for a given Dapp ID.\
The account id of the `DappRoot` contract is `9999999999999999999999999999999999999999999999999999999999999999`. It is a system contract and lives under the Dapp ID of 64 zeros, so commands address it as `0000000000000000000000000000000000000000000000000000000000000000::9999999999999999999999999999999999999999999999999999999999999999`.

`DappConfig` is a system-assisted exception to the normal internal-deployment rule: although
`DappRoot` sends the deployment message, the resulting configuration is routed under the requested
application Dapp ID.
{% endhint %}

Before sending any SHELL, calculate the deterministic `DappConfig` address:

```solidity
getConfigAddr(uint256 dapp_id)
```

```bash
tvm-cli -j run \
  0000000000000000000000000000000000000000000000000000000000000000::9999999999999999999999999999999999999999999999999999999999999999 \
  getConfigAddr \
  '{"dapp_id":"0x<HELLOWORLD_DAPP_ID>"}' \
  --abi "$ACKI_NACKI/contracts/0.79.3_compiled/dappconfig/DappRoot.abi.json"
```

Example output:

```json
{
  "config": "0:e1145dd60f6994cb3c52d345643c81eec274626251f2978742571c2610fec5d9"
}
```

Save the raw account ID as `<DAPPCONFIG_ACCOUNT_ID>`, then perform the preflight account check:

```bash
tvm-cli -j account <HELLOWORLD_DAPP_ID>::<DAPPCONFIG_ACCOUNT_ID>
echo "exit=$?"
```

If the command exits successfully and `acc_type` is `Active`, the configuration already exists:
**skip the funding and deployment commands below** and continue with the `getDetails` verification.
If the command exits with status `1` and its JSON `Error` starts with `failed to get account`, this
Dapp ID has no configuration yet and it is safe to proceed. Do not match the rest of the error text:
it comes from the HTTP layer and can change between versions.

The root contract must hold at least 100 SHELL before it can forward that amount to `DappRoot`. On
Shellnet, fund it without exchange flag `16`:

```bash
tvm-cli -j callx \
  --abi "$ACKI_NACKI/contracts/giver/GiverV3.abi.json" \
  --addr 0000000000000000000000000000000000000000000000000000000000000000::1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrency \
  '{"dest":"0:<HELLOWORLD_ACCOUNT_ID>","value":1000000000,"ecc":{"2":100000000000}}'
```

Here `value` is the VMSHELL attached to the delivery message; `ecc["2"]` is the separate SHELL
amount. A successful response contains `"aborted": false` and `"exit_code": 0`. On Mainnet,
transfer the SHELL to `helloWorld` from a funded wallet instead.

Now forward the deployment call to `DappRoot`:

```bash
tvm-cli -j call <YourAddress> sendTransaction \
  '{"dest":"0:9999999999999999999999999999999999999999999999999999999999999999","value":10000000,"bounce":false,"cc":{"2":100000000000},"flags":1,"payload":"te6ccgEBAQEABwAACVumOBNA"}' \
  --abi helloWorld.abi.json \
  --sign helloWorld.keys.json

```

Expected output, shortened:

```json
{
  "message_hash": "a7e96c06f1b9dcff79dd5de4eacf8af383bed174b12909f7fd94958b0bd54227",
  "tx_hash": "56f2be3070606c6ea3993c789f7528174b25d8f05a69e6d1b6b375ce6e0a53a4",
  "aborted": false,
  "exit_code": 0
}
```

{% hint style="info" %}
Upon deployment, `DappConfig.data.available_balance` is credited with **100 SHELL**. The contract
also receives a separate small VMSHELL deployment balance from `DappRoot`.
{% endhint %}

{% hint style="warning" %}
Do not repeat `deployNewConfigCustom` for the same Dapp ID. The address is deterministic, so a
second call cannot create another configuration; the SHELL sent to `DappRoot` has already been
burned before the deployment message reaches the existing address. If you are rerunning the guide,
always perform the `getConfigAddr` preflight above and skip deployment when the account is `Active`.
{% endhint %}

3. Verify the deployed `DappConfig`. The 100 SHELL sent during deployment is already usable credit.

The configuration belongs to the application Dapp, not to the system Dapp ID. Its canonical
address is `<HELLOWORLD_DAPP_ID>::<DAPPCONFIG_ACCOUNT_ID>`. Verify it:

```bash
tvm-cli -j account <HELLOWORLD_DAPP_ID>::<DAPPCONFIG_ACCOUNT_ID>

tvm-cli -j run \
  <HELLOWORLD_DAPP_ID>::<DAPPCONFIG_ACCOUNT_ID> \
  getDetails {} \
  --abi "$ACKI_NACKI/contracts/0.79.3_compiled/dappconfig/DappConfig.abi.json"
```

Example outputs, shortened:

```json
{
  "acc_type": "Active",
  "balance": "14999000000",
  "account_id": "e1145dd60f6994cb3c52d345643c81eec274626251f2978742571c2610fec5d9",
  "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```

```json
{
  "dapp_id": "0x2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "data": {
    "is_unlimit": false,
    "available_balance": "100000000000"
  }
}
```

#### **Step 2: Enabling Automatic Replenishment**

To automate the funding process, add balance check and token minting logic to your Dapp ID contracts.\
Use the quiet TVM instruction `gosh.mintshellq`, which requests VMSHELL from the available credit in the `DappConfig` contract for this Dapp ID:

```solidity
gosh.mintshellq(value)
```

* `value` - amount of nanoVMSHELL to mint<br>

For example, let's use the `getTokens()` function in the HelloWorld contract:

```solidity
function getTokens() private pure {
    if (address(this).balance > 100000000000) {     // 100 VMSHELL
        return;
    }
    gosh.mintshellq(100000000000);                  // 100 VMSHELL
}
```

This function mints 100 VMSHELL tokens automatically if the balance falls below the specified threshold.

Let's verify replenishment using `helloUniverse`, which was deployed with less than 100 VMSHELL.
Check its balance:

```bash
tvm-cli -j account <HELLOWORLD_DAPP_ID>::<HELLOUNIVERSE_ACCOUNT_ID>
```

The balance must be below `100000000000` nanoVMSHELL.

```json
{
  "acc_type": "Active",
  "balance": "9994062000",
  "account_id": "d59d3b5c6ffd2dfe8c84930920b1f2eb78e600b04ff466b59a6ee3077ac33d91",
  "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```

The exact balance depends on deployment and message-processing fees; this validation run started
with `9994062000` nanoVMSHELL.


Using the `getDetails()` method, you can view the available balance of the DappConfig contract.

`DappConfig` belongs to the Dapp it provides credit for. Its canonical extended address is
`<HELLOWORLD_DAPP_ID>::<DAPPCONFIG_ACCOUNT_ID>`.

```bash
tvm-cli -j run \
  <HELLOWORLD_DAPP_ID>::<DAPPCONFIG_ACCOUNT_ID> \
  getDetails {} \
  --abi "$ACKI_NACKI/contracts/0.79.3_compiled/dappconfig/DappConfig.abi.json"
```

Before the first successful mint, `data.available_balance` must be close to `100000000000` nanoSHELL.

```json
{
  "dapp_id": "0x2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "data": {
    "is_unlimit": false,
    "available_balance": "100000000000"
  }
}
```


Call `touch` in `helloUniverse` through the signed root method. Since the child balance is below
100 VMSHELL, `getTokens()` requests a replenishment:


Call the `touch()` function:

```bash
tvm-cli -j call <YourAddress> callExtTouch \
  '{"addr":"0:<HELLOUNIVERSE_ACCOUNT_ID>"}' \
  --abi helloWorld.abi.json \
  --sign helloWorld.keys.json
```

Expected output, shortened:

```json
{
  "message_hash": "fd6a357a9c6e17fc55eeb1256341031533c44e902c6d9b7c52e1c48130b507c6",
  "tx_hash": "0440d6ddfb98f5e307b2731d77975008f1e35d06f3a65b0bd0335f9af01185e2",
  "aborted": false,
  "exit_code": 0
}
```

and check the contract balance:

```bash
tvm-cli -j account <HELLOWORLD_DAPP_ID>::<HELLOUNIVERSE_ACCOUNT_ID>
```

The new balance is approximately the old balance plus 100 VMSHELL, plus the `msg.value` attached to
the internal `touch` call, minus transaction fees. This is why the increase can be slightly greater
or smaller than exactly `100000000000` nanoVMSHELL.

```json
{
  "acc_type": "Active",
  "balance": "110002062000",
  "account_id": "d59d3b5c6ffd2dfe8c84930920b1f2eb78e600b04ff466b59a6ee3077ac33d91",
  "dapp_id": "2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca"
}
```


Run `getDetails()` again. `data.available_balance` decreases by approximately 100 SHELL:

```bash
tvm-cli -j run \
  <HELLOWORLD_DAPP_ID>::<DAPPCONFIG_ACCOUNT_ID> \
  getDetails {} \
  --abi "$ACKI_NACKI/contracts/0.79.3_compiled/dappconfig/DappConfig.abi.json"
```

Example output after minting:

```json
{
  "dapp_id": "0x2578e31da9d921cedc819509b8d9a23a6e79b255fe7492830c76e959eb6079ca",
  "data": {
    "is_unlimit": false,
    "available_balance": "10592"
  }
}
```


{% hint style="info" %}
A small non-zero credit remainder is normal. Check the difference between readings instead of
asserting an exact final value. The account `ecc` field is not the spendable credit; use
`getDetails().data.available_balance`.
{% endhint %}

### Replenish `DappConfig` credit

An ordinary empty-payload SHELL transfer to `DappConfig` invokes its `receive()` function. The
function adds the received `ecc["2"]` amount to `data.available_balance`, keeps or converts the
small reserve needed by the contract, and burns excess physical SHELL. The credit remains recorded
in `available_balance` even though it is not the contract's ECC balance.

On Shellnet, this tested giver call adds 10 SHELL of credit:

```bash
tvm-cli -j callx \
  --abi "$ACKI_NACKI/contracts/giver/GiverV3.abi.json" \
  --addr 0000000000000000000000000000000000000000000000000000000000000000::1111111111111111111111111111111111111111111111111111111111111111 \
  -m sendCurrency \
  '{"dest":"0:<DAPPCONFIG_ACCOUNT_ID>","value":1000000000,"ecc":{"2":10000000000}}'
```

Expected transaction fields from the validation run:

```json
{
  "message_hash": "a73201b5967ea0ae9c46dd216c98e6fc1eeca96504da44934f2add13a0b31155",
  "tx_hash": "fbe43288add202d0b691f41baa251c0c94d6325f435e5d1640e59e3b70db4fb9",
  "aborted": false,
  "exit_code": 0
}
```

In that run, `getDetails()` changed as follows:

```text
available_balance before:       10592
available_balance after:  10000018097
```

The increase is approximately `10000000000` nanoSHELL. A small concurrent remainder adjustment is
normal, so compare the readings rather than requiring an exact delta.

On Mainnet, send the same token amounts from a single-custodian v2 wallet:

```bash
tvm-cli -j call <MSIG_ADDR> sendTransaction \
  '{"dest":"0:<DAPPCONFIG_ACCOUNT_ID>","value":1000000000,"bounce":false,"cc":{"2":10000000000},"flags":1,"payload":"","dapp_id":"0x<HELLOWORLD_DAPP_ID>"}' \
  --abi "$ACKI_NACKI/contracts/0.81.0_compiled/updatecustodianmultisigwallet_v2/UpdateCustodianMultisigWallet_v2.abi.json" \
  --sign /path/to/UpdateCustodianMultisigWallet_v2.keys.json
```

The v2 wallet requires `dapp_id` for ABI encoding and event reporting; the transfer itself is still
routed by `dest`.


## Fees

When transferring messages between contracts under the same Dapp ID, fees are distributed as follows:

* To create an outgoing message, payment is deducted from the sender’s balance.
* For relaying a message, payment is taken either from the sender's balance or deducted from the message balance (`msg.value`). The specific behavior depends on the flags set during transmission, as described [here](https://github.com/gosh-sh/TVM-Solidity-Compiler/blob/master/API.md#addresstransfer).
* Processing an incoming message is paid from the message balance (`msg.value`) and, if `tvm.accept()` is used, from the recipient’s balance.

When transferring messages between contracts under different Dapp IDs, the entire amount of tokens specified in `msg.value` (VMSHELL) is nullified. In this case, the recipient contract must assume responsibility for executing the initiated transaction by calling `tvm.accept()` within the invoked function. Otherwise, the transaction will fail with the error `Not enough funds`.

## Troubleshooting

### Error 621: `The account doesn't have a state` during contract deployment

#### Description

When running the deploy command, you may encounter the following error:

```json
{
  "code": 621,
  "message": "The account doesn't have a state"
}
```

#### Cause

This error means that the target account does not have an initialized state on the network.

### ✅ Solution

{% hint style="warning" %}
**Before an external deployment, you must fund the future contract address with VMSHELL.** An
internal deployment already carries `initialBalance` and does not use this procedure.
{% endhint %}

This can be done by calling the `sendTransaction` method with **flag `16`**.

In this case, you transfer **SHELL** tokens, which are automatically converted into **VMSHELL** tokens and credited to the balance of the account you intend to deploy.

#### Example Command

```bash
tvm-cli -j call <MSIG_ADDR> sendTransaction \
'{
  "dest":"0:<ACCOUNT_ID>",
  "value":1000000000,
  "cc":{"2":5000000000},
  "bounce":false,
  "flags":16,
  "payload":"",
  "dapp_id":"0x<ACCOUNT_ID>"
}' \
--abi "$ACKI_NACKI/contracts/0.81.0_compiled/updatecustodianmultisigwallet_v2/UpdateCustodianMultisigWallet_v2.abi.json" \
--sign /path/to/UpdateCustodianMultisigWallet_v2.keys.json
```

The v2 wallet ABI requires the seventh `dapp_id` argument. For an undeployed self-rooted account,
its future Dapp ID is the same as `<ACCOUNT_ID>`. The field is used for wallet event reporting; the
actual destination is still selected by `dest`.

As a result, the account balance will be credited with approximately **5 VMSHELL**. The separate
`value` field is the VMSHELL carrier for message delivery; `cc["2"]` is the 5 SHELL converted by
flag `16`.\
After the transaction is confirmed, you can safely run the deploy command again.

### Error 621: `The account has an invalid state`

If the account is `Uninit` and has a positive VMSHELL balance, but deployment returns:

```text
Error 621: The account has an invalid state
```

check the generated ABI:

```bash
grep -A1 '"header"' helloWorld.abi.json
```

It must contain `pubkey`:

```text
"header": ["pubkey", "time", "expire"],
```

If it does not, add `pragma AbiHeader pubkey;` to the Solidity source, compile again, run
`genaddr --save --setkey` again, and fund the **newly generated address**. Changing the source changes
the code hash and therefore the account ID; funds sent to the old address cannot be used to deploy
the recompiled TVC.

### `Can't find account in shard state` when reading an account <a href="#can-t-find-account-in-shard-state-when-reading-an-account" id="can-t-find-account-in-shard-state-when-reading-an-account"></a>

#### Description

A read request for an account that exists and is `Active` returns:

```text
Original error: Can't find account in shard state
```

#### Cause

The account was requested under a Dapp ID it does not belong to and no redirect exists for the
requested address. The response says nothing about the `dapp_id` being wrong.

The same `account_id` under two different `dapp_id` values:

```bash
# a Dapp ID the account does not belong to
curl "https://<node url>/v2/account?account_id=<account_id>&dapp_id=<other_dapp_id>"

Original error: Can't find account in shard state
```

```bash
# the account's own Dapp ID
curl "https://<node url>/v2/account?account_id=<account_id>&dapp_id=<account_id>"

{"boc":"te6ccgECsQEAK4IAA68YAELIErTqF4g16mt0LYolfPQkE0MGCoePmM0pNM5dwx1qQFALYGqM...
```

### ✅ Solution

Request the account under its own Dapp ID:

* a self-rooted contract — a Multisig Wallet, or any other contract deployed by an external message — has `dapp_id` equal to its `account_id`, and its canonical address is `<account_id>::<account_id>`;
* a contract deployed by an internal message from the root contract belongs to the Dapp ID of that root, so its canonical address is `<root_dapp_id>::<account_id>`;

The `dapp_id` of an account is also returned by the GraphQL `blockchain.account` query shown in the [Create your first Dapp ID](./#create-your-first-dapp-id) section, and by a successful `/v2/account` response.
