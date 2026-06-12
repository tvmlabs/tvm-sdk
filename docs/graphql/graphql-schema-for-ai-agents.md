---
description: Live GraphQL schema endpoints for AI agents.
status: stable
product: graphql
audience: ai-agent
last_verified: 2026-06-11
---

# GraphQL Schema for AI Agents

Use this page when you need to inspect the current Acki Nacki GraphQL schema before generating or validating queries.

## Networks

| Network | Endpoint |
| --- | --- |
| Shellnet | `https://shellnet.ackinacki.org/graphql` |
| Mainnet | `https://mainnet.ackinacki.org/graphql` |

Use Shellnet for examples, tests, and tutorials. Use Mainnet only when the user explicitly asks for Mainnet data or production endpoints.

## Retrieval Rules

Treat the live GraphQL server as the source of truth for field names, argument names, enum values, and connection shapes.

Do not store copied schema files in this repository. The schema can change, and duplicating it would create a second source of truth.

Do not infer that a Shellnet-only field exists on Mainnet. Inspect the target network schema before generating a query.

When writing examples, keep the endpoint visible in the snippet so the target network is unambiguous.

## Inspecting the Schema

Open the endpoint in a browser to use the GraphQL playground and schema explorer:

* [Shellnet GraphQL playground](https://shellnet.ackinacki.org/graphql)
* [Mainnet GraphQL playground](https://mainnet.ackinacki.org/graphql)

Agents that can issue GraphQL requests may also use introspection against the selected endpoint.

Minimal introspection query for root types:

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

Type lookup example:

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
