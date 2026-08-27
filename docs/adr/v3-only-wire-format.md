# ADR: One wire format (v3), no version probing

- **Status:** Accepted
- **Date:** 2026-08-27
- **Supersedes:** [`endpoint-probing.md`](endpoint-probing.md)
- **Scope:** `tvm_client` network and account layers, `tvm-cli` deploy commands

## Context

Acki Nacki nodes previously came in two kinds: pre-1.0.0 ("v2"), which took
`address=0:<hex>` and an optional `dst_dapp_id`, and 1.0.0+ ("v3"), which takes
`account_id` plus `dapp_id`. The SDK chose between them at runtime from the
GraphQL `info.version` field, which required probing `/graphql` before every
send and every account read, and required tolerating nodes that serve no
GraphQL at all.

The fleet is now on 1.0.0+. Keeping the branch meant keeping the probe, the
fallbacks, and a public `supports_dapp_id()` helper that told applications to
write version-conditional code of their own.

## Decision

1. **One wire format.** Every external message is `ExtMessageV3`; every account
   read uses `account_id=…&dapp_id=…`. `ExtMessageV2` is deleted.
2. **`dapp_id` is unconditionally required.** On `send_message` an empty value
   is `dapp_id_required`, raised before the network call; on `get_account`,
   and for a malformed value on either path, `validate_hex_id` refuses it
   with a hex-format error. Both checks run before any request goes out.
3. **`info.version` is not consulted, or even requested.** Both `info`
   selection sets drop the field, and `Endpoint::server_version`,
   `ServerLink::{server_version, supports_dapp_id}` and
   `ClientContext::supports_dapp_id` are removed.
4. **Neither `send_message` nor `get_account` touches GraphQL.** They read only
   in-memory endpoint state. `NetworkState::try_resolve_query_endpoint`, added
   for fail-fast version detection, is deleted with its last callers; ordinary
   GraphQL queries keep using `get_query_endpoint`.
5. **The REST path stays `/v2/`.** It is a URL segment, not a format version.
   There is no `/v3/` URL.
6. **`tvm-cli` deploys derive their own `dapp_id`.** A contract deployed
   through the CLI roots its own dapp, so `dapp_id == account_id`;
   `--dst-dapp-id` is removed from `deploy`, `deployx`, `deploy_message` and
   `fee deploy`, and kept on `send` and `sendfile`, where the destination
   comes from a prepared BOC.

## Consequences

- Sends no longer pay a GraphQL round-trip. A REST-only node needs no special
  handling, because nothing probes.
- Pre-1.0.0 nodes are unsupported. Pointing the SDK at one produces a
  server-side rejection, not a fallback.
- Applications that gated on `supports_dapp_id()` must drop the gate; there is
  no replacement and none is needed.
- Two defensive fallbacks are deliberately kept: `send_message` and
  `get_account` fill `account_id`/`dapp_id` from the request when a response
  omits them, rather than returning empty strings inside a successful result.
  Both have tests so they are not mistaken for leftovers.
- Restoring v2 means reverting this work, not flipping a flag.

## References

- Migration details for API consumers: `docs/MIGRATION-3.0.md`.
