---
description: GraphQL info query for API version and health parameters.
status: stable
product: graphql
audience: app-developer
last_verified: 2026-06-11
---

# Info API

Info query is used to get API version, as well as health parameters of the API, such as latency of blocks, messages and transactions

```graphql
query{
  info{
    version # API version
  }
}
```
