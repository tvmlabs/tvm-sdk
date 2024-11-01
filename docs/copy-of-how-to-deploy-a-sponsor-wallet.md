---
description: Create a simple wallet to sponsor your operations with TVM CLI
hidden: true
---

# Copy of How to deploy a Sponsor Wallet

## **Build and install CLI tool** <a href="#create-a-wallet" id="create-a-wallet"></a>

```
cd ~
git clone https://github.com/tvmlabs/tvm-sdk
cd tvm-sdk
cargo build --release
cd target/release
cp tvm-cli ~/.cargo/bin

```

Now path to tvm-cli is publicly accessible. You can also add it to your ENVs&#x20;

`export PATH=$PATH:~/tvm-sdk/target/release/tvm-cli`

## **Prepare wallet binary and ABI** <a href="#create-a-wallet" id="create-a-wallet"></a>

Create a folder :

```
cd ~
mkdir wallet
cd wallet
```

Download source files for the wallet from [wallet repo](https://github.com/tvmlabs/sdk-examples/tree/main/contracts/simpleWallet) and place them into this folder.

{% hint style="warning" %}
This is a test wallet, not a production wallet that underwent Formal Verification. We are working on multisig wallet that [will be published soon](https://github.com/gosh-sh/ackinacki-wallet).  Deployment process, although will be the same.&#x20;
{% endhint %}

## Configure CLI tool

We need to target it to the blockchain we will deploy to.

Here we target at testnet.

```
tvm-cli config --url ackinacki-testnet.tvmlabs.dev/graphql
```

## Generate seed phrase, keys and address

In Acki nacki blockchain the wallet address depends on its binary code and initial data that includes the owner's public key.

You can do it all with one command

```
tvm-cli genaddr wallet.tvc --genkey wallet.keys.json
```

`Raw address is your wallet address. Keys are saved to wallet.keys.json.`&#x20;

`Copy your seed phrase if you need it.`

![](https://dev.docs-gosh.pages.dev/images/n\_Acki\_Nacki\_c\_t\_n\_giver\_genn\_addr.jpg)

## **Request test tokens** <a href="#request-test-tokens-for-future-use" id="request-test-tokens-for-future-use"></a>

Request test tokens on your address. If you plan to test your contract systems then request enough tokens, as this wallet will be used as sponsor wallet for gas.

Please contact us in [Channel on Telegram](https://t.me/+1tWNH2okaPthMWU0).&#x20;

## Deploy your wallet

When you receive the tokens check the state of the pre-deployed contract. It should be **`Uninit`**:

```
tvm-cli account <YourAddress>
```

![](https://dev.docs-gosh.pages.dev/images/n\_Acki\_Nacki\_c\_t\_n\_giver\_account.jpg)

Now you are ready to deploy your wallet with the following command:

```
tvm-cli deploy --abi wallet.abi.json --sign wallet.keys.json wallet.tvc {}
```

The arguments of the constructor must be specified in curly brackets:\
`{<constructor arguments>}, We dont have any arguments here, so its empty.`&#x20;

![](https://dev.docs-gosh.pages.dev/images/n\_Acki\_Nacki\_c\_t\_n\_giver\_deploy.jpg)

Check the contract state again. This time, it is should be `Active`.

![](https://dev.docs-gosh.pages.dev/images/n\_Acki\_Nacki\_c\_t\_n\_giver\_account2.jpg)
