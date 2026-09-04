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
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use clap::ArgMatches;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;

use crate::crypto::KeySource;
use crate::crypto::classify;
use crate::crypto::mask_key_source;
use crate::global_config_path;
use crate::helpers::default_config_name;

const TESTNET: &str = "shellnet.ackinacki.org";
const MAINNET: &str = "mainnet.ackinacki.org";
pub const LOCALNET: &str = "http://127.0.0.1/";

fn default_url() -> String {
    TESTNET.to_string()
}

fn default_wc() -> i32 {
    0
}

fn default_retries() -> u8 {
    5
}

fn default_depool_fee() -> f32 {
    0.5
}

fn default_timeout() -> u32 {
    40000
}

fn default_out_of_sync() -> u32 {
    15
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_lifetime() -> u32 {
    60
}

fn default_endpoints() -> Vec<String> {
    vec![]
}

fn default_aliases() -> BTreeMap<String, ContractData> {
    BTreeMap::new()
}

fn default_endpoints_map() -> BTreeMap<String, Vec<String>> {
    FullConfig::default_map()
}

fn default_trace() -> String {
    "None".to_string()
}

fn default_config() -> Config {
    Config::new()
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    #[serde(default = "default_url")]
    pub url: String,
    #[serde(default = "default_wc")]
    pub wc: i32,
    pub addr: Option<String>,
    pub method: Option<String>,
    pub parameters: Option<String>,
    pub wallet: Option<String>,
    pub pubkey: Option<String>,
    pub abi_path: Option<String>,
    pub keys_path: Option<String>,
    #[serde(default = "default_retries")]
    pub retries: u8,
    #[serde(default = "default_timeout")]
    pub timeout: u32,
    #[serde(default = "default_timeout")]
    pub message_processing_timeout: u32,
    #[serde(default = "default_out_of_sync")]
    pub out_of_sync_threshold: u32,
    #[serde(default = "default_false")]
    pub is_json: bool,
    #[serde(default = "default_depool_fee")]
    pub depool_fee: f32,
    #[serde(default = "default_lifetime")]
    pub lifetime: u32,
    #[serde(default = "default_true")]
    pub no_answer: bool,
    #[serde(default = "default_false")]
    pub balance_in_vmshells: bool,
    #[serde(default = "default_false")]
    pub local_run: bool,
    #[serde(default = "default_false")]
    pub async_call: bool,
    #[serde(default = "default_trace")]
    pub debug_fail: String,

    // SDK authentication parameters
    pub project_id: Option<String>,
    pub access_key: Option<String>,
    pub api_token: Option<String>,
    ////////////////////////////////
    #[serde(default = "default_endpoints")]
    pub endpoints: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ContractData {
    pub abi_path: Option<String>,
    pub address: Option<String>,
    pub key_path: Option<String>,
}

/// Replaces an inline secret stored in place of a keypair path by its kind.
/// Only the copy being printed is changed: what the config file holds is the
/// caller's business, masking is a property of the output.
fn masked_key_path(key_path: &Option<String>) -> Option<String> {
    key_path.as_deref().map(|path| mask_key_source(path).to_owned())
}

/// The kind of secret a value carries where a keypair path belongs, or `None`
/// when it names a file. `classify` decides, so this cannot disagree with what
/// masking hides or with what `load_keypair` will do with the same value.
fn inline_secret_kind(value: &str) -> Option<&'static str> {
    match classify(value) {
        KeySource::Phrase => Some("seed phrase"),
        KeySource::SecretKey => Some("secret key"),
        KeySource::Path => None,
    }
}

/// Refuses a value that carries the secret itself where a keypair path belongs.
/// The same `--keys` on `call` or `deploy` may hold a phrase, which lives no
/// longer than the process; a config file keeps it, in clear text, in the
/// working directory. The message names the kind of secret and never the
/// secret. `what` is the option that would do the storing.
pub fn reject_inline_secret(value: &str, config_path: &str, what: &str) -> Result<(), String> {
    match inline_secret_kind(value) {
        None => Ok(()),
        Some(kind) => Err(format!(
            "`{what}` would store a {kind} in {config_path}, which is written in clear text: the \
             wallet would outlive the command and follow the directory into git, into backups and \
             into images. Pass a path to a keypair file instead, converting this one with \
             `tvm-cli getkeypair --output <file> --phrase \"<your {kind}>\"`. If the value \
             already names a file, write it as a path -- `./my keys.json` rather than \
             `my keys.json`, which reads as several words."
        )),
    }
}

impl Config {
    /// A copy of this config safe to print.
    pub fn masked_for_display(&self) -> Config {
        Config { keys_path: masked_key_path(&self.keys_path), ..self.clone() }
    }
}

impl ContractData {
    /// A copy of this alias safe to print.
    fn masked_for_display(&self) -> ContractData {
        ContractData { key_path: masked_key_path(&self.key_path), ..self.clone() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FullConfig {
    #[serde(default = "default_config")]
    pub config: Config,
    #[serde(default = "default_endpoints_map")]
    pub endpoints_map: BTreeMap<String, Vec<String>>,
    #[serde(default = "default_aliases")]
    pub aliases: BTreeMap<String, ContractData>,
    #[serde(default = "default_config_name")]
    pub path: String,
    /// Set when this config was adopted from the global one under another
    /// path, so that `to_file` knows the secrets in it belong to a different
    /// file. Never serialised: it describes where the values came from, not
    /// what they are.
    #[serde(skip)]
    inherited_from_global: bool,
    /// Top-level fields of the file this was read from that no config has.
    /// Writing replaces the whole file, so they would be dropped.
    #[serde(skip)]
    unrecognised_fields: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: default_url(),
            api_token: None,
            wc: default_wc(),
            addr: None,
            method: None,
            parameters: None,
            wallet: None,
            pubkey: None,
            abi_path: None,
            keys_path: None,
            retries: default_retries(),
            timeout: default_timeout(),
            message_processing_timeout: default_timeout(),
            is_json: default_false(),
            depool_fee: default_depool_fee(),
            lifetime: default_lifetime(),
            no_answer: default_true(),
            balance_in_vmshells: default_false(),
            local_run: default_false(),
            async_call: default_false(),
            endpoints: default_endpoints(),
            out_of_sync_threshold: default_out_of_sync(),
            debug_fail: default_trace(),
            project_id: None,
            access_key: None,
        }
    }
}

impl Default for FullConfig {
    fn default() -> Self {
        FullConfig {
            config: default_config(),
            endpoints_map: default_endpoints_map(),
            aliases: default_aliases(),
            path: default_config_name(),
            inherited_from_global: false,
            unrecognised_fields: Vec::new(),
        }
    }
}

impl Config {
    fn new() -> Self {
        let url = default_url();
        let endpoints = FullConfig::default_map()[&url].clone();
        Config {
            url,
            api_token: None,
            wc: default_wc(),
            addr: None,
            method: None,
            parameters: None,
            wallet: None,
            pubkey: None,
            abi_path: None,
            keys_path: None,
            retries: default_retries(),
            timeout: default_timeout(),
            message_processing_timeout: default_timeout(),
            is_json: default_false(),
            depool_fee: default_depool_fee(),
            lifetime: default_lifetime(),
            no_answer: default_true(),
            balance_in_vmshells: default_false(),
            local_run: default_false(),
            async_call: default_false(),
            endpoints,
            out_of_sync_threshold: default_out_of_sync(),
            debug_fail: default_trace(),
            project_id: None,
            access_key: None,
        }
    }
}

const MAIN_ENDPOINTS: &[&str] = &["mainnet.ackinacki.org"];
const NET_ENDPOINTS: &[&str] = &["shellnet.ackinacki.org"];
const SE_ENDPOINTS: &[&str] = &["http://localhost"];

pub fn resolve_net_name(url: &str) -> Option<String> {
    let url_regex = Regex::new(r"^\s*(?:https?://)?(?P<net>\w+\.evercloud\.dev)\s*")
        .expect("Regex compilation error");
    let ton_url_regex = Regex::new(r"^\s*(?:https?://)?(?P<net>\w+\.ton\.dev)\s*")
        .expect("Regex compilation error");
    let everos_url_regex = Regex::new(r"^\s*(?:https?://)?(?P<net>\w+\.everos\.dev)\s*")
        .expect("Regex compilation error");
    let mut net = None;
    for regex in [url_regex, ton_url_regex, everos_url_regex] {
        if let Some(captures) = regex.captures(url) {
            net = Some(
                captures
                    .name("net")
                    .expect("Unexpected: capture <net> was not found")
                    .as_str()
                    .replace("ton", "evercloud")
                    .replace("everos", "evercloud"),
            );
        }
    }
    if let Some(net) = net {
        if FullConfig::default_map().contains_key(&net) {
            return Some(net);
        }
    }
    match url {
        "main" | "mainnet" => return Some(MAINNET.to_string()),
        "dev" | "devnet" | "shellnet" => return Some(TESTNET.to_string()),
        _ => {}
    };
    // if url.contains("127.0.0.1") ||
    //     url.contains("0.0.0.0") ||
    //     url.contains("localhost") {
    //     return Some(LOCALNET.to_string());
    // }
    None
}

/// The field names of a config, taken from the struct itself so that the list
/// cannot fall behind it. Both config structs give every field a serde default
/// and ignore unknown keys, so any json object deserialises into a config of
/// pure defaults -- knowing the real names is the only thing that tells a
/// config from a file of some other kind.
fn field_names<T: Serialize>(value: &T) -> BTreeSet<String> {
    match serde_json::to_value(value) {
        Ok(serde_json::Value::Object(fields)) => fields.keys().cloned().collect(),
        // An empty set would call every field unknown and refuse every write.
        // Nothing here can fail to serialise, so this is an assertion rather
        // than a fallback.
        other => {
            debug_assert!(false, "a config struct did not serialise into an object: {other:?}");
            BTreeSet::new()
        }
    }
}

fn config_fields() -> BTreeSet<String> {
    field_names(&Config::default())
}

/// The keys of the shape the tool writes, without `path`: that one is the
/// config's own location rather than a setting, and a file whose only known key
/// is `path` is likelier to belong to another tool.
fn full_config_fields() -> BTreeSet<String> {
    let mut names = field_names(&FullConfig::default());
    names.remove("path");
    names
}

/// The fields of `object` that `known` does not name, reported under `prefix`
/// so a nested one can be told from a top-level one.
fn unrecognised_fields(
    object: &serde_json::Map<String, serde_json::Value>,
    known: &BTreeSet<String>,
    prefix: &str,
) -> Vec<String> {
    object.keys().filter(|field| !known.contains(*field)).map(|f| format!("{prefix}{f}")).collect()
}

/// Everything in a config of the written shape that this version cannot account
/// for, at every level whose fields are fixed. Settings live inside `config`
/// and inside each alias, which is where a field from another version turns up;
/// `endpoints_map` is keyed by url, so its keys are values, not field names.
fn unrecognised_in_full_config(fields: &serde_json::Map<String, serde_json::Value>) -> Vec<String> {
    let mut found = unrecognised_fields(fields, &field_names(&FullConfig::default()), "");
    if let Some(serde_json::Value::Object(config)) = fields.get("config") {
        found.extend(unrecognised_fields(config, &config_fields(), "config."));
    }
    if let Some(serde_json::Value::Object(aliases)) = fields.get("aliases") {
        let known = field_names(&ContractData::default());
        for (alias, data) in aliases {
            if let serde_json::Value::Object(data) = data {
                found.extend(unrecognised_fields(data, &known, &format!("aliases.{alias}.")));
            }
        }
    }
    found
}

/// Every refusal to read a config has to leave a way out, since the config is
/// read before the command runs -- including the `getkeypair` that fixing one
/// may require.
fn unusable_config(path: &str, reason: &str) -> String {
    format!(
        "{path} cannot be used as a config file: {reason}. Fix it, remove it, or point `--config` \
         at another file -- writing this one would replace everything in it."
    )
}

impl FullConfig {
    fn new(config: Config, path: String) -> Self {
        FullConfig {
            config,
            endpoints_map: Self::default_map(),
            aliases: BTreeMap::new(),
            path,
            inherited_from_global: false,
            unrecognised_fields: Vec::new(),
        }
    }

    pub fn default_map() -> BTreeMap<String, Vec<String>> {
        [(MAINNET, MAIN_ENDPOINTS), (TESTNET, NET_ENDPOINTS), (LOCALNET, SE_ENDPOINTS)]
            .iter()
            .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect()))
            .collect()
    }

    /// Reads the config file at `path`, or `None` when there is none there: a
    /// missing or empty file means this directory has no config of its own.
    ///
    /// A file that holds something but cannot be used as a config is an error.
    /// Reading it as an empty config would be worse than refusing: every value
    /// in it would be silently replaced by a default, and the next write would
    /// put that over the file.
    fn read_config_file(path: &str) -> Result<Option<FullConfig>, String> {
        let conf_str = match std::fs::read_to_string(path) {
            Ok(conf_str) => conf_str,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("failed to read the config file {path}: {e}")),
        };
        if conf_str.trim().is_empty() {
            return Ok(None);
        }
        let fields = match serde_json::from_str::<serde_json::Value>(&conf_str) {
            Ok(serde_json::Value::Object(fields)) => fields,
            Ok(_) => return Err(unusable_config(path, "it is not a json object")),
            Err(e) => return Err(unusable_config(path, &format!("it is not valid json: {e}"))),
        };
        let names = |known: BTreeSet<String>| fields.keys().any(|field| known.contains(field));
        // A file may name fields of both shapes, and it is read as exactly one
        // of them. What counts as recognised is the shape actually chosen: the
        // fields of the other one would otherwise pass for known while serde
        // quietly ignored them.
        let (mut full_config, unrecognised) = if names(full_config_fields()) {
            let parsed = serde_json::from_str::<FullConfig>(&conf_str)
                .map_err(|e| unusable_config(path, &format!("it does not fit a config: {e}")))?;
            (parsed, unrecognised_in_full_config(&fields))
        } else if names(config_fields()) {
            // The bare `Config` object that older versions of the tool wrote.
            let config = serde_json::from_str::<Config>(&conf_str)
                .map_err(|e| unusable_config(path, &format!("it does not fit a config: {e}")))?;
            let unrecognised = unrecognised_fields(&fields, &config_fields(), "");
            (FullConfig::new(config, path.to_string()), unrecognised)
        } else if fields.is_empty() {
            // An object with no fields names nothing and contradicts nothing.
            (FullConfig::default(), Vec::new())
        } else {
            return Err(unusable_config(path, "it names no field that a config has"));
        };
        full_config.unrecognised_fields = unrecognised;
        full_config.path = path.to_string();
        Ok(Some(full_config))
    }

    /// Reads the config for a run. A directory with no config of its own adopts
    /// the global one, marked as inherited so that a secret in it is not
    /// written out here.
    pub fn from_file(path: &str) -> Result<FullConfig, String> {
        if let Some(config) = Self::read_config_file(path)? {
            return Ok(config);
        }
        let global = global_config_path();
        // The global config sits next to the executable, so on a shared install
        // one unusable file there would stop every command for everyone. It is
        // a fallback: a command that does not depend on it says so and carries
        // on. Asked about that file itself, the read above has already failed.
        let mut config = match Self::read_config_file(&global) {
            Ok(config) => config.unwrap_or_default(),
            Err(e) => {
                eprintln!("Warning: {e} The default settings are used instead.");
                FullConfig::default()
            }
        };
        // Not inherited when this *is* the global config: there is one file,
        // and writing it back is what `config --global` is for.
        config.inherited_from_global = path != global;
        config.path = path.to_string();
        Ok(config)
    }

    /// A config adopted from the global one holds secrets that belong to that
    /// other file. Writing it here would either put a second clear-text copy of
    /// the wallet in a file that did not exist before, or drop the value and
    /// leave every later command in this directory running unsigned with
    /// nothing left to warn about -- the same silent downgrade, one step later.
    /// So the write stops and names what to fix. Replacing the value in the
    /// same command settles it too, but only the command that owns that value
    /// can: `config --keys <file>` for the config's own key, `config alias add
    /// <name> --keys <file>` for an alias, and neither for the other. `config
    /// clear --keys` removes the value rather than setting one. That is why the
    /// message names the file and the command that fixes it there, instead of
    /// offering `--keys` on whatever happens to be running.
    fn refuse_inherited_secret(&self) -> Result<(), String> {
        if !self.inherited_from_global {
            return Ok(());
        }
        // The advice has to name the file that holds the value and a command
        // that sets it there. `--keys` on this command would not do: `config
        // clear --keys` throws the key away, and most `config` subcommands do
        // not take it at all.
        let found = self
            .config
            .keys_path
            .as_deref()
            .and_then(inline_secret_kind)
            .map(|kind| {
                (format!("a {kind}"), "`tvm-cli config --global --keys <file>`".to_string())
            })
            .or_else(|| {
                self.aliases.iter().find_map(|(alias, data)| {
                    data.key_path.as_deref().and_then(inline_secret_kind).map(|kind| {
                        (
                            format!("a {kind} for the alias \"{alias}\""),
                            format!("`tvm-cli config --global alias add {alias} --keys <file>`"),
                        )
                    })
                })
            });
        match found {
            None => Ok(()),
            Some((held, fix)) => Err(format!(
                "the global config keeps {held} where a keypair path belongs, and this directory \
                 has no config of its own. Writing {} would either copy the wallet into a second \
                 clear-text file or drop it, leaving later commands here to run unsigned. Fix the \
                 global config: `tvm-cli getkeypair --output <file> --phrase \"<your secret>\"`, \
                 then {fix}.",
                self.path
            )),
        }
    }

    /// The tool replaces a config file wholesale, so anything in it that the
    /// tool cannot account for would be dropped. Reading such a file is fine --
    /// every command that only reads it keeps working -- but replacing it is
    /// not.
    fn refuse_to_drop_unrecognised_fields(&self) -> Result<(), String> {
        // What the global config carries stays in the global config: writing
        // this path replaces a different file, and refusing here would stop a
        // directory over a file it does not own. It is still reported, by
        // `warn_about_the_config_file`.
        if self.unrecognised_fields.is_empty() || self.inherited_from_global {
            return Ok(());
        }
        // `--config` names another file to use instead, which is no escape for
        // the global config: nothing else stands in for it.
        let escape = if self.path == global_config_path() {
            "Remove those fields from it, or delete the file to start from the defaults."
        } else {
            "Fix the file, or point `--config` at another one."
        };
        Err(format!(
            "{} holds fields that no config has ({}), and writing the config would replace the \
             whole file. {escape}",
            self.path,
            self.unrecognised_fields.join(", ")
        ))
    }

    /// Whether writing this config would go through. A command that does
    /// something irreversible before saving -- broadcasting a deploy, then
    /// recording its alias -- asks this first, so that the answer comes from
    /// the code that will do the writing rather than from a second guess at it.
    pub fn check_writable(&self) -> Result<(), String> {
        self.refuse_to_drop_unrecognised_fields()?;
        self.refuse_inherited_secret()
    }

    pub fn to_file(&self, path: &str) -> Result<(), String> {
        self.check_writable()?;
        let conf_str = serde_json::to_string_pretty(self)
            .map_err(|_| "failed to serialize config object".to_string())?;
        std::fs::write(path, conf_str)
            .map_err(|e| format!("failed to write config file {}: {}", path, e))?;
        Ok(())
    }

    pub fn print_endpoints(path: &str) -> Result<(), String> {
        let fconf = FullConfig::from_file(path)?;
        println!(
            "{}",
            serde_json::to_string_pretty(&fconf.endpoints_map)
                .unwrap_or("Failed to print endpoints map.".to_owned())
        );
        Ok(())
    }

    /// Reports what is worth knowing about the file this config came from:
    /// fields no config has, which are ignored and stop it being written, and
    /// secrets held where a keypair path belongs. Printing the config masks
    /// those, so nothing else tells their owner that what is on disk is the
    /// wallet itself.
    pub fn warn_about_the_config_file(&self) {
        if !self.unrecognised_fields.is_empty() {
            if self.inherited_from_global {
                // The values are not lost -- the global config keeps them --
                // but this run does not have them, and used to say nothing.
                eprintln!(
                    "Warning: the global config holds fields that no config has ({}). They are \
                     ignored, so what this directory inherits is not what that file says.",
                    self.unrecognised_fields.join(", ")
                );
            } else {
                eprintln!(
                    "Warning: {} holds fields that no config has ({}). They are ignored, and \
                     writing the config here is refused rather than dropping them.",
                    self.path,
                    self.unrecognised_fields.join(", ")
                );
            }
        }
        if let Some(kind) = self.config.keys_path.as_deref().and_then(inline_secret_kind) {
            if self.inherited_from_global {
                eprintln!(
                    "Warning: the global config stores a {kind} where a keypair path belongs. \
                     Commands here read it from there, and it is never written into {}; replace \
                     it with `tvm-cli config --global --keys <file>`.",
                    self.path
                );
            } else {
                // `config --keys` writes the config of the working directory,
                // which would leave a phrase in the global one untouched.
                let set = if self.path == global_config_path() {
                    "`tvm-cli config --global --keys <file>`"
                } else {
                    "`tvm-cli config --keys <file>`"
                };
                eprintln!(
                    "Warning: {} stores a {kind} in clear text where a keypair path belongs. \
                     Convert it with `tvm-cli getkeypair --output <file> --phrase \"<your \
                     {kind}>\"`, then {set}.",
                    self.path
                );
            }
        }
        for (alias, data) in &self.aliases {
            if let Some(kind) = data.key_path.as_deref().and_then(inline_secret_kind) {
                if self.inherited_from_global {
                    eprintln!(
                        "Warning: alias \"{alias}\" in the global config keeps a {kind} where a \
                         keypair path belongs. It is not written into {}.",
                        self.path
                    );
                } else {
                    eprintln!(
                        "Warning: alias \"{alias}\" in {} keeps a {kind} in clear text where a \
                         keypair path belongs.",
                        self.path
                    );
                }
            }
        }
    }

    pub fn print_aliases(&self) {
        let aliases: BTreeMap<&String, ContractData> =
            self.aliases.iter().map(|(name, data)| (name, data.masked_for_display())).collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&aliases)
                .unwrap_or("Failed to print aliases map.".to_owned())
        );
    }

    pub fn add_alias(
        &mut self,
        alias: &str,
        address: Option<String>,
        abi: Option<String>,
        key_path: Option<String>,
    ) -> Result<(), String> {
        if let Some(key_path) = key_path.as_deref() {
            reject_inline_secret(key_path, &self.path, "--keys")?;
        }
        // Given values replace what the alias holds and the rest of it stays.
        // The refusal above prints `alias add ... --keys <file>` as the way to
        // replace a key, and that must not cost the address and the ABI.
        let data = self.aliases.entry(alias.to_owned()).or_default();
        if address.is_some() {
            data.address = address;
        }
        if abi.is_some() {
            data.abi_path = abi;
        }
        if key_path.is_some() {
            data.key_path = key_path;
        }
        self.to_file(&self.path)
    }

    pub fn remove_alias(&mut self, alias: &str) -> Result<(), String> {
        self.aliases.remove(alias);
        self.to_file(&self.path)
    }

    pub fn add_endpoint(path: &str, url: &str, endpoints: &str) -> Result<(), String> {
        let mut fconf = FullConfig::from_file(path)?;
        let mut new_endpoints: Vec<String> =
            endpoints.replace(['[', ']'], "").split(',').map(|s| s.to_string()).collect();

        let old_endpoints = fconf.endpoints_map.entry(url.to_string()).or_default();
        old_endpoints.append(&mut new_endpoints);
        old_endpoints.sort();
        old_endpoints.dedup();
        fconf.to_file(path)
    }

    pub fn remove_endpoint(path: &str, url: &str) -> Result<(), String> {
        let mut fconf = FullConfig::from_file(path)?;
        if !fconf.endpoints_map.contains_key(url) {
            return Err("Endpoints map doesn't contain such url.".to_owned());
        }
        fconf.endpoints_map.remove(url);
        fconf.to_file(path)
    }

    pub fn reset_endpoints(path: &str) -> Result<(), String> {
        let mut fconf = FullConfig::from_file(path)?;
        fconf.endpoints_map = FullConfig::default_map();
        fconf.to_file(path)
    }
}

pub fn clear_config(
    full_config: &mut FullConfig,
    matches: &ArgMatches,
    is_json: bool,
) -> Result<(), String> {
    let config = &mut full_config.config;
    let is_json = config.is_json || is_json;
    if matches.is_present("URL") {
        let url = default_url();
        config.endpoints = FullConfig::default_map()[&url].clone();
        config.url = url;
    }
    if matches.is_present("API_TOKEN") {
        config.api_token = None;
    }
    if matches.is_present("ADDR") {
        config.addr = None;
    }
    if matches.is_present("WALLET") {
        config.wallet = None;
    }
    if matches.is_present("ABI") {
        config.abi_path = None;
    }
    if matches.is_present("KEYS") {
        config.keys_path = None;
    }
    if matches.is_present("METHOD") {
        config.method = None;
    }
    if matches.is_present("PARAMETERS") {
        config.parameters = None;
    }
    if matches.is_present("PUBKEY") {
        config.pubkey = None;
    }
    if matches.is_present("RETRIES") {
        config.retries = default_retries();
    }
    if matches.is_present("LIFETIME") {
        config.lifetime = default_lifetime();
    }
    if matches.is_present("TIMEOUT") {
        config.timeout = default_timeout();
    }
    if matches.is_present("MSG_TIMEOUT") {
        config.timeout = default_timeout();
    }
    if matches.is_present("WC") {
        config.wc = default_wc();
    }
    if matches.is_present("DEPOOL_FEE") {
        config.depool_fee = default_depool_fee();
    }
    if matches.is_present("NO_ANSWER") {
        config.no_answer = default_true();
    }
    if matches.is_present("BALANCE_IN_VMSHELLS") {
        config.balance_in_vmshells = default_false();
    }
    if matches.is_present("LOCAL_RUN") {
        config.local_run = default_false();
    }
    if matches.is_present("ASYNC_CALL") {
        config.async_call = default_false();
    }
    if matches.is_present("DEBUG_FAIL") {
        config.debug_fail = default_trace();
    }
    if matches.is_present("OUT_OF_SYNC") {
        config.out_of_sync_threshold = default_out_of_sync();
    }
    if matches.is_present("IS_JSON") {
        config.is_json = default_false();
    }
    if matches.is_present("PROJECT_ID") {
        config.project_id = None;
        if config.access_key.is_some() && !config.is_json {
            println!(
                "Warning: You have access_key set without project_id. It has no sense in case of authentication."
            );
        }
    }
    if matches.is_present("ACCESS_KEY") {
        config.access_key = None;
    }

    if !matches.args_present() {
        *config = Config::new();
    }

    full_config.to_file(&full_config.path)?;
    if !is_json {
        println!("Succeeded.");
    }
    Ok(())
}

pub fn set_config(
    full_config: &mut FullConfig,
    matches: &ArgMatches,
    is_json: bool,
) -> Result<(), String> {
    let config = &mut full_config.config;
    if let Some(s) = matches.value_of("URL") {
        let resolved_url = resolve_net_name(s).unwrap_or(s.to_owned());
        let empty: Vec<String> = Vec::new();
        config.endpoints = full_config.endpoints_map.get(&resolved_url).unwrap_or(&empty).clone();
        config.url = resolved_url;
    }
    if let Some(s) = matches.value_of("ADDR") {
        config.addr = Some(s.to_string());
    }
    if let Some(method) = matches.value_of("METHOD") {
        config.method = Some(method.to_string());
    }
    if let Some(parameters) = matches.value_of("PARAMETERS") {
        config.parameters = Some(parameters.to_string());
    }
    if let Some(s) = matches.value_of("WALLET") {
        config.wallet = Some(s.to_string());
    }
    if let Some(s) = matches.value_of("PUBKEY") {
        config.pubkey = Some(s.to_string());
    }
    if let Some(s) = matches.value_of("ABI") {
        config.abi_path = Some(s.to_string());
    }
    if let Some(s) = matches.value_of("KEYS") {
        reject_inline_secret(s, &full_config.path, "--keys")?;
        config.keys_path = Some(s.to_string());
    }
    if let Some(retries) = matches.value_of("RETRIES") {
        config.retries = u8::from_str_radix(retries, 10)
            .map_err(|e| format!(r#"failed to parse "retries": {}"#, e))?;
    }
    if let Some(lifetime) = matches.value_of("LIFETIME") {
        config.lifetime = u32::from_str_radix(lifetime, 10)
            .map_err(|e| format!(r#"failed to parse "lifetime": {}"#, e))?;
        if config.lifetime < 2 * config.out_of_sync_threshold {
            config.out_of_sync_threshold = config.lifetime >> 1;
        }
    }
    if let Some(timeout) = matches.value_of("TIMEOUT") {
        config.timeout = u32::from_str_radix(timeout, 10)
            .map_err(|e| format!(r#"failed to parse "timeout": {}"#, e))?;
    }
    if let Some(message_processing_timeout) = matches.value_of("MSG_TIMEOUT") {
        config.message_processing_timeout = u32::from_str_radix(message_processing_timeout, 10)
            .map_err(|e| format!(r#"failed to parse "message_processing_timeout": {}"#, e))?;
    }
    if let Some(wc) = matches.value_of("WC") {
        config.wc = i32::from_str_radix(wc, 10)
            .map_err(|e| format!(r#"failed to parse "workchain id": {}"#, e))?;
    }
    if let Some(depool_fee) = matches.value_of("DEPOOL_FEE") {
        let depool_fee = depool_fee
            .parse::<f32>()
            .map_err(|e| format!(r#"failed to parse "depool_fee": {}"#, e))?;
        // Json has no infinity and no NaN: either would be written as `null`,
        // and the config would not parse again -- a valid command locking the
        // file it had just written.
        if !depool_fee.is_finite() {
            return Err(r#""depool_fee" must be a finite number"#.to_string());
        }
        if depool_fee < 0.5 {
            return Err("Minimal value for depool fee is 0.5".to_string());
        }
        config.depool_fee = depool_fee;
    }
    if let Some(no_answer) = matches.value_of("NO_ANSWER") {
        config.no_answer = no_answer
            .parse::<bool>()
            .map_err(|e| format!(r#"failed to parse "no_answer": {}"#, e))?;
    }
    if let Some(balance_in_vmshells) = matches.value_of("BALANCE_IN_VMSHELLS") {
        config.balance_in_vmshells = balance_in_vmshells
            .parse::<bool>()
            .map_err(|e| format!(r#"failed to parse "balance_in_vmshells": {}"#, e))?;
    }
    if let Some(local_run) = matches.value_of("LOCAL_RUN") {
        config.local_run = local_run
            .parse::<bool>()
            .map_err(|e| format!(r#"failed to parse "local_run": {}"#, e))?;
    }
    if let Some(async_call) = matches.value_of("ASYNC_CALL") {
        config.async_call = async_call
            .parse::<bool>()
            .map_err(|e| format!(r#"failed to parse "async_call": {}"#, e))?;
    }
    if let Some(out_of_sync_threshold) = matches.value_of("OUT_OF_SYNC") {
        let time = u32::from_str_radix(out_of_sync_threshold, 10)
            .map_err(|e| format!(r#"failed to parse "out_of_sync_threshold": {}"#, e))?;
        if time * 2 > config.lifetime {
            return Err("\"out_of_sync\" should not exceed 0.5 * \"lifetime\".".to_string());
        }
        config.out_of_sync_threshold = time;
    }
    if let Some(debug_fail) = matches.value_of("DEBUG_FAIL") {
        let debug_fail = debug_fail.to_lowercase();
        config.debug_fail = if debug_fail == "full" {
            "Full".to_string()
        } else if debug_fail == "minimal" {
            "Minimal".to_string()
        } else if debug_fail == "none" {
            "None".to_string()
        } else {
            return Err(r#"Wrong value for "debug_fail" config."#.to_string());
        };
    }
    if let Some(is_json) = matches.value_of("IS_JSON") {
        config.is_json =
            is_json.parse::<bool>().map_err(|e| format!(r#"failed to parse "is_json": {}"#, e))?;
    }
    if let Some(s) = matches.value_of("PROJECT_ID") {
        config.project_id = Some(s.to_string());
    }
    if let Some(s) = matches.value_of("ACCESS_KEY") {
        config.access_key = Some(s.to_string());
        if config.project_id.is_none() && !(config.is_json || is_json) {
            println!(
                "Warning: You have access_key set without project_id. It has no sense in case of authentication."
            );
        }
    }
    if let Some(s) = matches.value_of("API_TOKEN") {
        config.api_token = Some(s.to_string());
    }
    full_config.to_file(&full_config.path)?;
    if !(full_config.config.is_json || is_json) {
        println!("Succeeded.");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::MAINNET;
    use super::TESTNET;
    use super::resolve_net_name;

    #[test]
    fn test_endpoints_resolver() {
        assert_eq!(resolve_net_name(""), None);
        assert_eq!(resolve_net_name("http://os.ton.dev"), None);
        assert_eq!(resolve_net_name("https://rustnet.ton.dev"), None);
        assert_eq!(resolve_net_name("rustnet.ton.com"), None);
        assert_eq!(resolve_net_name("https://example.com"), None);
        assert_eq!(resolve_net_name("http://localhost"), None);
        assert_eq!(resolve_net_name("https://localhost"), None);
        assert_eq!(resolve_net_name("localhost"), None);
        assert_eq!(resolve_net_name("http://127.0.0.1"), None);
        assert_eq!(resolve_net_name("https://127.0.0.1"), None);
        assert_eq!(resolve_net_name("https://127.0.0.2"), None);
        assert_eq!(resolve_net_name("https://127.1.0.1"), None);
        assert_eq!(resolve_net_name("https://0.0.0.1"), None);
        assert_eq!(resolve_net_name("https://1.0.0.0"), None);

        // assert_eq!(resolve_net_name("https://main.ton.dev"), Some(MAINNET.to_owned()));
        // assert_eq!(resolve_net_name("https://main.everos.dev"), Some(MAINNET.to_owned()));
        // assert_eq!(resolve_net_name("https://main.evercloud.dev"), Some(MAINNET.to_owned()));
        // assert_eq!(resolve_net_name("http://main.ton.dev"), Some(MAINNET.to_owned()));
        // assert_eq!(resolve_net_name("  http://main.ton.dev  "), Some(MAINNET.to_owned()));
        // assert_eq!(resolve_net_name("  https://main.ton.dev  "), Some(MAINNET.to_owned()));
        // assert_eq!(resolve_net_name("main.ton.dev"),
        // Some(MAINNET.to_owned())); assert_eq!(resolve_net_name("main.
        // everos.dev"), Some(MAINNET.to_owned()));
        // assert_eq!(resolve_net_name("main.evercloud.dev"),
        // Some(MAINNET.to_owned()));
        assert_eq!(resolve_net_name("main"), Some(MAINNET.to_owned()));
        assert_eq!(resolve_net_name("mainnet"), Some(MAINNET.to_owned()));
        assert_eq!(resolve_net_name("main.ton.com"), None);

        // assert_eq!(resolve_net_name("https://net.ton.dev"), Some(TESTNET.to_owned()));
        // assert_eq!(resolve_net_name("https://net.everos.dev"), Some(TESTNET.to_owned()));
        // assert_eq!(resolve_net_name("https://net.evercloud.dev"), Some(TESTNET.to_owned()));
        // assert_eq!(resolve_net_name("http://net.ton.dev"), Some(TESTNET.to_owned()));
        // assert_eq!(resolve_net_name("  http://net.ton.dev  "), Some(TESTNET.to_owned()));
        // assert_eq!(resolve_net_name("  https://net.ton.dev  "), Some(TESTNET.to_owned()));
        // assert_eq!(resolve_net_name("net.ton.dev"),
        // Some(TESTNET.to_owned()));
        assert_eq!(resolve_net_name("dev"), Some(TESTNET.to_owned()));
        assert_eq!(resolve_net_name("devnet"), Some(TESTNET.to_owned()));
        assert_eq!(resolve_net_name("shellnet"), Some(TESTNET.to_owned()));
    }
}
