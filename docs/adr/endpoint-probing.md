# ADR: Endpoint version probing and v2/v3 fallback policy

- **Status:** Superseded by [`v3-only-wire-format.md`](v3-only-wire-format.md) (2026-08-27)
- **Date:** 2026-07-07
- **Scope:** `tvm_client` network layer (`net/endpoint.rs`, `net/server_link.rs`,
  `account/mod.rs`) and the `tvm-cli` address parser (`tvm_cli/src/helpers.rs`)

> **Superseded.** NODE-3675 removed support for pre-1.0.0 servers, so there is
> no longer a wire format to choose and `info.version` is no longer consulted.
> Everything below describes the behaviour up to that change and is kept as a
> record of why the probing worked the way it did.

## Context

Acki Nacki nodes expose two API surfaces:

- **GraphQL** at `<endpoint>/graphql` — present on legacy (pre-dapp_id) nodes
  and on most current nodes; its `info { version }` field is the authoritative
  server-version signal.
- **REST** under `<endpoint>/v2/` — used for `account` reads and `messages`
  sends. The *URL path is always `/v2/`*; the "v2 vs v3" distinction is the
  **request wire format** (payload/query-string shape), not a URL version
  segment. There is no `/v3/` URL and no REST-level probing.

The production fleet is mixed: behind the mainnet load balancer some
block-managers are v3-compatible (`info.version >= "1.0.0"`, dapp_id API) and
some are v2-only, and some REST-only nodes expose no `/graphql` at all (a GET
there returns 404). The canonical description of the networks and their node
mix is `docs/networks.md` in the `acki-nacki` repository
(`gosh-sh/acki-nacki`).

The client must therefore decide, per operation, which wire format to use —
without hanging when `/graphql` is missing and without misclassifying a
v2-only node. This decision was re-debated repeatedly; this ADR freezes what
the code actually does.

## Decision

### 1. Version signal: a single fail-fast GraphQL `info` probe

- Endpoint addresses are normalized to `<scheme>://<host>[:port]/graphql`
  (scheme defaults to `https`, except `localhost`/`127.0.0.1`/`0.0.0.0` which
  get `http`) — `tvm_client/src/net/endpoint.rs:91-106`.
- The probe is a GET of `?query={info{version time latency rempEnabled}}`
  with `NetworkConfig.query_timeout` — `endpoint.rs:64`, `endpoint.rs:108-135`,
  `endpoint.rs:137-165`. HTTP 401 is surfaced as an `Unauthorized` error
  (`endpoint.rs:129-131`); any other failure (404 on a REST-only node,
  connection refusal, timeout) is just a resolve error.
- `info.version` is parsed into `major*1_000_000 + minor*1_000 + patch`
  (`endpoint.rs:185-207`). **"Server speaks the v3 dapp_id format" :=
  `version >= 1.0.0`** — `ServerLink::supports_dapp_id`,
  `tvm_client/src/net/server_link.rs:1310-1314`. An unresolved endpoint
  reports version `0` and is treated as pre-1.0.0
  (`server_link.rs:1300-1308`).
- Version detection uses `NetworkState::try_resolve_query_endpoint`
  (`server_link.rs:367-394`): **single attempt per configured address, no
  reconnect/retry loop** (unlike `get_query_endpoint`,
  `server_link.rs:347-365`, which retries up to `max_reconnect_timeout`).
  This is deliberate: a missing `/graphql` must fail fast, not hang, and must
  never turn into a hard `code 11: Server responded with code 404` error
  (see the comment at `server_link.rs:1032-1046`).

### 2. Probe result caching

- A successful probe caches the resolved endpoint in
  `NetworkState.query_endpoint` (`server_link.rs:386-388`); subsequent calls
  (`get_query_endpoint`, `server_version`, `supports_dapp_id`) reuse it
  without re-probing (`server_link.rs:375-381`).
- The cache is cleared on network suspend (`server_link.rs:167-171`) and by
  `invalidate_querying_endpoint` (`server_link.rs:266-268`); a **failed**
  probe is *not* cached — each operation that needs the version re-probes
  until one succeeds.
- Within one `send_message` attempt (including all its retries) the verdict
  is sampled **once**: all retries go to nodes of the same cluster, so a
  mid-attempt version flip is not expected (`server_link.rs:1044-1046`).
- (A separate 10-minute `resolved_endpoints` cache exists,
  `server_link.rs:61`, but its reader `get_resolved_endpoint` is currently
  `#[allow(dead_code)]`, `server_link.rs:405-412`.)

### 3. Per-operation fallback policy

The two REST operations resolve "which wire format" differently — this
asymmetry is intentional.

**`net.send_message` (POST `<endpoint>/v2/messages`,
`server_link.rs:1032-1119`):**

1. Probe GraphQL via `try_resolve_query_endpoint` (fail-fast).
2. Probe **succeeds** → trust `info.version`: `>= 1.0.0` sends the v3 wire
   format (`ExtMessageV3`: `dapp_id` + `account_id`,
   `net/tvm_gql.rs:91-100`), `< 1.0.0` sends v2 (`ExtMessageV2`:
   optional `dst_dapp_id`, `tvm_gql.rs:72-81`).
3. Probe **fails** (no GraphQL reachable) → **assume v3, no v2 fallback**
   (`server_link.rs:1047-1050`). Rationale: legacy `< 1.0.0` nodes always
   expose GraphQL, so a REST-only node is by definition a v3 node; and the
   request payload cannot disambiguate because the SDK always has a dapp_id
   (see §4).
4. The send never hard-fails on a missing `/graphql`; only the REST call
   itself can fail. Retries handle `WRONG_PRODUCER` / `THREAD_MISMATCH` /
   `TOKEN_EXPIRED` redirection, still on `/v2/messages`
   (`server_link.rs:1137-1197`).

**`account.get_account` (GET `<endpoint>/v2/account`,
`tvm_client/src/account/mod.rs:62-95`):**

1. Probe GraphQL (same fail-fast call, `account/mod.rs:68`).
2. Probe says **v3** → v3 query form directly:
   `?account_id=<hex64>&dapp_id=<hex64>` (`account/mod.rs:87-94`).
3. Probe says **v2** (authoritative `< 1.0.0` verdict) → legacy form
   `?address=0:<account_id>`; **an error from this call is trusted and
   returned as-is — no v3 retry** (`account/mod.rs:71-79`).
4. Probe **failed** (GraphQL unavailable) → try the **v2 form first**; if the
   server rejects it, **retry with the v3 form** (`account/mod.rs:80-84`).
   The comment marks this v2→v3 fallback as transitional, to be dropped once
   the whole fleet serves v3 (`account/mod.rs:67`).

Summary table:

| GraphQL probe result | `send_message` | `get_account` |
|---|---|---|
| reachable, `>= 1.0.0` | v3 wire format | v3 query form |
| reachable, `< 1.0.0` | v2 wire format | v2 form; its errors are final |
| unreachable (404/refused/timeout) | v3, no fallback | v2 first, on rejection retry v3 |

### 4. dapp_id / address format requirements

Version detection interacts with address handling as follows:

- **tvm-cli accepts only the new `dapp_id::account_id` address form** (two
  64-hex halves, no `0x`, no workchain). Legacy `0:…`, bare-hex,
  single-colon and 128-hex forms are rejected with a loud parse error
  (`tvm_cli/src/helpers.rs:281-303`). Consequently a CLI-originated request
  always carries a dapp_id, which is why the request payload cannot be used
  to distinguish a v2-only node (`server_link.rs:1038-1043`).
- In the SDK, `send_message` with an **empty** dapp_id against a v3 (or
  assumed-v3) server fails with `ErrorCode::DappIdRequired`
  (`server_link.rs:1053-1055`,
  `tvm_client/src/processing/errors.rs:237-242`). Against a v2 server an
  empty dapp_id is allowed and serialized as legacy `dst_dapp_id: null`
  (`server_link.rs:1056-1058`, `1110-1118`).
- `get_account` validates `dapp_id` as strict 64-hex only on the v3 path
  (`account/mod.rs:88`); the v2 path never touches it.
- Migration details for API consumers: `docs/MIGRATION-3.0.md`.

## Consequences

- A missing `/graphql` (HTTP 404) is a *classification signal*, not an
  error: the node is treated as REST-only v3. Sends and reads keep working.
- A v2-only node is detected **only** via its GraphQL `info.version`; there
  is no REST-based v2 detection for sends. If a hypothetical node were both
  GraphQL-less *and* v2-only, `send_message` would misclassify it as v3 —
  accepted, because such nodes do not exist in the fleet (legacy nodes always
  expose GraphQL).
- The first successful probe pins the version for the client's lifetime
  (until suspend/invalidation). Rolling fleet upgrades behind one LB can
  therefore serve mixed answers to different clients, but each client stays
  self-consistent.
- Legacy address input is a hard, descriptive CLI error — users must migrate
  to `dapp_id::account_id`.

## Open questions / deviations from earlier discussions

- Earlier discussions described one universal "GraphQL first, fallback v2,
  then v3" ladder. The implemented policy is **per-operation**: the full
  v2→v3 ladder exists only in `get_account`; `send_message` jumps straight
  to v3 when GraphQL is unavailable. Do not "unify" this without reading the
  rationale comments at `server_link.rs:1032-1050` and `account/mod.rs:62-67`.
- The `get_account` v2→v3 retry is explicitly transitional
  (`account/mod.rs:67`); removal is pending full fleet v3 rollout.
- `account_id` serialization in the v3 send path assumes a 256-bit
  `AddrStd` destination; `AddrVar` with non-aligned bit lengths may fail
  server-side validation (`server_link.rs:1094-1098`, NODE-3500 follow-up).
- The 10-minute `resolved_endpoints` cache reader is dead code
  (`server_link.rs:405-412`); either wire it up or remove it.
