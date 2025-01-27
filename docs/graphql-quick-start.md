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
    account(address:"0:1111111111111111111111111111111111111111111111111111111111111111"){
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

Click on the button "book" in the upper left corner of the screen. You will see the API documentation with all available fields.

## Request with curl

```
curl --location --request POST https://shellnet.ackinacki.org/graphql \
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
            "https://shellnet.ackinacki.org/graphql"
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
