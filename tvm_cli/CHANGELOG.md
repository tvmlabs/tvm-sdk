# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Removed

- `--dst-dapp-id` on `deploy`, `deployx`, `deploy_message` and `fee deploy`. The dapp_id of a contract deployed through the CLI equals its own account_id, so it is derived rather than supplied. This reverses the 3.0.0 change that made the flag mandatory on these commands.

### Changed (breaking)

- `send` and `sendfile` still take `--dst-dapp-id`, and it is now required in every case, including self-rooted destinations. Its help text said the flag was only needed for non-root dapps, which stopped being true when support for pre-1.0.0 servers was removed.
- `tvm_client`: sending a message and reading an account no longer probe GraphQL. Both go straight to REST. The probe was cached per client, so against a normal node this removes one round-trip per client rather than one per call; against a node that serves no GraphQL, where the probe failed and cached nothing, it removes one per call and the node needs no special handling.
- A seed phrase or a raw secret key is no longer written into a config file. `config --keys`, `config alias add --keys`, and the `--alias` of `deploy` and `deployx` — the four places whose value is stored rather than used and forgotten — take only a path to a keypair file. The two deploys refuse before broadcasting rather than where the alias is saved, so a deploy that cannot record its alias is never sent. Everywhere else `--keys` and `--sign` still accept a phrase, which lives no longer than the process. What counts as a secret is now decided in one place, the same for storing it, masking it and reading it: surrounding whitespace is trimmed first, a path separator marks a path, any whitespace left marks a phrase however its words are spelled, and long bare hex — with or without an `0x` prefix — marks a secret key. A key with a stray space or newline around it and a phrase with one mistyped word were each stored in full while the printed config claimed they had been masked; an `0x`-prefixed key was stored and printed in full. `getkeypair --phrase` now accepts those same forms, so the conversion the refusal asks for works on the value that was refused.
- A signing key held in the global config is no longer copied into a new config file. A directory with no config of its own adopts the global one, and the first `config` command wrote that copy out locally, leaving a second clear-text copy of the wallet in a file that had not existed before, without `--keys` ever being typed. Writing a config in such a directory now stops and names what to fix, since the alternatives are that copy or dropping the key and leaving every later command there to run unsigned. The key is still used by the commands that inherited it, a keypair path is carried over as before, and the global config itself is untouched. The message names the command that fixes the file the value is actually in, since `--keys` cannot: on `config clear` it removes the value, and the other `config` subcommands do not take it. `api_token` and `access_key` still travel this way and are tracked separately.
- A file the tool cannot account for is never replaced. Every config field has a default and unknown fields are ignored, so any json object used to parse into a config of pure defaults, and the next `config` command wrote that over the file: a trailing comma, a field of the wrong type, or `--config` pointed at a keypair file, a `tsconfig.json` or a JSON-RPC document each cost the whole file — the url, the wallet, the aliases map, or the wallet itself — with exit code 0 and nothing on standard error. A file is now read as a config only if it names a field a config has, the field names come from the config itself rather than a list kept by hand, so no valid config is turned away, and they are checked against the shape the file was actually read as and at every level that has fixed fields — inside `config` and inside each alias, which is where a field written by another version turns up. A file that holds extra fields still reads, with a warning; only writing over it is refused, and the message says how to step around it, since the config is read before any command runs, including the `getkeypair` that fixing one may need. A directory that inherits such a file from the global config is warned rather than stopped — the fields stay in the global file, and what the directory inherits is not what that file says.
- `config --depool_fee` refuses a value that a config file cannot hold. `inf` and `NaN` passed the minimum check and were written as `null`, after which the config would not parse at all — a valid command locking the file it had just written.
- `config clear --keys` and `config clear --api-token` take no value. Both accepted one and threw it away, so `config clear --keys ./new.keys.json` read as "store this path" and cleared the setting instead — next to the `--keys <file>` of `config` itself, an easy mistake to make. Every other option of `config clear` was already a plain flag.
- `config alias add` keeps the parts of an alias it is not given. Adding an alias that already exists replaced it whole, so setting its key dropped the contract address and the ABI path with no way to tell.
- The global config is read the same way as any other. It was parsed only in the shape the current version writes, so a global config in the older bare form was silently taken for an empty one — a key held there was dropped without a word, and commands that sign ran unsigned, while `config --global --list` showed the same file in full.
- An unusable global config no longer stops every command. It lives next to the executable, so on a shared install one stale or foreign file there took the tool down for everyone, `version` and `getkeypair` included — and `getkeypair` is what fixing a config may require. It is a fallback: a command that does not depend on it now warns and carries on with the defaults, while `config --global` still refuses to replace a file it cannot account for.

### Fixed

- A config file that already holds a seed phrase or a secret key in place of a keypair path now says so. Every run reports it on standard error, naming the file and, for an alias, the alias. Printing the configuration masks such a value, so nothing otherwise told its owner that the file on disk is the wallet itself; `config --global` reports the global config for the same reason. Reading such a config is unchanged: a phrase still signs, and a secret key still fails the way it always has, that field being read as a keypair file.
- A keypair path is no longer hidden as `<seed phrase>` because it contains a space. A value holding a path separator is printed in full — no wordlist has a word with one — so a config file keeping `./My Wallets/msig.keys.json` can be read back by its owner.
- A keypair file whose path contains a space can now be used, as long as the path holds a separator — `./My Wallets/msig.keys.json` rather than a bare `My Wallets msig.keys.json`, which is indistinguishable from a phrase. Nothing is looked up on disk. Values of `--keys`, `--sign` and `--setkey` were told apart by looking for an ASCII space in them, so such a path was read as a mnemonic and every command that signs failed with `Invalid bip39 phrase`. Whitespace around a value is now trimmed there too, so a phrase pasted with a trailing newline works wherever a bare one does. `genaddr` and `getkeypair` carried their own copies of that test and are fixed with it.
- `config` no longer prints the configuration when it failed to save it. A failed `config` used to print the settings it had not written and then the error, and under `--json` that put two documents on a stream that promises one.
- A seed phrase or secret key passed on the command line is no longer printed back. Values of `--keys`, `--sign`, `--setkey`, `--phrase` and `--keypair` now appear in the `Input arguments:` block as `<seed phrase>` or `<secret key>`, while a path to a keypair file is still shown in full. `genaddr` no longer repeats a phrase supplied with `--setkey`, in text or JSON output, and `config` / `config alias` no longer print a phrase held in place of a keypair path. A phrase the tool generates itself is still shown, since nothing else records it.
- A seed phrase written with ideographic spaces is no longer echoed in full. BIP-39 separates the words of a Japanese mnemonic with U+3000 rather than the ASCII space, and the Chinese and Korean wordlists are commonly written the same way; the masking test looked only for an ASCII space, so it took such a phrase for a filename and printed the whole wallet in the `Input arguments:` block, in `--keys` summaries and in `config` listings. Loading a phrase separated this way still fails — the parser is ASCII-only throughout — but it now fails without showing it.
- `tvm_client`: a malformed account response now says what was wrong with it. Reading an account discarded the parser's own complaint for a flat `Server response can not be parsed`, and with the pre-1.0.0 branch gone that is the only account-read path, so a body the SDK could not make sense of left nothing to debug from. The message now names the field it tripped on.
- `tvm_client` / `tvm-cli`: an invalid seed phrase is no longer repeated back in the `Invalid bip39 phrase` error. A phrase with one mistyped word is still nearly the whole wallet, so the message now carries only a short stub and the length, the way invalid secret keys were already reported.
- `tvm_client` / `tvm-cli`: a mistyped seed phrase from the Japanese, Korean or either Chinese wordlist no longer crashes the tool. Shortening a phrase for that error message counted bytes instead of characters, so a phrase in any non-Latin dictionary aborted with a panic rather than reporting `Invalid bip39 phrase`; both the stub and the length it quotes are now counted in characters.
- `genaddr`, `getkeypair`, `genphrase`, `account`, `decode` and the other subcommands listed after `deploy_message` no longer abort immediately in debug builds. `deploy_message` was registered under the wrong internal name, which tripped an argument-parser assertion during command dispatch. Release builds were unaffected.
- `tvm_abi`: refusing an `address` argument now says which form belongs there. A canonical `dapp_id::account_id` passed inside ABI arguments used to fail with an unrelated parser complaint; the message now states that this form names a contract for a command to act on, that ABI arguments take the on-chain `<workchain>:<account_id>` form, and what to pass instead. Every other malformed address is refused with the reason plus the expected form. Which form belongs where is documented in `docs/MIGRATION-3.x.md`.
- Error messages no longer print `0` in place of what they were built with. Twenty error variants across `tvm_block`, `tvm_vm`, `tvm_executor`, `tvm_abi` and `tvm_block_json` interpolated an integer literal instead of their own field, so the reason an argument was invalid, the VM exception code, and the exit code a contract refused a message with were all reported as zero. They now carry the real value.
- `deploy`, `deploy_message`, `fee deploy` and `deployx` now accept a filename in place of the constructor arguments, which the help has always promised. A filename used to reach the json parser unchanged and fail with `function arguments is not a json: expected value at line 1 column 1`; it is now read the way `call`, `run` and `debug` already read it, including a filename given in the `parameters` config field. A file that cannot be read is reported by name and reason instead of as broken json.
- `deploy`, `deploy_message` and `fee deploy` no longer abort immediately in debug builds. The three share one implementation, which asked the argument parser for `--alias`, `--output` and `--raw` whichever of them was running; each of those belongs to a single command, and asking for the others tripped a parser assertion. Release builds were unaffected.
- `account` no longer aborts immediately in debug builds. `--boc` declared a conflict with `--tvc` for every command that takes it, but `account` takes `--boc` alone, and the argument parser asserts when a declared conflict names an argument the command never registers. The conflict is now attached only where both options exist, so `run`, `runx` and `debug run` still refuse the two together. Release builds were unaffected. An entry above claimed this command was already fixed; that fix addressed a different cause.
- `call` and `fee call` no longer abort immediately in debug builds. They share an implementation with `message`, which alone defines `--lifetime`, `--timestamp`, `--output` and `--raw`; asking the argument parser for those while running `call` tripped the same assertion as the case above. Release builds were unaffected.
- Every `debug` subcommand no longer aborts immediately in debug builds. `debug call` was built as a renamed copy of `debug run`, and renaming a command leaves it registered under the name it was created with, so dispatching by the new one tripped a parser assertion before `debug call`, `debug run`, `debug deploy`, `debug message`, `debug replay` or `debug sequence-diagram` could start. Release builds were affected too, in their own way: with nothing registered under that name, `debug call` fell through to the `debug run` branch and traced as a getter, with unlimited gas, the account balance replaced by the maximum, and a message the contract never accepted reported as `Execution finished.`. It now traces the call it was asked for, and says `Execution failed:` with the reason when the message is rejected. Outside `--json` the exit code is 0 either way, which is its own defect: `debug message` reports a rejected message as a failed run and `debug call` does not. Under `--json` the rejection now fails the run, where the getter branch used to swallow it.
- `runx` and `debug run` no longer abort immediately in debug builds. Both resolve address, ABI and keys through the code `callx` and `debug call` use, which asked the parser for the `--keys` neither of them registers; `debug run` also asked for `--update`. A key named in the config file or in an alias is still used. Release builds were unaffected.
- `debug message` no longer aborts immediately in debug builds, `--help` included. Its `--boc` declared a conflict with a `--tvc` the command does not take, and the parser asserts when a declared conflict names an argument the command never registers — the defect `account` had at the top level. `debug run` and `debug call` define both and still refuse the two together. Release builds were unaffected.
- `multisig send`, `multisig deploy` and the `depool` commands that send through a wallet no longer abort immediately in debug builds. They share one set of wallet arguments, which asked for the address as `--addr` and the key as both `--sign` and `--keys`; `deploy` registers only `--keys` and the others only `--sign`, so each tripped the assertion on the other's arguments — `multisig send` as soon as `--sign` was left out, `multisig deploy` always. Release builds were unaffected.
- `test deploy` no longer aborts immediately in debug builds. It asked for a `--wc` it does not register, which selected no workchain everywhere else. Release builds were unaffected.
- `debug transaction`, `debug account`, `debug replay`, `debug message` and `test ticktock` no longer abort immediately in debug builds once they reach tracing or decoding. The tracing callback and the message decoder asked the running command for an `--abi`, and those five register none. What they decode with is unchanged: `--decode_abi` first, then the ABI named in the config file. Release builds were unaffected.
- `multisig deploy` reports a missing wallet or `--keys` at once instead of after downloading the wallet image. It resolved the wallet only after the download, so being told `multisig address is not defined` took as long as fetching a contract from GitHub.
- `proposal --help` lists `create`, `vote` and `decode` again. It was declared with an argument-parser call that replaces the whole help page rather than setting the description, so the command printed one line and named none of its subcommands.
- Giving `--config` no longer renames a `tonos-cli.conf.json` found in the current directory. The default config path was computed even when one was supplied, and computing it performs that migration.
- `debug call --update` and `debug message --update` no longer claim to have updated the account when nothing executed. The write-back sat outside the branch on the execution result, so a message the contract rejected still had the account file rewritten -- with the state the run had started from -- and announced as `successfully updated`.
- `body` no longer aborts immediately in debug builds. It asked for an `--output` it does not register — the command writes the message body to standard output — and printed it among the input arguments as `output: None`. That line is gone; release builds were otherwise unaffected.

## [3.0.2] - 2026-06-15

### Fixed
- `call`/`callx` and other message-sending commands no longer fail with `code 11: Server responded with code 404` when targeting a node that serves only the REST message API and returns 404 on `/graphql`. The GraphQL server-version probe is now best-effort: when GraphQL is reachable its `info.version` is trusted as before; when it is unavailable the send proceeds using the current v3 wire format instead of aborting.
- Decoding return values and bodies that span multiple cells no longer produces `WrongDataLayout` when the last field spills into a reference cell (off-by-32 fix in `tvm_abi`).

## [3.0.1] - 2026-06-08

### Fixed
- Fixed `decode::tests::test_decode_body_json` to use an existing manifest-relative wallet ABI fixture instead of a missing `tests/samples/wallet.abi.json` path.

## [3.0.0] - 2026-06-05

### Changed (breaking)
- Default HD key derivation path changed from `m/44'/396'/0'/0/0` to `m/44'/1331'/0'/0/0`. Key and address generation that rely on the default path now derive different keys from the same seed phrase. To keep previous keys, pass the old path explicitly.
- Address inputs now require the `dapp_id::account_id` form on all commands and against all nodes. Legacy `0:<hex>`, bare-hex, single-colon, and 128-hex address forms are no longer accepted.
- `deploy` and `deployx` now always require `--dst-dapp-id`, including `--fee` mode. Pass all zeros for a self-rooted dapp.

### Added
- `genaddr` additionally prints the `dapp_id::account_id` self-rooted address form (`dapp_account` in JSON output).

### Fixed
- Address values are passed through internal re-parsing in full `dapp_id::account_id` form, preserving destination dapp information across commands.

## [2.24.19] - 2026-04-17

### Added
- Support for extended address parsing via `SdkAddress::from_str`, including `dapp_id` extraction from user-provided addresses.

### Changed
- `call`, `callx`, and proposal commands now derive destination `dapp_id` from extended addresses.
- `deploy`, `deployx`, and `send` commands now accept explicit `--dst-dapp-id` where the destination `dapp_id` can not be derived from an address.

### Fixed
- Fixed `dump accounts` address validation for the current `SdkAddress` API.

## [2.24.18] - 2026-04-13

### Added
- `--log-path` global option and `TVM_CLI_LOG_PATH` env var to redirect all log output to a file (append mode), keeping stdout/stderr clean for automation scripts
- `--log-filter` option and `TVM_CLI_LOG_FILTER` env var for include/exclude module filtering (e.g. `tvm_client,-hyper`)
- Session startup info (command-line arguments, working directory) is logged to the log file

### Changed
- Console log output is now suppressed in `--json` (`-j`) mode
- Default console log level lowered from Warn to Error to reduce noise

## [2.24.13] - 2026-03-03

### Changed
- Migrated CLI argument definitions to current clap v3 API, removing deprecated usage

### Fixed
- `--abi` argument is now required for the `genaddr` subcommand, preventing confusing runtime errors when omitted

## 0.36.0

 - Supported [ABI 2.4](https://github.com/tonlabs/ever-abi/blob/master/CHANGELOG.md#version-240)

## 0.35.7

### Bug fixes
- Fixed double log initialization bug for runx subcommand

## 0.35.6

### New
- Fixed double log initialization bug

## 0.35.5

### New
- Migrated to ever-sdk 1.43.3

## 0.35.4

### New 
- Added `test` command and subcommands: `config`, `deploy`, `sign`, `ticktock`
- Added ability not to receive debug output for `debug` command using `nul` for output file name
- Added ability to set `initial_balance` for account deployment

## 0.34.1

### New 
- Fixed update_config command bug for solidity contracts

## 0.34.0

### New 
- Flag `--v2` in `multisig` and `depool` subcommands to support multisig v2.

## 0.33.0

### New
- Migrated to ever-sdk 0.41.1

## Version: 0.30.1

### New
- Added the `sign` command. It makes ED25519 signature for data encoded in base64 or hex using common `--keys` option;

## Version: 0.29.1

### New
- Added [sold](https://github.com/tonlabs/TON-Solidity-Compiler/tree/master/sold) functionality as feature;
- Improved behavior of the `decode msg` command. Now it doesn't require `--base64` flag to decode base64 input. It can
  also decode message by id in the blockchain and decode files with messages not in binary but with text in base64;
- Changed `debug transaction` and `debug account` commands flag `--empty_config` to `--default_config` which uses 
  current network config or default one if it is unavailable;
- Removed option `--saved_config` from call and run commands.

## Version: 0.28.12

### New
- Added ability to specify link to the abi file of json data instead of path.

## Version: 0.28.3

### Breaking changes:
 - `debug` commands `call`, `run` and `deploy` now take function parameters in alternative syntax. 

## Version: 0.28.1

### New
 - Updated version of SDK;
 - Added global tonos-cli config which is used to create default configs;
 - Added config parameters for Evercloud authentication;
 - Added new input format for `tonos-cli decode message` command.

## Version: 0.27.33

### New
- Added ability to call `account` command with address from config

### Bug fixes
- Fixed work with old config file


## Version: 0.27.31

### New
 - Clear alternative syntax parameters
 - Alias and abi methods completion


## Version: 0.27.30

### New
- Added alias functionality
- Added completion script to complete bash commands with aliases and abi methods.


## Version: 0.27.26

### New
- Added `--now <value>` option for `debug message` command.

## Version: 0.27.20

### New
- Enlarged decode fields for `decode body` command
- Added sequence diagram rendering command

## Version: 0.27.19

### Bug fixes
- Removed custom header from call command

## Version: 0.27.6

### Bug fixes
- Fixed `debug run` gas limits


## Version: 0.27.1

### Breaking changes:
 - Commands `convert` and `callex` were removed.


## Version: 0.26.45

### New
 - `tokio` library updated to `1.*` version
 
## Version: 0.26.44

### New


## Version: 0.26.35

### New
 - Add config param 42
 - Update libraries


## Version: 0.26.34

### New
 - Update libraries


## Version: 0.26.30

### New


## Version: 0.26.28

### New
 - Added network test and improved giver for parallel debot tests
 - Added Ubuntu 22 hint
 - Fixed tests to work in parallel


## Version: 0.26.26

### New
 - Fixed tests to work in parallel


## Version: 0.26.24

### New
  - Libraries update

## Version: 0.26.8

### New
 - Update endpoints
 - Added --boc flag for account command


## Version: 0.26.7

### New


## Version: 0.26.4

### New


## Version: 0.26.1

### New
 - Breaking change for getkeypair command: arguments are now specified with flags and can be omitted.


## Version: 0.25.23

### New

## Version: 0.25.15


### New
 - Add support copyleft

 
## Version: 0.25.11

### New


## Version: 0.25.7

### New
 - Unify genaddr abi param with other cmds
 - Add &#x60;account-wait&#x60; subcommand
 - Fixed client creation for local run
 - Fixed a bug with run parameters
 - Fixed runget
 - Refactored and improved debug on fail
 - inverted min_trace flag


## Version: 0.25.6

### New
 - Add &#x60;account-wait&#x60; subcommand
 - Fixed client creation for local run
 - Fixed a bug with run parameters
 - Fixed runget
 - Refactored and improved debug on fail
 - inverted min_trace flag


## Version: 0.25.3

### New
 - Refactored and improved debug on fail
 - inverted min_trace flag


## Version: 0.25.2

### New
 - Refactored and improved debug on fail
 - inverted min_trace flag


## Version: 0.24.59

### New
 - Block replaying
 - inverted min_trace flag


## Version: 0.24.56

### New


## Version: 0.24.51

### New


## Version: 0.24.48

### New


## Version: 0.24.46

### New
