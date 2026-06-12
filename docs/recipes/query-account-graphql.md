---
description: Query an Acki Nacki account through the GraphQL API.
status: stable
product: graphql
audience: app-developer
task: query-account-graphql
last_verified: 2026-06-11
---

# Query an Account with GraphQL

## Goal

Read account data from the Acki Nacki GraphQL API.

## Prerequisites

* You know the target network.
* You know the account address.
* For examples and tests, use Shellnet.

## Endpoint

```text
https://shellnet.ackinacki.org/graphql
```

## Query

```graphql
query {
  blockchain {
    account(address: "<YourAddress>") {
      info {
        balance(format: DEC)
        address
      }
    }
  }
}
```

## Curl Example

```bash
curl --location --request POST https://shellnet.ackinacki.org/graphql \
  --header 'Content-Type: application/json' \
  --data-raw '{"query":"query { blockchain { account(address:\"<YourAddress>\") { info { balance(format: DEC) address } } } }"}'
```

## Expected Result

The response contains account information under `data.blockchain.account.info`.

## Notes

Inspect the live schema before generating production queries, because Shellnet and Mainnet schemas may differ.

## Related Docs

* [GraphQL Quick Start](../graphql/graphql-quick-start.md)
* [Blockchain API](../graphql/blockchain-api.md)
* [GraphQL Schema for AI Agents](../graphql/graphql-schema-for-ai-agents.md)
