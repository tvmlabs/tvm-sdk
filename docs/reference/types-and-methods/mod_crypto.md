# Module crypto

## Module crypto

Crypto functions.

### Functions

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

### Types

[CryptoErrorCode](mod_crypto.md#cryptoerrorcode)

[SigningBoxHandle](mod_crypto.md#signingboxhandle)

[EncryptionBoxHandle](mod_crypto.md#encryptionboxhandle)

[EncryptionBoxInfo](mod_crypto.md#encryptionboxinfo) – Encryption box information.

[EncryptionAlgorithmAESVariant](mod_crypto.md#encryptionalgorithmaesvariant)

[EncryptionAlgorithmChaCha20Variant](mod_crypto.md#encryptionalgorithmchacha20variant)

[EncryptionAlgorithmNaclBoxVariant](mod_crypto.md#encryptionalgorithmnaclboxvariant)

[EncryptionAlgorithmNaclSecretBoxVariant](mod_crypto.md#encryptionalgorithmnaclsecretboxvariant)

[EncryptionAlgorithm](mod_crypto.md#encryptionalgorithm)

[CipherMode](mod_crypto.md#ciphermode)

[AesParamsEB](mod_crypto.md#aesparamseb)

[AesInfo](mod_crypto.md#aesinfo)

[ChaCha20ParamsEB](mod_crypto.md#chacha20paramseb)

[NaclBoxParamsEB](mod_crypto.md#naclboxparamseb)

[NaclSecretBoxParamsEB](mod_crypto.md#naclsecretboxparamseb)

[CryptoBoxSecretRandomSeedPhraseVariant](mod_crypto.md#cryptoboxsecretrandomseedphrasevariant) – Creates Crypto Box from a random seed phrase. This option can be used if a developer doesn't want the seed phrase to leave the core library's memory, where it is stored encrypted.

[CryptoBoxSecretPredefinedSeedPhraseVariant](mod_crypto.md#cryptoboxsecretpredefinedseedphrasevariant) – Restores crypto box instance from an existing seed phrase. This type should be used when Crypto Box is initialized from a seed phrase, entered by a user.

[CryptoBoxSecretEncryptedSecretVariant](mod_crypto.md#cryptoboxsecretencryptedsecretvariant) – Use this type for wallet reinitializations, when you already have `encrypted_secret` on hands. To get `encrypted_secret`, use `get_crypto_box_info` function after you initialized your crypto box for the first time.

[CryptoBoxSecret](mod_crypto.md#cryptoboxsecret) – Crypto Box Secret.

[CryptoBoxHandle](mod_crypto.md#cryptoboxhandle)

[BoxEncryptionAlgorithmChaCha20Variant](mod_crypto.md#boxencryptionalgorithmchacha20variant)

[BoxEncryptionAlgorithmNaclBoxVariant](mod_crypto.md#boxencryptionalgorithmnaclboxvariant)

[BoxEncryptionAlgorithmNaclSecretBoxVariant](mod_crypto.md#boxencryptionalgorithmnaclsecretboxvariant)

[BoxEncryptionAlgorithm](mod_crypto.md#boxencryptionalgorithm)

[ChaCha20ParamsCB](mod_crypto.md#chacha20paramscb)

[NaclBoxParamsCB](mod_crypto.md#naclboxparamscb)

[NaclSecretBoxParamsCB](mod_crypto.md#naclsecretboxparamscb)

[MnemonicDictionary](mod_crypto.md#mnemonicdictionary)

[ParamsOfFactorize](mod_crypto.md#paramsoffactorize)

[ResultOfFactorize](mod_crypto.md#resultoffactorize)

[ParamsOfModularPower](mod_crypto.md#paramsofmodularpower)

[ResultOfModularPower](mod_crypto.md#resultofmodularpower)

[ParamsOfTvmCrc16](mod_crypto.md#paramsoftoncrc16)

[ResultOfTvmCrc16](mod_crypto.md#resultoftoncrc16)

[ParamsOfGenerateRandomBytes](mod_crypto.md#paramsofgeneraterandombytes)

[ResultOfGenerateRandomBytes](mod_crypto.md#resultofgeneraterandombytes)

[ParamsOfConvertPublicKeyToTvmSafeFormat](mod_crypto.md#paramsofconvertpublickeytotonsafeformat)

[ResultOfConvertPublicKeyToTvmSafeFormat](mod_crypto.md#resultofconvertpublickeytotonsafeformat)

[KeyPair](mod_crypto.md#keypair)

[ParamsOfSign](mod_crypto.md#paramsofsign)

[ResultOfSign](mod_crypto.md#resultofsign)

[ParamsOfVerifySignature](mod_crypto.md#paramsofverifysignature)

[ResultOfVerifySignature](mod_crypto.md#resultofverifysignature)

[ParamsOfHash](mod_crypto.md#paramsofhash)

[ResultOfHash](mod_crypto.md#resultofhash)

[ParamsOfScrypt](mod_crypto.md#paramsofscrypt)

[ResultOfScrypt](mod_crypto.md#resultofscrypt)

[ParamsOfNaclSignKeyPairFromSecret](mod_crypto.md#paramsofnaclsignkeypairfromsecret)

[ParamsOfNaclSign](mod_crypto.md#paramsofnaclsign)

[ResultOfNaclSign](mod_crypto.md#resultofnaclsign)

[ParamsOfNaclSignOpen](mod_crypto.md#paramsofnaclsignopen)

[ResultOfNaclSignOpen](mod_crypto.md#resultofnaclsignopen)

[ResultOfNaclSignDetached](mod_crypto.md#resultofnaclsigndetached)

[ParamsOfNaclSignDetachedVerify](mod_crypto.md#paramsofnaclsigndetachedverify)

[ResultOfNaclSignDetachedVerify](mod_crypto.md#resultofnaclsigndetachedverify)

[ParamsOfNaclBoxKeyPairFromSecret](mod_crypto.md#paramsofnaclboxkeypairfromsecret)

[ParamsOfNaclBox](mod_crypto.md#paramsofnaclbox)

[ResultOfNaclBox](mod_crypto.md#resultofnaclbox)

[ParamsOfNaclBoxOpen](mod_crypto.md#paramsofnaclboxopen)

[ResultOfNaclBoxOpen](mod_crypto.md#resultofnaclboxopen)

[ParamsOfNaclSecretBox](mod_crypto.md#paramsofnaclsecretbox)

[ParamsOfNaclSecretBoxOpen](mod_crypto.md#paramsofnaclsecretboxopen)

[ParamsOfMnemonicWords](mod_crypto.md#paramsofmnemonicwords)

[ResultOfMnemonicWords](mod_crypto.md#resultofmnemonicwords)

[ParamsOfMnemonicFromRandom](mod_crypto.md#paramsofmnemonicfromrandom)

[ResultOfMnemonicFromRandom](mod_crypto.md#resultofmnemonicfromrandom)

[ParamsOfMnemonicFromEntropy](mod_crypto.md#paramsofmnemonicfromentropy)

[ResultOfMnemonicFromEntropy](mod_crypto.md#resultofmnemonicfromentropy)

[ParamsOfMnemonicVerify](mod_crypto.md#paramsofmnemonicverify)

[ResultOfMnemonicVerify](mod_crypto.md#resultofmnemonicverify)

[ParamsOfMnemonicDeriveSignKeys](mod_crypto.md#paramsofmnemonicderivesignkeys)

[ParamsOfHDKeyXPrvFromMnemonic](mod_crypto.md#paramsofhdkeyxprvfrommnemonic)

[ResultOfHDKeyXPrvFromMnemonic](mod_crypto.md#resultofhdkeyxprvfrommnemonic)

[ParamsOfHDKeyDeriveFromXPrv](mod_crypto.md#paramsofhdkeyderivefromxprv)

[ResultOfHDKeyDeriveFromXPrv](mod_crypto.md#resultofhdkeyderivefromxprv)

[ParamsOfHDKeyDeriveFromXPrvPath](mod_crypto.md#paramsofhdkeyderivefromxprvpath)

[ResultOfHDKeyDeriveFromXPrvPath](mod_crypto.md#resultofhdkeyderivefromxprvpath)

[ParamsOfHDKeySecretFromXPrv](mod_crypto.md#paramsofhdkeysecretfromxprv)

[ResultOfHDKeySecretFromXPrv](mod_crypto.md#resultofhdkeysecretfromxprv)

[ParamsOfHDKeyPublicFromXPrv](mod_crypto.md#paramsofhdkeypublicfromxprv)

[ResultOfHDKeyPublicFromXPrv](mod_crypto.md#resultofhdkeypublicfromxprv)

[ParamsOfChaCha20](mod_crypto.md#paramsofchacha20)

[ResultOfChaCha20](mod_crypto.md#resultofchacha20)

[ParamsOfCreateCryptoBox](mod_crypto.md#paramsofcreatecryptobox)

[RegisteredCryptoBox](mod_crypto.md#registeredcryptobox)

[ParamsOfAppPasswordProviderGetPasswordVariant](mod_crypto.md#paramsofapppasswordprovidergetpasswordvariant)

[ParamsOfAppPasswordProvider](mod_crypto.md#paramsofapppasswordprovider) – Interface that provides a callback that returns an encrypted password, used for cryptobox secret encryption

[ResultOfAppPasswordProviderGetPasswordVariant](mod_crypto.md#resultofapppasswordprovidergetpasswordvariant)

[ResultOfAppPasswordProvider](mod_crypto.md#resultofapppasswordprovider)

[ResultOfGetCryptoBoxInfo](mod_crypto.md#resultofgetcryptoboxinfo)

[ResultOfGetCryptoBoxSeedPhrase](mod_crypto.md#resultofgetcryptoboxseedphrase)

[ParamsOfGetSigningBoxFromCryptoBox](mod_crypto.md#paramsofgetsigningboxfromcryptobox)

[RegisteredSigningBox](mod_crypto.md#registeredsigningbox)

[ParamsOfGetEncryptionBoxFromCryptoBox](mod_crypto.md#paramsofgetencryptionboxfromcryptobox)

[RegisteredEncryptionBox](mod_crypto.md#registeredencryptionbox)

[ParamsOfAppSigningBoxGetPublicKeyVariant](mod_crypto.md#paramsofappsigningboxgetpublickeyvariant) – Get signing box public key

[ParamsOfAppSigningBoxSignVariant](mod_crypto.md#paramsofappsigningboxsignvariant) – Sign data

[ParamsOfAppSigningBox](mod_crypto.md#paramsofappsigningbox) – Signing box callbacks.

[ResultOfAppSigningBoxGetPublicKeyVariant](mod_crypto.md#resultofappsigningboxgetpublickeyvariant) – Result of getting public key

[ResultOfAppSigningBoxSignVariant](mod_crypto.md#resultofappsigningboxsignvariant) – Result of signing data

[ResultOfAppSigningBox](mod_crypto.md#resultofappsigningbox) – Returning values from signing box callbacks.

[ResultOfSigningBoxGetPublicKey](mod_crypto.md#resultofsigningboxgetpublickey)

[ParamsOfSigningBoxSign](mod_crypto.md#paramsofsigningboxsign)

[ResultOfSigningBoxSign](mod_crypto.md#resultofsigningboxsign)

[ParamsOfAppEncryptionBoxGetInfoVariant](mod_crypto.md#paramsofappencryptionboxgetinfovariant) – Get encryption box info

[ParamsOfAppEncryptionBoxEncryptVariant](mod_crypto.md#paramsofappencryptionboxencryptvariant) – Encrypt data

[ParamsOfAppEncryptionBoxDecryptVariant](mod_crypto.md#paramsofappencryptionboxdecryptvariant) – Decrypt data

[ParamsOfAppEncryptionBox](mod_crypto.md#paramsofappencryptionbox) – Interface for data encryption/decryption

[ResultOfAppEncryptionBoxGetInfoVariant](mod_crypto.md#resultofappencryptionboxgetinfovariant) – Result of getting encryption box info

[ResultOfAppEncryptionBoxEncryptVariant](mod_crypto.md#resultofappencryptionboxencryptvariant) – Result of encrypting data

[ResultOfAppEncryptionBoxDecryptVariant](mod_crypto.md#resultofappencryptionboxdecryptvariant) – Result of decrypting data

[ResultOfAppEncryptionBox](mod_crypto.md#resultofappencryptionbox) – Returning values from signing box callbacks.

[ParamsOfEncryptionBoxGetInfo](mod_crypto.md#paramsofencryptionboxgetinfo)

[ResultOfEncryptionBoxGetInfo](mod_crypto.md#resultofencryptionboxgetinfo)

[ParamsOfEncryptionBoxEncrypt](mod_crypto.md#paramsofencryptionboxencrypt)

[ResultOfEncryptionBoxEncrypt](mod_crypto.md#resultofencryptionboxencrypt)

[ParamsOfEncryptionBoxDecrypt](mod_crypto.md#paramsofencryptionboxdecrypt)

[ResultOfEncryptionBoxDecrypt](mod_crypto.md#resultofencryptionboxdecrypt)

[ParamsOfCreateEncryptionBox](mod_crypto.md#paramsofcreateencryptionbox)

[AppPasswordProvider](mod_crypto.md#apppasswordprovider) – Interface that provides a callback that returns an encrypted password, used for cryptobox secret encryption

[AppSigningBox](mod_crypto.md#appsigningbox) – Signing box callbacks.

[AppEncryptionBox](mod_crypto.md#appencryptionbox) – Interface for data encryption/decryption

## Functions

### factorize

Integer factorization

Performs prime factorization – decomposition of a composite number into a product of smaller prime integers (factors). See \[https://en.wikipedia.org/wiki/Integer\_factorization]

```ts
type ParamsOfFactorize = {
    composite: string
}

type ResultOfFactorize = {
    factors: string[]
}

function factorize(
    params: ParamsOfFactorize,
): Promise<ResultOfFactorize>;

function factorize_sync(
    params: ParamsOfFactorize,
): ResultOfFactorize;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `composite`: _string_ – Hexadecimal representation of u64 composite number.

#### Result

* `factors`: _string\[]_ – Two factors of composite or empty if composite can't be factorized.

### modular\_power

Modular exponentiation

Performs modular exponentiation for big integers (`base`^`exponent` mod `modulus`). See \[https://en.wikipedia.org/wiki/Modular\_exponentiation]

```ts
type ParamsOfModularPower = {
    base: string,
    exponent: string,
    modulus: string
}

type ResultOfModularPower = {
    modular_power: string
}

function modular_power(
    params: ParamsOfModularPower,
): Promise<ResultOfModularPower>;

function modular_power_sync(
    params: ParamsOfModularPower,
): ResultOfModularPower;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `base`: _string_ – `base` argument of calculation.
* `exponent`: _string_ – `exponent` argument of calculation.
* `modulus`: _string_ – `modulus` argument of calculation.

#### Result

* `modular_power`: _string_ – Result of modular exponentiation

### tvm\_crc16

Calculates CRC16 using TVM algorithm.

```ts
type ParamsOfTvmCrc16 = {
    data: string
}

type ResultOfTvmCrc16 = {
    crc: number
}

function tvm_crc16(
    params: ParamsOfTvmCrc16,
): Promise<ResultOfTvmCrc16>;

function tvm_crc16_sync(
    params: ParamsOfTvmCrc16,
): ResultOfTvmCrc16;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `data`: _string_ – Input data for CRC calculation.\
  Encoded with `base64`.

#### Result

* `crc`: _number_ – Calculated CRC for input data.

### generate\_random\_bytes

Generates random byte array of the specified length and returns it in `base64` format

```ts
type ParamsOfGenerateRandomBytes = {
    length: number
}

type ResultOfGenerateRandomBytes = {
    bytes: string
}

function generate_random_bytes(
    params: ParamsOfGenerateRandomBytes,
): Promise<ResultOfGenerateRandomBytes>;

function generate_random_bytes_sync(
    params: ParamsOfGenerateRandomBytes,
): ResultOfGenerateRandomBytes;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `length`: _number_ – Size of random byte array.

#### Result

* `bytes`: _string_ – Generated bytes encoded in `base64`.

### convert\_public\_key\_to\_tvm\_safe\_format

Converts public key to tvm safe\_format

```ts
type ParamsOfConvertPublicKeyToTvmSafeFormat = {
    public_key: string
}

type ResultOfConvertPublicKeyToTvmSafeFormat = {
    tvm_public_key: string
}

function convert_public_key_to_tvm_safe_format(
    params: ParamsOfConvertPublicKeyToTvmSafeFormat,
): Promise<ResultOfConvertPublicKeyToTvmSafeFormat>;

function convert_public_key_to_tvm_safe_format_sync(
    params: ParamsOfConvertPublicKeyToTvmSafeFormat,
): ResultOfConvertPublicKeyToTvmSafeFormat;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `public_key`: _string_ – Public key - 64 symbols hex string

#### Result

* `tvm_public_key`: _string_ – Public key represented in TVM safe format.

### generate\_random\_sign\_keys

Generates random ed25519 key pair.

```ts
type KeyPair = {
    public: string,
    secret: string
}

function generate_random_sign_keys(): Promise<KeyPair>;

function generate_random_sign_keys_sync(): KeyPair;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Result

* `public`: _string_ – Public key - 64 symbols hex string
* `secret`: _string_ – Private key - u64 symbols hex string

### sign

Signs a data using the provided keys.

```ts
type ParamsOfSign = {
    unsigned: string,
    keys: KeyPair
}

type ResultOfSign = {
    signed: string,
    signature: string
}

function sign(
    params: ParamsOfSign,
): Promise<ResultOfSign>;

function sign_sync(
    params: ParamsOfSign,
): ResultOfSign;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `unsigned`: _string_ – Data that must be signed encoded in `base64`.
* `keys`: [_KeyPair_](mod_crypto.md#keypair) – Sign keys.

#### Result

* `signed`: _string_ – Signed data combined with signature encoded in `base64`.
* `signature`: _string_ – Signature encoded in `hex`.

### verify\_signature

Verifies signed data using the provided public key. Raises error if verification is failed.

```ts
type ParamsOfVerifySignature = {
    signed: string,
    public: string
}

type ResultOfVerifySignature = {
    unsigned: string
}

function verify_signature(
    params: ParamsOfVerifySignature,
): Promise<ResultOfVerifySignature>;

function verify_signature_sync(
    params: ParamsOfVerifySignature,
): ResultOfVerifySignature;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `signed`: _string_ – Signed data that must be verified encoded in `base64`.
* `public`: _string_ – Signer's public key - 64 symbols hex string

#### Result

* `unsigned`: _string_ – Unsigned data encoded in `base64`.

### sha256

Calculates SHA256 hash of the specified data.

```ts
type ParamsOfHash = {
    data: string
}

type ResultOfHash = {
    hash: string
}

function sha256(
    params: ParamsOfHash,
): Promise<ResultOfHash>;

function sha256_sync(
    params: ParamsOfHash,
): ResultOfHash;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `data`: _string_ – Input data for hash calculation.\
  Encoded with `base64`.

#### Result

* `hash`: _string_ – Hash of input `data`.\
  Encoded with 'hex'.

### sha512

Calculates SHA512 hash of the specified data.

```ts
type ParamsOfHash = {
    data: string
}

type ResultOfHash = {
    hash: string
}

function sha512(
    params: ParamsOfHash,
): Promise<ResultOfHash>;

function sha512_sync(
    params: ParamsOfHash,
): ResultOfHash;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `data`: _string_ – Input data for hash calculation.\
  Encoded with `base64`.

#### Result

* `hash`: _string_ – Hash of input `data`.\
  Encoded with 'hex'.

### scrypt

Perform `scrypt` encryption

Derives key from `password` and `key` using `scrypt` algorithm. See \[https://en.wikipedia.org/wiki/Scrypt].

## Arguments

* `log_n` - The log2 of the Scrypt parameter `N`
* `r` - The Scrypt parameter `r`
* `p` - The Scrypt parameter `p`

## Conditions

* `log_n` must be less than `64`
* `r` must be greater than `0` and less than or equal to `4294967295`
* `p` must be greater than `0` and less than `4294967295`

## Recommended values sufficient for most use-cases

* `log_n = 15` (`n = 32768`)
* `r = 8`
* `p = 1`

```ts
type ParamsOfScrypt = {
    password: string,
    salt: string,
    log_n: number,
    r: number,
    p: number,
    dk_len: number
}

type ResultOfScrypt = {
    key: string
}

function scrypt(
    params: ParamsOfScrypt,
): Promise<ResultOfScrypt>;

function scrypt_sync(
    params: ParamsOfScrypt,
): ResultOfScrypt;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `password`: _string_ – The password bytes to be hashed. Must be encoded with `base64`.
* `salt`: _string_ – Salt bytes that modify the hash to protect against Rainbow table attacks. Must be encoded with `base64`.
* `log_n`: _number_ – CPU/memory cost parameter
* `r`: _number_ – The block size parameter, which fine-tunes sequential memory read size and performance.
* `p`: _number_ – Parallelization parameter.
* `dk_len`: _number_ – Intended output length in octets of the derived key.

#### Result

* `key`: _string_ – Derived key.\
  Encoded with `hex`.

### nacl\_sign\_keypair\_from\_secret\_key

Generates a key pair for signing from the secret key

**NOTE:** In the result the secret key is actually the concatenation of secret and public keys (128 symbols hex string) by design of [NaCL](http://nacl.cr.yp.to/sign.html). See also [the stackexchange question](https://crypto.stackexchange.com/questions/54353/).

```ts
type ParamsOfNaclSignKeyPairFromSecret = {
    secret: string
}

type KeyPair = {
    public: string,
    secret: string
}

function nacl_sign_keypair_from_secret_key(
    params: ParamsOfNaclSignKeyPairFromSecret,
): Promise<KeyPair>;

function nacl_sign_keypair_from_secret_key_sync(
    params: ParamsOfNaclSignKeyPairFromSecret,
): KeyPair;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `secret`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string

#### Result

* `public`: _string_ – Public key - 64 symbols hex string
* `secret`: _string_ – Private key - u64 symbols hex string

### nacl\_sign

Signs data using the signer's secret key.

```ts
type ParamsOfNaclSign = {
    unsigned: string,
    secret: string
}

type ResultOfNaclSign = {
    signed: string
}

function nacl_sign(
    params: ParamsOfNaclSign,
): Promise<ResultOfNaclSign>;

function nacl_sign_sync(
    params: ParamsOfNaclSign,
): ResultOfNaclSign;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `unsigned`: _string_ – Data that must be signed encoded in `base64`.
* `secret`: _string_ – Signer's secret key - unprefixed 0-padded to 128 symbols hex string (concatenation of 64 symbols secret and 64 symbols public keys). See `nacl_sign_keypair_from_secret_key`.

#### Result

* `signed`: _string_ – Signed data, encoded in `base64`.

### nacl\_sign\_open

Verifies the signature and returns the unsigned message

Verifies the signature in `signed` using the signer's public key `public` and returns the message `unsigned`.

If the signature fails verification, crypto\_sign\_open raises an exception.

```ts
type ParamsOfNaclSignOpen = {
    signed: string,
    public: string
}

type ResultOfNaclSignOpen = {
    unsigned: string
}

function nacl_sign_open(
    params: ParamsOfNaclSignOpen,
): Promise<ResultOfNaclSignOpen>;

function nacl_sign_open_sync(
    params: ParamsOfNaclSignOpen,
): ResultOfNaclSignOpen;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `signed`: _string_ – Signed data that must be unsigned.\
  Encoded with `base64`.
* `public`: _string_ – Signer's public key - unprefixed 0-padded to 64 symbols hex string

#### Result

* `unsigned`: _string_ – Unsigned data, encoded in `base64`.

### nacl\_sign\_detached

Signs the message using the secret key and returns a signature.

Signs the message `unsigned` using the secret key `secret` and returns a signature `signature`.

```ts
type ParamsOfNaclSign = {
    unsigned: string,
    secret: string
}

type ResultOfNaclSignDetached = {
    signature: string
}

function nacl_sign_detached(
    params: ParamsOfNaclSign,
): Promise<ResultOfNaclSignDetached>;

function nacl_sign_detached_sync(
    params: ParamsOfNaclSign,
): ResultOfNaclSignDetached;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `unsigned`: _string_ – Data that must be signed encoded in `base64`.
* `secret`: _string_ – Signer's secret key - unprefixed 0-padded to 128 symbols hex string (concatenation of 64 symbols secret and 64 symbols public keys). See `nacl_sign_keypair_from_secret_key`.

#### Result

* `signature`: _string_ – Signature encoded in `hex`.

### nacl\_sign\_detached\_verify

Verifies the signature with public key and `unsigned` data.

```ts
type ParamsOfNaclSignDetachedVerify = {
    unsigned: string,
    signature: string,
    public: string
}

type ResultOfNaclSignDetachedVerify = {
    succeeded: boolean
}

function nacl_sign_detached_verify(
    params: ParamsOfNaclSignDetachedVerify,
): Promise<ResultOfNaclSignDetachedVerify>;

function nacl_sign_detached_verify_sync(
    params: ParamsOfNaclSignDetachedVerify,
): ResultOfNaclSignDetachedVerify;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `unsigned`: _string_ – Unsigned data that must be verified.\
  Encoded with `base64`.
* `signature`: _string_ – Signature that must be verified.\
  Encoded with `hex`.
* `public`: _string_ – Signer's public key - unprefixed 0-padded to 64 symbols hex string.

#### Result

* `succeeded`: _boolean_ – `true` if verification succeeded or `false` if it failed

### nacl\_box\_keypair

Generates a random NaCl key pair

```ts
type KeyPair = {
    public: string,
    secret: string
}

function nacl_box_keypair(): Promise<KeyPair>;

function nacl_box_keypair_sync(): KeyPair;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Result

* `public`: _string_ – Public key - 64 symbols hex string
* `secret`: _string_ – Private key - u64 symbols hex string

### nacl\_box\_keypair\_from\_secret\_key

Generates key pair from a secret key

```ts
type ParamsOfNaclBoxKeyPairFromSecret = {
    secret: string
}

type KeyPair = {
    public: string,
    secret: string
}

function nacl_box_keypair_from_secret_key(
    params: ParamsOfNaclBoxKeyPairFromSecret,
): Promise<KeyPair>;

function nacl_box_keypair_from_secret_key_sync(
    params: ParamsOfNaclBoxKeyPairFromSecret,
): KeyPair;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `secret`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string

#### Result

* `public`: _string_ – Public key - 64 symbols hex string
* `secret`: _string_ – Private key - u64 symbols hex string

### nacl\_box

Public key authenticated encryption

Encrypt and authenticate a message using the senders secret key, the receivers public key, and a nonce.

```ts
type ParamsOfNaclBox = {
    decrypted: string,
    nonce: string,
    their_public: string,
    secret: string
}

type ResultOfNaclBox = {
    encrypted: string
}

function nacl_box(
    params: ParamsOfNaclBox,
): Promise<ResultOfNaclBox>;

function nacl_box_sync(
    params: ParamsOfNaclBox,
): ResultOfNaclBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `decrypted`: _string_ – Data that must be encrypted encoded in `base64`.
* `nonce`: _string_ – Nonce, encoded in `hex`
* `their_public`: _string_ – Receiver's public key - unprefixed 0-padded to 64 symbols hex string
* `secret`: _string_ – Sender's private key - unprefixed 0-padded to 64 symbols hex string

#### Result

* `encrypted`: _string_ – Encrypted data encoded in `base64`.

### nacl\_box\_open

Decrypt and verify the cipher text using the receivers secret key, the senders public key, and the nonce.

```ts
type ParamsOfNaclBoxOpen = {
    encrypted: string,
    nonce: string,
    their_public: string,
    secret: string
}

type ResultOfNaclBoxOpen = {
    decrypted: string
}

function nacl_box_open(
    params: ParamsOfNaclBoxOpen,
): Promise<ResultOfNaclBoxOpen>;

function nacl_box_open_sync(
    params: ParamsOfNaclBoxOpen,
): ResultOfNaclBoxOpen;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `encrypted`: _string_ – Data that must be decrypted.\
  Encoded with `base64`.
* `nonce`: _string_ – Nonce
* `their_public`: _string_ – Sender's public key - unprefixed 0-padded to 64 symbols hex string
* `secret`: _string_ – Receiver's private key - unprefixed 0-padded to 64 symbols hex string

#### Result

* `decrypted`: _string_ – Decrypted data encoded in `base64`.

### nacl\_secret\_box

Encrypt and authenticate message using nonce and secret key.

```ts
type ParamsOfNaclSecretBox = {
    decrypted: string,
    nonce: string,
    key: string
}

type ResultOfNaclBox = {
    encrypted: string
}

function nacl_secret_box(
    params: ParamsOfNaclSecretBox,
): Promise<ResultOfNaclBox>;

function nacl_secret_box_sync(
    params: ParamsOfNaclSecretBox,
): ResultOfNaclBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `decrypted`: _string_ – Data that must be encrypted.\
  Encoded with `base64`.
* `nonce`: _string_ – Nonce in `hex`
* `key`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string

#### Result

* `encrypted`: _string_ – Encrypted data encoded in `base64`.

### nacl\_secret\_box\_open

Decrypts and verifies cipher text using `nonce` and secret `key`.

```ts
type ParamsOfNaclSecretBoxOpen = {
    encrypted: string,
    nonce: string,
    key: string
}

type ResultOfNaclBoxOpen = {
    decrypted: string
}

function nacl_secret_box_open(
    params: ParamsOfNaclSecretBoxOpen,
): Promise<ResultOfNaclBoxOpen>;

function nacl_secret_box_open_sync(
    params: ParamsOfNaclSecretBoxOpen,
): ResultOfNaclBoxOpen;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `encrypted`: _string_ – Data that must be decrypted.\
  Encoded with `base64`.
* `nonce`: _string_ – Nonce in `hex`
* `key`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string

#### Result

* `decrypted`: _string_ – Decrypted data encoded in `base64`.

### mnemonic\_words

Prints the list of words from the specified dictionary

```ts
type ParamsOfMnemonicWords = {
    dictionary?: MnemonicDictionary
}

type ResultOfMnemonicWords = {
    words: string
}

function mnemonic_words(
    params: ParamsOfMnemonicWords,
): Promise<ResultOfMnemonicWords>;

function mnemonic_words_sync(
    params: ParamsOfMnemonicWords,
): ResultOfMnemonicWords;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier

#### Result

* `words`: _string_ – The list of mnemonic words

### mnemonic\_from\_random

Generates a random mnemonic

Generates a random mnemonic from the specified dictionary and word count

```ts
type ParamsOfMnemonicFromRandom = {
    dictionary?: MnemonicDictionary,
    word_count?: number
}

type ResultOfMnemonicFromRandom = {
    phrase: string
}

function mnemonic_from_random(
    params: ParamsOfMnemonicFromRandom,
): Promise<ResultOfMnemonicFromRandom>;

function mnemonic_from_random_sync(
    params: ParamsOfMnemonicFromRandom,
): ResultOfMnemonicFromRandom;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Mnemonic word count

#### Result

* `phrase`: _string_ – String of mnemonic words

### mnemonic\_from\_entropy

Generates mnemonic from pre-generated entropy

```ts
type ParamsOfMnemonicFromEntropy = {
    entropy: string,
    dictionary?: MnemonicDictionary,
    word_count?: number
}

type ResultOfMnemonicFromEntropy = {
    phrase: string
}

function mnemonic_from_entropy(
    params: ParamsOfMnemonicFromEntropy,
): Promise<ResultOfMnemonicFromEntropy>;

function mnemonic_from_entropy_sync(
    params: ParamsOfMnemonicFromEntropy,
): ResultOfMnemonicFromEntropy;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `entropy`: _string_ – Entropy bytes.\
  Hex encoded.
* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Mnemonic word count

#### Result

* `phrase`: _string_ – Phrase

### mnemonic\_verify

Validates a mnemonic phrase

The phrase supplied will be checked for word length and validated according to the checksum specified in BIP0039.

```ts
type ParamsOfMnemonicVerify = {
    phrase: string,
    dictionary?: MnemonicDictionary,
    word_count?: number
}

type ResultOfMnemonicVerify = {
    valid: boolean
}

function mnemonic_verify(
    params: ParamsOfMnemonicVerify,
): Promise<ResultOfMnemonicVerify>;

function mnemonic_verify_sync(
    params: ParamsOfMnemonicVerify,
): ResultOfMnemonicVerify;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `phrase`: _string_ – Phrase
* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Word count

#### Result

* `valid`: _boolean_ – Flag indicating if the mnemonic is valid or not

### mnemonic\_derive\_sign\_keys

Derives a key pair for signing from the seed phrase

Validates the seed phrase, generates master key and then derives the key pair from the master key and the specified path

```ts
type ParamsOfMnemonicDeriveSignKeys = {
    phrase: string,
    path?: string,
    dictionary?: MnemonicDictionary,
    word_count?: number
}

type KeyPair = {
    public: string,
    secret: string
}

function mnemonic_derive_sign_keys(
    params: ParamsOfMnemonicDeriveSignKeys,
): Promise<KeyPair>;

function mnemonic_derive_sign_keys_sync(
    params: ParamsOfMnemonicDeriveSignKeys,
): KeyPair;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `phrase`: _string_ – Phrase
* `path`?: _string_ – Derivation path, for instance "m/44'/396'/0'/0/0"
* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Word count

#### Result

* `public`: _string_ – Public key - 64 symbols hex string
* `secret`: _string_ – Private key - u64 symbols hex string

### hdkey\_xprv\_from\_mnemonic

Generates an extended master private key that will be the root for all the derived keys

```ts
type ParamsOfHDKeyXPrvFromMnemonic = {
    phrase: string,
    dictionary?: MnemonicDictionary,
    word_count?: number
}

type ResultOfHDKeyXPrvFromMnemonic = {
    xprv: string
}

function hdkey_xprv_from_mnemonic(
    params: ParamsOfHDKeyXPrvFromMnemonic,
): Promise<ResultOfHDKeyXPrvFromMnemonic>;

function hdkey_xprv_from_mnemonic_sync(
    params: ParamsOfHDKeyXPrvFromMnemonic,
): ResultOfHDKeyXPrvFromMnemonic;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `phrase`: _string_ – String with seed phrase
* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Mnemonic word count

#### Result

* `xprv`: _string_ – Serialized extended master private key

### hdkey\_derive\_from\_xprv

Returns extended private key derived from the specified extended private key and child index

```ts
type ParamsOfHDKeyDeriveFromXPrv = {
    xprv: string,
    child_index: number,
    hardened: boolean
}

type ResultOfHDKeyDeriveFromXPrv = {
    xprv: string
}

function hdkey_derive_from_xprv(
    params: ParamsOfHDKeyDeriveFromXPrv,
): Promise<ResultOfHDKeyDeriveFromXPrv>;

function hdkey_derive_from_xprv_sync(
    params: ParamsOfHDKeyDeriveFromXPrv,
): ResultOfHDKeyDeriveFromXPrv;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `xprv`: _string_ – Serialized extended private key
* `child_index`: _number_ – Child index (see BIP-0032)
* `hardened`: _boolean_ – Indicates the derivation of hardened/not-hardened key (see BIP-0032)

#### Result

* `xprv`: _string_ – Serialized extended private key

### hdkey\_derive\_from\_xprv\_path

Derives the extended private key from the specified key and path

```ts
type ParamsOfHDKeyDeriveFromXPrvPath = {
    xprv: string,
    path: string
}

type ResultOfHDKeyDeriveFromXPrvPath = {
    xprv: string
}

function hdkey_derive_from_xprv_path(
    params: ParamsOfHDKeyDeriveFromXPrvPath,
): Promise<ResultOfHDKeyDeriveFromXPrvPath>;

function hdkey_derive_from_xprv_path_sync(
    params: ParamsOfHDKeyDeriveFromXPrvPath,
): ResultOfHDKeyDeriveFromXPrvPath;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `xprv`: _string_ – Serialized extended private key
* `path`: _string_ – Derivation path, for instance "m/44'/396'/0'/0/0"

#### Result

* `xprv`: _string_ – Derived serialized extended private key

### hdkey\_secret\_from\_xprv

Extracts the private key from the serialized extended private key

```ts
type ParamsOfHDKeySecretFromXPrv = {
    xprv: string
}

type ResultOfHDKeySecretFromXPrv = {
    secret: string
}

function hdkey_secret_from_xprv(
    params: ParamsOfHDKeySecretFromXPrv,
): Promise<ResultOfHDKeySecretFromXPrv>;

function hdkey_secret_from_xprv_sync(
    params: ParamsOfHDKeySecretFromXPrv,
): ResultOfHDKeySecretFromXPrv;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `xprv`: _string_ – Serialized extended private key

#### Result

* `secret`: _string_ – Private key - 64 symbols hex string

### hdkey\_public\_from\_xprv

Extracts the public key from the serialized extended private key

```ts
type ParamsOfHDKeyPublicFromXPrv = {
    xprv: string
}

type ResultOfHDKeyPublicFromXPrv = {
    public: string
}

function hdkey_public_from_xprv(
    params: ParamsOfHDKeyPublicFromXPrv,
): Promise<ResultOfHDKeyPublicFromXPrv>;

function hdkey_public_from_xprv_sync(
    params: ParamsOfHDKeyPublicFromXPrv,
): ResultOfHDKeyPublicFromXPrv;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `xprv`: _string_ – Serialized extended private key

#### Result

* `public`: _string_ – Public key - 64 symbols hex string

### chacha20

Performs symmetric `chacha20` encryption.

```ts
type ParamsOfChaCha20 = {
    data: string,
    key: string,
    nonce: string
}

type ResultOfChaCha20 = {
    data: string
}

function chacha20(
    params: ParamsOfChaCha20,
): Promise<ResultOfChaCha20>;

function chacha20_sync(
    params: ParamsOfChaCha20,
): ResultOfChaCha20;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `data`: _string_ – Source data to be encrypted or decrypted.\
  Must be encoded with `base64`.
* `key`: _string_ – 256-bit key.\
  Must be encoded with `hex`.
* `nonce`: _string_ – 96-bit nonce.\
  Must be encoded with `hex`.

#### Result

* `data`: _string_ – Encrypted/decrypted data.\
  Encoded with `base64`.

### create\_crypto\_box

Creates a Crypto Box instance.

Crypto Box is a root crypto object, that encapsulates some secret (seed phrase usually) in encrypted form and acts as a factory for all crypto primitives used in SDK: keys for signing and encryption, derived from this secret.

Crypto Box encrypts original Seed Phrase with salt and password that is retrieved from `password_provider` callback, implemented on Application side.

When used, decrypted secret shows up in core library's memory for a very short period of time and then is immediately overwritten with zeroes.

```ts
type ParamsOfCreateCryptoBox = {
    secret_encryption_salt: string,
    secret: CryptoBoxSecret
}

type RegisteredCryptoBox = {
    handle: CryptoBoxHandle
}

function create_crypto_box(
    params: ParamsOfCreateCryptoBox,
    obj: AppPasswordProvider,
): Promise<RegisteredCryptoBox>;

function create_crypto_box_sync(
    params: ParamsOfCreateCryptoBox,
): RegisteredCryptoBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `secret_encryption_salt`: _string_ – Salt used for secret encryption. For example, a mobile device can use device ID as salt.
* `secret`: [_CryptoBoxSecret_](mod_crypto.md#cryptoboxsecret) – Cryptobox secret
* `obj`: [AppPasswordProvider](mod_AppPasswordProvider.md#apppasswordprovider) – Interface that provides a callback that returns an encrypted password, used for cryptobox secret encryption

#### Result

* `handle`: [_CryptoBoxHandle_](mod_crypto.md#cryptoboxhandle)

### remove\_crypto\_box

Removes Crypto Box. Clears all secret data.

```ts
type RegisteredCryptoBox = {
    handle: CryptoBoxHandle
}

function remove_crypto_box(
    params: RegisteredCryptoBox,
): Promise<void>;

function remove_crypto_box_sync(
    params: RegisteredCryptoBox,
): void;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: [_CryptoBoxHandle_](mod_crypto.md#cryptoboxhandle)

### get\_crypto\_box\_info

Get Crypto Box Info. Used to get `encrypted_secret` that should be used for all the cryptobox initializations except the first one.

```ts
type RegisteredCryptoBox = {
    handle: CryptoBoxHandle
}

type ResultOfGetCryptoBoxInfo = {
    encrypted_secret: string
}

function get_crypto_box_info(
    params: RegisteredCryptoBox,
): Promise<ResultOfGetCryptoBoxInfo>;

function get_crypto_box_info_sync(
    params: RegisteredCryptoBox,
): ResultOfGetCryptoBoxInfo;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: [_CryptoBoxHandle_](mod_crypto.md#cryptoboxhandle)

#### Result

* `encrypted_secret`: _string_ – Secret (seed phrase) encrypted with salt and password.

### get\_crypto\_box\_seed\_phrase

Get Crypto Box Seed Phrase.

Attention! Store this data in your application for a very short period of time and overwrite it with zeroes ASAP.

```ts
type RegisteredCryptoBox = {
    handle: CryptoBoxHandle
}

type ResultOfGetCryptoBoxSeedPhrase = {
    phrase: string,
    dictionary: MnemonicDictionary,
    wordcount: number
}

function get_crypto_box_seed_phrase(
    params: RegisteredCryptoBox,
): Promise<ResultOfGetCryptoBoxSeedPhrase>;

function get_crypto_box_seed_phrase_sync(
    params: RegisteredCryptoBox,
): ResultOfGetCryptoBoxSeedPhrase;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: [_CryptoBoxHandle_](mod_crypto.md#cryptoboxhandle)

#### Result

* `phrase`: _string_
* `dictionary`: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary)
* `wordcount`: _number_

### get\_signing\_box\_from\_crypto\_box

Get handle of Signing Box derived from Crypto Box.

```ts
type ParamsOfGetSigningBoxFromCryptoBox = {
    handle: number,
    hdpath?: string,
    secret_lifetime?: number
}

type RegisteredSigningBox = {
    handle: SigningBoxHandle
}

function get_signing_box_from_crypto_box(
    params: ParamsOfGetSigningBoxFromCryptoBox,
): Promise<RegisteredSigningBox>;

function get_signing_box_from_crypto_box_sync(
    params: ParamsOfGetSigningBoxFromCryptoBox,
): RegisteredSigningBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: _number_ – Crypto Box Handle.
* `hdpath`?: _string_ – HD key derivation path.\
  By default, Acki Nacki HD path is used.
* `secret_lifetime`?: _number_ – Store derived secret for this lifetime (in ms). The timer starts after each signing box operation. Secrets will be deleted immediately after each signing box operation, if this value is not set.

#### Result

* `handle`: [_SigningBoxHandle_](mod_crypto.md#signingboxhandle) – Handle of the signing box.

### get\_encryption\_box\_from\_crypto\_box

Gets Encryption Box from Crypto Box.

Derives encryption keypair from cryptobox secret and hdpath and stores it in cache for `secret_lifetime` or until explicitly cleared by `clear_crypto_box_secret_cache` method. If `secret_lifetime` is not specified - overwrites encryption secret with zeroes immediately after encryption operation.

```ts
type ParamsOfGetEncryptionBoxFromCryptoBox = {
    handle: number,
    hdpath?: string,
    algorithm: BoxEncryptionAlgorithm,
    secret_lifetime?: number
}

type RegisteredEncryptionBox = {
    handle: EncryptionBoxHandle
}

function get_encryption_box_from_crypto_box(
    params: ParamsOfGetEncryptionBoxFromCryptoBox,
): Promise<RegisteredEncryptionBox>;

function get_encryption_box_from_crypto_box_sync(
    params: ParamsOfGetEncryptionBoxFromCryptoBox,
): RegisteredEncryptionBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: _number_ – Crypto Box Handle.
* `hdpath`?: _string_ – HD key derivation path.\
  By default, Acki Nacki HD path is used.
* `algorithm`: [_BoxEncryptionAlgorithm_](mod_crypto.md#boxencryptionalgorithm) – Encryption algorithm.
* `secret_lifetime`?: _number_ – Store derived secret for encryption algorithm for this lifetime (in ms). The timer starts after each encryption box operation. Secrets will be deleted (overwritten with zeroes) after each encryption operation, if this value is not set.

#### Result

* `handle`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Handle of the encryption box.

### clear\_crypto\_box\_secret\_cache

Removes cached secrets (overwrites with zeroes) from all signing and encryption boxes, derived from crypto box.

```ts
type RegisteredCryptoBox = {
    handle: CryptoBoxHandle
}

function clear_crypto_box_secret_cache(
    params: RegisteredCryptoBox,
): Promise<void>;

function clear_crypto_box_secret_cache_sync(
    params: RegisteredCryptoBox,
): void;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: [_CryptoBoxHandle_](mod_crypto.md#cryptoboxhandle)

### register\_signing\_box

Register an application implemented signing box.

```ts
type RegisteredSigningBox = {
    handle: SigningBoxHandle
}

function register_signing_box(
    obj: AppSigningBox,
): Promise<RegisteredSigningBox>;

function register_signing_box_sync(): RegisteredSigningBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `obj`: [AppSigningBox](mod_AppSigningBox.md#appsigningbox) – Signing box callbacks.

#### Result

* `handle`: [_SigningBoxHandle_](mod_crypto.md#signingboxhandle) – Handle of the signing box.

### get\_signing\_box

Creates a default signing box implementation.

```ts
type KeyPair = {
    public: string,
    secret: string
}

type RegisteredSigningBox = {
    handle: SigningBoxHandle
}

function get_signing_box(
    params: KeyPair,
): Promise<RegisteredSigningBox>;

function get_signing_box_sync(
    params: KeyPair,
): RegisteredSigningBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `public`: _string_ – Public key - 64 symbols hex string
* `secret`: _string_ – Private key - u64 symbols hex string

#### Result

* `handle`: [_SigningBoxHandle_](mod_crypto.md#signingboxhandle) – Handle of the signing box.

### signing\_box\_get\_public\_key

Returns public key of signing key pair.

```ts
type RegisteredSigningBox = {
    handle: SigningBoxHandle
}

type ResultOfSigningBoxGetPublicKey = {
    pubkey: string
}

function signing_box_get_public_key(
    params: RegisteredSigningBox,
): Promise<ResultOfSigningBoxGetPublicKey>;

function signing_box_get_public_key_sync(
    params: RegisteredSigningBox,
): ResultOfSigningBoxGetPublicKey;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: [_SigningBoxHandle_](mod_crypto.md#signingboxhandle) – Handle of the signing box.

#### Result

* `pubkey`: _string_ – Public key of signing box.\
  Encoded with hex

### signing\_box\_sign

Returns signed user data.

```ts
type ParamsOfSigningBoxSign = {
    signing_box: SigningBoxHandle,
    unsigned: string
}

type ResultOfSigningBoxSign = {
    signature: string
}

function signing_box_sign(
    params: ParamsOfSigningBoxSign,
): Promise<ResultOfSigningBoxSign>;

function signing_box_sign_sync(
    params: ParamsOfSigningBoxSign,
): ResultOfSigningBoxSign;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `signing_box`: [_SigningBoxHandle_](mod_crypto.md#signingboxhandle) – Signing Box handle.
* `unsigned`: _string_ – Unsigned user data.\
  Must be encoded with `base64`.

#### Result

* `signature`: _string_ – Data signature.\
  Encoded with `hex`.

### remove\_signing\_box

Removes signing box from SDK.

```ts
type RegisteredSigningBox = {
    handle: SigningBoxHandle
}

function remove_signing_box(
    params: RegisteredSigningBox,
): Promise<void>;

function remove_signing_box_sync(
    params: RegisteredSigningBox,
): void;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: [_SigningBoxHandle_](mod_crypto.md#signingboxhandle) – Handle of the signing box.

### register\_encryption\_box

Register an application implemented encryption box.

```ts
type RegisteredEncryptionBox = {
    handle: EncryptionBoxHandle
}

function register_encryption_box(
    obj: AppEncryptionBox,
): Promise<RegisteredEncryptionBox>;

function register_encryption_box_sync(): RegisteredEncryptionBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `obj`: [AppEncryptionBox](mod_AppEncryptionBox.md#appencryptionbox) – Interface for data encryption/decryption

#### Result

* `handle`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Handle of the encryption box.

### remove\_encryption\_box

Removes encryption box from SDK

```ts
type RegisteredEncryptionBox = {
    handle: EncryptionBoxHandle
}

function remove_encryption_box(
    params: RegisteredEncryptionBox,
): Promise<void>;

function remove_encryption_box_sync(
    params: RegisteredEncryptionBox,
): void;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `handle`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Handle of the encryption box.

### encryption\_box\_get\_info

Queries info from the given encryption box

```ts
type ParamsOfEncryptionBoxGetInfo = {
    encryption_box: EncryptionBoxHandle
}

type ResultOfEncryptionBoxGetInfo = {
    info: EncryptionBoxInfo
}

function encryption_box_get_info(
    params: ParamsOfEncryptionBoxGetInfo,
): Promise<ResultOfEncryptionBoxGetInfo>;

function encryption_box_get_info_sync(
    params: ParamsOfEncryptionBoxGetInfo,
): ResultOfEncryptionBoxGetInfo;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `encryption_box`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Encryption box handle

#### Result

* `info`: [_EncryptionBoxInfo_](mod_crypto.md#encryptionboxinfo) – Encryption box information

### encryption\_box\_encrypt

Encrypts data using given encryption box Note.

Block cipher algorithms pad data to cipher block size so encrypted data can be longer then original data. Client should store the original data size after encryption and use it after decryption to retrieve the original data from decrypted data.

```ts
type ParamsOfEncryptionBoxEncrypt = {
    encryption_box: EncryptionBoxHandle,
    data: string
}

type ResultOfEncryptionBoxEncrypt = {
    data: string
}

function encryption_box_encrypt(
    params: ParamsOfEncryptionBoxEncrypt,
): Promise<ResultOfEncryptionBoxEncrypt>;

function encryption_box_encrypt_sync(
    params: ParamsOfEncryptionBoxEncrypt,
): ResultOfEncryptionBoxEncrypt;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `encryption_box`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Encryption box handle
* `data`: _string_ – Data to be encrypted, encoded in Base64

#### Result

* `data`: _string_ – Encrypted data, encoded in Base64.\
  Padded to cipher block size

### encryption\_box\_decrypt

Decrypts data using given encryption box Note.

Block cipher algorithms pad data to cipher block size so encrypted data can be longer then original data. Client should store the original data size after encryption and use it after decryption to retrieve the original data from decrypted data.

```ts
type ParamsOfEncryptionBoxDecrypt = {
    encryption_box: EncryptionBoxHandle,
    data: string
}

type ResultOfEncryptionBoxDecrypt = {
    data: string
}

function encryption_box_decrypt(
    params: ParamsOfEncryptionBoxDecrypt,
): Promise<ResultOfEncryptionBoxDecrypt>;

function encryption_box_decrypt_sync(
    params: ParamsOfEncryptionBoxDecrypt,
): ResultOfEncryptionBoxDecrypt;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `encryption_box`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Encryption box handle
* `data`: _string_ – Data to be decrypted, encoded in Base64

#### Result

* `data`: _string_ – Decrypted data, encoded in Base64.

### create\_encryption\_box

Creates encryption box with specified algorithm

```ts
type ParamsOfCreateEncryptionBox = {
    algorithm: EncryptionAlgorithm
}

type RegisteredEncryptionBox = {
    handle: EncryptionBoxHandle
}

function create_encryption_box(
    params: ParamsOfCreateEncryptionBox,
): Promise<RegisteredEncryptionBox>;

function create_encryption_box_sync(
    params: ParamsOfCreateEncryptionBox,
): RegisteredEncryptionBox;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `algorithm`: [_EncryptionAlgorithm_](mod_crypto.md#encryptionalgorithm) – Encryption algorithm specifier including cipher parameters (key, IV, etc)

#### Result

* `handle`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Handle of the encryption box.

## Types

### CryptoErrorCode

```ts
enum CryptoErrorCode {
    InvalidPublicKey = 100,
    InvalidSecretKey = 101,
    InvalidKey = 102,
    InvalidFactorizeChallenge = 106,
    InvalidBigInt = 107,
    ScryptFailed = 108,
    InvalidKeySize = 109,
    NaclSecretBoxFailed = 110,
    NaclBoxFailed = 111,
    NaclSignFailed = 112,
    Bip39InvalidEntropy = 113,
    Bip39InvalidPhrase = 114,
    Bip32InvalidKey = 115,
    Bip32InvalidDerivePath = 116,
    Bip39InvalidDictionary = 117,
    Bip39InvalidWordCount = 118,
    MnemonicGenerationFailed = 119,
    MnemonicFromEntropyFailed = 120,
    SigningBoxNotRegistered = 121,
    InvalidSignature = 122,
    EncryptionBoxNotRegistered = 123,
    InvalidIvSize = 124,
    UnsupportedCipherMode = 125,
    CannotCreateCipher = 126,
    EncryptDataError = 127,
    DecryptDataError = 128,
    IvRequired = 129,
    CryptoBoxNotRegistered = 130,
    InvalidCryptoBoxType = 131,
    CryptoBoxSecretSerializationError = 132,
    CryptoBoxSecretDeserializationError = 133,
    InvalidNonceSize = 134
}
```

One of the following value:

* `InvalidPublicKey = 100`
* `InvalidSecretKey = 101`
* `InvalidKey = 102`
* `InvalidFactorizeChallenge = 106`
* `InvalidBigInt = 107`
* `ScryptFailed = 108`
* `InvalidKeySize = 109`
* `NaclSecretBoxFailed = 110`
* `NaclBoxFailed = 111`
* `NaclSignFailed = 112`
* `Bip39InvalidEntropy = 113`
* `Bip39InvalidPhrase = 114`
* `Bip32InvalidKey = 115`
* `Bip32InvalidDerivePath = 116`
* `Bip39InvalidDictionary = 117`
* `Bip39InvalidWordCount = 118`
* `MnemonicGenerationFailed = 119`
* `MnemonicFromEntropyFailed = 120`
* `SigningBoxNotRegistered = 121`
* `InvalidSignature = 122`
* `EncryptionBoxNotRegistered = 123`
* `InvalidIvSize = 124`
* `UnsupportedCipherMode = 125`
* `CannotCreateCipher = 126`
* `EncryptDataError = 127`
* `DecryptDataError = 128`
* `IvRequired = 129`
* `CryptoBoxNotRegistered = 130`
* `InvalidCryptoBoxType = 131`
* `CryptoBoxSecretSerializationError = 132`
* `CryptoBoxSecretDeserializationError = 133`
* `InvalidNonceSize = 134`

### SigningBoxHandle

```ts
type SigningBoxHandle = number
```

### EncryptionBoxHandle

```ts
type EncryptionBoxHandle = number
```

### EncryptionBoxInfo

Encryption box information.

```ts
type EncryptionBoxInfo = {
    hdpath?: string,
    algorithm?: string,
    options?: any,
    public?: any
}
```

* `hdpath`?: _string_ – Derivation path, for instance "m/44'/396'/0'/0/0"
* `algorithm`?: _string_ – Cryptographic algorithm, used by this encryption box
* `options`?: _any_ – Options, depends on algorithm and specific encryption box implementation
* `public`?: _any_ – Public information, depends on algorithm

### EncryptionAlgorithmAESVariant

```ts
type EncryptionAlgorithmAESVariant = {
    value: AesParamsEB
}
```

* `value`: [_AesParamsEB_](mod_crypto.md#aesparamseb)

### EncryptionAlgorithmChaCha20Variant

```ts
type EncryptionAlgorithmChaCha20Variant = {
    value: ChaCha20ParamsEB
}
```

* `value`: [_ChaCha20ParamsEB_](mod_crypto.md#chacha20paramseb)

### EncryptionAlgorithmNaclBoxVariant

```ts
type EncryptionAlgorithmNaclBoxVariant = {
    value: NaclBoxParamsEB
}
```

* `value`: [_NaclBoxParamsEB_](mod_crypto.md#naclboxparamseb)

### EncryptionAlgorithmNaclSecretBoxVariant

```ts
type EncryptionAlgorithmNaclSecretBoxVariant = {
    value: NaclSecretBoxParamsEB
}
```

* `value`: [_NaclSecretBoxParamsEB_](mod_crypto.md#naclsecretboxparamseb)

### EncryptionAlgorithm

```ts
type EncryptionAlgorithm = ({
    type: 'AES'
} & EncryptionAlgorithmAESVariant) | ({
    type: 'ChaCha20'
} & EncryptionAlgorithmChaCha20Variant) | ({
    type: 'NaclBox'
} & EncryptionAlgorithmNaclBoxVariant) | ({
    type: 'NaclSecretBox'
} & EncryptionAlgorithmNaclSecretBoxVariant)
```

Depends on value of the `type` field.

When _type_ is _'AES'_

* `value`: [_AesParamsEB_](mod_crypto.md#aesparamseb)

When _type_ is _'ChaCha20'_

* `value`: [_ChaCha20ParamsEB_](mod_crypto.md#chacha20paramseb)

When _type_ is _'NaclBox'_

* `value`: [_NaclBoxParamsEB_](mod_crypto.md#naclboxparamseb)

When _type_ is _'NaclSecretBox'_

* `value`: [_NaclSecretBoxParamsEB_](mod_crypto.md#naclsecretboxparamseb)

Variant constructors:

```ts
function encryptionAlgorithmAES(value: AesParamsEB): EncryptionAlgorithm;
function encryptionAlgorithmChaCha20(value: ChaCha20ParamsEB): EncryptionAlgorithm;
function encryptionAlgorithmNaclBox(value: NaclBoxParamsEB): EncryptionAlgorithm;
function encryptionAlgorithmNaclSecretBox(value: NaclSecretBoxParamsEB): EncryptionAlgorithm;
```

### CipherMode

```ts
enum CipherMode {
    CBC = "CBC",
    CFB = "CFB",
    CTR = "CTR",
    ECB = "ECB",
    OFB = "OFB"
}
```

One of the following value:

* `CBC = "CBC"`
* `CFB = "CFB"`
* `CTR = "CTR"`
* `ECB = "ECB"`
* `OFB = "OFB"`

### AesParamsEB

```ts
type AesParamsEB = {
    mode: CipherMode,
    key: string,
    iv?: string
}
```

* `mode`: [_CipherMode_](mod_crypto.md#ciphermode)
* `key`: _string_
* `iv`?: _string_

### AesInfo

```ts
type AesInfo = {
    mode: CipherMode,
    iv?: string
}
```

* `mode`: [_CipherMode_](mod_crypto.md#ciphermode)
* `iv`?: _string_

### ChaCha20ParamsEB

```ts
type ChaCha20ParamsEB = {
    key: string,
    nonce: string
}
```

* `key`: _string_ – 256-bit key.\
  Must be encoded with `hex`.
* `nonce`: _string_ – 96-bit nonce.\
  Must be encoded with `hex`.

### NaclBoxParamsEB

```ts
type NaclBoxParamsEB = {
    their_public: string,
    secret: string,
    nonce: string
}
```

* `their_public`: _string_ – 256-bit key.\
  Must be encoded with `hex`.
* `secret`: _string_ – 256-bit key.\
  Must be encoded with `hex`.
* `nonce`: _string_ – 96-bit nonce.\
  Must be encoded with `hex`.

### NaclSecretBoxParamsEB

```ts
type NaclSecretBoxParamsEB = {
    key: string,
    nonce: string
}
```

* `key`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string
* `nonce`: _string_ – Nonce in `hex`

### CryptoBoxSecretRandomSeedPhraseVariant

Creates Crypto Box from a random seed phrase. This option can be used if a developer doesn't want the seed phrase to leave the core library's memory, where it is stored encrypted.

This type should be used upon the first wallet initialization, all further initializations should use `EncryptedSecret` type instead.

Get `encrypted_secret` with `get_crypto_box_info` function and store it on your side.

```ts
type CryptoBoxSecretRandomSeedPhraseVariant = {
    dictionary: MnemonicDictionary,
    wordcount: number
}
```

* `dictionary`: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary)
* `wordcount`: _number_

### CryptoBoxSecretPredefinedSeedPhraseVariant

Restores crypto box instance from an existing seed phrase. This type should be used when Crypto Box is initialized from a seed phrase, entered by a user.

This type should be used only upon the first wallet initialization, all further initializations should use `EncryptedSecret` type instead.

Get `encrypted_secret` with `get_crypto_box_info` function and store it on your side.

```ts
type CryptoBoxSecretPredefinedSeedPhraseVariant = {
    phrase: string,
    dictionary: MnemonicDictionary,
    wordcount: number
}
```

* `phrase`: _string_
* `dictionary`: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary)
* `wordcount`: _number_

### CryptoBoxSecretEncryptedSecretVariant

Use this type for wallet reinitializations, when you already have `encrypted_secret` on hands. To get `encrypted_secret`, use `get_crypto_box_info` function after you initialized your crypto box for the first time.

It is an object, containing seed phrase or private key, encrypted with `secret_encryption_salt` and password from `password_provider`.

Note that if you want to change salt or password provider, then you need to reinitialize the wallet with `PredefinedSeedPhrase`, then get `EncryptedSecret` via `get_crypto_box_info`, store it somewhere, and only after that initialize the wallet with `EncryptedSecret` type.

```ts
type CryptoBoxSecretEncryptedSecretVariant = {
    encrypted_secret: string
}
```

* `encrypted_secret`: _string_ – It is an object, containing encrypted seed phrase or private key (now we support only seed phrase).

### CryptoBoxSecret

Crypto Box Secret.

```ts
type CryptoBoxSecret = ({
    type: 'RandomSeedPhrase'
} & CryptoBoxSecretRandomSeedPhraseVariant) | ({
    type: 'PredefinedSeedPhrase'
} & CryptoBoxSecretPredefinedSeedPhraseVariant) | ({
    type: 'EncryptedSecret'
} & CryptoBoxSecretEncryptedSecretVariant)
```

Depends on value of the `type` field.

When _type_ is _'RandomSeedPhrase'_

Creates Crypto Box from a random seed phrase. This option can be used if a developer doesn't want the seed phrase to leave the core library's memory, where it is stored encrypted.

This type should be used upon the first wallet initialization, all further initializations should use `EncryptedSecret` type instead.

Get `encrypted_secret` with `get_crypto_box_info` function and store it on your side.

* `dictionary`: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary)
* `wordcount`: _number_

When _type_ is _'PredefinedSeedPhrase'_

Restores crypto box instance from an existing seed phrase. This type should be used when Crypto Box is initialized from a seed phrase, entered by a user.

This type should be used only upon the first wallet initialization, all further initializations should use `EncryptedSecret` type instead.

Get `encrypted_secret` with `get_crypto_box_info` function and store it on your side.

* `phrase`: _string_
* `dictionary`: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary)
* `wordcount`: _number_

When _type_ is _'EncryptedSecret'_

Use this type for wallet reinitializations, when you already have `encrypted_secret` on hands. To get `encrypted_secret`, use `get_crypto_box_info` function after you initialized your crypto box for the first time.

It is an object, containing seed phrase or private key, encrypted with `secret_encryption_salt` and password from `password_provider`.

Note that if you want to change salt or password provider, then you need to reinitialize the wallet with `PredefinedSeedPhrase`, then get `EncryptedSecret` via `get_crypto_box_info`, store it somewhere, and only after that initialize the wallet with `EncryptedSecret` type.

* `encrypted_secret`: _string_ – It is an object, containing encrypted seed phrase or private key (now we support only seed phrase).

Variant constructors:

```ts
function cryptoBoxSecretRandomSeedPhrase(dictionary: MnemonicDictionary, wordcount: number): CryptoBoxSecret;
function cryptoBoxSecretPredefinedSeedPhrase(phrase: string, dictionary: MnemonicDictionary, wordcount: number): CryptoBoxSecret;
function cryptoBoxSecretEncryptedSecret(encrypted_secret: string): CryptoBoxSecret;
```

### CryptoBoxHandle

```ts
type CryptoBoxHandle = number
```

### BoxEncryptionAlgorithmChaCha20Variant

```ts
type BoxEncryptionAlgorithmChaCha20Variant = {
    value: ChaCha20ParamsCB
}
```

* `value`: [_ChaCha20ParamsCB_](mod_crypto.md#chacha20paramscb)

### BoxEncryptionAlgorithmNaclBoxVariant

```ts
type BoxEncryptionAlgorithmNaclBoxVariant = {
    value: NaclBoxParamsCB
}
```

* `value`: [_NaclBoxParamsCB_](mod_crypto.md#naclboxparamscb)

### BoxEncryptionAlgorithmNaclSecretBoxVariant

```ts
type BoxEncryptionAlgorithmNaclSecretBoxVariant = {
    value: NaclSecretBoxParamsCB
}
```

* `value`: [_NaclSecretBoxParamsCB_](mod_crypto.md#naclsecretboxparamscb)

### BoxEncryptionAlgorithm

```ts
type BoxEncryptionAlgorithm = ({
    type: 'ChaCha20'
} & BoxEncryptionAlgorithmChaCha20Variant) | ({
    type: 'NaclBox'
} & BoxEncryptionAlgorithmNaclBoxVariant) | ({
    type: 'NaclSecretBox'
} & BoxEncryptionAlgorithmNaclSecretBoxVariant)
```

Depends on value of the `type` field.

When _type_ is _'ChaCha20'_

* `value`: [_ChaCha20ParamsCB_](mod_crypto.md#chacha20paramscb)

When _type_ is _'NaclBox'_

* `value`: [_NaclBoxParamsCB_](mod_crypto.md#naclboxparamscb)

When _type_ is _'NaclSecretBox'_

* `value`: [_NaclSecretBoxParamsCB_](mod_crypto.md#naclsecretboxparamscb)

Variant constructors:

```ts
function boxEncryptionAlgorithmChaCha20(value: ChaCha20ParamsCB): BoxEncryptionAlgorithm;
function boxEncryptionAlgorithmNaclBox(value: NaclBoxParamsCB): BoxEncryptionAlgorithm;
function boxEncryptionAlgorithmNaclSecretBox(value: NaclSecretBoxParamsCB): BoxEncryptionAlgorithm;
```

### ChaCha20ParamsCB

```ts
type ChaCha20ParamsCB = {
    nonce: string
}
```

* `nonce`: _string_ – 96-bit nonce.\
  Must be encoded with `hex`.

### NaclBoxParamsCB

```ts
type NaclBoxParamsCB = {
    their_public: string,
    nonce: string
}
```

* `their_public`: _string_ – 256-bit key.\
  Must be encoded with `hex`.
* `nonce`: _string_ – 96-bit nonce.\
  Must be encoded with `hex`.

### NaclSecretBoxParamsCB

```ts
type NaclSecretBoxParamsCB = {
    nonce: string
}
```

* `nonce`: _string_ – Nonce in `hex`

### MnemonicDictionary

```ts
enum MnemonicDictionary {
    Ton = 0,
    English = 1,
    ChineseSimplified = 2,
    ChineseTraditional = 3,
    French = 4,
    Italian = 5,
    Japanese = 6,
    Korean = 7,
    Spanish = 8
}
```

One of the following value:

* `Ton = 0` – TON compatible dictionary
* `English = 1` – English BIP-39 dictionary
* `ChineseSimplified = 2` – Chinese simplified BIP-39 dictionary
* `ChineseTraditional = 3` – Chinese traditional BIP-39 dictionary
* `French = 4` – French BIP-39 dictionary
* `Italian = 5` – Italian BIP-39 dictionary
* `Japanese = 6` – Japanese BIP-39 dictionary
* `Korean = 7` – Korean BIP-39 dictionary
* `Spanish = 8` – Spanish BIP-39 dictionary

### ParamsOfFactorize

```ts
type ParamsOfFactorize = {
    composite: string
}
```

* `composite`: _string_ – Hexadecimal representation of u64 composite number.

### ResultOfFactorize

```ts
type ResultOfFactorize = {
    factors: string[]
}
```

* `factors`: _string\[]_ – Two factors of composite or empty if composite can't be factorized.

### ParamsOfModularPower

```ts
type ParamsOfModularPower = {
    base: string,
    exponent: string,
    modulus: string
}
```

* `base`: _string_ – `base` argument of calculation.
* `exponent`: _string_ – `exponent` argument of calculation.
* `modulus`: _string_ – `modulus` argument of calculation.

### ResultOfModularPower

```ts
type ResultOfModularPower = {
    modular_power: string
}
```

* `modular_power`: _string_ – Result of modular exponentiation

### ParamsOfTonCrc16

```ts
type ParamsOfTonCrc16 = {
    data: string
}
```

* `data`: _string_ – Input data for CRC calculation.\
  Encoded with `base64`.

### ResultOfTonCrc16

```ts
type ResultOfTonCrc16 = {
    crc: number
}
```

* `crc`: _number_ – Calculated CRC for input data.

### ParamsOfGenerateRandomBytes

```ts
type ParamsOfGenerateRandomBytes = {
    length: number
}
```

* `length`: _number_ – Size of random byte array.

### ResultOfGenerateRandomBytes

```ts
type ResultOfGenerateRandomBytes = {
    bytes: string
}
```

* `bytes`: _string_ – Generated bytes encoded in `base64`.

### ParamsOfConvertPublicKeyToTonSafeFormat

```ts
type ParamsOfConvertPublicKeyToTonSafeFormat = {
    public_key: string
}
```

* `public_key`: _string_ – Public key - 64 symbols hex string

### ResultOfConvertPublicKeyToTonSafeFormat

```ts
type ResultOfConvertPublicKeyToTonSafeFormat = {
    ton_public_key: string
}
```

* `ton_public_key`: _string_ – Public key represented in TON safe format.

### KeyPair

```ts
type KeyPair = {
    public: string,
    secret: string
}
```

* `public`: _string_ – Public key - 64 symbols hex string
* `secret`: _string_ – Private key - u64 symbols hex string

### ParamsOfSign

```ts
type ParamsOfSign = {
    unsigned: string,
    keys: KeyPair
}
```

* `unsigned`: _string_ – Data that must be signed encoded in `base64`.
* `keys`: [_KeyPair_](mod_crypto.md#keypair) – Sign keys.

### ResultOfSign

```ts
type ResultOfSign = {
    signed: string,
    signature: string
}
```

* `signed`: _string_ – Signed data combined with signature encoded in `base64`.
* `signature`: _string_ – Signature encoded in `hex`.

### ParamsOfVerifySignature

```ts
type ParamsOfVerifySignature = {
    signed: string,
    public: string
}
```

* `signed`: _string_ – Signed data that must be verified encoded in `base64`.
* `public`: _string_ – Signer's public key - 64 symbols hex string

### ResultOfVerifySignature

```ts
type ResultOfVerifySignature = {
    unsigned: string
}
```

* `unsigned`: _string_ – Unsigned data encoded in `base64`.

### ParamsOfHash

```ts
type ParamsOfHash = {
    data: string
}
```

* `data`: _string_ – Input data for hash calculation.\
  Encoded with `base64`.

### ResultOfHash

```ts
type ResultOfHash = {
    hash: string
}
```

* `hash`: _string_ – Hash of input `data`.\
  Encoded with 'hex'.

### ParamsOfScrypt

```ts
type ParamsOfScrypt = {
    password: string,
    salt: string,
    log_n: number,
    r: number,
    p: number,
    dk_len: number
}
```

* `password`: _string_ – The password bytes to be hashed. Must be encoded with `base64`.
* `salt`: _string_ – Salt bytes that modify the hash to protect against Rainbow table attacks. Must be encoded with `base64`.
* `log_n`: _number_ – CPU/memory cost parameter
* `r`: _number_ – The block size parameter, which fine-tunes sequential memory read size and performance.
* `p`: _number_ – Parallelization parameter.
* `dk_len`: _number_ – Intended output length in octets of the derived key.

### ResultOfScrypt

```ts
type ResultOfScrypt = {
    key: string
}
```

* `key`: _string_ – Derived key.\
  Encoded with `hex`.

### ParamsOfNaclSignKeyPairFromSecret

```ts
type ParamsOfNaclSignKeyPairFromSecret = {
    secret: string
}
```

* `secret`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string

### ParamsOfNaclSign

```ts
type ParamsOfNaclSign = {
    unsigned: string,
    secret: string
}
```

* `unsigned`: _string_ – Data that must be signed encoded in `base64`.
* `secret`: _string_ – Signer's secret key - unprefixed 0-padded to 128 symbols hex string (concatenation of 64 symbols secret and 64 symbols public keys). See `nacl_sign_keypair_from_secret_key`.

### ResultOfNaclSign

```ts
type ResultOfNaclSign = {
    signed: string
}
```

* `signed`: _string_ – Signed data, encoded in `base64`.

### ParamsOfNaclSignOpen

```ts
type ParamsOfNaclSignOpen = {
    signed: string,
    public: string
}
```

* `signed`: _string_ – Signed data that must be unsigned.\
  Encoded with `base64`.
* `public`: _string_ – Signer's public key - unprefixed 0-padded to 64 symbols hex string

### ResultOfNaclSignOpen

```ts
type ResultOfNaclSignOpen = {
    unsigned: string
}
```

* `unsigned`: _string_ – Unsigned data, encoded in `base64`.

### ResultOfNaclSignDetached

```ts
type ResultOfNaclSignDetached = {
    signature: string
}
```

* `signature`: _string_ – Signature encoded in `hex`.

### ParamsOfNaclSignDetachedVerify

```ts
type ParamsOfNaclSignDetachedVerify = {
    unsigned: string,
    signature: string,
    public: string
}
```

* `unsigned`: _string_ – Unsigned data that must be verified.\
  Encoded with `base64`.
* `signature`: _string_ – Signature that must be verified.\
  Encoded with `hex`.
* `public`: _string_ – Signer's public key - unprefixed 0-padded to 64 symbols hex string.

### ResultOfNaclSignDetachedVerify

```ts
type ResultOfNaclSignDetachedVerify = {
    succeeded: boolean
}
```

* `succeeded`: _boolean_ – `true` if verification succeeded or `false` if it failed

### ParamsOfNaclBoxKeyPairFromSecret

```ts
type ParamsOfNaclBoxKeyPairFromSecret = {
    secret: string
}
```

* `secret`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string

### ParamsOfNaclBox

```ts
type ParamsOfNaclBox = {
    decrypted: string,
    nonce: string,
    their_public: string,
    secret: string
}
```

* `decrypted`: _string_ – Data that must be encrypted encoded in `base64`.
* `nonce`: _string_ – Nonce, encoded in `hex`
* `their_public`: _string_ – Receiver's public key - unprefixed 0-padded to 64 symbols hex string
* `secret`: _string_ – Sender's private key - unprefixed 0-padded to 64 symbols hex string

### ResultOfNaclBox

```ts
type ResultOfNaclBox = {
    encrypted: string
}
```

* `encrypted`: _string_ – Encrypted data encoded in `base64`.

### ParamsOfNaclBoxOpen

```ts
type ParamsOfNaclBoxOpen = {
    encrypted: string,
    nonce: string,
    their_public: string,
    secret: string
}
```

* `encrypted`: _string_ – Data that must be decrypted.\
  Encoded with `base64`.
* `nonce`: _string_ – Nonce
* `their_public`: _string_ – Sender's public key - unprefixed 0-padded to 64 symbols hex string
* `secret`: _string_ – Receiver's private key - unprefixed 0-padded to 64 symbols hex string

### ResultOfNaclBoxOpen

```ts
type ResultOfNaclBoxOpen = {
    decrypted: string
}
```

* `decrypted`: _string_ – Decrypted data encoded in `base64`.

### ParamsOfNaclSecretBox

```ts
type ParamsOfNaclSecretBox = {
    decrypted: string,
    nonce: string,
    key: string
}
```

* `decrypted`: _string_ – Data that must be encrypted.\
  Encoded with `base64`.
* `nonce`: _string_ – Nonce in `hex`
* `key`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string

### ParamsOfNaclSecretBoxOpen

```ts
type ParamsOfNaclSecretBoxOpen = {
    encrypted: string,
    nonce: string,
    key: string
}
```

* `encrypted`: _string_ – Data that must be decrypted.\
  Encoded with `base64`.
* `nonce`: _string_ – Nonce in `hex`
* `key`: _string_ – Secret key - unprefixed 0-padded to 64 symbols hex string

### ParamsOfMnemonicWords

```ts
type ParamsOfMnemonicWords = {
    dictionary?: MnemonicDictionary
}
```

* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier

### ResultOfMnemonicWords

```ts
type ResultOfMnemonicWords = {
    words: string
}
```

* `words`: _string_ – The list of mnemonic words

### ParamsOfMnemonicFromRandom

```ts
type ParamsOfMnemonicFromRandom = {
    dictionary?: MnemonicDictionary,
    word_count?: number
}
```

* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Mnemonic word count

### ResultOfMnemonicFromRandom

```ts
type ResultOfMnemonicFromRandom = {
    phrase: string
}
```

* `phrase`: _string_ – String of mnemonic words

### ParamsOfMnemonicFromEntropy

```ts
type ParamsOfMnemonicFromEntropy = {
    entropy: string,
    dictionary?: MnemonicDictionary,
    word_count?: number
}
```

* `entropy`: _string_ – Entropy bytes.\
  Hex encoded.
* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Mnemonic word count

### ResultOfMnemonicFromEntropy

```ts
type ResultOfMnemonicFromEntropy = {
    phrase: string
}
```

* `phrase`: _string_ – Phrase

### ParamsOfMnemonicVerify

```ts
type ParamsOfMnemonicVerify = {
    phrase: string,
    dictionary?: MnemonicDictionary,
    word_count?: number
}
```

* `phrase`: _string_ – Phrase
* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Word count

### ResultOfMnemonicVerify

```ts
type ResultOfMnemonicVerify = {
    valid: boolean
}
```

* `valid`: _boolean_ – Flag indicating if the mnemonic is valid or not

### ParamsOfMnemonicDeriveSignKeys

```ts
type ParamsOfMnemonicDeriveSignKeys = {
    phrase: string,
    path?: string,
    dictionary?: MnemonicDictionary,
    word_count?: number
}
```

* `phrase`: _string_ – Phrase
* `path`?: _string_ – Derivation path, for instance "m/44'/396'/0'/0/0"
* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Word count

### ParamsOfHDKeyXPrvFromMnemonic

```ts
type ParamsOfHDKeyXPrvFromMnemonic = {
    phrase: string,
    dictionary?: MnemonicDictionary,
    word_count?: number
}
```

* `phrase`: _string_ – String with seed phrase
* `dictionary`?: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary) – Dictionary identifier
* `word_count`?: _number_ – Mnemonic word count

### ResultOfHDKeyXPrvFromMnemonic

```ts
type ResultOfHDKeyXPrvFromMnemonic = {
    xprv: string
}
```

* `xprv`: _string_ – Serialized extended master private key

### ParamsOfHDKeyDeriveFromXPrv

```ts
type ParamsOfHDKeyDeriveFromXPrv = {
    xprv: string,
    child_index: number,
    hardened: boolean
}
```

* `xprv`: _string_ – Serialized extended private key
* `child_index`: _number_ – Child index (see BIP-0032)
* `hardened`: _boolean_ – Indicates the derivation of hardened/not-hardened key (see BIP-0032)

### ResultOfHDKeyDeriveFromXPrv

```ts
type ResultOfHDKeyDeriveFromXPrv = {
    xprv: string
}
```

* `xprv`: _string_ – Serialized extended private key

### ParamsOfHDKeyDeriveFromXPrvPath

```ts
type ParamsOfHDKeyDeriveFromXPrvPath = {
    xprv: string,
    path: string
}
```

* `xprv`: _string_ – Serialized extended private key
* `path`: _string_ – Derivation path, for instance "m/44'/396'/0'/0/0"

### ResultOfHDKeyDeriveFromXPrvPath

```ts
type ResultOfHDKeyDeriveFromXPrvPath = {
    xprv: string
}
```

* `xprv`: _string_ – Derived serialized extended private key

### ParamsOfHDKeySecretFromXPrv

```ts
type ParamsOfHDKeySecretFromXPrv = {
    xprv: string
}
```

* `xprv`: _string_ – Serialized extended private key

### ResultOfHDKeySecretFromXPrv

```ts
type ResultOfHDKeySecretFromXPrv = {
    secret: string
}
```

* `secret`: _string_ – Private key - 64 symbols hex string

### ParamsOfHDKeyPublicFromXPrv

```ts
type ParamsOfHDKeyPublicFromXPrv = {
    xprv: string
}
```

* `xprv`: _string_ – Serialized extended private key

### ResultOfHDKeyPublicFromXPrv

```ts
type ResultOfHDKeyPublicFromXPrv = {
    public: string
}
```

* `public`: _string_ – Public key - 64 symbols hex string

### ParamsOfChaCha20

```ts
type ParamsOfChaCha20 = {
    data: string,
    key: string,
    nonce: string
}
```

* `data`: _string_ – Source data to be encrypted or decrypted.\
  Must be encoded with `base64`.
* `key`: _string_ – 256-bit key.\
  Must be encoded with `hex`.
* `nonce`: _string_ – 96-bit nonce.\
  Must be encoded with `hex`.

### ResultOfChaCha20

```ts
type ResultOfChaCha20 = {
    data: string
}
```

* `data`: _string_ – Encrypted/decrypted data.\
  Encoded with `base64`.

### ParamsOfCreateCryptoBox

```ts
type ParamsOfCreateCryptoBox = {
    secret_encryption_salt: string,
    secret: CryptoBoxSecret
}
```

* `secret_encryption_salt`: _string_ – Salt used for secret encryption. For example, a mobile device can use device ID as salt.
* `secret`: [_CryptoBoxSecret_](mod_crypto.md#cryptoboxsecret) – Cryptobox secret

### RegisteredCryptoBox

```ts
type RegisteredCryptoBox = {
    handle: CryptoBoxHandle
}
```

* `handle`: [_CryptoBoxHandle_](mod_crypto.md#cryptoboxhandle)

### ParamsOfAppPasswordProviderGetPasswordVariant

```ts
type ParamsOfAppPasswordProviderGetPasswordVariant = {
    encryption_public_key: string
}
```

* `encryption_public_key`: _string_ – Temporary library pubkey, that is used on application side for password encryption, along with application temporary private key and nonce. Used for password decryption on library side.

### ParamsOfAppPasswordProvider

Interface that provides a callback that returns an encrypted password, used for cryptobox secret encryption

To secure the password while passing it from application to the library, the library generates a temporary key pair, passes the pubkey to the passwordProvider, decrypts the received password with private key, and deletes the key pair right away.

Application should generate a temporary nacl\_box\_keypair and encrypt the password with naclbox function using nacl\_box\_keypair.secret and encryption\_public\_key keys + nonce = 24-byte prefix of encryption\_public\_key.

```ts
type ParamsOfAppPasswordProvider = ({
    type: 'GetPassword'
} & ParamsOfAppPasswordProviderGetPasswordVariant)
```

Depends on value of the `type` field.

When _type_ is _'GetPassword'_

* `encryption_public_key`: _string_ – Temporary library pubkey, that is used on application side for password encryption, along with application temporary private key and nonce. Used for password decryption on library side.

Variant constructors:

```ts
function paramsOfAppPasswordProviderGetPassword(encryption_public_key: string): ParamsOfAppPasswordProvider;
```

### ResultOfAppPasswordProviderGetPasswordVariant

```ts
type ResultOfAppPasswordProviderGetPasswordVariant = {
    encrypted_password: string,
    app_encryption_pubkey: string
}
```

* `encrypted_password`: _string_ – Password, encrypted and encoded to base64. Crypto box uses this password to decrypt its secret (seed phrase).
* `app_encryption_pubkey`: _string_ – Hex encoded public key of a temporary key pair, used for password encryption on application side.\
  Used together with `encryption_public_key` to decode `encrypted_password`.

### ResultOfAppPasswordProvider

```ts
type ResultOfAppPasswordProvider = ({
    type: 'GetPassword'
} & ResultOfAppPasswordProviderGetPasswordVariant)
```

Depends on value of the `type` field.

When _type_ is _'GetPassword'_

* `encrypted_password`: _string_ – Password, encrypted and encoded to base64. Crypto box uses this password to decrypt its secret (seed phrase).
* `app_encryption_pubkey`: _string_ – Hex encoded public key of a temporary key pair, used for password encryption on application side.\
  Used together with `encryption_public_key` to decode `encrypted_password`.

Variant constructors:

```ts
function resultOfAppPasswordProviderGetPassword(encrypted_password: string, app_encryption_pubkey: string): ResultOfAppPasswordProvider;
```

### ResultOfGetCryptoBoxInfo

```ts
type ResultOfGetCryptoBoxInfo = {
    encrypted_secret: string
}
```

* `encrypted_secret`: _string_ – Secret (seed phrase) encrypted with salt and password.

### ResultOfGetCryptoBoxSeedPhrase

```ts
type ResultOfGetCryptoBoxSeedPhrase = {
    phrase: string,
    dictionary: MnemonicDictionary,
    wordcount: number
}
```

* `phrase`: _string_
* `dictionary`: [_MnemonicDictionary_](mod_crypto.md#mnemonicdictionary)
* `wordcount`: _number_

### ParamsOfGetSigningBoxFromCryptoBox

```ts
type ParamsOfGetSigningBoxFromCryptoBox = {
    handle: number,
    hdpath?: string,
    secret_lifetime?: number
}
```

* `handle`: _number_ – Crypto Box Handle.
* `hdpath`?: _string_ – HD key derivation path.\
  By default, Acki Nacki HD path is used.
* `secret_lifetime`?: _number_ – Store derived secret for this lifetime (in ms). The timer starts after each signing box operation. Secrets will be deleted immediately after each signing box operation, if this value is not set.

### RegisteredSigningBox

```ts
type RegisteredSigningBox = {
    handle: SigningBoxHandle
}
```

* `handle`: [_SigningBoxHandle_](mod_crypto.md#signingboxhandle) – Handle of the signing box.

### ParamsOfGetEncryptionBoxFromCryptoBox

```ts
type ParamsOfGetEncryptionBoxFromCryptoBox = {
    handle: number,
    hdpath?: string,
    algorithm: BoxEncryptionAlgorithm,
    secret_lifetime?: number
}
```

* `handle`: _number_ – Crypto Box Handle.
* `hdpath`?: _string_ – HD key derivation path.\
  By default, Acki Nacki HD path is used.
* `algorithm`: [_BoxEncryptionAlgorithm_](mod_crypto.md#boxencryptionalgorithm) – Encryption algorithm.
* `secret_lifetime`?: _number_ – Store derived secret for encryption algorithm for this lifetime (in ms). The timer starts after each encryption box operation. Secrets will be deleted (overwritten with zeroes) after each encryption operation, if this value is not set.

### RegisteredEncryptionBox

```ts
type RegisteredEncryptionBox = {
    handle: EncryptionBoxHandle
}
```

* `handle`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Handle of the encryption box.

### ParamsOfAppSigningBoxGetPublicKeyVariant

Get signing box public key

```ts
type ParamsOfAppSigningBoxGetPublicKeyVariant = {

}
```

### ParamsOfAppSigningBoxSignVariant

Sign data

```ts
type ParamsOfAppSigningBoxSignVariant = {
    unsigned: string
}
```

* `unsigned`: _string_ – Data to sign encoded as base64

### ParamsOfAppSigningBox

Signing box callbacks.

```ts
type ParamsOfAppSigningBox = ({
    type: 'GetPublicKey'
} & ParamsOfAppSigningBoxGetPublicKeyVariant) | ({
    type: 'Sign'
} & ParamsOfAppSigningBoxSignVariant)
```

Depends on value of the `type` field.

When _type_ is _'GetPublicKey'_

Get signing box public key

When _type_ is _'Sign'_

Sign data

* `unsigned`: _string_ – Data to sign encoded as base64

Variant constructors:

```ts
function paramsOfAppSigningBoxGetPublicKey(): ParamsOfAppSigningBox;
function paramsOfAppSigningBoxSign(unsigned: string): ParamsOfAppSigningBox;
```

### ResultOfAppSigningBoxGetPublicKeyVariant

Result of getting public key

```ts
type ResultOfAppSigningBoxGetPublicKeyVariant = {
    public_key: string
}
```

* `public_key`: _string_ – Signing box public key

### ResultOfAppSigningBoxSignVariant

Result of signing data

```ts
type ResultOfAppSigningBoxSignVariant = {
    signature: string
}
```

* `signature`: _string_ – Data signature encoded as hex

### ResultOfAppSigningBox

Returning values from signing box callbacks.

```ts
type ResultOfAppSigningBox = ({
    type: 'GetPublicKey'
} & ResultOfAppSigningBoxGetPublicKeyVariant) | ({
    type: 'Sign'
} & ResultOfAppSigningBoxSignVariant)
```

Depends on value of the `type` field.

When _type_ is _'GetPublicKey'_

Result of getting public key

* `public_key`: _string_ – Signing box public key

When _type_ is _'Sign'_

Result of signing data

* `signature`: _string_ – Data signature encoded as hex

Variant constructors:

```ts
function resultOfAppSigningBoxGetPublicKey(public_key: string): ResultOfAppSigningBox;
function resultOfAppSigningBoxSign(signature: string): ResultOfAppSigningBox;
```

### ResultOfSigningBoxGetPublicKey

```ts
type ResultOfSigningBoxGetPublicKey = {
    pubkey: string
}
```

* `pubkey`: _string_ – Public key of signing box.\
  Encoded with hex

### ParamsOfSigningBoxSign

```ts
type ParamsOfSigningBoxSign = {
    signing_box: SigningBoxHandle,
    unsigned: string
}
```

* `signing_box`: [_SigningBoxHandle_](mod_crypto.md#signingboxhandle) – Signing Box handle.
* `unsigned`: _string_ – Unsigned user data.\
  Must be encoded with `base64`.

### ResultOfSigningBoxSign

```ts
type ResultOfSigningBoxSign = {
    signature: string
}
```

* `signature`: _string_ – Data signature.\
  Encoded with `hex`.

### ParamsOfAppEncryptionBoxGetInfoVariant

Get encryption box info

```ts
type ParamsOfAppEncryptionBoxGetInfoVariant = {

}
```

### ParamsOfAppEncryptionBoxEncryptVariant

Encrypt data

```ts
type ParamsOfAppEncryptionBoxEncryptVariant = {
    data: string
}
```

* `data`: _string_ – Data, encoded in Base64

### ParamsOfAppEncryptionBoxDecryptVariant

Decrypt data

```ts
type ParamsOfAppEncryptionBoxDecryptVariant = {
    data: string
}
```

* `data`: _string_ – Data, encoded in Base64

### ParamsOfAppEncryptionBox

Interface for data encryption/decryption

```ts
type ParamsOfAppEncryptionBox = ({
    type: 'GetInfo'
} & ParamsOfAppEncryptionBoxGetInfoVariant) | ({
    type: 'Encrypt'
} & ParamsOfAppEncryptionBoxEncryptVariant) | ({
    type: 'Decrypt'
} & ParamsOfAppEncryptionBoxDecryptVariant)
```

Depends on value of the `type` field.

When _type_ is _'GetInfo'_

Get encryption box info

When _type_ is _'Encrypt'_

Encrypt data

* `data`: _string_ – Data, encoded in Base64

When _type_ is _'Decrypt'_

Decrypt data

* `data`: _string_ – Data, encoded in Base64

Variant constructors:

```ts
function paramsOfAppEncryptionBoxGetInfo(): ParamsOfAppEncryptionBox;
function paramsOfAppEncryptionBoxEncrypt(data: string): ParamsOfAppEncryptionBox;
function paramsOfAppEncryptionBoxDecrypt(data: string): ParamsOfAppEncryptionBox;
```

### ResultOfAppEncryptionBoxGetInfoVariant

Result of getting encryption box info

```ts
type ResultOfAppEncryptionBoxGetInfoVariant = {
    info: EncryptionBoxInfo
}
```

* `info`: [_EncryptionBoxInfo_](mod_crypto.md#encryptionboxinfo)

### ResultOfAppEncryptionBoxEncryptVariant

Result of encrypting data

```ts
type ResultOfAppEncryptionBoxEncryptVariant = {
    data: string
}
```

* `data`: _string_ – Encrypted data, encoded in Base64

### ResultOfAppEncryptionBoxDecryptVariant

Result of decrypting data

```ts
type ResultOfAppEncryptionBoxDecryptVariant = {
    data: string
}
```

* `data`: _string_ – Decrypted data, encoded in Base64

### ResultOfAppEncryptionBox

Returning values from signing box callbacks.

```ts
type ResultOfAppEncryptionBox = ({
    type: 'GetInfo'
} & ResultOfAppEncryptionBoxGetInfoVariant) | ({
    type: 'Encrypt'
} & ResultOfAppEncryptionBoxEncryptVariant) | ({
    type: 'Decrypt'
} & ResultOfAppEncryptionBoxDecryptVariant)
```

Depends on value of the `type` field.

When _type_ is _'GetInfo'_

Result of getting encryption box info

* `info`: [_EncryptionBoxInfo_](mod_crypto.md#encryptionboxinfo)

When _type_ is _'Encrypt'_

Result of encrypting data

* `data`: _string_ – Encrypted data, encoded in Base64

When _type_ is _'Decrypt'_

Result of decrypting data

* `data`: _string_ – Decrypted data, encoded in Base64

Variant constructors:

```ts
function resultOfAppEncryptionBoxGetInfo(info: EncryptionBoxInfo): ResultOfAppEncryptionBox;
function resultOfAppEncryptionBoxEncrypt(data: string): ResultOfAppEncryptionBox;
function resultOfAppEncryptionBoxDecrypt(data: string): ResultOfAppEncryptionBox;
```

### ParamsOfEncryptionBoxGetInfo

```ts
type ParamsOfEncryptionBoxGetInfo = {
    encryption_box: EncryptionBoxHandle
}
```

* `encryption_box`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Encryption box handle

### ResultOfEncryptionBoxGetInfo

```ts
type ResultOfEncryptionBoxGetInfo = {
    info: EncryptionBoxInfo
}
```

* `info`: [_EncryptionBoxInfo_](mod_crypto.md#encryptionboxinfo) – Encryption box information

### ParamsOfEncryptionBoxEncrypt

```ts
type ParamsOfEncryptionBoxEncrypt = {
    encryption_box: EncryptionBoxHandle,
    data: string
}
```

* `encryption_box`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Encryption box handle
* `data`: _string_ – Data to be encrypted, encoded in Base64

### ResultOfEncryptionBoxEncrypt

```ts
type ResultOfEncryptionBoxEncrypt = {
    data: string
}
```

* `data`: _string_ – Encrypted data, encoded in Base64.\
  Padded to cipher block size

### ParamsOfEncryptionBoxDecrypt

```ts
type ParamsOfEncryptionBoxDecrypt = {
    encryption_box: EncryptionBoxHandle,
    data: string
}
```

* `encryption_box`: [_EncryptionBoxHandle_](mod_crypto.md#encryptionboxhandle) – Encryption box handle
* `data`: _string_ – Data to be decrypted, encoded in Base64

### ResultOfEncryptionBoxDecrypt

```ts
type ResultOfEncryptionBoxDecrypt = {
    data: string
}
```

* `data`: _string_ – Decrypted data, encoded in Base64.

### ParamsOfCreateEncryptionBox

```ts
type ParamsOfCreateEncryptionBox = {
    algorithm: EncryptionAlgorithm
}
```

* `algorithm`: [_EncryptionAlgorithm_](mod_crypto.md#encryptionalgorithm) – Encryption algorithm specifier including cipher parameters (key, IV, etc)

### AppPasswordProvider

Interface that provides a callback that returns an encrypted password, used for cryptobox secret encryption

To secure the password while passing it from application to the library, the library generates a temporary key pair, passes the pubkey to the passwordProvider, decrypts the received password with private key, and deletes the key pair right away.

Application should generate a temporary nacl\_box\_keypair and encrypt the password with naclbox function using nacl\_box\_keypair.secret and encryption\_public\_key keys + nonce = 24-byte prefix of encryption\_public\_key.

```ts

export interface AppPasswordProvider {
    get_password(params: ParamsOfAppPasswordProviderGetPasswordVariant): Promise<ResultOfAppPasswordProviderGetPasswordVariant>,
}
```

### get\_password

```ts
type ParamsOfAppPasswordProviderGetPasswordVariant = ParamsOfAppPasswordProviderGetPasswordVariant

type ResultOfAppPasswordProviderGetPasswordVariant = ResultOfAppPasswordProviderGetPasswordVariant

function get_password(
    params: ParamsOfAppPasswordProviderGetPasswordVariant,
): Promise<ResultOfAppPasswordProviderGetPasswordVariant>;

function get_password_sync(
    params: ParamsOfAppPasswordProviderGetPasswordVariant,
): ResultOfAppPasswordProviderGetPasswordVariant;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `encryption_public_key`: _string_ – Temporary library pubkey, that is used on application side for password encryption, along with application temporary private key and nonce. Used for password decryption on library side.

#### Result

* `encrypted_password`: _string_ – Password, encrypted and encoded to base64. Crypto box uses this password to decrypt its secret (seed phrase).
* `app_encryption_pubkey`: _string_ – Hex encoded public key of a temporary key pair, used for password encryption on application side.\
  Used together with `encryption_public_key` to decode `encrypted_password`.

### AppSigningBox

Signing box callbacks.

```ts

export interface AppSigningBox {
    get_public_key(): Promise<ResultOfAppSigningBoxGetPublicKeyVariant>,
    sign(params: ParamsOfAppSigningBoxSignVariant): Promise<ResultOfAppSigningBoxSignVariant>,
}
```

### get\_public\_key

Get signing box public key

```ts
type ResultOfAppSigningBoxGetPublicKeyVariant = ResultOfAppSigningBoxGetPublicKeyVariant

function get_public_key(): Promise<ResultOfAppSigningBoxGetPublicKeyVariant>;

function get_public_key_sync(): ResultOfAppSigningBoxGetPublicKeyVariant;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Result

* `public_key`: _string_ – Signing box public key

### sign

Sign data

```ts
type ParamsOfAppSigningBoxSignVariant = ParamsOfAppSigningBoxSignVariant

type ResultOfAppSigningBoxSignVariant = ResultOfAppSigningBoxSignVariant

function sign(
    params: ParamsOfAppSigningBoxSignVariant,
): Promise<ResultOfAppSigningBoxSignVariant>;

function sign_sync(
    params: ParamsOfAppSigningBoxSignVariant,
): ResultOfAppSigningBoxSignVariant;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `unsigned`: _string_ – Data to sign encoded as base64

#### Result

* `signature`: _string_ – Data signature encoded as hex

### AppEncryptionBox

Interface for data encryption/decryption

```ts

export interface AppEncryptionBox {
    get_info(): Promise<ResultOfAppEncryptionBoxGetInfoVariant>,
    encrypt(params: ParamsOfAppEncryptionBoxEncryptVariant): Promise<ResultOfAppEncryptionBoxEncryptVariant>,
    decrypt(params: ParamsOfAppEncryptionBoxDecryptVariant): Promise<ResultOfAppEncryptionBoxDecryptVariant>,
}
```

### get\_info

Get encryption box info

```ts
type ResultOfAppEncryptionBoxGetInfoVariant = ResultOfAppEncryptionBoxGetInfoVariant

function get_info(): Promise<ResultOfAppEncryptionBoxGetInfoVariant>;

function get_info_sync(): ResultOfAppEncryptionBoxGetInfoVariant;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Result

* `info`: [_EncryptionBoxInfo_](mod_crypto.md#encryptionboxinfo)

### encrypt

Encrypt data

```ts
type ParamsOfAppEncryptionBoxEncryptVariant = ParamsOfAppEncryptionBoxEncryptVariant

type ResultOfAppEncryptionBoxEncryptVariant = ResultOfAppEncryptionBoxEncryptVariant

function encrypt(
    params: ParamsOfAppEncryptionBoxEncryptVariant,
): Promise<ResultOfAppEncryptionBoxEncryptVariant>;

function encrypt_sync(
    params: ParamsOfAppEncryptionBoxEncryptVariant,
): ResultOfAppEncryptionBoxEncryptVariant;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `data`: _string_ – Data, encoded in Base64

#### Result

* `data`: _string_ – Encrypted data, encoded in Base64

### decrypt

Decrypt data

```ts
type ParamsOfAppEncryptionBoxDecryptVariant = ParamsOfAppEncryptionBoxDecryptVariant

type ResultOfAppEncryptionBoxDecryptVariant = ResultOfAppEncryptionBoxDecryptVariant

function decrypt(
    params: ParamsOfAppEncryptionBoxDecryptVariant,
): Promise<ResultOfAppEncryptionBoxDecryptVariant>;

function decrypt_sync(
    params: ParamsOfAppEncryptionBoxDecryptVariant,
): ResultOfAppEncryptionBoxDecryptVariant;
```

NOTE: Sync version is available only for `lib-node` binding.

#### Parameters

* `data`: _string_ – Data, encoded in Base64

#### Result

* `data`: _string_ – Decrypted data, encoded in Base64
