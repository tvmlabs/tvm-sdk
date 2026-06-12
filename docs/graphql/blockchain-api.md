---
description: Overview of the GraphQL blockchain root type and cursor-based pagination.
status: stable
product: graphql
audience: app-developer
last_verified: 2026-06-11
---

# Blockchain API

`blockchain` root type is API that includes such basic real-time data as:

* blocks
* transactions
* account data
  * account info
  * account transactions
  * account messages

This API is natively Graph-oriented API .

We followed GraphQL best practices and implemented Relay Cursor Connections Specification for pagination for all lists. You can read more here [https://relay.dev/graphql/connections.htm](https://relay.dev/graphql/connections.htm)
