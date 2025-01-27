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

### [client](mod_client.md) – Provides information about library.

[get\_api\_reference](mod_client.md#get_api_reference) – Returns Core Library API reference

[version](mod_client.md#version) – Returns Core Library version

[config](mod_client.md#config) – Returns Core Library API reference

[build\_info](mod_client.md#build_info) – Returns detailed information about this build.

[resolve\_app\_request](mod_client.md#resolve_app_request) – Resolves application request processing result

### [crypto](mod_crypto.md) – Crypto functions.

[factorize](mod_crypto.md#factorize) – Integer factorization

[modular\_power](mod_crypto.md#modular_power) – Modular exponentiation

[tvm\_crc16](mod_crypto.md#tvm_crc16) – Calculates CRC16 using TVM algorithm.

[generate\_random\_bytes](mod_crypto.md#generate_random_bytes) – Generates random byte array of the specified length and returns it in `base64` format

[convert\_public\_key\_to\_tvm\_safe\_format](mod_crypto.md#convert_public_key_to_tvm_safe_format) – Converts public key to tvm safe\_format

[generate\_random\_sign\_keys](mod_crypto.md#generate_random_sign_keys) – Generates random ed25519 key pair.

[sign](mod_crypto.md#sign) – Signs a data using the provided keys.

[verify\_signature](mod_crypto.md#verify_signature) – Verifies signed data using the provided public key. Raises error if verification is failed.

[sha256](mod_crypto.md#sha256) – Calculates SHA256 hash of the specified data.

[sha512](mod_crypto.md#sha512) – Calculates SHA512 hash of the specified data.

[scrypt](mod_crypto.md#scrypt) – Perform `scrypt` encryption

[nacl\_sign\_keypair\_from\_secret\_key](mod_crypto.md#nacl_sign_keypair_from_secret_key) – Generates a key pair for signing from the secret key

[nacl\_sign](mod_crypto.md#nacl_sign) – Signs data using the signer's secret key.

[nacl\_sign\_open](mod_crypto.md#nacl_sign_open) – Verifies the signature and returns the unsigned message

[nacl\_sign\_detached](mod_crypto.md#nacl_sign_detached) – Signs the message using the secret key and returns a signature.

[nacl\_sign\_detached\_verify](mod_crypto.md#nacl_sign_detached_verify) – Verifies the signature with public key and `unsigned` data.

[nacl\_box\_keypair](mod_crypto.md#nacl_box_keypair) – Generates a random NaCl key pair

[nacl\_box\_keypair\_from\_secret\_key](mod_crypto.md#nacl_box_keypair_from_secret_key) – Generates key pair from a secret key

[nacl\_box](mod_crypto.md#nacl_box) – Public key authenticated encryption

[nacl\_box\_open](mod_crypto.md#nacl_box_open) – Decrypt and verify the cipher text using the receivers secret key, the senders public key, and the nonce.

[nacl\_secret\_box](mod_crypto.md#nacl_secret_box) – Encrypt and authenticate message using nonce and secret key.

[nacl\_secret\_box\_open](mod_crypto.md#nacl_secret_box_open) – Decrypts and verifies cipher text using `nonce` and secret `key`.

[mnemonic\_words](mod_crypto.md#mnemonic_words) – Prints the list of words from the specified dictionary

[mnemonic\_from\_random](mod_crypto.md#mnemonic_from_random) – Generates a random mnemonic

[mnemonic\_from\_entropy](mod_crypto.md#mnemonic_from_entropy) – Generates mnemonic from pre-generated entropy

[mnemonic\_verify](mod_crypto.md#mnemonic_verify) – Validates a mnemonic phrase

[mnemonic\_derive\_sign\_keys](mod_crypto.md#mnemonic_derive_sign_keys) – Derives a key pair for signing from the seed phrase

[hdkey\_xprv\_from\_mnemonic](mod_crypto.md#hdkey_xprv_from_mnemonic) – Generates an extended master private key that will be the root for all the derived keys

[hdkey\_derive\_from\_xprv](mod_crypto.md#hdkey_derive_from_xprv) – Returns extended private key derived from the specified extended private key and child index

[hdkey\_derive\_from\_xprv\_path](mod_crypto.md#hdkey_derive_from_xprv_path) – Derives the extended private key from the specified key and path

[hdkey\_secret\_from\_xprv](mod_crypto.md#hdkey_secret_from_xprv) – Extracts the private key from the serialized extended private key

[hdkey\_public\_from\_xprv](mod_crypto.md#hdkey_public_from_xprv) – Extracts the public key from the serialized extended private key

[chacha20](mod_crypto.md#chacha20) – Performs symmetric `chacha20` encryption.

[create\_crypto\_box](mod_crypto.md#create_crypto_box) – Creates a Crypto Box instance.

[remove\_crypto\_box](mod_crypto.md#remove_crypto_box) – Removes Crypto Box. Clears all secret data.

[get\_crypto\_box\_info](mod_crypto.md#get_crypto_box_info) – Get Crypto Box Info. Used to get `encrypted_secret` that should be used for all the cryptobox initializations except the first one.

[get\_crypto\_box\_seed\_phrase](mod_crypto.md#get_crypto_box_seed_phrase) – Get Crypto Box Seed Phrase.

[get\_signing\_box\_from\_crypto\_box](mod_crypto.md#get_signing_box_from_crypto_box) – Get handle of Signing Box derived from Crypto Box.

[get\_encryption\_box\_from\_crypto\_box](mod_crypto.md#get_encryption_box_from_crypto_box) – Gets Encryption Box from Crypto Box.

[clear\_crypto\_box\_secret\_cache](mod_crypto.md#clear_crypto_box_secret_cache) – Removes cached secrets (overwrites with zeroes) from all signing and encryption boxes, derived from crypto box.

[register\_signing\_box](mod_crypto.md#register_signing_box) – Register an application implemented signing box.

[get\_signing\_box](mod_crypto.md#get_signing_box) – Creates a default signing box implementation.

[signing\_box\_get\_public\_key](mod_crypto.md#signing_box_get_public_key) – Returns public key of signing key pair.

[signing\_box\_sign](mod_crypto.md#signing_box_sign) – Returns signed user data.

[remove\_signing\_box](mod_crypto.md#remove_signing_box) – Removes signing box from SDK.

[register\_encryption\_box](mod_crypto.md#register_encryption_box) – Register an application implemented encryption box.

[remove\_encryption\_box](mod_crypto.md#remove_encryption_box) – Removes encryption box from SDK

[encryption\_box\_get\_info](mod_crypto.md#encryption_box_get_info) – Queries info from the given encryption box

[encryption\_box\_encrypt](mod_crypto.md#encryption_box_encrypt) – Encrypts data using given encryption box Note.

[encryption\_box\_decrypt](mod_crypto.md#encryption_box_decrypt) – Decrypts data using given encryption box Note.

[create\_encryption\_box](mod_crypto.md#create_encryption_box) – Creates encryption box with specified algorithm

### [abi](mod_abi.md) – Provides message encoding and decoding according to the ABI specification.

[encode\_message\_body](mod_abi.md#encode_message_body) – Encodes message body according to ABI function call.

[attach\_signature\_to\_message\_body](mod_abi.md#attach_signature_to_message_body)

[encode\_message](mod_abi.md#encode_message) – Encodes an ABI-compatible message

[encode\_internal\_message](mod_abi.md#encode_internal_message) – Encodes an internal ABI-compatible message

[attach\_signature](mod_abi.md#attach_signature) – Combines `hex`-encoded `signature` with `base64`-encoded `unsigned_message`. Returns signed message encoded in `base64`.

[decode\_message](mod_abi.md#decode_message) – Decodes message body using provided message BOC and ABI.

[decode\_message\_body](mod_abi.md#decode_message_body) – Decodes message body using provided body BOC and ABI.

[encode\_account](mod_abi.md#encode_account) – Creates account state BOC

[decode\_account\_data](mod_abi.md#decode_account_data) – Decodes account data using provided data BOC and ABI.

[update\_initial\_data](mod_abi.md#update_initial_data) – Updates initial account data with initial values for the contract's static variables and owner's public key. This operation is applicable only for initial account data (before deploy). If the contract is already deployed, its data doesn't contain this data section any more.

[encode\_initial\_data](mod_abi.md#encode_initial_data) – Encodes initial account data with initial values for the contract's static variables and owner's public key into a data BOC that can be passed to `encode_tvc` function afterwards.

[decode\_initial\_data](mod_abi.md#decode_initial_data) – Decodes initial values of a contract's static variables and owner's public key from account initial data This operation is applicable only for initial account data (before deploy). If the contract is already deployed, its data doesn't contain this data section any more.

[decode\_boc](mod_abi.md#decode_boc) – Decodes BOC into JSON as a set of provided parameters.

[encode\_boc](mod_abi.md#encode_boc) – Encodes given parameters in JSON into a BOC using param types from ABI.

[calc\_function\_id](mod_abi.md#calc_function_id) – Calculates contract function ID by contract ABI

[get\_signature\_data](mod_abi.md#get_signature_data) – Extracts signature from message body and calculates hash to verify the signature

### [boc](mod_boc.md) – BOC manipulation module.

[decode\_tvc](mod_boc.md#decode_tvc) – Decodes tvc according to the tvc spec. Read more about tvc structure [here](https://github.com/tvmlabs/tvm-sdk/blob/00fd198e4f8f404d5f495c6a65d84b54fe76881b/tvm_struct/src/scheme/mod.rs#L31)

[parse\_message](mod_boc.md#parse_message) – Parses message boc into a JSON

[parse\_transaction](mod_boc.md#parse_transaction) – Parses transaction boc into a JSON

[parse\_account](mod_boc.md#parse_account) – Parses account boc into a JSON

[parse\_block](mod_boc.md#parse_block) – Parses block boc into a JSON

[parse\_shardstate](mod_boc.md#parse_shardstate) – Parses shardstate boc into a JSON

[get\_blockchain\_config](mod_boc.md#get_blockchain_config) – Extract blockchain configuration from key block and also from zerostate.

[get\_boc\_hash](mod_boc.md#get_boc_hash) – Calculates BOC root hash

[get\_boc\_depth](mod_boc.md#get_boc_depth) – Calculates BOC depth

[get\_code\_from\_tvc](mod_boc.md#get_code_from_tvc) – Extracts code from TVC contract image

[cache\_get](mod_boc.md#cache_get) – Get BOC from cache

[cache\_set](mod_boc.md#cache_set) – Save BOC into cache or increase pin counter for existing pinned BOC

[cache\_unpin](mod_boc.md#cache_unpin) – Unpin BOCs with specified pin defined in the `cache_set`. Decrease pin reference counter for BOCs with specified pin defined in the `cache_set`. BOCs which have only 1 pin and its reference counter become 0 will be removed from cache

[encode\_boc](mod_boc.md#encode_boc) – Encodes bag of cells (BOC) with builder operations. This method provides the same functionality as Solidity TvmBuilder. Resulting BOC of this method can be passed into Solidity and C++ contracts as TvmCell type.

[get\_code\_salt](mod_boc.md#get_code_salt) – Returns the contract code's salt if it is present.

[set\_code\_salt](mod_boc.md#set_code_salt) – Sets new salt to contract code.

[decode\_state\_init](mod_boc.md#decode_state_init) – Decodes contract's initial state into code, data, libraries and special options.

[encode\_state\_init](mod_boc.md#encode_state_init) – Encodes initial contract state from code, data, libraries ans special options (see input params)

[encode\_external\_in\_message](mod_boc.md#encode_external_in_message) – Encodes a message

[get\_compiler\_version](mod_boc.md#get_compiler_version) – Returns the compiler version used to compile the code.

### [processing](mod_processing.md) – Message processing module.

[monitor\_messages](mod_processing.md#monitor_messages) – Starts monitoring for the processing results of the specified messages.

[get\_monitor\_info](mod_processing.md#get_monitor_info) – Returns summary information about current state of the specified monitoring queue.

[fetch\_next\_monitor\_results](mod_processing.md#fetch_next_monitor_results) – Fetches next resolved results from the specified monitoring queue.

[cancel\_monitor](mod_processing.md#cancel_monitor) – Cancels all background activity and releases all allocated system resources for the specified monitoring queue.

[send\_messages](mod_processing.md#send_messages) – Sends specified messages to the blockchain.

[send\_message](mod_processing.md#send_message) – Sends message to the network

[wait\_for\_transaction](mod_processing.md#wait_for_transaction) – Performs monitoring of the network for the result transaction of the external inbound message processing.

[process\_message](mod_processing.md#process_message) – Creates message, sends it to the network and monitors its processing.

### [utils](mod_utils.md) – Misc utility Functions.

[convert\_address](mod_utils.md#convert_address) – Converts address from any TVM format to any TVM format

[get\_address\_type](mod_utils.md#get_address_type) – Validates and returns the type of any TVM address.

[calc\_storage\_fee](mod_utils.md#calc_storage_fee) – Calculates storage fee for an account over a specified time period

[compress\_zstd](mod_utils.md#compress_zstd) – Compresses data using Zstandard algorithm

[decompress\_zstd](mod_utils.md#decompress_zstd) – Decompresses data using Zstandard algorithm

### [tvm](mod_tvm.md)

[run\_executor](mod_tvm.md#run_executor) – Emulates all the phases of contract execution locally

[run\_tvm](mod_tvm.md#run_tvm) – Executes get-methods of ABI-compatible contracts

[run\_get](mod_tvm.md#run_get) – Executes a get-method of FIFT contract

### [net](mod_net.md) – Network access.

[query](mod_net.md#query) – Performs DAppServer GraphQL query.

[batch\_query](mod_net.md#batch_query) – Performs multiple queries per single fetch.

[query\_collection](mod_net.md#query_collection) – Queries collection data

[aggregate\_collection](mod_net.md#aggregate_collection) – Aggregates collection data.

[wait\_for\_collection](mod_net.md#wait_for_collection) – Returns an object that fulfills the conditions or waits for its appearance

[unsubscribe](mod_net.md#unsubscribe) – Cancels a subscription

[subscribe\_collection](mod_net.md#subscribe_collection) – Creates a collection subscription

[subscribe](mod_net.md#subscribe) – Creates a subscription

[suspend](mod_net.md#suspend) – Suspends network module to stop any network activity

[resume](mod_net.md#resume) – Resumes network module to enable network activity

[find\_last\_shard\_block](mod_net.md#find_last_shard_block) – Returns ID of the last block in a specified account shard

[fetch\_endpoints](mod_net.md#fetch_endpoints) – Requests the list of alternative endpoints from server

[set\_endpoints](mod_net.md#set_endpoints) – Sets the list of endpoints to use on reinit

[get\_endpoints](mod_net.md#get_endpoints) – Requests the list of alternative endpoints from server

[query\_counterparties](mod_net.md#query_counterparties) – Allows to query and paginate through the list of accounts that the specified account has interacted with, sorted by the time of the last internal message between accounts

[query\_transaction\_tree](mod_net.md#query_transaction_tree) – Returns a tree of transactions triggered by a specific message.

[create\_block\_iterator](mod_net.md#create_block_iterator) – Creates block iterator.

[resume\_block\_iterator](mod_net.md#resume_block_iterator) – Resumes block iterator.

[create\_transaction\_iterator](mod_net.md#create_transaction_iterator) – Creates transaction iterator.

[resume\_transaction\_iterator](mod_net.md#resume_transaction_iterator) – Resumes transaction iterator.

[iterator\_next](mod_net.md#iterator_next) – Returns next available items.

[remove\_iterator](mod_net.md#remove_iterator) – Removes an iterator

[get\_signature\_id](mod_net.md#get_signature_id) – Returns signature ID for configured network if it should be used in messages signature



