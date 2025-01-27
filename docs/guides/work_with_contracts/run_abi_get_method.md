# Run ABI Get Method

Run ABI compatible get methods

* [Run get method](run_abi_get_method.md#run-get-method)
* [Source code](run_abi_get_method.md#source-code)

## Run get method

With low level SDK get method is executed in 3 steps:

1. Download the latest Account State (BOC)
2. Encode message that calls the method
3. Execute the message locally on the downloaded state:

Here is the sample that executes the get method `getTimestamp` on the latest account's state.

1. account boc is downloaded with `blockchain` API
2. message that calls contract's function `getTimestamp` is encoded with `encode_message` function
3. message is executed on local TVM with `run_tvm` method

```javascript
 async function getAccount(address) {

    // `boc` or bag of cells - native blockchain data layout. Account's boc contains full account state (code and data) that
    // we will  need to execute get methods.
    const query = `
        query {
          blockchain {
            account(
              address: "${address}"
            ) {
               info {
                balance(format: DEC)
                boc
              }
            }
          }
        }`
    const {result}  = await client.net.query({query})
    const info = result.data.blockchain.account.info
    return info
}
async function runGetMethod(methodName, address, accountState) {
    // Execute the get method `getTimestamp` on the latest account's state
    // This can be managed in 3 steps:
    // 1. Download the latest Account State (BOC) 
    // 2. Encode message
    // 3. Execute the message locally on the downloaded state

    // Encode the message with `getTimestamp` call
    const { message } = await client.abi.encode_message({
        // Define contract ABI in the Application
            abi: {
            type: 'Contract',
            value: HelloWallet.abi,
        },
        address,
        call_set: {
            function_name: methodName,
            input: {},
        },
        signer: { type: 'None' },
    });

    // Execute `getTimestamp` get method  (execute the message locally on TVM)
    console.log('Run `getTimestamp` get method');
    const response = await client.tvm.run_tvm({
        message,
        account: accountState,
        abi: {
            type: 'Contract',
            value: HelloWallet.abi,
        },
    });
    return response.decoded.output
}
```

## Source code

[https://github.com/tvmlabs/sdk-examples/blob/main/js/nodejs/helloWorld/](https://github.com/tvmlabs/sdk-examples/blob/main/js/nodejs/helloWorld/)
