# Transactions

## Get transaction info by hash

```graphql
query{
  blockchain{
    transaction(hash:"b0e26c42164ec0137913fdcd754aa819323a6a4b9ef5188863b021c3801e7ae4"){
      id
      hash
      balance_delta
      aborted
      lt
      now
    }
  }
}
```

Result:

```graphql
{
  "data": {
    "blockchain": {
      "transaction": {
        "id": "transaction/b0e26c42164ec0137913fdcd754aa819323a6a4b9ef5188863b021c3801e7ae4",
        "hash": "b0e26c42164ec0137913fdcd754aa819323a6a4b9ef5188863b021c3801e7ae4",
        "balance_delta": "0x0",
        "aborted": false,
        "lt": "0x15bb39a23783",
        "now": 1645453010
      }
    }
  }
}
```

## Get transaction by inbound message hash

```graphql
query{
  transactions(filter:{
    in_msg:{
      eq:"d158f7437080f6835792ac0ef9cccbcbd2874a63e8f7f1db9dcb97edc06f410d"
    }
  })
  {
    id
    account_addr
    balance_delta(format:DEC)
    now
    now_string
  }
}


```

Result:

```graphql
{
  "data": {
    "transactions": [
      {
        "id": "6e87500efde682124e92f58769b7578fa546f22d46aa844249b51e6c8b63fce4",
        "account_addr": "0:802f5476cc99425184757c0672fe7737e94b3cbc2cee59e156dfe61c0c0c630c",
        "balance_delta": "-5110030",
        "now": 1682457905,
        "now_string": "2023-04-25 21:25:05.000"
      }
    ]
  }
}
```

## Calculate account fees for transaction

```graphql
query{
  blockchain{
    transaction(hash:"998fee062e8daad96e88ce43adb52832b2e653d9a824912bc83051060932aceb"){
      ext_in_msg_fee(format:DEC)
      storage{
        storage_fees_collected(format:DEC)
      }
      compute{
        gas_fees(format:DEC)
      }
      action{
        total_fwd_fees(format:DEC)
      }      
    }
  }
}
```

You need to sum up these values to get the total fee the account paid for the transaction

```graphql
{
  "data": {
    "blockchain": {
      "transaction": {
        "ext_in_msg_fee": "2062000",
        "storage": {
          "storage_fees_collected": "270"
        },
        "compute": {
          "gas_fees": "10741000"
        },
        "action": {
          "total_fwd_fees": null
        }
      }
    }
  }
```

## Paginate blockchain transactions

Paginate workchain transactions within timestamp range, sorted by blockchain logical time.

Look at this sample. See the parameters documentation below.

```graphql
query{
  blockchain{
      transactions(
        master_seq_no_range: {
          start: 1725467896
          end: 1725554296
      }
      ){
        edges{
          node{
            id
            now
          }
          cursor
        }
        pageInfo{
          startCursor
          hasNextPage
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
      "transactions": {
        "edges": [
          {
            "node": {
              "id": "transaction/d5ec03df890727a90eb80960560664ca49d9842e0a9f6f3c188ad1a49fca7264",
              "now": 1647429363
            },
            "cursor": "528cc96m05"
          },
          {
            "node": {
              "id": "transaction/39c2c3174aa414003bc55e68b6a08710cbcf6ce1b71dec1a0c8962fdcef5fc76",
              "now": 1647429363
            },
            "cursor": "528cc96m06"
          },
          {
            "node": {
              "id": "transaction/fa674f4c537aeaac951ebe4fd9eb2a1d094aae26d611f126652bee92827f3ed2",
              "now": 1647429363
            },
            "cursor": "528cc96m07"
          }
        ],
        "pageInfo": {
          "startCursor": "528cc96m05",
          "hasNextPage": true
        }
      }
    }
  }
}
```

### Filter parameters

You can filter by

* `master_seq_no_range : {start: Timestamp, end: Timestamp} - start and end timestamp for pagination range`
* `max_balance_delta`: String
* `min_balance_delta`: String

### Pagination parameters

Use `cursor`, {`first`, `after`} or {`last`, `before`} filters for pagination.

{% hint style="success" %}
We followed GraphQL best practices and implemented Relay Cursor Connections Specification for pagination for all list types. You can read more here [https://relay.dev/graphql/connections.htm](https://relay.dev/graphql/connections.htm)
{% endhint %}

Use end`Cursor` and `hasNextPage` == true condition to paginate forward.

Use `startCursor` and `hasPreviousPage` == true condition to paginate backwards like this:

```graphql
query{
  blockchain{
      transactions(
        before: "528cc96m05"
      ){
        edges{
          node{
            id
            now
          }
          cursor
        }
        pageInfo{
          startCursor
          hasPreviousPage
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
      "transactions": {
        "edges": [
          {
            "node": {
              "id": "transaction/2ddaa133fb753c34d72bbae1b9040b773d2b9360612874da5fce19bcdd4b37a1",
              "now": 1647429363
            },
            "cursor": "528cc96m02"
          },
          {
            "node": {
              "id": "transaction/051363b194ebf6cf0ecab31643b34254aef7ec859d027211c0701df9046ad0bc",
              "now": 1647429363
            },
            "cursor": "528cc96m03"
          },
          {
            "node": {
              "id": "transaction/d7b764cd13e5b945aff381e45d1b2b855bf81444950f6b83eb355ac9b2d53ef5",
              "now": 1647429363
            },
            "cursor": "528cc96m04"
          }
        ],
        "pageInfo": {
          "startCursor": "528cc96m02",
          "hasPreviousPage": true
        }
      }
    }
  }
}
```

## How to get other transactions data

Acki Nacki Node is not a simple node with key-value storage, but a node with an embedded SQL DB, so using SQL queries you can get any other data and analytics. Use DB UI like Beaver to have manual access to this data
