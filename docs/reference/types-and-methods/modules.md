# Modules

## Common Types

### ResponseHandler

```ts
type ResponseHandler = (params: any, responseType: number) => void;
```

Handles additional function responses.

Where:

* `params`: _any_ – Response parameters. Actual type depends on API function.
* `responseType`: _number_ – Function specific response type.

## Modules

### [client](mod\_client.md) – Provides information about library.

[get\_api\_reference](mod\_client.md#get\_api\_reference) – Returns Core Library API reference

[version](mod\_client.md#version) – Returns Core Library version

[config](mod\_client.md#config) – Returns Core Library API reference

[build\_info](mod\_client.md#build\_info) – Returns detailed information about this build.

[resolve\_app\_request](mod\_client.md#resolve\_app\_request) – Resolves application request processing result

### [crypto](broken-reference) – Crypto functions.

[factorize](broken-reference) – Integer factorization

[modular\_power](broken-reference) – Modular exponentiation

[ton\_crc16](broken-reference) – Calculates CRC16 using TON algorithm.

[generate\_random\_bytes](broken-reference) – Generates random byte array of the specified length and returns it in `base64` format

[convert\_public\_key\_to\_ton\_safe\_format](broken-reference) – Converts public key to ton safe\_format

[generate\_random\_sign\_keys](broken-reference) – Generates random ed25519 key pair.

[sign](broken-reference) – Signs a data using the provided keys.

[verify\_signature](broken-reference) – Verifies signed data using the provided public key. Raises error if verification is failed.

[sha256](broken-reference) – Calculates SHA256 hash of the specified data.

[sha512](broken-reference) – Calculates SHA512 hash of the specified data.

[scrypt](broken-reference) – Perform `scrypt` encryption

[nacl\_sign\_keypair\_from\_secret\_key](broken-reference) – Generates a key pair for signing from the secret key

[nacl\_sign](broken-reference) – Signs data using the signer's secret key.

[nacl\_sign\_open](broken-reference) – Verifies the signature and returns the unsigned message

[nacl\_sign\_detached](broken-reference) – Signs the message using the secret key and returns a signature.

[nacl\_sign\_detached\_verify](broken-reference) – Verifies the signature with public key and `unsigned` data.

[nacl\_box\_keypair](broken-reference) – Generates a random NaCl key pair

[nacl\_box\_keypair\_from\_secret\_key](broken-reference) – Generates key pair from a secret key

[nacl\_box](broken-reference) – Public key authenticated encryption

[nacl\_box\_open](broken-reference) – Decrypt and verify the cipher text using the receivers secret key, the senders public key, and the nonce.

[nacl\_secret\_box](broken-reference) – Encrypt and authenticate message using nonce and secret key.

[nacl\_secret\_box\_open](broken-reference) – Decrypts and verifies cipher text using `nonce` and secret `key`.

[mnemonic\_words](broken-reference) – Prints the list of words from the specified dictionary

[mnemonic\_from\_random](broken-reference) – Generates a random mnemonic

[mnemonic\_from\_entropy](broken-reference) – Generates mnemonic from pre-generated entropy

[mnemonic\_verify](broken-reference) – Validates a mnemonic phrase

[mnemonic\_derive\_sign\_keys](broken-reference) – Derives a key pair for signing from the seed phrase

[hdkey\_xprv\_from\_mnemonic](broken-reference) – Generates an extended master private key that will be the root for all the derived keys

[hdkey\_derive\_from\_xprv](broken-reference) – Returns extended private key derived from the specified extended private key and child index

[hdkey\_derive\_from\_xprv\_path](broken-reference) – Derives the extended private key from the specified key and path

[hdkey\_secret\_from\_xprv](broken-reference) – Extracts the private key from the serialized extended private key

[hdkey\_public\_from\_xprv](broken-reference) – Extracts the public key from the serialized extended private key

[chacha20](broken-reference) – Performs symmetric `chacha20` encryption.

[create\_crypto\_box](broken-reference) – Creates a Crypto Box instance.

[remove\_crypto\_box](broken-reference) – Removes Crypto Box. Clears all secret data.

[get\_crypto\_box\_info](broken-reference) – Get Crypto Box Info. Used to get `encrypted_secret` that should be used for all the cryptobox initializations except the first one.

[get\_crypto\_box\_seed\_phrase](broken-reference) – Get Crypto Box Seed Phrase.

[get\_signing\_box\_from\_crypto\_box](broken-reference) – Get handle of Signing Box derived from Crypto Box.

[get\_encryption\_box\_from\_crypto\_box](broken-reference) – Gets Encryption Box from Crypto Box.

[clear\_crypto\_box\_secret\_cache](broken-reference) – Removes cached secrets (overwrites with zeroes) from all signing and encryption boxes, derived from crypto box.

[register\_signing\_box](broken-reference) – Register an application implemented signing box.

[get\_signing\_box](broken-reference) – Creates a default signing box implementation.

[signing\_box\_get\_public\_key](broken-reference) – Returns public key of signing key pair.

[signing\_box\_sign](broken-reference) – Returns signed user data.

[remove\_signing\_box](broken-reference) – Removes signing box from SDK.

[register\_encryption\_box](broken-reference) – Register an application implemented encryption box.

[remove\_encryption\_box](broken-reference) – Removes encryption box from SDK

[encryption\_box\_get\_info](broken-reference) – Queries info from the given encryption box

[encryption\_box\_encrypt](broken-reference) – Encrypts data using given encryption box Note.

[encryption\_box\_decrypt](broken-reference) – Decrypts data using given encryption box Note.

[create\_encryption\_box](broken-reference) – Creates encryption box with specified algorithm

### [abi](mod\_abi.md) – Provides message encoding and decoding according to the ABI specification.

[encode\_message\_body](mod\_abi.md#encode\_message\_body) – Encodes message body according to ABI function call.

[attach\_signature\_to\_message\_body](mod\_abi.md#attach\_signature\_to\_message\_body)

[encode\_message](mod\_abi.md#encode\_message) – Encodes an ABI-compatible message

[encode\_internal\_message](mod\_abi.md#encode\_internal\_message) – Encodes an internal ABI-compatible message

[attach\_signature](mod\_abi.md#attach\_signature) – Combines `hex`-encoded `signature` with `base64`-encoded `unsigned_message`. Returns signed message encoded in `base64`.

[decode\_message](mod\_abi.md#decode\_message) – Decodes message body using provided message BOC and ABI.

[decode\_message\_body](mod\_abi.md#decode\_message\_body) – Decodes message body using provided body BOC and ABI.

[encode\_account](mod\_abi.md#encode\_account) – Creates account state BOC

[decode\_account\_data](mod\_abi.md#decode\_account\_data) – Decodes account data using provided data BOC and ABI.

[update\_initial\_data](mod\_abi.md#update\_initial\_data) – Updates initial account data with initial values for the contract's static variables and owner's public key. This operation is applicable only for initial account data (before deploy). If the contract is already deployed, its data doesn't contain this data section any more.

[encode\_initial\_data](mod\_abi.md#encode\_initial\_data) – Encodes initial account data with initial values for the contract's static variables and owner's public key into a data BOC that can be passed to `encode_tvc` function afterwards.

[decode\_initial\_data](mod\_abi.md#decode\_initial\_data) – Decodes initial values of a contract's static variables and owner's public key from account initial data This operation is applicable only for initial account data (before deploy). If the contract is already deployed, its data doesn't contain this data section any more.

[decode\_boc](mod\_abi.md#decode\_boc) – Decodes BOC into JSON as a set of provided parameters.

[encode\_boc](mod\_abi.md#encode\_boc) – Encodes given parameters in JSON into a BOC using param types from ABI.

[calc\_function\_id](mod\_abi.md#calc\_function\_id) – Calculates contract function ID by contract ABI

[get\_signature\_data](mod\_abi.md#get\_signature\_data) – Extracts signature from message body and calculates hash to verify the signature

### [boc](mod\_boc.md) – BOC manipulation module.

[decode\_tvc](mod\_boc.md#decode\_tvc) – Decodes tvc according to the tvc spec. Read more about tvc structure here https://github.com/tonlabs/ever-struct/blob/main/src/scheme/mod.rs#L30

[parse\_message](mod\_boc.md#parse\_message) – Parses message boc into a JSON

[parse\_transaction](mod\_boc.md#parse\_transaction) – Parses transaction boc into a JSON

[parse\_account](mod\_boc.md#parse\_account) – Parses account boc into a JSON

[parse\_block](mod\_boc.md#parse\_block) – Parses block boc into a JSON

[parse\_shardstate](mod\_boc.md#parse\_shardstate) – Parses shardstate boc into a JSON

[get\_blockchain\_config](mod\_boc.md#get\_blockchain\_config) – Extract blockchain configuration from key block and also from zerostate.

[get\_boc\_hash](mod\_boc.md#get\_boc\_hash) – Calculates BOC root hash

[get\_boc\_depth](mod\_boc.md#get\_boc\_depth) – Calculates BOC depth

[get\_code\_from\_tvc](mod\_boc.md#get\_code\_from\_tvc) – Extracts code from TVC contract image

[cache\_get](mod\_boc.md#cache\_get) – Get BOC from cache

[cache\_set](mod\_boc.md#cache\_set) – Save BOC into cache or increase pin counter for existing pinned BOC

[cache\_unpin](mod\_boc.md#cache\_unpin) – Unpin BOCs with specified pin defined in the `cache_set`. Decrease pin reference counter for BOCs with specified pin defined in the `cache_set`. BOCs which have only 1 pin and its reference counter become 0 will be removed from cache

[encode\_boc](mod\_boc.md#encode\_boc) – Encodes bag of cells (BOC) with builder operations. This method provides the same functionality as Solidity TvmBuilder. Resulting BOC of this method can be passed into Solidity and C++ contracts as TvmCell type.

[get\_code\_salt](mod\_boc.md#get\_code\_salt) – Returns the contract code's salt if it is present.

[set\_code\_salt](mod\_boc.md#set\_code\_salt) – Sets new salt to contract code.

[decode\_state\_init](mod\_boc.md#decode\_state\_init) – Decodes contract's initial state into code, data, libraries and special options.

[encode\_state\_init](mod\_boc.md#encode\_state\_init) – Encodes initial contract state from code, data, libraries ans special options (see input params)

[encode\_external\_in\_message](mod\_boc.md#encode\_external\_in\_message) – Encodes a message

[get\_compiler\_version](mod\_boc.md#get\_compiler\_version) – Returns the compiler version used to compile the code.

### [processing](mod\_processing.md) – Message processing module.

[monitor\_messages](mod\_processing.md#monitor\_messages) – Starts monitoring for the processing results of the specified messages.

[get\_monitor\_info](mod\_processing.md#get\_monitor\_info) – Returns summary information about current state of the specified monitoring queue.

[fetch\_next\_monitor\_results](mod\_processing.md#fetch\_next\_monitor\_results) – Fetches next resolved results from the specified monitoring queue.

[cancel\_monitor](mod\_processing.md#cancel\_monitor) – Cancels all background activity and releases all allocated system resources for the specified monitoring queue.

[send\_messages](mod\_processing.md#send\_messages) – Sends specified messages to the blockchain.

[send\_message](mod\_processing.md#send\_message) – Sends message to the network

[wait\_for\_transaction](mod\_processing.md#wait\_for\_transaction) – Performs monitoring of the network for the result transaction of the external inbound message processing.

[process\_message](mod\_processing.md#process\_message) – Creates message, sends it to the network and monitors its processing.

### [utils](mod\_utils.md) – Misc utility Functions.

[convert\_address](mod\_utils.md#convert\_address) – Converts address from any TON format to any TON format

[get\_address\_type](mod\_utils.md#get\_address\_type) – Validates and returns the type of any TON address.

[calc\_storage\_fee](mod\_utils.md#calc\_storage\_fee) – Calculates storage fee for an account over a specified time period

[compress\_zstd](mod\_utils.md#compress\_zstd) – Compresses data using Zstandard algorithm

[decompress\_zstd](mod\_utils.md#decompress\_zstd) – Decompresses data using Zstandard algorithm

### [tvm](mod\_tvm.md)

[run\_executor](mod\_tvm.md#run\_executor) – Emulates all the phases of contract execution locally

[run\_tvm](mod\_tvm.md#run\_tvm) – Executes get-methods of ABI-compatible contracts

[run\_get](mod\_tvm.md#run\_get) – Executes a get-method of FIFT contract

### [net](broken-reference) – Network access.

[query](broken-reference) – Performs DAppServer GraphQL query.

[batch\_query](broken-reference) – Performs multiple queries per single fetch.

[query\_collection](broken-reference) – Queries collection data

[aggregate\_collection](broken-reference) – Aggregates collection data.

[wait\_for\_collection](broken-reference) – Returns an object that fulfills the conditions or waits for its appearance

[unsubscribe](broken-reference) – Cancels a subscription

[subscribe\_collection](broken-reference) – Creates a collection subscription

[subscribe](broken-reference) – Creates a subscription

[suspend](broken-reference) – Suspends network module to stop any network activity

[resume](broken-reference) – Resumes network module to enable network activity

[find\_last\_shard\_block](broken-reference) – Returns ID of the last block in a specified account shard

[fetch\_endpoints](broken-reference) – Requests the list of alternative endpoints from server

[set\_endpoints](broken-reference) – Sets the list of endpoints to use on reinit

[get\_endpoints](broken-reference) – Requests the list of alternative endpoints from server

[query\_counterparties](broken-reference) – Allows to query and paginate through the list of accounts that the specified account has interacted with, sorted by the time of the last internal message between accounts

[query\_transaction\_tree](broken-reference) – Returns a tree of transactions triggered by a specific message.

[create\_block\_iterator](broken-reference) – Creates block iterator.

[resume\_block\_iterator](broken-reference) – Resumes block iterator.

[create\_transaction\_iterator](broken-reference) – Creates transaction iterator.

[resume\_transaction\_iterator](broken-reference) – Resumes transaction iterator.

[iterator\_next](broken-reference) – Returns next available items.

[remove\_iterator](broken-reference) – Removes an iterator

[get\_signature\_id](broken-reference) – Returns signature ID for configured network if it should be used in messages signature

### [debot](mod\_debot.md) – [UNSTABLE](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/UNSTABLE.md) [DEPRECATED](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/DEPRECATED.md) Module for working with debot.

[init](mod\_debot.md#init) – [UNSTABLE](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/UNSTABLE.md) [DEPRECATED](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/DEPRECATED.md) Creates and instance of DeBot.

[start](mod\_debot.md#start) – [UNSTABLE](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/UNSTABLE.md) [DEPRECATED](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/DEPRECATED.md) Starts the DeBot.

[fetch](mod\_debot.md#fetch) – [UNSTABLE](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/UNSTABLE.md) [DEPRECATED](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/DEPRECATED.md) Fetches DeBot metadata from blockchain.

[execute](mod\_debot.md#execute) – [UNSTABLE](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/UNSTABLE.md) [DEPRECATED](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/DEPRECATED.md) Executes debot action.

[send](mod\_debot.md#send) – [UNSTABLE](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/UNSTABLE.md) [DEPRECATED](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/DEPRECATED.md) Sends message to Debot.

[remove](mod\_debot.md#remove) – [UNSTABLE](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/UNSTABLE.md) [DEPRECATED](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/DEPRECATED.md) Destroys debot handle.

### [proofs](mod\_proofs.md) – [UNSTABLE](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/UNSTABLE.md) [DEPRECATED](https://github.com/tvmlabs/tvm-sdk/blob/main/docs/reference/types-and-methods/DEPRECATED.md) Module for proving data, retrieved from TONOS API.

[proof\_block\_data](mod\_proofs.md#proof\_block\_data) – Proves that a given block's data, which is queried from TONOS API, can be trusted.

[proof\_transaction\_data](mod\_proofs.md#proof\_transaction\_data) – Proves that a given transaction's data, which is queried from TONOS API, can be trusted.

[proof\_message\_data](mod\_proofs.md#proof\_message\_data) – Proves that a given message's data, which is queried from TONOS API, can be trusted.
