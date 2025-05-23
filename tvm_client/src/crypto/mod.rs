// Copyright 2018-2021 TON Labs LTD.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

pub(crate) mod boxes;
pub(crate) mod encscrypt;
mod errors;
pub(crate) mod hash;
pub(crate) mod hdkey;
pub(crate) mod internal;
pub(crate) mod keys;
pub(crate) mod math;
pub(crate) mod mnemonic;
pub(crate) mod nacl;

pub use errors::Error;
pub use errors::ErrorCode;
pub(crate) mod encryption;
#[cfg(test)]
mod tests;

pub use encryption::ParamsOfChaCha20;
pub use encryption::ResultOfChaCha20;
pub use encryption::chacha20;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

pub use crate::crypto::boxes::crypto_box::AppPasswordProvider;
pub use crate::crypto::boxes::crypto_box::BoxEncryptionAlgorithm;
pub use crate::crypto::boxes::crypto_box::ChaCha20ParamsCB;
pub use crate::crypto::boxes::crypto_box::CryptoBoxHandle;
pub use crate::crypto::boxes::crypto_box::CryptoBoxSecret;
pub use crate::crypto::boxes::crypto_box::NaclBoxParamsCB;
pub use crate::crypto::boxes::crypto_box::NaclSecretBoxParamsCB;
pub use crate::crypto::boxes::crypto_box::ParamsOfCreateCryptoBox;
pub use crate::crypto::boxes::crypto_box::ParamsOfGetSigningBoxFromCryptoBox;
pub use crate::crypto::boxes::crypto_box::RegisteredCryptoBox;
pub use crate::crypto::boxes::crypto_box::ResultOfGetCryptoBoxInfo;
pub use crate::crypto::boxes::crypto_box::ResultOfGetCryptoBoxSeedPhrase;
pub use crate::crypto::boxes::crypto_box::ResultOfGetPassword;
pub use crate::crypto::boxes::crypto_box::clear_crypto_box_secret_cache;
pub use crate::crypto::boxes::crypto_box::create_crypto_box;
pub use crate::crypto::boxes::crypto_box::get_crypto_box_info;
pub use crate::crypto::boxes::crypto_box::get_crypto_box_seed_phrase;
pub use crate::crypto::boxes::crypto_box::get_encryption_box_from_crypto_box;
pub use crate::crypto::boxes::crypto_box::get_signing_box_from_crypto_box;
pub use crate::crypto::boxes::crypto_box::remove_crypto_box;
pub use crate::crypto::boxes::encryption_box::CipherMode;
pub use crate::crypto::boxes::encryption_box::EncryptionAlgorithm;
pub use crate::crypto::boxes::encryption_box::EncryptionBox;
pub use crate::crypto::boxes::encryption_box::EncryptionBoxHandle;
pub use crate::crypto::boxes::encryption_box::EncryptionBoxInfo;
pub use crate::crypto::boxes::encryption_box::ParamsOfEncryptionBoxDecrypt;
pub use crate::crypto::boxes::encryption_box::ParamsOfEncryptionBoxEncrypt;
pub use crate::crypto::boxes::encryption_box::ParamsOfEncryptionBoxGetInfo;
pub use crate::crypto::boxes::encryption_box::RegisteredEncryptionBox;
pub use crate::crypto::boxes::encryption_box::ResultOfEncryptionBoxDecrypt;
pub use crate::crypto::boxes::encryption_box::ResultOfEncryptionBoxEncrypt;
pub use crate::crypto::boxes::encryption_box::ResultOfEncryptionBoxGetInfo;
pub use crate::crypto::boxes::encryption_box::aes::AesEncryptionBox;
pub use crate::crypto::boxes::encryption_box::aes::AesInfo;
pub use crate::crypto::boxes::encryption_box::aes::AesParamsEB;
pub use crate::crypto::boxes::encryption_box::chacha20::ChaCha20EncryptionBox;
pub use crate::crypto::boxes::encryption_box::chacha20::ChaCha20ParamsEB;
pub use crate::crypto::boxes::encryption_box::create_encryption_box;
pub use crate::crypto::boxes::encryption_box::encryption_box_decrypt;
pub use crate::crypto::boxes::encryption_box::encryption_box_encrypt;
pub use crate::crypto::boxes::encryption_box::encryption_box_get_info;
pub use crate::crypto::boxes::encryption_box::nacl_box::NaclBoxParamsEB;
pub use crate::crypto::boxes::encryption_box::nacl_box::NaclEncryptionBox;
pub use crate::crypto::boxes::encryption_box::nacl_secret_box::NaclSecretBoxParamsEB;
pub use crate::crypto::boxes::encryption_box::nacl_secret_box::NaclSecretEncryptionBox;
pub use crate::crypto::boxes::encryption_box::register_encryption_box;
pub use crate::crypto::boxes::encryption_box::remove_encryption_box;
pub use crate::crypto::boxes::signing_box::ParamsOfSigningBoxSign;
pub use crate::crypto::boxes::signing_box::RegisteredSigningBox;
pub use crate::crypto::boxes::signing_box::ResultOfSigningBoxGetPublicKey;
pub use crate::crypto::boxes::signing_box::ResultOfSigningBoxSign;
pub use crate::crypto::boxes::signing_box::SigningBox;
pub use crate::crypto::boxes::signing_box::SigningBoxHandle;
pub use crate::crypto::boxes::signing_box::get_signing_box;
pub use crate::crypto::boxes::signing_box::register_signing_box;
pub use crate::crypto::boxes::signing_box::remove_signing_box;
pub use crate::crypto::boxes::signing_box::signing_box_get_public_key;
pub use crate::crypto::boxes::signing_box::signing_box_sign;
pub use crate::crypto::encscrypt::ParamsOfScrypt;
pub use crate::crypto::encscrypt::ResultOfScrypt;
pub use crate::crypto::encscrypt::scrypt;
pub use crate::crypto::hash::ParamsOfHash;
pub use crate::crypto::hash::ResultOfHash;
pub use crate::crypto::hash::sha256;
pub use crate::crypto::hash::sha512;
pub use crate::crypto::hdkey::ParamsOfHDKeyDeriveFromXPrv;
pub use crate::crypto::hdkey::ParamsOfHDKeyDeriveFromXPrvPath;
pub use crate::crypto::hdkey::ParamsOfHDKeyPublicFromXPrv;
pub use crate::crypto::hdkey::ParamsOfHDKeySecretFromXPrv;
pub use crate::crypto::hdkey::ParamsOfHDKeyXPrvFromMnemonic;
pub use crate::crypto::hdkey::ResultOfHDKeyDeriveFromXPrv;
pub use crate::crypto::hdkey::ResultOfHDKeyDeriveFromXPrvPath;
pub use crate::crypto::hdkey::ResultOfHDKeyPublicFromXPrv;
pub use crate::crypto::hdkey::ResultOfHDKeySecretFromXPrv;
pub use crate::crypto::hdkey::ResultOfHDKeyXPrvFromMnemonic;
pub use crate::crypto::hdkey::hdkey_derive_from_xprv;
pub use crate::crypto::hdkey::hdkey_derive_from_xprv_path;
pub use crate::crypto::hdkey::hdkey_public_from_xprv;
pub use crate::crypto::hdkey::hdkey_secret_from_xprv;
pub use crate::crypto::hdkey::hdkey_xprv_from_mnemonic;
pub use crate::crypto::keys::KeyPair;
pub use crate::crypto::keys::ParamsOfConvertPublicKeyToTonSafeFormat;
pub use crate::crypto::keys::ParamsOfSign;
pub use crate::crypto::keys::ParamsOfVerifySignature;
pub use crate::crypto::keys::ResultOfConvertPublicKeyToTonSafeFormat;
pub use crate::crypto::keys::ResultOfSign;
pub use crate::crypto::keys::ResultOfVerifySignature;
pub use crate::crypto::keys::convert_public_key_to_tvm_safe_format;
pub use crate::crypto::keys::generate_random_sign_keys;
pub use crate::crypto::keys::sign;
pub use crate::crypto::keys::verify_signature;
pub use crate::crypto::math::ParamsOfFactorize;
pub use crate::crypto::math::ParamsOfGenerateRandomBytes;
pub use crate::crypto::math::ParamsOfModularPower;
pub use crate::crypto::math::ParamsOfTonCrc16;
pub use crate::crypto::math::ResultOfFactorize;
pub use crate::crypto::math::ResultOfGenerateRandomBytes;
pub use crate::crypto::math::ResultOfModularPower;
pub use crate::crypto::math::ResultOfTonCrc16;
pub use crate::crypto::math::factorize;
pub use crate::crypto::math::generate_random_bytes;
pub use crate::crypto::math::modular_power;
pub use crate::crypto::math::tvm_crc16;
pub use crate::crypto::mnemonic::MnemonicDictionary;
pub use crate::crypto::mnemonic::ParamsOfMnemonicDeriveSignKeys;
pub use crate::crypto::mnemonic::ParamsOfMnemonicFromEntropy;
pub use crate::crypto::mnemonic::ParamsOfMnemonicFromRandom;
pub use crate::crypto::mnemonic::ParamsOfMnemonicVerify;
pub use crate::crypto::mnemonic::ParamsOfMnemonicWords;
pub use crate::crypto::mnemonic::ResultOfMnemonicFromEntropy;
pub use crate::crypto::mnemonic::ResultOfMnemonicFromRandom;
pub use crate::crypto::mnemonic::ResultOfMnemonicVerify;
pub use crate::crypto::mnemonic::ResultOfMnemonicWords;
pub use crate::crypto::mnemonic::mnemonic_derive_sign_keys;
pub use crate::crypto::mnemonic::mnemonic_from_entropy;
pub use crate::crypto::mnemonic::mnemonic_from_random;
pub use crate::crypto::mnemonic::mnemonic_verify;
pub use crate::crypto::mnemonic::mnemonic_words;
pub use crate::crypto::nacl::ParamsOfNaclBox;
pub use crate::crypto::nacl::ParamsOfNaclBoxKeyPairFromSecret;
pub use crate::crypto::nacl::ParamsOfNaclBoxOpen;
pub use crate::crypto::nacl::ParamsOfNaclSecretBox;
pub use crate::crypto::nacl::ParamsOfNaclSecretBoxOpen;
pub use crate::crypto::nacl::ParamsOfNaclSign;
pub use crate::crypto::nacl::ParamsOfNaclSignDetached;
pub use crate::crypto::nacl::ParamsOfNaclSignDetachedVerify;
pub use crate::crypto::nacl::ParamsOfNaclSignKeyPairFromSecret;
pub use crate::crypto::nacl::ParamsOfNaclSignOpen;
pub use crate::crypto::nacl::ResultOfNaclBox;
pub use crate::crypto::nacl::ResultOfNaclBoxOpen;
pub use crate::crypto::nacl::ResultOfNaclSign;
pub use crate::crypto::nacl::ResultOfNaclSignDetached;
pub use crate::crypto::nacl::ResultOfNaclSignDetachedVerify;
pub use crate::crypto::nacl::ResultOfNaclSignOpen;
pub use crate::crypto::nacl::nacl_box;
pub use crate::crypto::nacl::nacl_box_keypair;
pub use crate::crypto::nacl::nacl_box_keypair_from_secret_key;
pub use crate::crypto::nacl::nacl_box_open;
pub use crate::crypto::nacl::nacl_secret_box;
pub use crate::crypto::nacl::nacl_secret_box_open;
pub use crate::crypto::nacl::nacl_sign;
pub use crate::crypto::nacl::nacl_sign_detached;
pub use crate::crypto::nacl::nacl_sign_detached_verify;
pub use crate::crypto::nacl::nacl_sign_keypair_from_secret_key;
pub use crate::crypto::nacl::nacl_sign_open;

pub fn default_mnemonic_word_count() -> u8 {
    12
}

pub fn default_hdkey_derivation_path() -> String {
    "m/44'/396'/0'/0/0".into()
}

pub fn default_hdkey_compliant() -> bool {
    true
}

fn deserialize_mnemonic_dictionary<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<MnemonicDictionary, D::Error> {
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

fn deserialize_mnemonic_word_count<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u8, D::Error> {
    Ok(Option::deserialize(deserializer)?.unwrap_or(default_mnemonic_word_count()))
}

fn deserialize_hdkey_derivation_path<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    Ok(Option::deserialize(deserializer)?.unwrap_or(default_hdkey_derivation_path()))
}

/// Crypto config.
#[derive(Deserialize, Serialize, Debug, Clone, ApiType)]
pub struct CryptoConfig {
    /// Mnemonic dictionary that will be used by default in crypto functions.
    /// If not specified, `English` dictionary will be used.
    #[serde(
        default = "MnemonicDictionary::default",
        deserialize_with = "deserialize_mnemonic_dictionary"
    )]
    pub mnemonic_dictionary: MnemonicDictionary,

    /// Mnemonic word count that will be used by default in crypto functions.
    /// If not specified the default value will be 12.
    #[serde(
        default = "default_mnemonic_word_count",
        deserialize_with = "deserialize_mnemonic_word_count"
    )]
    pub mnemonic_word_count: u8,

    /// Derivation path that will be used by default in crypto functions.
    /// If not specified `m/44'/396'/0'/0/0` will be used.
    #[serde(
        default = "default_hdkey_derivation_path",
        deserialize_with = "deserialize_hdkey_derivation_path"
    )]
    pub hdkey_derivation_path: String,
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            mnemonic_dictionary: Default::default(),
            mnemonic_word_count: default_mnemonic_word_count(),
            hdkey_derivation_path: default_hdkey_derivation_path(),
        }
    }
}
