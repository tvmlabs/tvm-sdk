---
description: Inspect live Acki Nacki GraphQL schemas for Shellnet and Mainnet.
status: stable
product: graphql
audience: app-developer
task: inspect-graphql-schema
last_verified: 2026-06-11
---

# Inspect GraphQL Schema

## Goal

Find the current GraphQL fields, arguments, enums, and root types before generating a query.

## Endpoints

| Network | Endpoint |
| --- | --- |
| Shellnet | `https://shellnet.ackinacki.org/graphql` |
| Mainnet | `https://mainnet.ackinacki.org/graphql` |

## Root Type Query

```graphql
query {
  __schema {
    queryType {
      name
    }
    mutationType {
      name
    }
    subscriptionType {
      name
    }
  }
}
```

## Type Lookup Query

```graphql
query {
  __type(name: "Query") {
    fields {
      name
      args {
        name
        type {
          kind
          name
          ofType {
            kind
            name
          }
        }
      }
    }
  }
}
```

## Notes

Do not store copied schemas in this repository. The live GraphQL servers are the source of truth.

## Related Docs

* [GraphQL Schema for AI Agents](../graphql/graphql-schema-for-ai-agents.md)
* [GraphQL Quick Start](../graphql/graphql-quick-start.md)
