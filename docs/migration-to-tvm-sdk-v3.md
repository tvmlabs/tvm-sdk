# Migration to TVM SDK v3

TVM SDK v3.0 introduces first-class support for **Dapp ID** in Acki Nacki. A Dapp ID identifies a decentralized contract system and is required when working with v3-compatible nodes (`GraphQL info.version >= "1.0.0"`).

The SDK remains backward-compatible with legacy nodes, but several public APIs and CLI flows have changed. Developers must now pass `account_id` and `dapp_id` explicitly in places where a legacy `0:<account>` address was previously enough.

## What changed

The main migration change is the new extended address format:

```
<dapp_id>::<account_id>
```

For self-rooted contracts, `dapp_id` equals `account_id`, so the address uses the same 64-character hex value on both sides.

TVM SDK v3.0 also updates Rust SDK parameters, JSON bindings, and `tvm-cli` commands to use `dapp_id` as routing context. For CLI commands, the extended address form is now required on all nodes, including legacy nodes.

## Who should read this guide

Read this guide if you:

* Upgrade `tvm_client` or `tvm-cli` to version `3.0.0` or later
* Use Rust, JavaScript, Python, or generated bindings based on the TVM Client JSON interface
* Call `get_account`, `send_message`, or `process_message`
* Use `tvm-cli account`, `deploy`, `deployx`, `call`, `callx`, `send`, or `message`
* Store contract addresses in a database, config, deployment output, or aliases

## Migration guide

{% hint style="info" %}
This page is an overview. Use the detailed [MIGRATION-3.0.md](/broken/pages/4754d737b4dafcf32d321913810c928e4a2a2106) guide for exact API changes, CLI examples, known errors, and the migration checklist.
{% endhint %}

Before upgrading, review the guide and plan updates for SDK calls, CLI scripts, stored addresses, deployment outputs, and language bindings that expose the TVM SDK JSON API. Stored legacy `0:<account>` values must be converted before they are passed to current `tvm-cli` commands.

## Support window

Support for these migration-related changes must be implemented before the release of the new node version with State v2 support.

Until that release, integrations should support both:

* Legacy nodes with `GraphQL info.version < "1.0.0"`
* Dapp ID-aware nodes with `GraphQL info.version >= "1.0.0"`

Use SDK runtime capability detection mechanisms, such as `supports_dapp_id()`, for version-dependent logic instead of hard-coding behavior based on environment names or infrastructure types.
