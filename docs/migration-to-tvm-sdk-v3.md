---
description: Migration guide for TVM SDK v3.0 and Acki Nacki GraphQL API v1.0.
status: stable
product: sdk
audience: app-developer
last_verified: 2026-06-11
---

# Migration to 3.0 SDK and 1.0 GQL API

TVM SDK v3.0 and updated Acki Nacki service APIs introduce **Dapp ID** support in address format. The migration affects SDK calls, `tvm-cli` tool, GraphQL queries, and Block Keeper / Block Manager REST APIs.

## What changed

Account address is now represented by two raw 64-character hex values:

```text
<dapp_id>::<account_id>
```

For self-rooted contracts, `dapp_id` equals `account_id`. Legacy `0:<account_id>` values are not enough for Dapp ID-aware routing and must be converted before they are passed to SDK, CLI, GraphQL, or REST APIs.

## SDK changes

SDK consumers must pass Dapp ID  explicitly in message processing parameters.

Key changes:

* `account.get_account` returns account BOC, optional `dapp_id`, and `state_timestamp`.
* `ParamsOfSendMessage` includes `thread_id` and a required `dapp_id`.
* `ParamsOfProcessMessage` includes a required `dapp_id`.
* `ResultOfSendMessage` returns `message_hash`, `block_hash`, `tx_hash`, execution result fields, `thread_id`, producers list, and response time.

Pass `dapp_id` as a raw 64-character hex string without the `0:` prefix.

## CLI changes

Commands that take an address argument require the extended address format:

```text
<dapp_id>::<account_id>
```

`call`, `callx`, and proposal commands derive destination `dapp_id` from extended addresses. `deploy` and `deployx` always require `--dst-dapp-id`. Scripts and aliases that still pass `0:<account_id>` must be updated before using SDK/CLI 3.0.

## Detailed SDK migration guide

{% hint style="info" %}
This page is an overview. Use the detailed [MIGRATION-3.0.md](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/MIGRATION-3.0.md) guide for exact API changes, CLI examples, known errors, and the migration checklist.
{% endhint %}

Before upgrading, review the guide and plan updates for SDK calls, CLI scripts, stored addresses, deployment outputs, and language bindings that expose the TVM SDK JSON API. Stored legacy `0:<account>` values must be converted before they are passed to current `tvm-cli` commands. The same applies to hard-coded GraphQL queries and REST URLs: strip the `0:` prefix and supply a separate `dapp_id`.

## GraphQL API changes

Use `blockchain.account` with separate `account_id` and `dapp_id` arguments.

Before:

```graphql
query {
  blockchain {
    account(address: "0:abcdef...") {
      info { boc }
    }
  }
}
```

After:

```graphql
query {
  blockchain {
    account(
      account_id: "abcdef..."
      dapp_id: "cba987..."
    ) {
      info { boc }
    }
  }
}
```

## REST API changes

### `GET /v2/account`

The endpoint now requires `account_id` and `dapp_id` query parameters and rejects `address=0:...`.

Before:

```http
GET /v2/account?address=0:abcdef...
```

After:

```http
GET /v2/account?account_id=<64hex>&dapp_id=<64hex>
```

Validation behavior:

| Condition                     | Response                                                         |
| ----------------------------- | ---------------------------------------------------------------- |
| Missing `account_id`          | `400 account_id parameter required`                              |
| Missing `dapp_id`             | `400 dapp_id parameter required`                                 |
| Prefixed (`0:`) or not 64 hex | `400 Invalid <field>: expected 64 hex characters without prefix` |
| Account not found             | `404`                                                            |

Success response:

```json
{ "boc": "...", "account_id": "<hex>", "dapp_id": "<hex>", "state_timestamp": 1710000000000 }
```

### `POST /v2/messages`

Each external message must include both `account_id` and `dapp_id`:

```json
[
  {
    "id": "...",
    "body": "...",
    "account_id": "<64hex>",
    "dapp_id": "<64hex>",
    "thread_id": "..."
  }
]
```

Requests missing `account_id` or `dapp_id` are rejected with `400`. Response `result` and `error.data` objects also carry `account_id` and `dapp_id`.

## Compatibility window

Support for these migration-related changes must be implemented before the release of the new node version with v0.16.3 release.

Until that release, integrations should support both:

* Legacy nodes with `GraphQL info.version < "1.0.0"`
* Dapp ID-aware nodes with `GraphQL info.version >= "1.0.0"`

Treat nodes reporting `GraphQL info.version >= "1.0.0"` as requiring separate `account_id` and `dapp_id` fields across SDK, GraphQL, and REST flows.
