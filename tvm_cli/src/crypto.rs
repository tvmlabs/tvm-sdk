// Copyright 2018-2021 TON DEV SOLUTIONS LTD.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.
use tvm_client::crypto::KeyPair;
use tvm_client::crypto::MnemonicDictionary;
use tvm_client::crypto::ParamsOfHDKeyDeriveFromXPrvPath;
use tvm_client::crypto::ParamsOfHDKeySecretFromXPrv;
use tvm_client::crypto::ParamsOfHDKeyXPrvFromMnemonic;
use tvm_client::crypto::ParamsOfMnemonicFromRandom;
use tvm_client::crypto::ParamsOfNaclSignKeyPairFromSecret;
use tvm_client::crypto::hdkey_derive_from_xprv_path;
use tvm_client::crypto::hdkey_secret_from_xprv;
use tvm_client::crypto::hdkey_xprv_from_mnemonic;
use tvm_client::crypto::mnemonic_from_random;
use tvm_client::crypto::nacl_sign_keypair_from_secret_key;

use crate::Config;
use crate::helpers::HD_PATH;
use crate::helpers::WORD_COUNT;
use crate::helpers::check_dir;
use crate::helpers::create_client_local;
use crate::helpers::read_keys;

pub fn load_keypair(keys: &str) -> Result<KeyPair, String> {
    match classify(keys) {
        KeySource::Phrase => generate_keypair_from_mnemonic(keys.trim()),
        // A path, or a raw secret key -- which this function has never accepted
        // and still does not, failing as a file that cannot be read. Trimmed
        // like the phrase: `classify` read the value without the whitespace
        // around it, so the rest of the function has to as well.
        KeySource::SecretKey | KeySource::Path => read_keys(keys.trim()),
    }
}

/// What a `--keys` / `--sign` / `--phrase` argument turned out to hold.
#[derive(Debug, PartialEq, Eq)]
pub enum KeySource {
    /// A seed phrase: several words with whitespace between them.
    ///
    /// Whitespace, not the ASCII space: BIP-39 separates the Japanese wordlist
    /// with U+3000 IDEOGRAPHIC SPACE, and the Chinese and Korean lists are
    /// commonly written that way too.
    Phrase,
    /// A raw secret key in hex, optionally with the public key appended --
    /// what `generate_keypair_from_secret` accepts.
    SecretKey,
    /// A path to a keypair file.
    Path,
}

/// What a value holding a key turned out to be.
///
/// One predicate, used by everything that has to tell these apart: masking for
/// display, refusing to store a secret in the config file, and `load_keypair`
/// choosing how to read the value. They used to have a classifier each, and two
/// hand-maintained rules over the same value disagree exactly where it costs
/// most -- a wallet written to disk while the screen reports it masked.
///
/// The rules, in order:
///
/// * surrounding whitespace is not part of the value, so it is trimmed first --
///   a copied key with a trailing newline is still the key;
/// * a path separator settles it: no wordlist holds a word with one, and a hex
///   key has no punctuation at all, so this is the way to name a keypair file
///   whose name would otherwise read as a secret;
/// * any remaining whitespace makes it a phrase, whatever the words are made
///   of, since one mistyped word leaves the wallet a trivial search away;
/// * otherwise a `0x` prefix is dropped and long bare hex is a secret key.
///
/// The separator rule is a blanket one, so a phrase written with `/` between
/// its words reads as a path: it is neither masked nor refused. Nothing writes
/// a phrase that way, and the alternative -- masking every value with a slash
/// in it -- hides the paths this exists to show.
///
/// Nothing here consults the filesystem: what a value means cannot depend on
/// the directory the tool happened to run from, and a keypair file may well be
/// named before `getkeypair` writes it.
pub fn classify(value: &str) -> KeySource {
    let value = value.trim();
    if value.contains('/') || value.contains('\\') {
        return KeySource::Path;
    }
    if value.chars().any(char::is_whitespace) {
        return KeySource::Phrase;
    }
    let hex = secret_key_hex(value);
    if hex.len() >= 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return KeySource::SecretKey;
    }
    KeySource::Path
}

/// A secret key as `classify` reads it: without the whitespace around it and
/// without an `0x` prefix. Whatever is recognised as a key has to be usable as
/// one, or refusing a value would send its owner to a command that rejects it.
pub fn secret_key_hex(value: &str) -> &str {
    let value = value.trim();
    value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")).unwrap_or(value)
}

/// Renders a `--keys` / `--sign` / `--setkey` / `--keypair` argument for
/// display. The path is useful diagnostics and is kept; an inline secret is
/// the wallet itself and is replaced by its kind.
///
/// Whether a value is a secret is decided by `classify`, so what is masked
/// here, what the config file refuses to store, and what `load_keypair` treats
/// as a phrase cannot drift apart.
pub fn mask_key_source(value: &str) -> &str {
    match classify(value) {
        KeySource::Phrase => "<seed phrase>",
        KeySource::SecretKey => "<secret key>",
        KeySource::Path => value,
    }
}

/// Renders an argument that is always a secret and never a path, such as
/// `getkeypair --phrase`, so an unrecognised shape is still withheld.
pub fn mask_secret(value: &str) -> &str {
    match classify(value) {
        KeySource::Phrase => "<seed phrase>",
        KeySource::SecretKey => "<secret key>",
        KeySource::Path => "<secret>",
    }
}

pub fn gen_seed_phrase() -> Result<String, String> {
    let client = create_client_local()?;
    mnemonic_from_random(
        client,
        ParamsOfMnemonicFromRandom {
            dictionary: Some(MnemonicDictionary::English),
            word_count: Some(WORD_COUNT),
            ..Default::default()
        },
    )
    .map_err(|e| format!("{}", e))
    .map(|r| r.phrase)
}

pub fn generate_keypair_from_mnemonic(mnemonic: &str) -> Result<KeyPair, String> {
    let client = create_client_local()?;
    let hdk_master = hdkey_xprv_from_mnemonic(
        client.clone(),
        ParamsOfHDKeyXPrvFromMnemonic {
            dictionary: Some(MnemonicDictionary::English),
            word_count: Some(WORD_COUNT),
            phrase: mnemonic.to_string(),
            ..Default::default()
        },
    )
    .map_err(|e| format!("{}", e))?;

    let hdk_root = hdkey_derive_from_xprv_path(
        client.clone(),
        ParamsOfHDKeyDeriveFromXPrvPath {
            xprv: hdk_master.xprv.clone(),
            path: HD_PATH.to_string(),
            ..Default::default()
        },
    )
    .map_err(|e| format!("{}", e))?;

    let secret = hdkey_secret_from_xprv(
        client.clone(),
        ParamsOfHDKeySecretFromXPrv { xprv: hdk_root.xprv.clone(), ..Default::default() },
    )
    .map_err(|e| format!("{}", e))?;

    let mut keypair: KeyPair = nacl_sign_keypair_from_secret_key(
        client,
        ParamsOfNaclSignKeyPairFromSecret { secret: secret.secret.clone(), ..Default::default() },
    )
    .map_err(|e| format!("failed to get KeyPair from secret key: {}", e))?;

    // special case if secret contains public key too.
    let secret =
        hex::decode(&keypair.secret).map_err(|e| format!("failed to decode the keypair: {}", e))?;
    if secret.len() > 32 {
        keypair.secret = hex::encode(&secret[..32]);
    }
    Ok(keypair)
}

pub fn generate_keypair_from_secret(secret: String) -> Result<KeyPair, String> {
    let client = create_client_local()?;
    let mut keypair: KeyPair = nacl_sign_keypair_from_secret_key(
        client,
        ParamsOfNaclSignKeyPairFromSecret { secret, ..Default::default() },
    )
    .map_err(|e| format!("failed to get KeyPair from secret key: {}", e))?;
    // special case if secret contains public key too.
    let secret =
        hex::decode(&keypair.secret).map_err(|e| format!("failed to decode the keypair: {}", e))?;
    if secret.len() > 32 {
        keypair.secret = hex::encode(&secret[..32]);
    }
    Ok(keypair)
}

pub fn generate_mnemonic(keypath: Option<&str>, config: &Config) -> Result<(), String> {
    let mnemonic = gen_seed_phrase()?;
    if !config.is_json {
        println!("Succeeded.");
        println!(r#"Seed phrase: "{}""#, mnemonic);
    } else {
        println!("{{");
        println!("  \"phrase\": \"{}\"", mnemonic);
        println!("}}");
    }
    if let Some(path) = keypath {
        generate_keypair(Some(path), Some(&mnemonic), config)?;
        if !config.is_json {
            println!("Keypair saved to {}", path);
        }
    }
    Ok(())
}

pub fn extract_pubkey(mnemonic: &str, is_json: bool) -> Result<(), String> {
    let keypair = generate_keypair_from_mnemonic(mnemonic)?;
    if !is_json {
        println!("Succeeded.");
        println!("Public key: {}", keypair.public);
        println!();
        qr2term::print_qr(&keypair.public)
            .map_err(|e| format!("failed to print the QR code: {}", e))?;
        println!();
    } else {
        println!("{{");
        println!("  \"Public key\": \"{}\"", keypair.public);
        println!("}}");
    }
    Ok(())
}

pub fn generate_keypair(
    keys_path: Option<&str>,
    mnemonic: Option<&str>,
    config: &Config,
) -> Result<(), String> {
    let mnemonic = match mnemonic {
        Some(mnemonic) => mnemonic.to_owned(),
        None => {
            if !config.is_json {
                println!("Generating seed phrase.");
            }
            let phrase = gen_seed_phrase()?;
            if !config.is_json {
                println!(r#"Seed phrase: "{}""#, phrase);
            }
            phrase
        }
    };

    let keys = match classify(&mnemonic) {
        KeySource::Phrase => generate_keypair_from_mnemonic(mnemonic.trim())?,
        KeySource::SecretKey | KeySource::Path => {
            generate_keypair_from_secret(secret_key_hex(&mnemonic).to_string())?
        }
    };
    let keys_json = serde_json::to_string_pretty(&keys)
        .map_err(|e| format!("failed to serialize the keypair: {}", e))?;
    if let Some(keys_path) = keys_path {
        let folder_path = keys_path.trim_end_matches(|c| c != '/').trim_end_matches('/');
        check_dir(folder_path)?;
        std::fs::write(keys_path, &keys_json)
            .map_err(|e| format!("failed to create file with keys: {}", e))?;
        if !config.is_json {
            println!("Keypair successfully saved to {}.", keys_path);
        }
    } else {
        if !config.is_json {
            print!("Keypair: ");
        }
        println!("{}", keys_json);
    }
    if !config.is_json {
        println!("Succeeded.");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEY: &str = "c4415c03aa9d824e89ff4555cd12497aef1d5123f839803b0268e27ba6052354";
    const TEST_PHRASE: &str =
        "multiply extra monitor fog rocket defy attack right night jaguar hollow enlist";

    /// One predicate stands behind masking, the config refusal and
    /// `load_keypair`, so every row here is answered the same way by all three.
    /// Most of them are values that an earlier, hand-maintained pair of
    /// classifiers disagreed about, each disagreement being a wallet written to
    /// disk while the screen said it had been masked.
    #[test]
    fn classify_tells_a_secret_from_a_path() {
        let long_key = format!("{TEST_KEY}{TEST_KEY}");
        let cases: Vec<(String, KeySource)> = vec![
            (TEST_PHRASE.to_string(), KeySource::Phrase),
            // One mistyped word still leaves the wallet a trivial search away.
            (TEST_PHRASE.replace("enlist", "enl1st"), KeySource::Phrase),
            (TEST_PHRASE.replace(' ', ", "), KeySource::Phrase),
            // BIP-39 separates the Japanese wordlist with U+3000.
            (TEST_PHRASE.replace(' ', "\u{3000}"), KeySource::Phrase),
            (TEST_KEY.to_string(), KeySource::SecretKey),
            // Whitespace around a key does not stop it being the key.
            (format!("{TEST_KEY} "), KeySource::SecretKey),
            (format!(" {TEST_KEY}"), KeySource::SecretKey),
            (format!("\t{TEST_KEY}\n"), KeySource::SecretKey),
            (format!("\u{a0}{TEST_KEY}"), KeySource::SecretKey),
            (format!("\u{3000}{TEST_KEY}"), KeySource::SecretKey),
            (TEST_KEY.to_uppercase(), KeySource::SecretKey),
            (format!("0x{TEST_KEY}"), KeySource::SecretKey),
            (format!("0X{TEST_KEY}"), KeySource::SecretKey),
            (long_key.clone(), KeySource::SecretKey),
            (format!("0x{long_key}"), KeySource::SecretKey),
            // A path separator settles it: no wordlist holds a word with one,
            // and a hex key has no punctuation at all.
            ("key.json".to_string(), KeySource::Path),
            ("./key.json".to_string(), KeySource::Path),
            ("./My Wallets/msig.keys.json".to_string(), KeySource::Path),
            ("C:\\keys\\msig.json".to_string(), KeySource::Path),
            // Without the backslash arm this one reads as a phrase.
            ("C:\\My Wallets\\msig.json".to_string(), KeySource::Path),
            // One character short of a secret key.
            (TEST_KEY[1..].to_string(), KeySource::Path),
            // A keypair file named after its public key, spelled as a path.
            (format!("./{TEST_KEY}"), KeySource::Path),
            (String::new(), KeySource::Path),
            (" \t\n".to_string(), KeySource::Path),
        ];
        for (value, expected) in cases {
            assert_eq!(classify(&value), expected, "classifying {value:?}");
        }
    }

    #[test]
    fn test_generate_keypair() {
        let mnemonic =
            "multiply extra monitor fog rocket defy attack right night jaguar hollow enlist";
        let keypair = generate_keypair_from_mnemonic(mnemonic).unwrap();
        assert_eq!(
            &keypair.public,
            "04ad311dadcbf7fe4bc20d62e0fbfa195ab5f099009b40045632b997daf4b3b1"
        );
        assert_eq!(
            &keypair.secret,
            "c4415c03aa9d824e89ff4555cd12497aef1d5123f839803b0268e27ba6052354"
        );

        let mnemonic =
            "penalty nut enrich input palace flame safe session torch depth various hunt";
        let keypair = generate_keypair_from_mnemonic(mnemonic).unwrap();
        assert_eq!(
            &keypair.public,
            "3d79dd47d7c09e38bdee00de578eb480142b8bb1456f1aa82e0ff0a85096a72d"
        );
        assert_eq!(
            &keypair.secret,
            "d50dc3fc9bea78b9b582573403905f3c4da3de85a6d1635ff40a77d770fb8864"
        );
    }

    #[test]
    fn test_invalid_mnemonic() {
        let invalid_phrases = vec![
            "multiply extra monitor fog rocket defy attack right night jaguar hollow enlist ",
            "multiply  extra monitor fog rocket defy attack right night jaguar hollow enlist",
            "multipl extra monitor fog rocket defy attack right night jaguar hollow enlist",
            "s",
            "extra",
            "",
            " ",
            "123",
            "extra/1",
            "extra .1",
            "extra ,1",
            "0x0",
            "0:3333333333333333333333333333333333333333333333333333333333333333",
            "-alert()-",
            "multiply extra monitor fog rocket defy attack right night jaguar hollow enlist multiply extra monitor fog rocket defy attack right night jaguar hollow enlist multiply extra monitor fog rocket defy attack right night jaguar hollow enlist multiply extra monitor fog rocket defy attack right night jaguar hollow enlist multiply extra monitor fog rocket defy attack right night jaguar hollow enlist multiply extra monitor fog rocket defy attack right night jaguar hollow enlist",
        ];

        for phrase in invalid_phrases {
            assert!(generate_keypair_from_mnemonic(phrase).is_err());
        }
    }

    const PHRASE: &str =
        "multiply extra monitor fog rocket defy attack right night jaguar hollow enlist";
    const SECRET_HEX: &str = "c4415c03aa9d824e89ff4555cd12497aef1d5123f839803b0268e27ba6052354";

    #[test]
    fn mask_key_source_hides_a_seed_phrase() {
        assert_eq!(mask_key_source(PHRASE), "<seed phrase>");
    }

    #[test]
    fn mask_key_source_hides_a_raw_secret_key() {
        assert_eq!(mask_key_source(SECRET_HEX), "<secret key>");
        // A secret concatenated with its public key, as accepted by
        // generate_keypair_from_secret.
        assert_eq!(mask_key_source(&format!("{SECRET_HEX}{SECRET_HEX}")), "<secret key>");
    }

    /// BIP-39 separates the words of a Japanese mnemonic with U+3000
    /// IDEOGRAPHIC SPACE, not the ASCII space, and the Chinese and Korean
    /// wordlists are commonly written the same way. A phrase like this
    /// contains no ASCII space at all, so a masker that looks only for one
    /// classifies the whole wallet as a filename and prints it.
    #[test]
    fn mask_key_source_hides_a_phrase_separated_by_unicode_spaces() {
        let japanese = "あいこくしん\u{3000}あいさつ\u{3000}あいだ\u{3000}あおぞら\u{3000}\
                        あかちゃん\u{3000}あきる\u{3000}あけがた\u{3000}あさい\u{3000}\
                        あさひ\u{3000}あしあと\u{3000}あじわう\u{3000}あずかる";
        assert_eq!(mask_key_source(japanese), "<seed phrase>");

        // Any other separator Unicode calls whitespace gets the same
        // treatment: the cost of being wrong is asymmetric.
        assert_eq!(mask_key_source("word\u{00a0}word"), "<seed phrase>");
        assert_eq!(mask_key_source("word\tword"), "<seed phrase>");
    }

    #[test]
    fn mask_key_source_keeps_a_keypair_path() {
        assert_eq!(mask_key_source("keys/key0"), "keys/key0");
        assert_eq!(mask_key_source("wallet.keys.json"), "wallet.keys.json");
        // Hex, but far too short to be a key.
        assert_eq!(mask_key_source("deadbeef"), "deadbeef");
    }

    #[test]
    fn mask_secret_never_echoes_its_argument() {
        assert_eq!(mask_secret(PHRASE), "<seed phrase>");
        assert_eq!(mask_secret(SECRET_HEX), "<secret key>");
        // getkeypair --phrase is never a path, so an unrecognised shape is
        // withheld rather than printed.
        assert_eq!(mask_secret("deadbeef"), "<secret>");
    }
}
