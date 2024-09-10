# GraphQL Quick Start

Let's start with observing API playground of Acki Nacki testnet [https://hard.ackinacki.org/graphql](https://hard.ackinacki.org/graphql).

Learn how to read API documentation in the playground.

Then move to making an api request with curl.

And integrate it with TVM SDK.

## Playground

Go to [https://hard.ackinacki.org/graphql](https://hard.ackinacki.org/graphql)

Insert this query in the left part.

```graphql
query{
blockchain{
    account(address:"0:ee150cacfc7508f522dbd9bd6c705238ef316b324244843eea3e81e35ae2a962"){
      info{
        balance(format:DEC)
        address
      }
    }
  }
}
```

Now click play button and you will see the result:

## Documentation

Click on the button "schema" on the right. You will see the API documentation with all available fields

## Request with curl

```
curl --location --request POST 'https://hard.ackinacki.org/graphql' \
--header 'Content-Type: application/json' \
--data-raw '{"query":"query($address: String!){\n  blockchain{\n    account(address:$address){\n      info{\n        balance(format:DEC)\n      }\n    }\n  }\n}","variables":{"address":"0:ee150cacfc7508f522dbd9bd6c705238ef316b324244843eea3e81e35ae2a962"}}'
```

## Request with SDK (JavaScript)

```javascript
const {TvmClient} = require("@tvmsdk/core");
const {libNode} = require("@tvmsdk/lib-node");

TonClient.useBinaryLibrary(libNode)

const client = new TvmClient({
    network: {
        endpoints: [
            "https://hard.ackinacki.org/graphql"
        ],
    },
});

(async () => {
    try {
        // Get account balance. 
        const query = `
            query {
              blockchain {
                account(
                  address: "${address}"
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
