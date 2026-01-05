# Query Collection

## When you may need collections?

If you want to apply some custom filters and sortings on the data.

## About collections

There are a few collections with blockchain data:

* _accounts_: blockchain account data;
* _transactions_: transactions related to accounts;
* _messages_: input and output messages related to transactions;
* _blocks_: blockchain blocks.

[Use `query_collection` method to query data that can be filtered and sorted](/broken/pages/bdX3yGskXJEFBTwTxP5Y#query_collection).

## Usage

```javascript
await client.net.query_collection({
    collection: 'accounts',
    filter: {
        id: {
            eq: wallet1Address
        }
    },
    result: 'balance'
})
```

## Sample source code

**Core**

[https://github.com/tonlabs/sdk-samples/tree/master/core-examples/node-js/query](https://github.com/tonlabs/sdk-samples/tree/master/core-examples/node-js/query)
