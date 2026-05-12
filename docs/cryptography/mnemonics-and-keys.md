# Mnemonics and Keys

This guide explains which mnemonic standards the SDK uses, how it derives signing keys from a mnemonic, how to create signing keys without a mnemonic, and which signature algorithm is used for messages.

## Standards and algorithms

The SDK mnemonic and signing APIs are based on these standards and algorithms:

| Purpose                         | SDK API                                                     | Standard or algorithm                                                                                                                                                             |
| ------------------------------- | ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Mnemonic dictionary             | `mnemonic_from_random`, `mnemonic_words`, `mnemonic_verify` | `MnemonicDictionary::English` by default: English BIP-39 dictionary                                                                                                               |
| Supported mnemonic dictionaries | `MnemonicDictionary`                                        | `Ton = 0`, `English = 1`, `ChineseSimplified = 2`, `ChineseTraditional = 3`, `French = 4`, `Italian = 5`, `Japanese = 6`, `Korean = 7`, `Spanish = 8`                             |
| BIP-39 mnemonic validation      | `mnemonic_verify`                                           | BIP-39 word list and checksum validation                                                                                                                                          |
| BIP-39 mnemonic to seed         | `hdkey_xprv_from_mnemonic`, `mnemonic_derive_sign_keys`     | PBKDF2-HMAC-SHA512, 2048 iterations                                                                                                                                               |
| HD master key                   | `hdkey_xprv_from_mnemonic`                                  | BIP-32-style master key: HMAC-SHA512 with key `Bitcoin seed`                                                                                                                      |
| HD derivation                   | `hdkey_derive_from_xprv`, `hdkey_derive_from_xprv_path`     | BIP-32-style derivation over secp256k1/k256 extended private keys                                                                                                                 |
| Default derivation path         | `mnemonic_derive_sign_keys`                                 | <p><br>Currently uses <code>m/44'/396'/0'/0/0</code>.<br>Will switch to <code>1331</code> after Acki Nacki is registered in the SLIP-0044 registry via the SatoshiLabs PR<br></p> |
| Signing key pair                | `mnemonic_derive_sign_keys`, `generate_random_sign_keys`    | Ed25519 key pair; public key is derived from a 32-byte signing secret                                                                                                             |
| Message signature               | `sign`, `nacl_sign`, `nacl_sign_detached`, ABI signing      | Ed25519 / NaCl signature, 64 bytes / 512 bits                                                                                                                                     |

By default, the SDK crypto config uses:

```javascript
const DEFAULT_MNEMONIC_DICTIONARY = 1; // English BIP-39
const DEFAULT_MNEMONIC_WORD_COUNT = 12;
const DEFAULT_HD_PATH = "m/44'/396'/0'/0/0"; // Will switch to 1331 after Acki Nacki is registered in the SLIP-0044 registry via the SatoshiLabs PR
```

For reproducible behavior, pass `dictionary`, `word_count`, and `path` explicitly instead of relying on defaults. The mnemonic APIs described in this guide use the crypto config default, which is English BIP-39. Some higher-level CryptoBox seed phrase helpers use `Ton` as their own default, so always check the API you call or pass the dictionary explicitly. The `Ton` dictionary is TON-compatible and uses a different seed validation and seed derivation flow than BIP-39 dictionaries.

## Mnemonic generation

To generate a random mnemonic, use `mnemonic_from_random`. Specify the dictionary and a number of words. The most common SDK setup uses the English BIP-39 dictionary and 12 words:

```javascript
const SEED_PHRASE_WORD_COUNT = 12;
const SEED_PHRASE_DICTIONARY_ENGLISH = 1;

const { phrase } = await client.crypto.mnemonic_from_random({
    dictionary: SEED_PHRASE_DICTIONARY_ENGLISH,
    word_count: SEED_PHRASE_WORD_COUNT,
});

console.log(`Generated seed phrase: "${phrase}"`);
```

Result:

```
Generated seed phrase: "garden wedding range mixed during left powder grid modify safe recycle cup"
```

For BIP-39 dictionaries the supported word counts are 12, 15, 18, 21, and 24. The phrase is generated with the selected BIP-39 word list and checksum. You can inspect the selected word list with `mnemonic_words`.

## Key pair generation from mnemonic

The simplest way to get a signing key pair from a mnemonic is `mnemonic_derive_sign_keys`.

```javascript
const HD_PATH = "m/44'/396'/0'/0/0"; //Will switch to 1331 after Acki Nacki is registered in the SLIP-0044 registry via the SatoshiLabs PR

const keyPair = await client.crypto.mnemonic_derive_sign_keys({
    phrase,
    path: HD_PATH,
    dictionary: SEED_PHRASE_DICTIONARY_ENGLISH,
    word_count: SEED_PHRASE_WORD_COUNT,
});

console.log("Generated key pair:");
console.log(keyPair);
```

Result:

```
Generated key pair:
{
  public: '4085d11b6d607c44ef0e8ddc535786af1a4b1f971e758206cd222ed3eba47d8b',
  secret: 'e90866b307ea6a72c216a34786762e648e9b382779fdfb88cf7b1e900a6bf0e2'
}
```

Internally, for a BIP-39 mnemonic the SDK performs this flow:

1. Validates the phrase against the selected BIP-39 dictionary and checksum.
2. Converts the phrase to a 64-byte seed with PBKDF2-HMAC-SHA512, 2048 iterations, salt `mnemonic`.
3. Builds the HD master private key with HMAC-SHA512 using key `Bitcoin seed`.
4. Derives an extended private key by the requested BIP-32-style derivation path.
5. Takes the derived 32-byte secret and creates an Ed25519 signing key pair from it.

For `dictionary: 0` (`Ton`), the SDK uses TON-compatible mnemonic validation and seed derivation, then applies the same HD path derivation and Ed25519 signing key creation.

## Key pair generation without mnemonic

If you do not need a human-readable recovery phrase, generate a signing key pair directly with `generate_random_sign_keys`:

```javascript
const simpleKeys = await client.crypto.generate_random_sign_keys();

console.log("Key pair not from mnemonic:");
console.log(simpleKeys);
```

Result:

```
Key pair not from mnemonic:
{
  public: 'de996e3004e2bc73b47e8a4fce665847194e2245ddbfc30d9ec2014913249f50',
  secret: 'a761156ff1ad497d4d52a32e32720cc3ef8b0d7c259f6d91b9f236d6288e12a3'
}
```

This API generates 32 random bytes, treats them as an Ed25519 signing secret, and derives the public key from that secret. The returned `secret` is a 64-symbol hex string, that is, 32 bytes.

The lower-level NaCl helper `nacl_sign_keypair_from_secret_key` has a different return format: its `secret` is `secret || public`, 64 bytes / 128 hex symbols, as expected by NaCl signing functions. Use `generate_random_sign_keys` when you need a regular SDK `KeyPair` for ABI signing.

## Keys derivation

### Master (root) key

To derive a key by path manually, first generate an extended master private key from the mnemonic with `hdkey_xprv_from_mnemonic`.

```javascript
const { xprv: hdkRoot } = await client.crypto.hdkey_xprv_from_mnemonic({
    dictionary: SEED_PHRASE_DICTIONARY_ENGLISH,
    word_count: SEED_PHRASE_WORD_COUNT,
    phrase,
});

console.log(`Serialized extended master private key:\n${hdkRoot}`);
```

Result:

```
Serialized extended master private key:
xprv9s21ZrQH143K45hXeaopM1rAUJDszLAcwFkxrZ4njANoGhFPYFsB7rzspWC8wAnWoZ2bPia7covh3mVVboC2nEswu18iEHs5LjVknSWMR2w
```

For BIP-39 mnemonics this function uses PBKDF2-HMAC-SHA512, then creates the master extended private key with HMAC-SHA512 and key `Bitcoin seed`.

### Derived key

Derive the extended private key by path with `hdkey_derive_from_xprv_path`.

```javascript
const HD_PATH = "m/44'/396'/0'/0/0"; // Will switch to 1331 after Acki Nacki is registered in the SLIP-0044 registry via the SatoshiLabs PR

const { xprv: extendedPrKey } = await client.crypto.hdkey_derive_from_xprv_path({
    xprv: hdkRoot,
    path: HD_PATH,
});

console.log(`Serialized derived extended private key:\n${extendedPrKey}`);
```

Result:

```
Serialized derived extended private key:
xprvA45BBKdrZKobCbeFvC316LZ6AVDXbDn8Sa3btCMCcgTRM4CRxX4Tg3fk7sNNXPza9aMiS6mBMp7wfHdmT23bri6YgwHbTJgXqKnJNNHAw98
```

The default derivation path used by `mnemonic_derive_sign_keys` is:

```
m/44'/396'/0'/0/0 // Will switch to 1331 after Acki Nacki is registered in the SLIP-0044 registry via the SatoshiLabs PR
```

The SDK derivation is BIP-32-style derivation over secp256k1/k256 extended private keys. Hardened path elements are marked with `'`. After derivation, the derived 32-byte private key is used as an Ed25519 signing secret. This is the SDK's compatibility flow and is not SLIP-0010 Ed25519 derivation.

To extract the private key bytes from the derived extended key, use `hdkey_secret_from_xprv`.

```javascript
const { secret } = await client.crypto.hdkey_secret_from_xprv({
    xprv: extendedPrKey,
});

console.log(`Derived private key:\n${secret}`);
```

Result:

```
Derived private key:
e90866b307ea6a72c216a34786762e648e9b382779fdfb88cf7b1e900a6bf0e2
```

### Generate keys for signature

To build an Ed25519 signing key pair from a derived 32-byte secret, use `nacl_sign_keypair_from_secret_key` if you need the NaCl key format:

```javascript
const naclKeyPair = await client.crypto.nacl_sign_keypair_from_secret_key({
    secret,
});

console.log("NaCl key pair for signing:");
console.log(naclKeyPair);
```

Result:

```
NaCl key pair for signing:
{
  public: '4085d11b6d607c44ef0e8ddc535786af1a4b1f971e758206cd222ed3eba47d8b',
  secret: 'e90866b307ea6a72c216a34786762e648e9b382779fdfb88cf7b1e900a6bf0e24085d11b6d607c44ef0e8ddc535786af1a4b1f971e758206cd222ed3eba47d8b'
}
```

For ABI message signing, prefer `mnemonic_derive_sign_keys`, because it returns the SDK `KeyPair` format directly:

```javascript
const keyPair = await client.crypto.mnemonic_derive_sign_keys({
    phrase,
    path: HD_PATH,
    dictionary: SEED_PHRASE_DICTIONARY_ENGLISH,
    word_count: SEED_PHRASE_WORD_COUNT,
});
```

You can use this key pair in ABI methods such as `abi.encode_message`, `abi.encode_message_body`, and other functions that accept SDK signing keys.

## Message signing

The SDK signs messages with Ed25519.

For raw user data:

* `crypto.sign` accepts unsigned data in base64 and an SDK `KeyPair`.
* `crypto.nacl_sign` returns signed data in NaCl attached-signature format.
* `crypto.nacl_sign_detached` returns a detached 64-byte signature encoded as hex.

For ABI external inbound messages, the ABI serializer prepares the message body, calculates the representation hash of the bag of cells that must be signed, and signs that hash with Ed25519. The resulting 512-bit signature is placed into the message body according to the ABI signing rules.
