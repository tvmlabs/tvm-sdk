# Migrating to tvm_client / tvm-cli 3.x (dapp_id)

Starting in 3.0.0, the SDK and CLI added first-class support for the
**dAPP ID** concept in the SDK and CLI for Acki Nacki, and the public Rust
types and several CLI inputs changed accordingly. What "affects you" means
has itself changed since 3.0.0 shipped: 3.0.0 kept talking to nodes older
than GraphQL `info.version` `1.0.0` by picking a wire format at runtime; the
current release supports only `1.0.0`+ nodes and does not consult
`info.version` at all. This guide covers both the original 3.0.0 migration
and what changed again since.

| | 3.0.0 | current |
|---|---|---|
| `deploy` / `deployx` `--dst-dapp-id` | required, all zeros for self-rooted | removed; derived automatically |
| `send` / `sendfile` `--dst-dapp-id` | required only for non-root dapps | required always |
| pre-1.0.0 (v2) nodes | supported, wire format chosen at runtime | not supported |
| `ClientContext::supports_dapp_id()` | available, recommended for gating | removed |

---

## TL;DR

| Old | New |
|---|---|
| `ParamsOfGetAccount { address: "0:abc…" }` | `ParamsOfGetAccount { account_id: "abc…", dapp_id: "def…" }` |
| `ResultOfGetAccount.dapp_id: Option<String>` | `String` (always populated) |
| `ParamsOfSendMessage.dst_dapp_id: Option<String>` | `dapp_id: String` |
| `ParamsOfProcessMessage.dst_dapp_id` | `dapp_id: String` |
| `ResultOfSendMessage` — no account_id/dapp_id | now exposes both (`String`) |
| `tvm-cli account 0:abc…` (any node) | `tvm-cli account <dapp>::<acc>` |
| `tvm-cli deploy/deployx` — no flag needed (2.x) | requires `--dst-dapp-id <hex>` on all nodes (3.0.0) |
| `tvm-cli deploy/deployx` — `--dst-dapp-id <hex>` required (3.0.0) | flag removed; `dapp_id` derived automatically (current) |
| `deployed_at: "0:abc…"` | `deployed_at: "<acc>::<acc>"` |

---

## Am I affected?

You are affected if **any** of the following is true:

- You upgrade `tvm_client` or `tvm-cli` from a version before 3.0.0.
- You consume the SDK through Rust, Node.js, Python, or any binding
  that maps to `ParamsOfGetAccount`, `ParamsOfSendMessage`,
  `ParamsOfProcessMessage`, `ResultOfGetAccount`, or
  `ResultOfSendMessage`.
- You drive `tvm-cli account`, `deploy`, `deployx`, `call`, `callx`,
  `send`, `sendfile`, or `message` against a node.

3.0.0 asked which node version you were talking to, because the choice
changed the wire format. That question has one answer now: the current
release talks only to `1.0.0`+ (Acki Nacki) nodes. A pre-`1.0.0` node is
rejected server-side, not detected and worked around — there is no
fallback wire format to fall back to.

---

## Concepts

A `dapp_id` is a 64-character hex string that identifies of a Decentralized
Contract System on the Acki Nacki blockchain. This ID is equal to the
address of the root smart contract, which is deployed using an external
message. All contracts deployed with internal messages automatically receive
the same Dapp ID. Whether from the same root contract, or from contracts
deployed by the root contract. Every contract belongs to exactly one dapp,
and — **in the current release** — every send and every account read must
carry its dapp_id. 3.0.0 only enforced this against a `>= 1.0.0` node; see
the version-specific notes under `ParamsOfGetAccount`, `ParamsOfSendMessage`,
`ParamsOfProcessMessage` and `account` below for what a pre-1.0.0 node
allowed instead.

The SDK introduces an **extended address form**:

```
<dapp_id_hex64>::<account_id_hex64>
```

For self-rooted contracts (the common case for `tvm-cli deploy`),
**`dapp_id == account_id`**, so a self-rooted deployment ends up at
`<hex>::<hex>` (both halves identical). The `::` separator is what
distinguishes the new form from the legacy `<wc>:<hex>` form.

### Where each form belongs

The extended form addresses a **contract**, not a **cell**. It is what you pass
where a command names the account it acts on, and what the SDK takes as
`account_id` + `dapp_id`:

```sh
tvm-cli account <dapp_id>::<account_id>
tvm-cli call <dapp_id>::<account_id> <method> '{…}' --abi <abi>
```

An `address` **inside ABI arguments** is a different thing: it is encoded into
an on-chain address cell, which has no room for a Dapp ID. Those keep the
on-chain form `<workchain>:<account_id>` — in practice `0:<account_id>`:

```sh
tvm-cli call <dapp_id>::<account_id> sendTransaction \
  '{"dest":"0:<account_id>","value":1000000000,"bounce":false}' \
  --abi GiverV3.abi.json --sign giver.keys.json
```

Both forms therefore appear in one command line, and that is not a
transitional state: the account address argument is strict about the extended
form, and ABI `address` parameters are strict about the on-chain form. When a
contract is self-rooted, the account half of its extended address is exactly
what an ABI argument wants — take the part after `::` and prefix `0:`.

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
    dapp_id:    "def...".to_string(),  // strict 64-hex; required
};
let res = get_account(client, params).await?;
let account_id: String = res.account_id;  // always populated
let dapp_id:    String = res.dapp_id;     // always populated
let boc:        String = res.boc;
```

Notes:
- `account_id` and `dapp_id` are validated up front. Pass them as
  64-character hex with no `0x` and no workchain prefix.
- An empty or malformed `dapp_id` (or `account_id`) is refused before any
  request goes out, with error code `512 InvalidData`: `` Invalid data:
  `dapp_id` must be a 64-character hex string (no 0x prefix, no
  workchain) ``. (3.0.0 allowed an empty `dapp_id` when talking to a
  pre-1.0.0 server, skipping the field on the wire; pre-1.0.0 servers are
  no longer supported, so this is always an error now.) `get_account` has
  no separate "empty" case the way sending a message does — see
  `518 DappIdRequired` below for that distinction.

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
    dapp_id: "def...".to_string(),  // String, not Option<String>; required
};
let res = send_message(client, params, |_| async {}).await?;
let account_id: String = res.account_id;  // always populated
let dapp_id:    String = res.dapp_id;     // always populated
```

`account_id` and `dapp_id` in the response are always populated: taken
directly from the server's response when it returns them, and derived
locally (destination address hex / mirrored from the request) as a
fallback when it doesn't. Your downstream code does not need to branch on
server version.

An empty `dapp_id` produces error code `518 DappIdRequired`. 3.0.0 allowed
an empty value here only when talking to a pre-1.0.0 server; those servers
are no longer supported, so an empty `dapp_id` is always an error now. A
malformed (non-empty) `dapp_id` is a different error; see `512
InvalidData` under Common pitfalls below.

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
    dapp_id: "def...".to_string(),  // required
};
```

An empty `dapp_id` produces error code `518 DappIdRequired`. 3.0.0 allowed
an empty value here only when talking to a pre-1.0.0 server; those servers
are no longer supported, so an empty `dapp_id` is always an error now.

### New helper API (3.0.0 only — removed since)

3.0.0 added a helper for gating version-conditional application code:

```rust
let server_link = context.get_server_link()?;
server_link.state().get_query_endpoint().await?;       // force resolution
if context.supports_dapp_id().await? {
    // server is v>=1.0.0
}
```

`supports_dapp_id()` was the recommended way to gate version-conditional
code in your own application logic. `ClientContext::supports_dapp_id()`,
`ServerLink::supports_dapp_id()` and `ServerLink::server_version()` are all
removed in the current release: every server the SDK talks to now supports
`dapp_id`, so there is nothing left to gate on. Delete the gate along with
whatever branch it guarded.

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

The `dapp_id::account_id` double-colon form is **required**. Legacy
`0:abc...` and bare-hex address inputs are no longer accepted as CLI input.

```bash
tvm-cli account <dapp_id>::<account_id>
```

Passing a legacy `0:abc...` or bare-hex address errors with:

```
address must be in the form `dapp_id::account_id`
```

The output always shows both `account_id` and `dapp_id` as plain fields.

> **3.0.0 only:** against a legacy (pre-1.0.0) node, the `dapp_id` was
> dropped on the wire automatically and the node's response carried no
> `dapp_id`, so JSON output showed `null` (text output showed `None`).
> Pre-1.0.0 nodes are no longer supported, so this no longer happens —
> `dapp_id` is always forwarded on the wire and always populated in the
> output.

### `deploy` / `deployx`

**Current release:** `--dst-dapp-id` is **removed** from `deploy`, `deployx`,
`deploy_message` and `fee deploy`. A contract deployed through the CLI roots
its own dapp, so its `dapp_id` equals its own `account_id`, and the CLI
derives it automatically — nothing to pass:

```bash
tvm-cli deployx \
  --abi MyContract.abi.json \
  --keys keys.json \
  MyContract.tvc '{...}'
```

Passing `--dst-dapp-id` to `deploy`, `deploy_message` or `fee deploy` now
fails: none of the three declare the argument, and clap rejects it as
unexpected before anything runs.

> **`deployx` parser trap:** `deployx` sets `allow_hyphen_values` +
> `trailing_var_arg` to support its alternative constructor-argument syntax,
> so clap does not reject an unrecognized `--flag` — it silently absorbs it
> as the next positional instead. A script that still passes the removed
> flag,
> ```
> tvm-cli deployx --abi X.abi.json --dst-dapp-id <hex> contract.tvc '{}'
> ```
> does **not** get a clean "unexpected argument" error. `--dst-dapp-id` is
> taken as the TVC filename; the command prints `Deploying...` and then
> fails with:
> ```
> Error: failed to read smart contract file --dst-dapp-id: No such file or directory
> ```
> If you hit that message while migrating a `deployx` script, look for a
> stray `--dst-dapp-id` on the command line before you go looking for a
> file problem — there isn't one. `deploy` doesn't have this trap: it has
> no `trailing_var_arg`, so the same mistake there is rejected immediately
> instead.

The `deployed_at` field and any saved alias use the extended
`dapp_id::account_id` form. For a self-rooted contract both halves are
identical:

```json
{ "deployed_at": "abc123...::abc123..." }
```

**3.0.0 (historical):** the flag was **required** on every node, including
when running in `--fee` estimation mode:

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

Omitting `--dst-dapp-id` in 3.0.0 errored with:

```
--dst-dapp-id is required (pass a 64-character hex dapp_id)
```

Deploying into an existing dapp by passing its `dapp_id` — the second
3.0.0 example above — has no current-release equivalent: a CLI deploy is
always self-rooted now.

### `call` / `callx` / `message`

None of these commands declares a `--dst-dapp-id` flag. The destination
address must be supplied in the `dapp_id::account_id` form; the CLI extracts
the dapp_id from the address and forwards it to the SDK. Legacy `0:<acc>`
address inputs are no longer accepted for `call`/`callx`.

The two spell the same call differently. `call` takes the address, the method
and the arguments as positionals; `callx` takes them as options, and — because
everything after the first positional is treated as arguments — every option
must come before them:

```bash
tvm-cli call <dapp>::<acc> <method> '{...}' --abi <abi>
tvm-cli callx --addr <dapp>::<acc> --abi <abi> --method <method> '{...}'
```

`message` doesn't send anything — it generates and signs a BOC and stops —
so there is no destination dapp_id involved at all; the address it takes is
whatever the encoded call needs, on-chain form included where ABI expects
it.

`call` and `message` reject `--dst-dapp-id` as an unknown argument: neither
has ever declared it, and clap fails before either command runs.

`callx` does not. It carries the same `allow_hyphen_values` +
`trailing_var_arg` pair as `deployx` (see the parser trap above, and the
pitfall entry below, which already names `callx` and `runx`), so an
unrecognized `--dst-dapp-id` is absorbed into the function arguments rather
than rejected. What happens next depends on the call, and neither outcome
is the one you asked for:

- **A method that takes no arguments.** The absorbed flag and its value sit
  unused among the arguments and nothing complains. The message is sent —
  to the `dapp_id` in the address, never the one named by the flag.
- **A method that takes arguments, passed as a JSON object** (the form
  shown above). The absorbed pair lands beside the JSON, which stops
  `callx` from reading the object as a whole and sends it looking for each
  declared input by name instead. It fails with `argument "<name>" of type
  "<type>" not found` — the same symptom as the pitfall entry below — and
  nothing is sent.

So on `callx` the flag is either silently ignored or turned into a
confusing error about an unrelated argument. If you relied on 3.0.0's
advice that it "overrides any dapp_id embedded in the address," note that
it now does the opposite where it works at all: the flag is dropped and the
address wins. Remove `--dst-dapp-id` from every `callx` script.

### `send` / `sendfile`

Both commands send a message prepared elsewhere (a signed BOC via `send`, a
`.boc` file via `sendfile`). A prepared message carries no dapp_id of its
own to derive one from, so `--dst-dapp-id` is their only source for it —
and unlike `deploy`/`deployx`, the flag is **kept** on both:

```bash
tvm-cli send '<message-boc-json>' --abi contract.abi.json --dst-dapp-id <dapp-id>
tvm-cli sendfile message.boc --dst-dapp-id <dapp-id>
```

3.0.0 required the flag only when the destination was outside the sender's
own dapp; a self-rooted destination could omit it. That shortcut existed
because pre-1.0.0 servers were still supported; now that they aren't,
`--dst-dapp-id` is required in every case, including self-rooted
destinations. Omitting it fails.

---

## Common pitfalls

### "argument 'X' of type 'Y' not found" / a file that doesn't exist

`deployx`, `callx` and `runx` accept an alternative `--name value`
constructor syntax, which needs `allow_hyphen_values` + `trailing_var_arg`.
That combination means any `--flag` clap doesn't recognize is absorbed as a
positional instead of being rejected outright — so a typo'd flag
(`--dst_dapp_id` for `--dst-dapp-id`) or a flag that no longer exists on
that command (`--dst-dapp-id` on `deployx`, see the `deploy` / `deployx`
section above) doesn't produce "unrecognized argument". It surfaces
downstream instead: as `argument 'X' of type 'Y' not
found` when it displaces a constructor argument, or as `failed to read
smart contract file <flag>: No such file or directory` when it displaces
the TVC path. `deploy` doesn't have this trap — no `trailing_var_arg` —
so the same mistake there is rejected immediately as an unexpected
argument.

### "Invalid address [Invalid argument: 0]"

You passed an extended `dapp::acc` address to a code path that goes
straight to TVM message encoding (e.g. an older version of `runx`).
Upgrade to a build that includes the fix in `tvm_cli/src/message.rs`;
it normalizes the address before encoding.

### `518 DappIdRequired`

You sent a message with an empty `dapp_id`. This code is specific to
sending: it comes from an explicit empty-value check that runs before the
network call. Provide `dapp_id` explicitly: Rust API, set it on
`ParamsOfSendMessage` or `ParamsOfProcessMessage`; CLI, pass
`--dst-dapp-id` on `send`/`sendfile` — the only commands that still take
it. A *malformed but non-empty* `dapp_id` does not hit this check — see
`512 InvalidData` below.

### `512 InvalidData`: `` `dapp_id` must be a 64-character hex string ``

Two different call sites reach this, and the split is by emptiness, not by
API:

- **Sending** (`send_message`, `process_message`; CLI `send
  --dst-dapp-id <value>` / `sendfile --dst-dapp-id <value>` — the only two
  commands that still take the flag): an *empty* `dapp_id` is caught first
  and reported as `518 DappIdRequired` (above). A *malformed, non-empty*
  `dapp_id` skips that check and fails the same 64-hex-digit format check
  `account_id` gets, surfacing here as `512 InvalidData` instead.
- **`get_account`** (Rust API, or a binding around it): there is no
  separate empty-value check at all. Empty and malformed values both fail
  the same format check and both surface as `512 InvalidData` — `518`
  never happens here.

Pass a real 64-character hex value for both fields. (`tvm-cli account`
can't produce this error: its own address parser already rejects anything
that isn't two 64-hex halves, with a different message (`` address `…`
must be in the form `dapp_id::account_id` ``), before `get_account` is
ever called.)

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
- [ ] Audit CLI shell scripts calling `send` or `sendfile`: use
      `--dst-dapp-id` (hyphens, not underscores), it is now required in
      every case including self-rooted destinations, and pass a real
      64-hex value (not the extended `dapp::acc` form) to the flag itself.
- [ ] Remove `--dst-dapp-id` from any script calling `deploy`, `deployx`,
      `deploy_message` or `fee deploy` — the flag no longer exists on
      those commands. On `deployx` a leftover flag is silently absorbed
      as a positional rather than rejected; see the parser trap above.
- [ ] If you store contract addresses in a database / config, migrate
      stored values to the extended `dapp::acc` form. The CLI now
      requires this form on every command; legacy `0:<hex>` stored
      values must be converted before use.
