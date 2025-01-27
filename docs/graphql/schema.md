# Schema

A schema defines a type system of GraphQL API. It describes the complete set of possible data (objects, fields, relationships, everything) that a client can access.

* Root types
* Non-root types
* Query types
* Subscription types (coming soon)
* Mutation types
* Syntax

## Root types

GraphQL schema has three root types:

* query
* mutation
* subscription  (coming soon)

The [query type](https://graphql.github.io/graphql-spec/June2018/#sec-Type-System) defines GraphQL operations that retrieve data from the server.

The [mutation type](https://graphql.github.io/graphql-spec/June2018/#sec-Type-System) defines GraphQL operations that change data on the server. It is analogous to performing HTTP verbs such as `POST`, `PATCH`, and `DELETE`. Mutations are used to send messages to the blockchain. We recommend to do it only via SDK, not directly.

The **subscription** root type – a long‐lived request that fetches data in response to source events.

Check out [TVM-SDK net module](../reference/types-and-methods/mod_net.md) - the wrapper over GraphQL API for root queries and subscriptions.

## Non-root types

See non-root type descriptions in Field descriptions section.

## Query types

**Root resolvers**

| name        | description                                                                                                                                  |
| ----------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| info        | Info query is used to get GraphQL API version, as well as health parameters of the API, such as latency of blocks, messages and transactions |
| blockchain  | API that includes a set of functions for pagination of blocks, key blocks, transactions and account’s transactions and messages.             |
| blocks      | Blocks collection                                                                                                                            |
| accounts    | Accounts collection                                                                                                                          |
| messages    | Messages collection                                                                                                                          |
| transaction | Transactions collection                                                                                                                      |

## Subscription types (Soon)

* blocks
* accounts
* messages
* transaction

## Mutation types

* postRequests - used to send messages to blockchain.

## Syntax

Read about GraphQL syntax in its [`official documentation`](https://graphql.org/). In this example we query account info:

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
    transactions(last:1){
      edges{
        node{
          hash
          now
          balance_delta
        }
      }
    }
  }
  }
}
```

Here you can see a request for account's information and the last transaction with a subset of fields. All available fields with their descriptions can be found in data schema in playground.

A selection set must contain only scalar fields, otherwise you will get an error. A scalar field describes one discrete piece of information available to a request within a selection set. If field is an object, you need to specify the fields of this object.

Read more in the next sections.
