# Accounts

## Get account info

To get account info **including BOC, data and code**, use the following GraphQL query:

```graphql
query {
  blockchain{
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
```

Result:

```graphql
{
  "data": {
    "blockchain": {
      "account": {
        "info": {
          "address": "0:653b9a6452c7a982c6dc92b2da9eba832ade1c467699ebb3b43dca6d77b780dd",
          "acc_type": 1,
          "balance": "0x223e8b8cef379b",
          "last_paid": 1647425518,
          "last_trans_lt": "0x2a9059e77c4",
          "boc": "te6ccgECDwEAApkAAnXABlO5pkUsepgsbckrLanrqDKt4cRnaZ67O0Pcptd7eA3SHoR9QxGNv3AAAAqkFnnfEciPouM7zebTQAIBAJNniOOihCJZNr2ArCaziee6VYr6JdmUdNs82Mlm2VJbMQAAAX+SNgQJwAJMlgo4O1jZEmyBymkHd/cTX5Y2hWW2OCWru/YnrscYSAIm/wD0pCAiwAGS9KDhiu1TWDD0oQUDAQr0pCD0oQQAAAIBIAgGAez/fyHtRNAg10nCAY4R0//TP9MA+Gp/+GH4Zvhj+GKOPvQFjQhgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAE+GpwAYBA9A7yvdcL//hicPhjcPhmf/hh4tMAAZ+BAgDXGCD5AVj4QvkQ8qje0z8BBwBajh74QyG5IJ8wIPgjgQPoqIIIG3dAoLnekvhj4IA08jTY0x8B8AH4R26S8jzeAgEgDgkCAnULCgDVtF1VjXwgt0cKdqJoaf/pn+mAfDU//DD8M3wx/DFvfSBo/AB8JRDjgscSwQwLpDt0ABC4ZGfCwGUAOeegZwD9AUA056BnwOfA5Ln9gBB8NW+YfCFkZf/8IeeFn/wjZ4WAfCUA52T2qj/8M8ABCbRar5/ADAH++EFujlztRNAg10nCAY4R0//TP9MA+Gp/+GH4Zvhj+GKOPvQFjQhgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAE+GpwAYBA9A7yvdcL//hicPhjcPhmf/hh4t74RvJzcfhm0fgA+ELIy//4Q88LP/hGzwsA+EoBzg0ADMntVH/4ZwBq3nAi0NYCMdIAMNwhxwCQ4CHXDR+S8jzhUxGQ4cEEIoIQ/////byxkvI84AHwAfhHbpLyPN4=",
          "data": "te6ccgEBAQEATAAAk2eI46KEIlk2vYCsJrOJ57pVivol2ZR02zzYyWbZUlsxAAABf5I2BAnAAkyWCjg7WNkSbIHKaQd39xNfljaFZbY4Jau79ieuxxhI",
          "code": "te6ccgECDQEAAg4AAib/APSkICLAAZL0oOGK7VNYMPShAwEBCvSkIPShAgAAAgEgBgQB7P9/Ie1E0CDXScIBjhHT/9M/0wD4an/4Yfhm+GP4Yo4+9AWNCGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAT4anABgED0DvK91wv/+GJw+GNw+GZ/+GHi0wABn4ECANcYIPkBWPhC+RDyqN7TPwEFAFqOHvhDIbkgnzAg+COBA+iogggbd0Cgud6S+GPggDTyNNjTHwHwAfhHbpLyPN4CASAMBwICdQkIANW0XVWNfCC3Rwp2omhp/+mf6YB8NT/8MPwzfDH8MW99IGj8AHwlEOOCxxLBDAukO3QAELhkZ8LAZQA556BnAP0BQDTnoGfA58Dkuf2AEHw1b5h8IWRl//wh54Wf/CNnhYB8JQDnZPaqP/wzwAEJtFqvn8AKAf74QW6OXO1E0CDXScIBjhHT/9M/0wD4an/4Yfhm+GP4Yo4+9AWNCGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAT4anABgED0DvK91wv/+GJw+GNw+GZ/+GHi3vhG8nNx+GbR+AD4QsjL//hDzws/+EbPCwD4SgHOCwAMye1Uf/hnAGrecCLQ1gIx0gAw3CHHAJDgIdcNH5LyPOFTEZDhwQQighD////9vLGS8jzgAfAB+EdukvI83g==",
          "library": null,
          "data_hash": "a34d868df79e09b2c3af67c1a9e210c1afef27f2376ee4ea5b00d20e7e55c058",
          "code_hash": "59ba6d164798169031c8ca18fa10c7038e7ad73b8d64f4c990e029a5dcfa59c3",
          "library_hash": null
        }
      }
    }
  }
}
```

fields:

* `address` is full account address that consists of workchainID:address
* `acc_type`
  * 0 – uninit (Account has balance but no code)
  * 1 – active (Account has balance and code)
  * 2 – frozen(Account has been frozen for some reasons)
  * 3 - nonExist (Account was deleted)
* `last_paid` - unixtime of the most recent storage payment (happens each transaction execution)
* `balance` - tokens on account (Note: to deploy smart contract code you need to have non-zero balance)
* `last_trans_lt` - logical time of last account transaction
* `boc` - Bag of cells with the account struct encoded as base64 (contains code, data, library and other header information).
* `data` - bag of cells with the account's data
* `code` - bag of cells with the account's code
* `library` - If present, contains library code used in smart-contract.
* `data_hash` - hash of account data
* `code_hash` - hash of account code
* `library_hash` - library field hash

## Filter accounts

If you need to filter accounts by some condition and paginate them, use accounts collection.

Use id(account address) as cursor for pagination.

#### Paginate accounts having same code\_hash:

```graphql
query{
  accounts(
    filter:{
      code_hash:{
        eq:"80d6c47c4a25543c9b397b71716f3fae1e2c5d247174c52e2c19bd896442b105"
      }
      id:{
        gt:"0:001a338e4af5fe33307c2b0d04de453513019942748eb2b290ca3db0adfe8343"
      }
    }
    orderBy:[
      {
        path:"id",
        direction:ASC
      }
    ]
  ){
    id
    balance(format:DEC)
    last_paid
  }
}
```

#### Paginate accounts having same code\_hash, updated after timestamp

```graphql
query{
  accounts(
    filter:{
      workchain_id:{
        eq:0
      },
      code_hash:{
        eq:"80d6c47c4a25543c9b397b71716f3fae1e2c5d247174c52e2c19bd896442b105"
      }
      last_paid:{
        ge:1687023485
      }
      id:{
        gt:"0:001a338e4af5fe33307c2b0d04de453513019942748eb2b290ca3db0adfe8343"
      }
    }
    orderBy:[
      {path:"last_paid", direction:ASC}      
      {path:"id", direction:ASC}
    ]
  ){
    id
    balance(format:DEC)
    last_paid
  }
}
```

## Get a list of accounts

You can enumerate a list of account addresses to get their balances and other metadata.

{% hint style="warning" %}
You can not get the list of BOCs of accounts. Retrieve each account's BOC individually with this query.
{% endhint %}

```graphql
query{
  accounts(
    filter:{
    	id:{
        in:[
          "0:001b3abc5f9e906990c2eee7a1664be20b1b47fdf5c140331e6003786735f453",
          "0:2ef022951ae41da58f16f5e3f10d8660c919c13304723a401050fb02027301f6",
          "0:34b83eee15f43580261c4ec654c2b03dcb2b8a99ab0b5257105be815cf040c6b",
          "0:684c7604fcc86bece98136f83c8370bd73feb574c11b27b2f03b2a53c778230c",
          "0:8f0be7f1e442ad576785c9b77dbe3ffb362260e828736957158449802397a48e"
        ]
      }
  	}
  ){
    id
    balance(format:DEC)
    last_paid
    last_trans_lt
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

To get account events, we need to get Account's external outbound message. Their type is `ExtOut.` `Body` field contains ABI-encoded information with Event data. You can parse it with SDK function [`abi.decode_message_body`](https://docs.everos.dev/ever-sdk/reference/types-and-methods/mod\_abi#decode\_message\_body).

```graphql
query{
  blockchain{
    account(address:"0:454abb3c7db044603a9fb0802d3c6507b08d6b04855baa9a60802d9ecd34edad"){
      messages(msg_type:[ExtOut],first:2){
        edges{
          node{
            hash
            body
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
                "hash": "315ca96ae9ded116b98692491d8accf1e01acd48e85b1db53b63615cd37f433b",
                "body": "te6ccgEBAQEAeQAA7VuEb3wAAAAAAAAAyWDwxNxg8kTcAACAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABg78TcCAUAAT1bfCW7sAABPW5DHEo+AAAAAAAAAAAAAAAIs3ySDgAAAAEAAHA+pB7Z9IAAAAAAAAAAAAAAAAAAAABA",
                "created_at_string": "2021-07-17 03:00:34.000"
              },
              "cursor": "59838dc005df77c11120003"
            },
            {
              "node": {
                "hash": "b54d2053c965cf2e41e265fd67a0b71a896fd06df8c305d1cb95da6780947113",
                "body": "te6ccgEBAQEALAAAU0UWNxJg8sTcn/M7F8rhuMCstb3zywTdpGh9GrmjYzToX9gcm8mXZJpVcA==",
                "created_at_string": "2021-07-17 03:00:34.000"
              },
              "cursor": "59838dc005df77c11120d01"
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

### Account external calls

If you want to collect external calls of an account, filter by msg\_type = `ExtIn`. `Body` field contains ABI-encoded information with Event data. You can parse it with SDK function [`abi.decode_message_body`](https://docs.everos.dev/ever-sdk/reference/types-and-methods/mod\_abi#decode\_message\_body). Lets get the last external call:

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

### Transfers between 2 accounts

In this example we retrieve last 30 messages between elector contract and some validator wallet with value more than some number:

```graphql
query{
  blockchain{
    account(address:"-1:3333333333333333333333333333333333333333333333333333333333333333"){
      messages(last:30, counterparties:["-1:99392dea1c5035feddb1bb3db9e71138d82868f7460c6da3dca26f0520798ebd"],
       min_value:"58579566000" ){
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
          hasPreviousPage
        }
      }
    }
  }
}
```

Result:

```graphql
{
  "data": {
    "blockchain": {
      "account": {
        "messages": {
          "edges": [
            {
              "node": {
                "src": "-1:3333333333333333333333333333333333333333333333333333333333333333",
                "dst": "-1:99392dea1c5035feddb1bb3db9e71138d82868f7460c6da3dca26f0520798ebd",
                "id": "message/958ee60bb2233e9e94d6c36465c0941632535d9dd1f9cb8e6b67616f1d33959e",
                "hash": "958ee60bb2233e9e94d6c36465c0941632535d9dd1f9cb8e6b67616f1d33959e",
                "value": "1625586165251876",
                "msg_type": 0,
                "created_at_string": "2022-03-15 18:12:16.000"
              },
              "cursor": "5edf8c1m0e01"
            },
          .....
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

We see that previous page exists and can continue pagination.

## Subscribe for accounts state updates (soon)

You can subscribe for accounts metadata updates: balances or/and last\_trans\_lt, for example Whenever account is updated, you will receive its data:

```graphql
subscription{
  accounts(
    filter:{
      id:{
        in:[
        "-1:3333333333333333333333333333333333333333333333333333333333333333",
        "0:557957cba74ab1dc544b4081be81f1208ad73997d74ab3b72d95864a41b779a4"
        ]
      }
    }
  ){
    id
    balance(format:DEC)
    last_trans_lt
    last_paid
  }
}
```

## Subscribe for accounts transactions (soon)

You can subscribe for transactions of a list of accounts

```graphql
subscription{
  transactions(
    filter:{
      account_addr:{
        in:[
          "-1:3333333333333333333333333333333333333333333333333333333333333333",
          "0:557957cba74ab1dc544b4081be81f1208ad73997d74ab3b72d95864a41b779a4"
        ]
      }
    }
  ){
    id
    lt
    balance_delta
  }
}
```

## Subscribe for accounts messages (soon)

You can subscribe for messages of a list of accounts.

You can even subscribe only for external outbound messages, like here:

```graphql
subscription{
  messages(
    filter:{
      src:{
        in:[
          "-1:3333333333333333333333333333333333333333333333333333333333333333",
          "0:557957cba74ab1dc544b4081be81f1208ad73997d74ab3b72d95864a41b779a4"
        ]
      }
      msg_type: {
        eq:2
      }
    }
  ){
    id
    msg_type
    src
    dst
    value
  }
}
```
