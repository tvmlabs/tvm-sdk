# Account queries

## Get account info

To get account info **including its state (BOC), data and code**, use the following GraphQL query:

<pre class="language-graphql"><code class="lang-graphql"><strong>query {
</strong>  blockchain{
   account(address:"0:653b9a6452c7a982c6dc92b2da9eba832ade1c467699ebb3b43dca6d77b780dd"){
    info{
      address
      acc_type
      balance
      last_paid
      last_trans_lt
      boc
      data
      code
      library
      data_hash
      code_hash
      library_hash
    }
  }
  }
}
</code></pre>

Result:

```graphql
{
  "data": {
    "blockchain": {
      "account": {
        "events": {
          "edges": [
            {
              "node": {
                "msg_id": "541be3a1be3224687158d9dcd39f313ffcb1d03d5428b2a7f51d702b177755d2",
                "body": "te6ccgEBAQEAOAAAayoqI7SAFmHd/5cK3iZgSQPbLrz9F5UkzXuik7iu9XDZGTVWJFFAAAAAAAAAAAAAAAAAAATiEA==",
                "created_at": 1775532241
              },
              "cursor": "769d478d100670000000000000000000000000000000000000000000000000000000000000000000062bc501a12102"
            },
            {
              "node": {
                "msg_id": "11ea69af1314ff8c77cb82d6bf020928a15b0bc2c1505a25b5f65b339b60e194",
                "body": "te6ccgEBAQEAOAAAayoqI7SAFJMDNXNXk8vxpapdbJTHe2M1weAqWxceYB4zXilZqp1gAAAAAAAAAAAAAAAAADDUEA==",
                "created_at": 1775547492
              },
              "cursor": "769d4b46400670000000000000000000000000000000000000000000000000000000000000000000062bcff350302"
            }
          ],
          "pageInfo": {
            "hasNextPage": false
          }
        }
      }
    }
  }
}
```

## Get transactions within timestamp range

### Use-cases

* Paginate transactions to get both transactions and messages of account within the required timestamp range
* Collect account transactions with detailed fees information
* Collect account balance history by pre-processing `balance_delta` changes on your side
* Query new account transactions to trigger some logic on your side
* Optionally filter transactions by `Aborted` type or `balance_delta` value
* Pull transactions for a period if your websocket subscription failed (use last`Transaction.chain_order` field as `after` cursor ;-) )

### Filter parameters

You can filter account transactions by these parameters:

```graphql
aborted: Boolean
min_balance_delta: String
max_balance_delta: String
```

### Pagination parameters

Use `cursor`, {`first`, `after`} or {`last`, `before`} filters for pagination.

{% hint style="success" %}
We followed GraphQL best practices and implemented Relay Cursor Connections Specification for pagination for all list types. You can read more here [https://relay.dev/graphql/connections.htm](https://relay.dev/graphql/connections.htm)
{% endhint %}

Let's paginate some account transactions from the very first one:

```graphql
query {
  blockchain{
   account(address:"0:653b9a6452c7a982c6dc92b2da9eba832ade1c467699ebb3b43dca6d77b780dd"){
    transactions
    {
      edges{
        node{
          hash
          in_message{
            hash
            value
            body
          }
          out_messages{
            hash
            value
            body
          }
          
        }
      }
      pageInfo{
        endCursor
        hasNextPage
      }
    }
  }
  }
}
```

Result

```graphql
{
  "data": {
    "blockchain": {
      "account": {
        "transactions": {
          "edges": [
            {
              "node": {
                "hash": "c8153cd353bf90c7c1214d8c1a50a30ea6d0d900f0f6c7242d1434644c1e49fb",
                "hash": "c8153cd353bf90c7c1214d8c1a50a30ea6d0d900f0f6c7242d1434644c1e49fb",
                "in_message": {
                  "hash": "c2b064872a2ce6db65ca724a03d1be5de37abe784c658ef4d5998249b9643144",
                  "value": "0x229bd2a5eb3ef4",
                  "body": null
                },
                "out_messages": []
              }
            },
            ...
          ],
          "pageInfo": {
            "endCursor": "5286af50052a33e50104",
            "hasNextPage": true
          }
        }
      }
    }
  }
}
```

Use `endCursor` field for further pagination and `hasNextPage` for identifying if more records exist.

## Get messages within timestamp range

Use-cases:

* get transfers that some account sent or received
* get account's events
* get external calls of an account
* get transfers between an account and some counterparty account
* get account events to an external address
* optionally filter messages by value amount
* Pull messages for a period if your websocket subscription failed (use Message`.chain_order` field as `after` cursor ;-) )

In all these cases you need to paginate account messages with some filters applied. Lets see how to do it.

### Filter parameters

You can filter messages by these parameters:

```graphql
master_seq_no_range: {start: Timestamp, end: Timestamp} # Time interval for pagination
counterparties: [String!]
msg_type: [BlockchainMessageTypeFilterEnum!]
min_value: String

enum BlockchainMessageTypeFilterEnum {
    ExtIn #    External inbound
    ExtOut #    External outbound
    IntIn #    Internal inbound
    IntOut #    Internal outbound
}
```

### Pagination parameters

Use `cursor`, {`first`, `after`} or {`last`, `before`} filters for pagination.

{% hint style="success" %}
We followed GraphQL best practices and implemented Relay Cursor Connections Specification for pagination for all list types. You can read more here [https://relay.dev/graphql/connections.htm](https://relay.dev/graphql/connections.htm)
{% endhint %}

### Account transfers

Lets get first 2 transfers some account received or sent. So we need to get incoming and outcoming internal messages. We separated `internal` message type into 2 types: `IntIn` and `IntOut` for search convenience. This way it is possible also to get only deposits, and only withdrawals.

```graphql
query{
  blockchain{
    account(address:"-1:99392dea1c5035feddb1bb3db9e71138d82868f7460c6da3dca26f0520798ebd"){
      messages(msg_type:[IntIn, IntOut],first:2){
        edges{
          node{
            src
            dst
            id
            hash
            value(format:DEC)
            msg_type
            created_at_string
          }
          cursor
        }
        pageInfo{
          hasNextPage
        }
      }
    }
  }
}
```

Result. We see that the next page exists, we can continue pagination.

```graphql
{
  "data": {
    "blockchain": {
      "account": {
        "messages": {
          "edges": [
            {
              "node": {
                "src": "0:7db5e456a7c41306c23c588fb0561fe63443a6f17d7e2a08672369636980678f",
                "dst": "-1:99392dea1c5035feddb1bb3db9e71138d82868f7460c6da3dca26f0520798ebd",
                "id": "message/a74d826adf7f00153e034e1ee4de4f6e5a38843ee8d14c744bfcbf3c0df9f73d",
                "hash": "a74d826adf7f00153e034e1ee4de4f6e5a38843ee8d14c744bfcbf3c0df9f73d",
                "value": "1090000000",
                "msg_type": 0,
                "created_at_string": "2021-07-17 21:08:16.000"
              },
              "cursor": "59876bem0400"
            },
            {
              "node": {
                "src": "-1:99392dea1c5035feddb1bb3db9e71138d82868f7460c6da3dca26f0520798ebd",
                "dst": "-1:3333333333333333333333333333333333333333333333333333333333333333",
                "id": "message/ead06f194b988c1658215e178e68522f27cc018df1830bcfe779d9b9ce7fee93",
                "hash": "ead06f194b988c1658215e178e68522f27cc018df1830bcfe779d9b9ce7fee93",
                "value": "1000000000",
                "msg_type": 0,
                "created_at_string": "2021-07-17 21:08:24.000"
              },
              "cursor": "59876bem0401"
            }
          ],
          "pageInfo": {
            "hasNextPage": true
          }
        }
      }
    }
  }
}
```

### Account events

To get account events run this query.

```graphql
query {
  blockchain {
    account(
      address: "0:1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a"
    ) {
      events(
        dst: ":0000000000000000000000000000000000000000000000000000000000000267"
        last: 2
      ) {
        edges {
          node {
            msg_id
            body
            created_at
          }
          cursor
        }
        pageInfo {
          hasNextPage
        }
      }
    }
  }
}
```

Result

```graphql
{
  "data": {
    "blockchain": {
      "account": {
        "events": {
          "edges": [
            {
              "node": {
                "msg_id": "541be3a1be3224687158d9dcd39f313ffcb1d03d5428b2a7f51d702b177755d2",
                "body": "te6ccgEBAQEAOAAAayoqI7SAFmHd/5cK3iZgSQPbLrz9F5UkzXuik7iu9XDZGTVWJFFAAAAAAAAAAAAAAAAAAATiEA==",
                "created_at": 1775532241
              },
              "cursor": "769d478d100670000000000000000000000000000000000000000000000000000000000000000000062bc501a12102"
            },
            {
              "node": {
                "msg_id": "11ea69af1314ff8c77cb82d6bf020928a15b0bc2c1505a25b5f65b339b60e194",
                "body": "te6ccgEBAQEAOAAAayoqI7SAFJMDNXNXk8vxpapdbJTHe2M1weAqWxceYB4zXilZqp1gAAAAAAAAAAAAAAAAADDUEA==",
                "created_at": 1775547492
              },
              "cursor": "769d4b46400670000000000000000000000000000000000000000000000000000000000000000000062bcff350302"
            }
          ],
          "pageInfo": {
            "hasNextPage": false
          }
        }
      }
    }
  }
}
```

Then, by decoding the \`body\` of that message you can obtain the data attached to the event.\
You can parse it with SDK function [`abi.decode_message_body`](https://docs.everos.dev/ever-sdk/reference/types-and-methods/mod_abi#decode_message_body) or use tvm-cli comand:\
For example:

```
tvm-cli decode body te6ccgEBAQEAOAAAayoqI7SAFmHd/5cK3iZgSQPbLrz9F5UkzXuik7iu9XDZGTVWJFFAAAAAAAAAAAAAAAAAAATiEA== --abi ./contracts/0.79.3_compiled/exchange/Exchange.abi.json
```

As a result, you will get something approximately like this:

```
Input arguments:
    body: te6ccgEBAQEAOAAAayoqI7SAFmHd/5cK3iZgSQPbLrz9F5UkzXuik7iu9XDZGTVWJFFAAAAAAAAAAAAAAAAAAATiEA==
     abi: ./contracts/0.79.3_compiled/exchange/Exchange.abi.json


UsdcMigrated: {
  "from": "0:b30eeffcb856f13302481ed975e7e8bca9266bdd149dc577ab86c8c9aab1228a",
  "value": "10000"
}
Signature: None
Header: null
FunctionId: 9981240F
```

### Account external calls

If you want to collect external calls of an account, filter by msg\_type = `ExtIn`. `Body` field contains ABI-encoded information with Event data. You can parse it with SDK function [`abi.decode_message_body`](https://docs.everos.dev/ever-sdk/reference/types-and-methods/mod_abi#decode_message_body). Lets get the last external call:

```graphql
query{
  blockchain{
    account(address:"0:3d10c4d6dfc5d3cf6f8ac3d7468b792b91385c087da8f59669569493c7c0e28e"){
      messages(msg_type:[ExtIn],last:1){
        edges{
          node{
            hash
            body
            created_at_string
          }
          cursor
        }
        pageInfo{
          hasPreviousPage
        }
      }
    }
  }
}
```

Result

```graphql
{
  "data": {
    "blockchain": {
      "account": {
        "messages": {
          "edges": [
            {
              "node": {
                "hash": "3ebc5a30f598825a015b99048b3f9baeb1d60818aa77ec6ceb3b84254e649723",
                "body": "te6ccgEBAQEAewAA8cb04wBQrr+dL/xBeDVKUIHJpF+ixQ9vsl7rIu8BtyRr72MIA9l87nY/maACAjMiwTkNeYlx+Vm3AtMvU000ZYXOo+U6Dh8fZKMrMO68do6VqlWYBXM3BEnQiVL3dDmtSMAAAGABANbS2JO2i4ap0DtYk7XgW5WzcGA=",
                "created_at_string": "2022-04-07 12:32:53.000"
              },
              "cursor": "5f7bcee00615d8d7711c0000"
            }
          ],
          "pageInfo": {
            "hasPreviousPage": true
          }
        }
      }
    }
  }
}
```
