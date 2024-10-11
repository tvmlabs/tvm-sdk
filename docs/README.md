---
description: Overview of SDK components
---

# About Acki Nacki SDK

Acki Nacki SDK is a customised for Acki Nacki TVM SDK that consists of

* Client Libraries
* CLI&#x20;
* Local Blockchain

Core TVM-SDK client library is written in Rust, with bindings for other programming languages.

JS/TS guides are present here as reference guides meaning bindings in other languages have the same api calls.

**Get quick help in our telegram channel:**

[![Channel on Telegram](https://img.shields.io/badge/chat-on%20telegram-9cf.svg)](https://t.me/+1tWNH2okaPthMWU0)

* [TVM SDK](./#tvm-sdk)
  * [Supported languages](./#supported-languages)
    * [Rust (core library)](./#rust-core-library)
    * [Official Javascript(Typescript) binding](./#official-javascripttypescript-binding)
    * [Community bindings](./#community-bindings)
    * [If you did not find the language you need](./#if-you-did-not-find-the-language-you-need)
  * [Use-cases](./#use-cases)
  * [Quick Start](./#quick-start)
  * [Versioning](./#versioning)
  * [How to avoid Soft Breaking Problems](./#how-to-avoid-soft-breaking-problems)
  * [Build client library](./#build-client-library)
  * [Build artifacts](./#build-artifacts)
  * [Run tests](./#run-tests)
  * [Download precompiled binaries](./#download-precompiled-binaries)

## Supported languages

### Rust (core library)

Repository: https://github.com/tvmlabs/tvm-sdk

**What is Core Client Library?**

Core Client Library is written in Rust that can be dynamically linked. It provides all heavy-computation components and functions, such as TVM Virtual Machine,  Transaction Executor, ABI-related functions, BOC manipulation functions, crypto functions.

The decision to create the Rust library was made after a period of time using pure JavaScript to implement these use cases.

We ended up with very slow work of pure JavaScript and decided to move all this to Rust library and link it to Javascript as a compiled binary including a wasm module for browser applications.

Also this approach provided an opportunity to easily create bindings for any programming language and platform, thus, to make it possible to develop distributed applications (DApps) for any possible use-cases, such as: mobile DApps, web DApps, server-side DApps, enterprise DApp, desktop Dapps etc.

Client Library exposes all the functionality through a few of exported functions. All interaction with library is performed using JSON-RPC like protocol via C .h file.

### Official Javascript(Typescript) binding

Repository: [JavaScript SDK](https://github.com/tvmlabs/tvm-sdk-js)

You need to install core package and the package with binary for your platform. [See the documentation.](https://github.com/tvmlabs/tvm-sdk-js#library-distribution)

| Platform                       | Package                                                            |
| ------------------------------ | ------------------------------------------------------------------ |
| core package for all platforms | [@tvmsdk/core](https://www.npmjs.com/package/@tvmsdk/core)         |
| Node.js                        | [@tvmsdk/lib-node](https://www.npmjs.com/package/@tvmsdk/lib-node) |
| Web                            | [@tvmsdk/lib-web](https://www.npmjs.com/package/@tvmsdk/lib-web)   |

### Community bindings

| Language | Repository                                                                                                                                                                               |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Java     | <p><a href="https://github.com/radianceteam/ton-client-java">radianceteam/ton-client-java</a><br><a href="https://github.com/deplant/java4ever-binding">laugan/java4ever-binding</a></p> |
| .NET     | [everscale-actions/everscale-dotnet](https://github.com/everscale-actions/everscale-dotnet)                                                                                              |

### If you did not find the language you need

* use library module `json_interface` which provides access to library functions through JSON-RPC interface. This interface exports several extern "C" functions. So you can build a dynamic or static link library and link it to your application as any other external libraries. The JSON Interface is fully "C" compliant. You can find description in section [JSON Interface](for-binding-developers/json\_interface.md).
* write your own binding to chosen language and share it with community.

If you choose using JSON Interface please read this document [JSON Interface](for-binding-developers/json\_interface.md).\
Here you can find directions how to use `json_interface` and write your own binding.

## Use-cases

With TVM SDK you can implement logic of any complexity on TVM compatible blockchains (Everscale, Gosh, TON, Venom, etc).

* Create and send messages to blockchain
* Process messages reliably (supports retries and message expiration mechanics)
* Supports TVM Solidity and ABI compatible contracts
* Emulate transactions locally
* Run get methods
* Get account state
* Query blockchain data (blocks, transactions, messages)
* Sign data/check signature, calculate hashes (sha256, sha512), encrypt/decrypt data
* Validate addresses
* Work with blockchain native types (bag of cells or BOCs): encode, decode, calculate hash, etc
* Works on top of GraphQL API and compatible with Evernode-SE/DS, Evercloud.

## Quick Start

[Quick Start (Javascript binding)](quick\_start.md)

[Error descriptions](reference/error\_codes.md)

[JavaScript SDK Types and Methods (API Reference)](https://tonlabs.github.io/ever-sdk-js/)

[Core Types and Methods (API Reference)](reference/types-and-methods/modules.md)

[Guides](guides/installation/add\_sdk\_to\_your\_app.md)

## Versioning

We aim to follow semver practises, although before the mainnet launch we may introduce breaking changes in any release: patch and minor. Check the CHANGELOG.md file for breaking changes.

## How to avoid Soft Breaking Problems

Soft Breaking is API changes that include only new optional fields in the existing structures. This changes are fully backward compatible for JSON Interface.

But in Rust such changes can produce some problems with an old client code.

Look at the example below:

1. There is an API v1.0 function `foo` and the corresponding params structure:

```rust
#[derive(Default)]
struct ParamsOfFoo {
    pub foo: String,
}

pub fn foo(params: ParamsOfFoo)
```

1. Application uses this function in this way:

```rust
foo(ParamsOfFoo {
    foo: "foo".into(),
});
```

1. API v.1.1 introduces new field in `ParamsOfFoo`:

```rust
#[derive(Default)]
struct ParamsOfFoo {
    pub foo: String,
    pub bar: Option<String>,
}
```

From the perspective of JSON-interface it isn't breaking change because the new parameter is optional. But code snippet (2) will produce Rust compilation error.

1. To avoid such problems we recommend to use default implementation inside structure initialisation:

```rust
foo(ParamsOfFoo {
    foo: "foo".into(),
    ..Default::default(),
});
```

For all Ton Client API structures `Default` trait is implemented.

## Build client library

The best way to build client libraries is to use build scripts from this repo.

**Note**: The scripts are written in JavaScript so you have to install Node.js (v.10 or newer) to run them. Also make sure you have the latest version of Rust installed.

To build a binary for a specific target (or binding), navigate to the relevant folder and run `node build.js`.

The resulting binaries are placed to `bin` folder in the gz-compressed format.

Note that the build script generates binaries compatible with the platform used to run the script. For example, if you run it on Mac OS, you get binaries targeted at Darwin (macOS) platform.

**Note**: You need latest version of rust. Upgrade it with `rustup update` command. Check version with `rustc --version`, it should be above or equal to `1.47.0`.

## Build artifacts

Rebuild `api.json`:

```shell
cd toncli
cargo run api -o ../tools
```

Rebuild `docs`:

```shell
cd tools
npm i
tsc
node index docs -o ../docs
```

Rebuild `modules.ts`:

```shell
cd tools
npm i
tsc
node index binding -l ts -o ../../ever-sdk-js/packages/core/src
```

## Run tests

To run test suite use standard Rust test command

```
cargo test
```

SDK tests need GraphQL endpoint to run on. Such an API is exposed by a Block Keeper which runs in real networks, Local Network or public testnets.

Local Network is used by default with address `http://localhost` and port 80. If you launch it on another port you need to specify it explicitly like this: `http://localhost:port`. If you need to run tests on a real  network use the following environment variables to override the default parameters

```
TON_USE_SE: true/false - flag defining if tests run against local network (true) or a real network (false)
TON_NETWORK_ADDRESS - Block Keeper addresses separated by comma.
TON_GIVER_SECRET - Sponsor Wallet secret key. If not defined, default Local Network giver keys are used
TON_GIVER_ADDRESS - Address of the Sponsor Wallet to use for prepaying accounts before deploying test contracts. If not defined, the address is calculated using `GiverV2.tvc` and configured public key
```

## Download precompiled binaries (temporarily not maintained)

Instead of building library yourself, you can download the **latest** precompiled binaries from TVM SDK Binaries Store.

| Platform | Major | Download links                                                                                                                                           |
| -------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Win32    | 0     | [`ton_client.lib`](https://binaries.tonlabs.io/tonclient\_0\_win32\_lib.gz), [`ton_client.dll`](https://binaries.tonlabs.io/tonclient\_0\_win32\_dll.gz) |
|          | 1     | [`ton_client.lib`](https://binaries.tonlabs.io/tonclient\_1\_win32\_lib.gz), [`ton_client.dll`](https://binaries.tonlabs.io/tonclient\_1\_win32\_dll.gz) |
| macOS    | 0     | [`libton_client.dylib`](https://binaries.tonlabs.io/tonclient\_0\_darwin.gz)                                                                             |
|          | 1     | (x86\_64)[`libton_client.dylib`](https://binaries.tonlabs.io/tonclient\_1\_darwin.gz)                                                                    |
|          | 1     | (aarch64)[`libton_client.dylib`](https://binaries.tonlabs.io/tonclient\_1\_darwin\_arm64.gz)                                                             |
| Linux    | 0     | [`libton_client.so`](https://binaries.tonlabs.io/tonclient\_0\_linux.gz)                                                                                 |
|          | 1     | [`libton_client.so`](https://binaries.tonlabs.io/tonclient\_1\_linux.gz)                                                                                 |

If you want an older version of library (e.g. `0.25.0` for macOS), you need to choose a link to your platform from the list above and replace `0` with a version: [https://binaries.tonlabs.io/tonclient\_**0\_25\_0**\_darwin.gz](https://binaries.tonlabs.io/tonclient\_0\_25\_0\_darwin.gz)

_Downloaded archive is gzipped file_
