# Messages

## Get message info by hash

```graphql
query{
  blockchain{
    message(hash:"6669e01217c9e85673961bd3714d90fa7b65e2b9d1a50a4720576b1b30551f9e"){
      id
      src
      dst
      value(format:DEC)
      msg_type_name
      created_at
      # check other available fields in the schema in playground
    }
  }
}
```

Result

```graphql
{
  "data": {
    "blockchain": {
      "message": {
        "id": "6669e01217c9e85673961bd3714d90fa7b65e2b9d1a50a4720576b1b30551f9e",
        "src": "0:1111111111111111111111111111111111111111111111111111111111111111",
        "dst": "0:20fcf0b46bc179bd6ad7ac2c3a303ec94dca9a1a2f421e2c35ebff55db35bdc4",
        "value": "10000000000000",
        "msg_type_name": "Internal",
        "created_at": 1786776396
      }
    }
  }
}
```

`src` and `dst` are addresses in the `0:<account_id>` form. To read one of these accounts with
`blockchain.account`, drop the `0:` prefix and pass the account id together with its `dapp_id`.
