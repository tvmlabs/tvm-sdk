# Migrating to tvm_client / tvm-cli 3.0.0 (dapp_id)

This release adds first-class support for the **dAPP ID** concept in
the SDK and CLI for Acki Nacki (GraphQL `info.version >= "1.0.0"`).
The on-the-wire format is selected at runtime, so the SDK keeps talking
to older nodes — but the public Rust types and several CLI inputs have
changed. This guide walks you through the migration.

---

## TL;DR

| Old | New |
|---|---|
| `ParamsOfGetAccount { address: "0:abc…" }` | `ParamsOfGetAccount { account_id: "abc…", dapp_id: "def…" }` |
| `ResultOfGetAccount.dapp_id: Option<String>` | `String` (always populated) |
| `ParamsOfSendMessage.dst_dapp_id: Option<String>` | `dapp_id: String` |
| `ParamsOfProcessMessage.dst_dapp_id` | `dapp_id: String` |
| `ResultOfSendMessage` — no account_id/dapp_id | now exposes both (`String`) |
| `tvm-cli account 0:abc…` (against v3 node) | `tvm-cli account <dapp>::<acc>` |
| `tvm-cli deploy/deployx` — no flag needed | requires `--dst-dapp-id <hex>` on v3 nodes |
| `deployed_at: "0:abc…"` | `deployed_at: "<acc>::<acc>"` |

---

## Am I affected?

You are affected if **any** of the following is true:

- You upgrade `tvm_client` or `tvm-cli` to 3.0.0+ from a prior version.
- You consume the SDK through Rust, Node.js, Python, or any binding
  that maps to `ParamsOfGetAccount`, `ParamsOfSendMessage`,
  `ParamsOfProcessMessage`, `ResultOfGetAccount`, or
  `ResultOfSendMessage`.
- You drive `tvm-cli account`, `deploy`, `deployx`, `call`, `callx`,
  `send`, or `message` against a node reporting `info.version >=
  "1.0.0"` over GraphQL.

You are **not** affected if you only talk to legacy (`< 1.0.0`) nodes
through the older SDK — the new SDK handles them transparently.

---

## Concepts

A `dapp_id` is a 64-character hex string that identifies of a Decentralized
Contract System on the Acki Nacki blockchain. This ID is equal to the
address of the root smart contract, which is deployed using an external
message. All contracts deployed with internal messages automatically receive
the same Dapp ID. Whether from the same root contract, or from contracts
deployed by the root contract. On v3 nodes every contract belongs to exactly
one dapp, and every external message routed through `/v2/messages` and
`/v2/account` must carry it.

The SDK introduces an **extended address form**:

```
<dapp_id_hex64>::<account_id_hex64>
```

For self-rooted contracts (the common case for `tvm-cli deploy`),
**`dapp_id == account_id`**, so a self-rooted deployment ends up at
`<hex>::<hex>` (both halves identical). The `::` separator is what
distinguishes the new form from the legacy `<wc>:<hex>` form.

---

## Rust SDK migration

### `ParamsOfGetAccount` and `ResultOfGetAccount`

```rust
// Before:
let params = ParamsOfGetAccount {
    address: "0:abc...".to_string(),
};
let res = get_account(client, params).await?;
let dapp_id: Option<String> = res.dapp_id;
let boc: String = res.boc;

// After:
let params = ParamsOfGetAccount {
    account_id: "abc...".to_string(),  // strict 64-hex, no 0x, no wc
    dapp_id:    "def...".to_string(),  // strict 64-hex; required on v>=1.0.0
};
let res = get_account(client, params).await?;
let account_id: String = res.account_id;  // always populated
let dapp_id:    String = res.dapp_id;     // always populated
let boc:        String = res.boc;
```

Notes:
- `account_id` and `dapp_id` are validated up front. Pass them as
  64-character hex with no `0x` and no workchain prefix.
- On `< 1.0.0` (GraphQL `info.version`) servers, `dapp_id` may be
  `""` (empty); the SDK skips the field on the wire.
- On `>= 1.0.0` (GraphQL `info.version`) servers, an empty `dapp_id`
  produces an error code `518 DappIdRequired`.

### `ParamsOfSendMessage` and `ResultOfSendMessage`

```rust
// Before:
let params = ParamsOfSendMessage {
    message: msg_boc,
    abi: Some(abi),
    thread_id: None,
    send_events: false,
    dst_dapp_id: Some("def...".into()),
};
let res = send_message(client, params, |_| async {}).await?;
// res had no account_id / dapp_id

// After:
let params = ParamsOfSendMessage {
    message: msg_boc,
    abi: Some(abi),
    thread_id: None,
    send_events: false,
    dapp_id: "def...".to_string(),  // String, not Option<String>
};
let res = send_message(client, params, |_| async {}).await?;
let account_id: String = res.account_id;  // always populated
let dapp_id:    String = res.dapp_id;     // always populated
```

For v3-compatible nodes `account_id` and `dapp_id` come from the response;
for legacy servers they are derived locally (destination address hex / mirrored
from request). Either way, your downstream code does not need to branch on
server version.

### `ParamsOfProcessMessage`

```rust
// Before:
let params = ParamsOfProcessMessage {
    message_encode_params,
    send_events: false,
    dst_dapp_id: None,
};

// After:
let params = ParamsOfProcessMessage {
    message_encode_params,
    send_events: false,
    dapp_id: "def...".to_string(),  // empty string allowed only on v<1.0.0
};
```

### New helper API

```rust
let server_link = context.get_server_link()?;
server_link.state().get_query_endpoint().await?;       // force resolution
if context.supports_dapp_id().await? {
    // server is v>=1.0.0
}
```

`supports_dapp_id()` is the recommended way to gate version-conditional
code in your own application logic.

---

## JS / Python / language-binding migration

Any binding that maps directly to the Rust JSON interface sees the
same renames at the JSON level:

```diff
- { "address": "0:abc..." }
+ { "account_id": "abc...", "dapp_id": "def..." }

- { "dst_dapp_id": null }
+ { "dapp_id": "def..." }
```

If your binding parses `ResultOfGetAccount.dapp_id` as a nullable
field, change it to a plain string. New non-null `account_id` field
is always present.

---

## tvm-cli migration

### `account`

The legacy form still works against `< 1.0.0` (GraphQL `info.version`) nodes:

```bash
tvm-cli account 0:abc...
```

Against a `>= 1.0.0` (GraphQL `info.version`) node, `account_id` alone is not
enough — provide the `dapp_id` via the extended form:

```bash
# Extended form, double-colon (preferred):
tvm-cli account <dapp_id>::<account_id>

# Single-colon when the dapp portion is 64-hex:
tvm-cli account <dapp_id>:<account_id>
```

If you forget the `dapp_id` on a v3-compatible node, the CLI errors with:

```
address `...` is missing dapp_id; v>=1.0.0 servers require the form
`dapp_id::account_id` or `dapp_id:account_id`
```

The output now always shows both `account_id` and `dapp_id` as plain
fields (JSON `dapp_id` is `null` when the legacy node didn't return one;
text output prints `None`).

### `deploy` / `deployx`

The CLI flag `--dst-dapp-id` is unchanged in name. It is now **required**
on `>= 1.0.0` nodes. Examples:

```bash
# Self-rooted deployment (dapp_id == future account_id): pass all zeros
tvm-cli deployx \
  --abi MyContract.abi.json \
  --keys keys.json \
  --dst-dapp-id 0000000000000000000000000000000000000000000000000000000000000000 \
  MyContract.tvc '{...}'

# Deployment into an existing dapp: pass that dapp_id
tvm-cli deployx \
  --abi MyContract.abi.json \
  --keys keys.json \
  --dst-dapp-id <64-hex-of-existing-dapp> \
  MyContract.tvc '{...}'
```

If you forget `--dst-dapp-id` on a v3-compatible node, the CLI errors with:

```
--dst-dapp-id is required when deploying to a v>=1.0.0 server
(pass a 64-character hex dapp_id; use all zeros for a self-rooted dapp)
```

The `deployed_at` field and any saved alias now use the extended
`dapp_id::account_id` form. For a self-rooted contract both halves are
identical:

```json
{ "deployed_at": "abc123...::abc123..." }
```

### `call`, `callx`, `send`, `message`

If a destination address is passed in the new `dapp_id::account_id`
form, the dapp_id is automatically extracted and forwarded to the
SDK. The legacy `--dst-dapp-id` flag is still accepted for backward
compatibility (it overrides any dapp_id embedded in the address).

```bash
# New form — dapp_id derived from the address:
tvm-cli call <dapp>::<acc> --abi ... --method ... '{...}'

# Legacy form with explicit flag (still works):
tvm-cli call 0:<acc> --abi ... --method ... '{...}' --dst-dapp-id <dapp>
```

---

## Common pitfalls

### "argument 'X' of type 'Y' not found"

Your CLI flag almost certainly uses underscores instead of hyphens.
The flag is `--dst-dapp-id`, not `--dst_dapp_id`. With the wrong name,
clap treats the flag as part of the positional `PARAMS` list and the
function-argument parser then can't find the next constructor argument.

### "Invalid address [Invalid argument: 0]"

You passed an extended `dapp::acc` address to a code path that goes
straight to TVM message encoding (e.g. an older version of `runx`).
Upgrade to a build that includes the fix in `tvm_cli/src/message.rs`;
it normalizes the address before encoding.

### `518 DappIdRequired`

You sent a message to a `>= 1.0.0` server with an empty `dapp_id`.
Provide it explicitly (Rust API: set `dapp_id`; CLI: pass
`--dst-dapp-id` or use the extended address form).

### `ResultOfGetAccount.dapp_id` no longer compiles

The type changed from `Option<String>` to `String`. Remove any
`.as_deref()`, `.unwrap_or("None")`, or `.is_some()` calls; use the
field directly. If you need a nullable representation downstream,
emit `null` when the string is empty.

---

## Quick checklist

- [ ] Replace every `ParamsOfGetAccount { address }` with
      `{ account_id, dapp_id }`. Strip the `0:` prefix from the input.
- [ ] Replace every `dst_dapp_id: Option<String>` with
      `dapp_id: String` in `ParamsOfSendMessage` and
      `ParamsOfProcessMessage`.
- [ ] Update consumers of `ResultOfGetAccount.dapp_id` to treat it as
      `String`.
- [ ] Update consumers of `ResultOfSendMessage` to read the new
      `account_id` and `dapp_id` fields if you need them.
- [ ] Audit CLI shell scripts: use `--dst-dapp-id` (hyphens), and pass
      a real 64-hex value (not the extended `dapp::acc` form) to the
      flag itself.
- [ ] If you store contract addresses in a database / config, decide
      whether to migrate stored values to the extended
      `dapp::acc` form. Both forms are accepted by the SDK; the
      extended form is preferred going forward.
