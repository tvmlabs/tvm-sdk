# Quick Start (JS)

**Prerequisites**

* Rust v1.76+
* Node.js v18.19.1
* Python 3
* Python 3 setuptools

In this quick start we will lean how to

* Generate the keys
* Calculate contract address
* Top up contract address with funds from a sponsor wallet
* run the contract's get method
* run the contract's method on-chain (send transaction)

1.  Deploys the `helloWorld` contract:

    2.1 Generates key pair for the contract;

    2.2 Calculates future address of the contract;

    2.3 Sends to the future address of the contract some tokens required for deploy;

    2.4 Deploys the `helloWorld` contract;
2. Gets account info and print balance of the `helloWorld` contract
3. Runs account's get method `getTimestamp`
4. Executes `touch` method for newly deployed `helloWorld` contract
5. Runs contract's get method locally after account is updated
6. Sends some tokens from `helloWorld` contract to a random account

Info

For testing your developed applications, you can use Acki Nacki development blockchain\
at **ackinacki-testnet.tvmlabs.dev**

To replenish the balance of wallet-contract, please contact us in [Channel on Telegram](https://t.me/+1tWNH2okaPthMWU0).

We will do all the work in this quick start in a separate `~/test-sdk` folder. Let's create it:

```
cd ~
mkdir test-sdk
```

### **Build core TVM library for Node.js** <a href="#build-core-tvm-library-for-nodejs" id="build-core-tvm-library-for-nodejs"></a>

1.Clone the repository to a separate directory:

```
cd ~/test-sdk
git clone https://github.com/tvmlabs/tvm-sdk-js.git
```

2.Run build:

```
cd tvm-sdk-js/packages/lib-node/build
cargo run
```

As a result, the builded binding `tvmsdk.node` will be placed into the folder `~/test-sdk/tvm-sdk-js/packages/lib-node`.

### &#x20;<a href="#create-a-wallet" id="create-a-wallet"></a>

### **Prepare demo application** <a href="#prepare-demo-application" id="prepare-demo-application"></a>

1.Clone repository contains the demo application:

```
cd ~/test-sdk
git clone https://github.com/tvmlabs/sdk-examples.git
cd sdk-examples/js/nodejs/helloWorld
```

2.Configure wallet for using in the demo app:

To do this, in the demo folder, edit `.env` file with following content:

```
WALLET_ADDRESS=YOUR_WALLET_ADDRESS
WALLET_KEYS=FULL_PATH_TO_YOUR_WALLET_KEYS_FILE # should be absolute path
```

3.Install the packages `@tvmsdk/core` and `@tvmsdk/lib-node` for the demo application:

```
npm install
```

4.Replace the binary file in `@tvmsdk/lib-node` with an Acki Nacki-compatible one, which was builded early:

```
cp ~/test-sdk/tvm-sdk-js/packages/lib-node/tvmsdk.node ~/test-sdk/sdk-examples/js/nodejs/helloWorld/node_modules/@tvmsdk/lib-node/
```

### **Run it** <a href="#run-it" id="run-it"></a>

Go to the folder with the demo application and run it:

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

### **Source code** <a href="#source-code" id="source-code"></a>

The source code of all the components used can be found [here](https://github.com/tvmlabs/sdk-examples)
