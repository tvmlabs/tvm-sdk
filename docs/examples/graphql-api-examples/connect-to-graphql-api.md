# Connect to GraphQL API

### HTTPS

{% tabs %}
{% tab title="Curl" %}
```bash
curl --location --request POST https://shellnet.ackinacki.org/graphql \
--header 'Content-Type: application/json' \
--data-raw '{"query":"query{\n  blockchain{\n    blocks(last:1){\n      edges{\n        node{\n          hash\n          seq_no\n        }\n      }\n    }\n  }\n}","variables":{}}'
```
{% endtab %}

{% tab title="tvm-sdk-js" %}
```javascript
const {TvmClient} = require("@tvmsdk/core");
const {libNode} = require("@tvmsdk/lib-node");

TvmClient.useBinaryLibrary(libNode)

const client = new TvmClient({
    network: {
        endpoints: [
            "endpoint"
        ],
    },
});

(async () => {
    try {
        queryString = `
            query{
                blockchain{
                blocks(last:1){
                    edges{
                    node{
                        hash
                        seq_no
                    }
                    }
                }
                }
            }
        `
        let {seq_no, hash} = (await client.net.query({ 
            "query": queryString }))
        .result.data.blockchain.blocks.edges[0].node;
        console.log("The last masterchain block seqNo is " + seq_no+ '\n' + "the hash is" + hash);
        client.close();
}
    catch (error) {
            console.error(error);
    }
}
)()
```
{% endtab %}

{% tab title="JS fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Content-Type", "application/json");

var graphql = JSON.stringify({
  query: "query{\n  blockchain{\n    blocks(last:1){\n      edges{\n        node{\n          hash\n          seq_no\n        }\n      }\n    }\n  }\n}",
  variables: {}
})
var requestOptions = {
  method: 'POST',
  headers: myHeaders,
  body: graphql,
  redirect: 'follow'
};

fetch("endpoint", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="Postman" %}
```
URL: endpoint
Body: GraphQL
Query:

query{
  blockchain{
    blocks(last:1){
      edges{
        node{
          hash
          seq_no
        }
      }
    }
  }
}
```
{% endtab %}
{% endtabs %}

In the next section find out how to work with GraphQL Web playground and easily explore blockchain data with it.
