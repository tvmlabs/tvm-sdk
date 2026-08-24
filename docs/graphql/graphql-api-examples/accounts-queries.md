# Account queries

An account is identified by two ids: `account_id` and `dapp_id`, the two halves of the address
`dapp_id::account_id`. Both are bare 64-character hex, without `0:` and without `0x`. For a contract
deployed by an external message the two are the same value.

The examples below query the Shellnet giver, `0000000000000000000000000000000000000000000000000000000000000000::1111111111111111111111111111111111111111111111111111111111111111`, at [https://shellnet.ackinacki.org/graphql](https://shellnet.ackinacki.org/graphql), and the results are what it answered.

## Get account info

To get account info **including its state (BOC), data and code**, use the following GraphQL query:

```graphql
query {
  blockchain{
    account(
      account_id:"1111111111111111111111111111111111111111111111111111111111111111"
      dapp_id:"0000000000000000000000000000000000000000000000000000000000000000"
    ){
      info{
        address
        dapp_id
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
```

Result (`boc`, `data` and `code` are shortened here):

```graphql
{
  "data": {
    "blockchain": {
      "account": {
        "info": {
          "address": "1111111111111111111111111111111111111111111111111111111111111111",
          "dapp_id": "0000000000000000000000000000000000000000000000000000000000000000",
          "acc_type": 1,
          "balance": "0xe8d4a51000",
          "last_paid": 0,
          "last_trans_lt": "0x693445",
          "boc": "te6ccgECQwEADcMAArEYACIiIiIiIiIiIiIiIiIi...",
          "data": "te6ccgEBAwEAdQABiRKKVYYEWpo8MA+Z75WNVTar...",
          "code": "te6ccgECPwEADPEABCSK7VMg4wMgwP/jAiDA/uMC...",
          "library": null,
          "data_hash": "",
          "code_hash": "2bc3395b4d88658c7e3e591af5af55c00e44dfa9540af1f250b9f1892cdfb233",
          "library_hash": null
        }
      }
    }
  }
}
```

`address` and `dapp_id` come back as bare hex ids: put them together as `dapp_id::account_id` to get
the address `tvm-cli` takes.

Ask for `balance(format: DEC)` to read the balance as a decimal string instead of hex.

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
master_seq_no_range: {start: Timestamp, end: Timestamp} # Time interval for pagination
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
    account(
      account_id:"1111111111111111111111111111111111111111111111111111111111111111"
      dapp_id:"0000000000000000000000000000000000000000000000000000000000000000"
    ){
      transactions(first:2)
      {
        edges{
          node{
            id
            tr_type_name
            aborted
            in_msg
            out_msgs
            total_fees(format:DEC)
            balance_delta(format:DEC)
          }
          cursor
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
                "id": "b241fe2a2cb57143ff6b4b1464a6656e421bdd20059dc24584e40aab43c4e8cb",
                "tr_type_name": "Ordinary",
                "aborted": false,
                "in_msg": "cc713b7d9e474685cd4f6b32af98af42cad56900e0464eedfbd05d43c7e7e0a3",
                "out_msgs": [
                  "6669e01217c9e85673961bd3714d90fa7b65e2b9d1a50a4720576b1b30551f9e",
                  "aeee9fe3f21d7abd543c21b4fc9d2d7363209b458c3c2f81092d52137b1e2530"
                ],
                "total_fees": "0",
                "balance_delta": "-4503599627370496"
              },
              "cursor": "76a800b4c00670000000000000000000000000000000000000000000000000000000000000000000057dd7e300"
            },
            ...
          ],
          "pageInfo": {
            "endCursor": "76a800b5100670000000000000000000000000000000000000000000000000000000000000000000057dd7f200",
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
* optionally filter messages by value amount
* Pull messages for a period if your websocket subscription failed (use Message`.chain_order` field as `after` cursor ;-) )

In all these cases you need to paginate account messages with some filters applied. Lets see how to do it.

### Filter parameters

You can filter messages by these parameters:

```graphql
master_seq_no_range: {start: Timestamp, end: Timestamp} # Time interval for pagination
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
    account(
      account_id:"1111111111111111111111111111111111111111111111111111111111111111"
      dapp_id:"0000000000000000000000000000000000000000000000000000000000000000"
    ){
      messages(msg_type:[IntIn, IntOut],first:2){
        edges{
          node{
            src
            dst
            id
            value(format:DEC)
            msg_type_name
            created_at
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
                "src": "0:1111111111111111111111111111111111111111111111111111111111111111",
                "dst": "0:20fcf0b46bc179bd6ad7ac2c3a303ec94dca9a1a2f421e2c35ebff55db35bdc4",
                "id": "6669e01217c9e85673961bd3714d90fa7b65e2b9d1a50a4720576b1b30551f9e",
                "value": "10000000000000",
                "msg_type_name": "Internal",
                "created_at": 1786776396
              },
              "cursor": "76a800b4c00670000000000000000000000000000000000000000000000000000000000000000000057dd7e30100"
            },
            {
              "node": {
                "src": "0:1111111111111111111111111111111111111111111111111111111111111111",
                "dst": "0:20fcf0b46bc179bd6ad7ac2c3a303ec94dca9a1a2f421e2c35ebff55db35bdc4",
                "id": "702aef3b6d68992336331dffe769f4ee7723bc426821e9febfd350610b7be54c",
                "value": "10000000000000",
                "msg_type_name": "Internal",
                "created_at": 1786776401
              },
              "cursor": "76a800b5100670000000000000000000000000000000000000000000000000000000000000000000057dd7f20100"
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

`src` and `dst` inside a message are addresses in the `0:<account_id>` form, the form ABI arguments
take.

### Account events

To get account events run this query.

```graphql
query {
  blockchain {
    account(
      account_id:"1111111111111111111111111111111111111111111111111111111111111111"
      dapp_id:"0000000000000000000000000000000000000000000000000000000000000000"
    ) {
      events(last: 2) {
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

The `events` field also takes an optional `dst` argument — an external address, to keep only the
events sent to it.

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
                "msg_id": "817643f65d7a8116674492c0e6a91ce396b6fdc48281263bb0354cc685edcbb9",
                "body": "te6ccgEBAgEAOgABViEI6kGACKO9XRrSp/MyNp2giu9c9UdJWECBMDSbqBUIP4txza4o7msoARABABOgAAAAAiO5rKAE",
                "created_at": 1787602945
              },
              "cursor": "76a8ca8010067000000000000000000000000000000000000000000000000000000000000000000005a404cf0002"
            },
            {
              "node": {
                "msg_id": "976cc1168835aa44f730d57f1cc222ab49fe828184086e4a4fbf8ec745551342",
                "body": "te6ccgEBAgEAOgABViEI6kGADlN+oMULTKHSGxY2dOyh/axJf+BLEmM6JF8gOxXWtycI7msoARABABOgAAAAAiO5rKAE",
                "created_at": 1787603388
              },
              "cursor": "76a8ca9bc0067000000000000000000000000000000000000000000000000000000000000000000005a40a0c0002"
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

Then, by decoding the `body` of that message you can obtain the data attached to the event.\
You can parse it with SDK function [`abi.decode_message_body`](../../reference/types-and-methods/mod_abi.md#decode_message_body) or use tvm-cli comand:\
For example:

```
tvm-cli decode body te6ccgEBAgEAOgABViEI6kGADlN+oMULTKHSGxY2dOyh/axJf+BLEmM6JF8gOxXWtycI7msoARABABOgAAAAAiO5rKAE --abi GiverV3.abi.json
```

As a result, you will get something approximately like this:

```
SentCurrencyWithFlag: {
  "dst": "0:729bf506285a650e90d8b1b3a7650fed624bff02589319d122f901d8aeb5b938",
  "value": "2000000000",
  "value2": {
    "2": "2000000000"
  },
  "flag": "16"
}
Signature: None
Header: null
FunctionId: 43A4362C
```

### Account external calls

If you want to collect external calls of an account, filter by msg\_type = `ExtIn`. Lets get the last external call:

```graphql
query{
  blockchain{
    account(
      account_id:"1111111111111111111111111111111111111111111111111111111111111111"
      dapp_id:"0000000000000000000000000000000000000000000000000000000000000000"
    ){
      messages(msg_type:[ExtIn],last:1){
        edges{
          node{
            id
            boc
            created_at
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
                "id": "14b88ce8e8b7279b557ad493313a7b7a02105fea38c81e1cc9dce4ef4a9fbbdf",
                "boc": "te6ccgEBAwEArAAB5YgAIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIEjIM7yj4+Ptizf3EO+6TJSs0WGY/vtsdNIaOxLXURWrSkzb48ZzZhg1Bta4fAoh3xzpZ6lH1bQ0BvhpUXeQWkOAAABoDV3B/ZqjKnkd9KKKIBAU6ADlN+oMULTKHSGxY2dOyh/axJf+BLEmM6JF8gOxXWtycI7msoAQICABOgAAAAAiO5rKAE",
                "created_at": 1787603389
              },
              "cursor": "76a8ca9bd0067000000000000000000000000000000000000000000000000000000000000000000005a40a0f0000"
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

The `boc` field carries the whole message. Pass it to `tvm-cli` together with the contract ABI to see
which function was called:

```
tvm-cli decode msg te6ccgEBAwEArAAB5YgAIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIiIEjIM7yj4+Ptizf3EO+6TJSs0WGY/vtsdNIaOxLXURWrSkzb48ZzZhg1Bta4fAoh3xzpZ6lH1bQ0BvhpUXeQWkOAAABoDV3B/ZqjKnkd9KKKIBAU6ADlN+oMULTKHSGxY2dOyh/axJf+BLEmM6JF8gOxXWtycI7msoAQICABOgAAAAAiO5rKAE --abi GiverV3.abi.json
```

```
  "BodyCall": {
    "sendCurrencyWithFlag": {
      "dest": "0:729bf506285a650e90d8b1b3a7650fed624bff02589319d122f901d8aeb5b938",
      "value": "2000000000",
      "ecc": {
        "2": "2000000000"
      },
      "flag": "2"
    },
    "BodyHeader": {
      "expire": "1787603428",
      "time": "1787603388406",
      "pubkey": "None"
    }
  }
```
