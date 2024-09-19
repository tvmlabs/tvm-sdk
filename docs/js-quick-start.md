---
description: >-
  This is a reference quick start guide in JS, other languages have the same api
  and should implement the integration the same way
---

# Quick Start

## **Prerequisites** <a href="#prerequisites" id="prerequisites"></a>

* You should have a [Sponsor Wallet](how-to-deploy-a-sponsor-wallet.md) deployed.&#x20;
* Node.js v18

{% hint style="success" %}
The next 3 dependencies are needed only to build TVM-SDK binary for Node.js engine for your platform. Soon we will start publishing them for your convenience in [tvm-sdk-js](https://github.com/tvmlabs/tvm-sdk-js) github releases. And you will not require to have these preinstalled.&#x20;
{% endhint %}

* Rust v1.76+
* Python 3
* Python 3 setuptools

## What we will learn

In this quick start we will lean how to&#x20;

* Generate the keys
* Calculate contract address
* Top up contract address with funds from a sponsor wallet
* run the contract's get method
* run the contract's method on-chain (send transaction)

## **Create test project** <a href="#build-core-tvm-library-for-nodejs" id="build-core-tvm-library-for-nodejs"></a>

We will do all the work in this quick start in a separate `~/test-sdk` folder. Let's create it:

```
cd ~
mkdir test-sdk
git clone https://github.com/tvmlabs/sdk-examples.git
cd sdk-examples/js/nodejs/helloWorld
```

Now you need to specify your Sponsor Wallet credentials in order for the App to use it for test contract work. To do this, in the App folder, edit `.env` file with following content:

```
WALLET_ADDRESS=YOUR_WALLET_ADDRESS
WALLET_KEYS=FULL_PATH_TO_YOUR_WALLET_KEYS_FILE # should be absolute path
```

Install the packages `@tvmsdk/core` and `@tvmsdk/lib-node` for the demo application:

```
npm install
```

Before running the test project we need to add TVM-SDK binary built for your platform. &#x20;

Let's do it.

## **Build TVM SDK binary** <a href="#build-core-tvm-library-for-nodejs" id="build-core-tvm-library-for-nodejs"></a>

Clone the TVM-SDK repository to a separate directory:

```
cd ~/test-sdk
git clone https://github.com/tvmlabs/tvm-sdk-js.git
```

Run build:

```
cd tvm-sdk-js/packages/lib-node/build
cargo run
```

As a result, the tvm-sdk binary for Node engine`tvmsdk.node` will be placed into the folder `~/test-sdk/tvm-sdk-js/packages/lib-node`.

Place the binary file to  /node\_modules/@tvmsdk/lib-node:

```
cp ~/test-sdk/tvm-sdk-js/packages/lib-node/tvmsdk.node ~/test-sdk/sdk-examples/js/nodejs/helloWorld/node_modules/@tvmsdk/lib-node/
```

## **Run our App!** <a href="#run-it" id="run-it"></a>

Go to the folder with the HelloWorld application and run it:

```
cd ~/test-sdk/sdk-examples/js/nodejs/helloWorld
node index.js
```

You will see a result similar to the following:

```
wallet keys fname: /home/username/wallet/wallet.keys.json
Future address of helloWorld contract is: 0:90e7941f8eb4806097598e1653a97fc6f8951423e4f12b417d67b4b186633771
Transferring 1000000000 tokens from wallet to 0:90e7941f8eb4806097598e1653a97fc6f8951423e4f12b417d67b4b186633771
Success. Tokens were transferred

Deploying helloWorld contract
Success. Contract was deployed

helloWorld balance is 986483999
Run `getTimestamp` get method
`timestamp` value is {
  value0: '0x0000000000000000000000000000000000000000000000000000000066cdf75f'
}
Calling touch function
Success. TransactionId is: c7b7cb19d4b7f4d56c854c593dfe68c4f2cfc508af6a766a5c841d2fbfde417a

Waiting for account update
Success. Account was updated, it took 0 sec.

Run `getTimestamp` get method
Updated `timestamp` value is {
  value0: '0x0000000000000000000000000000000000000000000000000000000066cdf763'
}
Sending 100000000 tokens to 0:2b8436113a37866f5f8258f0e1645872a2a7168ef8b8115405de804a368477f8
Success. Target account will receive: 99000000 tokens

Normal exit
```
