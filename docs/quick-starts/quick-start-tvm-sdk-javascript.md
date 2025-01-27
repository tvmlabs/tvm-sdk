# Quick Start TVM SDK JavaScript

### **Prerequisites**

* Rust v1.76+
* Node.js v18.19.1
* Python 3
* Python 3 setuptools
* [TVM-CLI and Multisig Wallet](../how-to-deploy-a-multisig-wallet.md)

**This demo app implements the following scenario:**

1. Creates and initializes an instance of the SDK client.
2. Deploys the `helloWorld` contract:\
   2.1. Generates a key pair for the contract.\
   2.2. Calculates the future address of the contract.\
   2.3. Sends tokens to the future address of the contract, which are required for deployment.\
   2.4. Deploys the `helloWorld` contract.
3. Retrieves account information and prints the balance of the `helloWorld` contract.
4. Runs the account's `get` method `timestamp`.
5. Executes the `touch` method for the newly deployed `helloWorld` contract.
6. Calls the `get` method again to ensure the timestamp has changed.
7. Sends tokens from the `helloWorld` contract to a random account.

{% hint style="info" %}
For testing your developed applications, you can the test network at [`shellnet.ackinacki.org`](https://shellnet.ackinacki.org)

To replenish the balance of the Multisig wallet contract, please contact us in the [Telegram channel](https://t.me/tvmlabs).
{% endhint %}

We will perform all the tasks in this quick start within a separate `~/test-sdk` folder. Let's create it:

```
cd ~
mkdir test-sdk
```

### **Build core TVM library for Node.js**

1. Clone the repository into a separate directory:

```
cd ~/test-sdk
git clone https://github.com/tvmlabs/tvm-sdk-js.git
```

2. Run build:

```
cd tvm-sdk-js/packages/lib-node/build
cargo run
```

As a result, the built binding `tvmsdk.node` will be placed in the folder `~/test-sdk/tvm-sdk-js/packages/lib-node`.

### **Prepare demo application**

1. Clone the repository containing the demo application:

```
cd ~/test-sdk
git clone https://github.com/tvmlabs/sdk-examples.git
cd sdk-examples/js/nodejs/helloWorld
```

2. Configure the Multisig wallet  for use in the demo app:

To do this, in the demo folder, edit the `.env` file with the following content:

```
WALLET_ADDRESS=YOUR_MULTISIG_WALLET_ADDRESS
WALLET_KEYS=FULL_PATH_TO_YOUR_MULTISIG_WALLET_KEYS_FILE  # the absolute path must be specified
```

3. Install the `@tvmsdk/core` and `@tvmsdk/lib-node` packages for the demo application:

```
npm install
```

4. Replace the binary file in `@tvmsdk/lib-node` with the Acki Nacki - compatible one that was built earlier:

```
cp ~/test-sdk/tvm-sdk-js/packages/lib-node/tvmsdk.node ~/test-sdk/sdk-examples/js/nodejs/helloWorld/node_modules/@tvmsdk/lib-node/
```

### **Run it**

Go to the folder containing the demo application and run it:

```
cd ~/test-sdk/sdk-examples/js/nodejs/helloWorld
node index.js
```

You will see a result similar to the following:

{% hint style="info" %}
All amounts are specified in nanotokens.
{% endhint %}

```
wallet keys fname: /home/username/wallet/wallet.keys.json
Future address of helloWorld contract is: 0:ef6e287ce266c9ab6bc1190b3bed061bef935796e4a0d659eb28ddcc6f9ecd03
Transferring 2000000000 nanoSHELL tokens from Multisig wallet to 0:ef6e287ce266c9ab6bc1190b3bed061bef935796e4a0d659eb28ddcc6f9ecd03
Success. Tokens were transferred

Deploying helloWorld contract
Success. Contract was deployed

helloWorld balance is 983952999 nanoVMSHELL
Run `timestamp` get method
`timestamp` value is { timestamp: '1736843146' }
Calling `touch` function
Success. TransactionId is: d9c26ef8a0adae234c500d020b298fa600f3e1b8b27758240eff654cd9b85c39

Run `timestamp` get method
Updated `timestamp` value is { timestamp: '1736843151' }
Sending 100000000 nanoSHELL tokens to 0:a088cb42523b9cacf79ca598b9070c160a13674edc8de9c662636caa7969e506
Normal exit
```

### **Source code**

The source code of all the components used can be found [here](https://github.com/tvmlabs/sdk-examples)
