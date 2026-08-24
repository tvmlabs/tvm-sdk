# GraphQL Quick Start

Let's start with observing API playground of Acki Nacki testnet [https://shellnet.ackinacki.org/graphql](https://shellnet.ackinacki.org/graphql).

Learn how to read API documentation in the playground.

Then move to making an api request with curl.

And integrate it with TVM SDK.

## Playground

Go to [https://shellnet.ackinacki.org/graphql](https://shellnet.ackinacki.org/graphql)

Insert this query in the left part.

```graphql
query{
blockchain{
    account(
      account_id:"1111111111111111111111111111111111111111111111111111111111111111"
      dapp_id:"0000000000000000000000000000000000000000000000000000000000000000"
    ){
      info{
        balance(format:DEC)
        address
        dapp_id
      }
    }
  }
}
```

An account is identified by two ids: `account_id` and `dapp_id`, the two halves of the address
`dapp_id::account_id`. Both are bare 64-character hex, without `0:` and without `0x`. For a contract
deployed by an external message the two are the same value.

Now click play button and you will see the result:

## Documentation

Click on the button "book" in the upper left corner of the screen. You will see the API documentation with all available fields.

## Request with curl

```
curl --location --request POST https://shellnet.ackinacki.org/graphql \
--header 'Content-Type: application/json' \
--data-raw '{"query":"query($account_id: String!, $dapp_id: String!){\n  blockchain{\n    account(account_id:$account_id, dapp_id:$dapp_id){\n      info{\n        balance(format:DEC)\n      }\n    }\n  }\n}","variables":{"account_id":"1111111111111111111111111111111111111111111111111111111111111111","dapp_id":"0000000000000000000000000000000000000000000000000000000000000000"}}'
```

## Request with SDK (JavaScript)

```javascript
const {TvmClient} = require("@tvmsdk/core");
const {libNode} = require("@tvmsdk/lib-node");

TvmClient.useBinaryLibrary(libNode)

const client = new TvmClient({
    network: {
        endpoints: [
            "https://shellnet.ackinacki.org/graphql"
        ],
    },
});

const account_id = "1111111111111111111111111111111111111111111111111111111111111111";
const dapp_id = "0000000000000000000000000000000000000000000000000000000000000000";

(async () => {
    try {
        // Get account balance. 
        const query = `
            query {
              blockchain {
                account(
                  account_id: "${account_id}"
                  dapp_id: "${dapp_id}"
                ) {
                   info {
                    balance(format: DEC)
                  }
                }
              }
            }`
        const {result}  = await client.net.query({query})
        console.log(`The account balance is ${result.data.blockchain.account.info.balance}`);
        client.close();
    }
    catch (error) {
        console.error(error);
    }
}
)()
```
