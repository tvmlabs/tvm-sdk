# Query Collections: Query Language

## Content table

* [What is a collection](query-collections-query-language.md#what-is-a-collection)
* [Query a collection](query-collections-query-language.md#query-a-collection)
* [Subscription (soon)](query-collections-query-language.md#subscription-soon)
* [Filtration](query-collections-query-language.md#filtration)
  * [Scalar filters](query-collections-query-language.md#scalar-filters)
  * [Array filters](query-collections-query-language.md#array-filters)
  * [Structure filters](query-collections-query-language.md#structure-filters)
* [OR operator](query-collections-query-language.md#or-operator)
* [Nested field](query-collections-query-language.md#nested-fields)
* [Sorting and limiting](query-collections-query-language.md#sorting-and-limiting)
* [Working with u64 and u128 numbers](query-collections-query-language.md#working-with-u64-and-u128-numbers)
  * [U64String](query-collections-query-language.md#u64string)
  * [U1024String](query-collections-query-language.md#u1024string)
  * [GraphQL interaction](query-collections-query-language.md#graphql-interaction)

## What is a collection

Collection is a data set of one document type.

There are 4 types of documents:

* blocks
* accounts
* transactions
* messages

All TVM Platform products provide query, analytics and subscription functionality for blocks, transactions, messages. And full collection of accounts.

```graphql
query{
    # Query collections
    accounts
    blocks
    transactions
    messages
}

# soon
subscription{
    # Subscribe to data updates
    accounts
    blocks
    transactions
    messages
}
```

## Support in Client Libraries

**`tvm-sdk`** provides wrappers for convenient work with collections from applications: `net.query_collection`, `net.subscribe_collection (soon).`

## Query a collection

Account collection query sample that returns the specified account's balance

```graphql
query {
  accounts(
    filter: {
      id: {eq: "0:1111111111111111111111111111111111111111111111111111111111111111"}
    }
  ) {
    id
    balance
  }
}
```

To perform a query over a collection, choose a collection and result projection. **Optionally** specify a filter, sorting order and the maximum number of items in the results list.

```graphql
query {
  transactions(
    filter: {
      orig_status: {ge: 0}
    }
    orderBy: {path:"now",direction:DESC}
    limit :5
  )
  {
    id
    now
  }
}
```

The example above demonstrates a query to the `transactions` collection with the following parameters:

* `filter`: a JSON object matching the internal collection structure. It supports additional conditions for particular fields. In the example above, the `orig_status` field of a transaction must be equal 0 (uninit).
* `orderby`: sort by "now" field in DESC order
* `limit`: show only top 5 objects
* `result`: is a result projection that determines structural subset used for returning items. In the example above the request is limited to two fields: `id` and `now`. Note that results have to follow GraphQL rules.

Read more about filtration, sorting and limiting below in this section.

## Subscription (Soon)

In this example, we start a subscription and get a result whenever a new block is generated.

```graphql
subscription{
  blocks{
    id
  }
}
```

The `filter` and `result` parameters are the same as in the `query` method. The `filter` parameter narrows the action down to a subset of monitored items. In this case, the filter is empty: all items are included into monitoring.

## Filtration

Filters applied to query functions are data structures matching collection item with several extra features:

* The value for scalar fields (e.g. strings, numbers etc.) is a structure with the `scalar filter`.
* The value for array fields is a structure with an `array filter`.
* The value for nested structures is a filter for `nested structure`.

These filter types will be described in more details below in this section.

> Filtration applies only to collection query types

### Scalar filters

Scalar filter is a structure with one or more predefined fields. Each field defines a specific scalar operation and a reference value:

* `eq`: item value must be equal to the specified value;
* `ne`: item value must not be equal to the specified value;
* `gt`: item value must be greater than the specified value;
* `lt`: item value must be less than specified value;
* `ge`: item value must be greater than or equal to the specified value;
* `le`: item value must be less than or equal to the specified value;
* `in`: item value must be contained in the specified array of values;
* `notIn`: item value must not be contained within the specified array of values.

Scalar filter example 1

```graphql
filter: {
    id: { eq: 'e19948d53c4fc8d405fbb8bde4af83039f37ce6bc9d0fc07bbd47a1cf59a8465'},
    status: { in: [0, 1, 2] }
}
```

Scalar filter example 2

```graphql
filter: {
    now: { gt: 1563449, lt: 2063449 }
}
```

The logic from the above snippet can be expressed in the following way:

```graphql
(transaction.now > 1563449) && (transaction.now < 2063449)
```

### Array filters

Array filters are used for array (list) fields. Each has to contain at least one of the predefined operators:

* `any`: used when at least one array item matches the nested filter;
* `all`: used when all items matches the nested filter.

The `any` or `all` must contain a nested filter for an array item.

Array operators are mutually exclusive and can not be combined. For empty arrays, the array filter is assumed to be false.

### Structure filters

If an item is a structure, then a filter has to contain fields named as fields of this item. Each nested filter field contains a condition for the appropriate field of an item. The `AND` operator is used to combine conditions for several fields.

## OR operator

You can combine several struct filters over collection with logical OR in a single query. Just specify `OR` field in collection struct filter.

Determine all messages related to the specified account:

```graphql
query {
  messages(
  filter:{
    src: { eq: "0:a52f6a7ea6bc7279728cbff01ad1e8b1dfc386098cfac1f381ae3959bf2ae9db" },
    OR: 
    {
       dst: { eq: "0:a52f6a7ea6bc7279728cbff01ad1e8b1dfc386098cfac1f381ae3959bf2ae9db" }
    }
})
  {
    id
    src
    dst
    value
  }
}
```

Request messages of myAcc or messages with value more than 10000 nG (combine several `OR` operators) :

```graphql
query {
  messages(
  filter:{
    src: { eq: "0:a52f6a7ea6bc7279728cbff01ad1e8b1dfc386098cfac1f381ae3959bf2ae9db" },
    OR: 
    {
        dst: { eq: "0:a52f6a7ea6bc7279728cbff01ad1e8b1dfc386098cfac1f381ae3959bf2ae9db" },
        OR: 
        {
               value: { gt: "10000" }
        }
    }
})
  {
    id
    src
    dst
    value
  }
}
```

## Nested fields

For example, the _transactions_ collection has the `in_message` field that stores the relevant message item. The message item exists in _messages_ collection and has the `id` value equal to the `in_msg` value in _transactions_. Block join is present in Messages and Transactions collections.

## Sorting and limiting

> Sorting and limiting applies only to collection query types

By default, retrieval order for several items is not defined. To specify it, use the `orderBy` parameter of `query` method.

The sort order is represented by an array or sort descriptors. These structures contain two fields: `path` and `direction`:

* `path` specifies a path from a root item of the collection to the field that determines the order of return items. The path includes field names separated by dot.
* `direction` specifies the sorting order: ASC or DESC (ascending and descending).

You can specify more than one field to define an order. If two items have equal values for the first sort descriptor, then second descriptor is used for comparison, etc. If values of sorting fields are the same for several items, then the order of these items is not defined.

The `limit` parameter determines the maximum number of items returned. This parameter has a default value of 50 and can not exceed it. If specified limit exceeds 50, 50 is used.

## Working with u64 and u128 numbers

All the numbers larger than 2^32 are stored as hexadecimal strings with a string length prefix as defined below.

### U64String

All number types in range (2^32 ... 2^64) are encoded as a string using the following format:

```
"MN...N"
```

where:

* `M` – one char with hex (length-1) of hexadecimal representation of a number.
* `N...N` – hexadecimal lowercased representation of a number.

Number examples:

* `11` – 1
* `12` – 2
* `1a` – 10
* `2ff` – 255
* `fffffffffffffffff` - 0xffffffffffffffff = 2^(2 \* 16)-1 = 2^32-1

### U1024String

All number types in range (2^64 ... 2^1024] are encoded as a string using the following format:

```
"MMN...N"
```

where:

* `MM` – two chars with hex (length-1) of hexadecimal representation of a number.
* `N...N` – hexadecimal lowercased representation of a number.

Number examples:

* `011` – 1
* `012` – 2
* `01a` – 10
* `02ff` – 255
* `ffff..ff` - 2^(2 \*256) - 1 = 2^512 - 1

### GraphQL interaction

Within the GraphQL filter fields these numbers can be represented as follows:

1. Hexadecimal number string starting with a `0x` prefix for example `0x10f0345ae`. Note that you can specify characters for hexadecimal numbers in any letter case, for example `0xa4b` is the same as a `0xA4B`.
2. Decimal number representation, for example `100034012`.

GraphQL always returns large numbers as a hexadecimal number string starting with a `0x` prefix; for example `0xa34ff`. Note that GraphQL always returns characters in lower case.

To interact with large numbers in GraphQl one needs to use `BigInt(value)` where `value` can be both hexadecimal with `0x` prefix or a decimal number.
