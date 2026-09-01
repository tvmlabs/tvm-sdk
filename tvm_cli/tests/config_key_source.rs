use std::fs;
use std::path::Path;
use std::path::PathBuf;

use assert_cmd::Command;
use predicates::prelude::*;
use testdir::testdir;

const BIN_NAME: &str = "tvm-cli";

/// A throwaway phrase, already used as a test vector elsewhere in this repo.
const PHRASE: &str =
    "multiply extra monitor fog rocket defy attack right night jaguar hollow enlist";
/// The secret key the phrase above derives to.
const SECRET_KEY: &str = "c4415c03aa9d824e89ff4555cd12497aef1d5123f839803b0268e27ba6052354";

const TVC: &str = "tests/decode_fields.tvc";
const ABI: &str = "tests/test_abi_v2.1.abi.json";
/// Self-rooted `dapp_id::account_id`: the strict form the CLI requires of
/// external input.
const ADDRESS: &str = "ece57bcc6c530283becbbd8a3b24d3c5987cdddc3c8b7b33be6e4a6312490415\
                       ::ece57bcc6c530283becbbd8a3b24d3c5987cdddc3c8b7b33be6e4a6312490415";

/// A config file of the test's own, so that neither the developer's
/// `tvm_cli/tvm-cli.conf.json` nor the global config in the build directory
/// decides what these tests observe. The file has to exist and hold a field a
/// config has: an empty or missing one adopts the global config instead.
fn config_with(contents: &str) -> PathBuf {
    let path = testdir!().join("tvm-cli.conf.json");
    fs::write(&path, contents).unwrap();
    path
}

fn empty_config() -> PathBuf {
    config_with(r#"{"retries": 1}"#)
}

fn tvm_cli(config: &Path) -> Command {
    let mut cmd = Command::cargo_bin(BIN_NAME).unwrap();
    cmd.arg("--config").arg(config);
    cmd
}

/// Holds the executable's inode for as long as the link exists, so the link
/// goes when the test does: test directories outlive the build, and a rebuild
/// would otherwise leave a gigabyte of unreferenced binary behind each run.
struct ExeLink(PathBuf);

impl Drop for ExeLink {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// The global config lives next to the executable, so running the binary
/// through a link in the test's own directory gives the test a global config of
/// its own instead of the one shared by the whole build directory. A hard link
/// rather than a copy: the debug binary is most of a gigabyte.
fn cli_with_global_config(dir: &Path, global: &str) -> (Command, ExeLink) {
    let exe = dir.join(BIN_NAME);
    if !exe.exists() {
        fs::hard_link(assert_cmd::cargo::cargo_bin(BIN_NAME), &exe).unwrap();
    }
    fs::write(dir.join(".tvm-cli.global.conf.json"), global).unwrap();
    (Command::new(&exe), ExeLink(exe))
}

fn global_config_holding_the_phrase() -> String {
    format!(r#"{{"config": {{"retries": 1, "keys_path": "{PHRASE}"}}}}"#)
}

/// The whole point of the refusal: whatever else happens, the secret must not
/// end up in the file. Checked against the first three words too, since a
/// config that stored the phrase mangled would be just as bad.
fn assert_no_secret_on_disk(config: &Path) {
    let stored = fs::read_to_string(config).unwrap();
    assert!(!stored.contains(PHRASE), "the phrase reached {}", config.display());
    assert!(!stored.contains("multiply extra monitor"), "part of the phrase reached the config");
    assert!(!stored.contains(SECRET_KEY), "the secret key reached {}", config.display());
}

#[test]
fn config_keys_refuses_a_seed_phrase() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("config")
        .arg("--keys")
        .arg(PHRASE)
        .assert()
        .failure()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains("would store a seed phrase"));
    assert_no_secret_on_disk(&config);
}

#[test]
fn config_keys_refuses_a_raw_secret_key() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("config")
        .arg("--keys")
        .arg(SECRET_KEY)
        .assert()
        .failure()
        .stdout(predicate::str::contains(SECRET_KEY).not())
        .stdout(predicate::str::contains("would store a secret key"));
    assert_no_secret_on_disk(&config);
}

/// The config file is written once, after every option has been applied, so a
/// refused `--keys` must take the whole command down with it rather than leave
/// the other options half-saved.
#[test]
fn a_refused_key_leaves_the_other_options_unwritten() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("config")
        .arg("--url")
        .arg("http://example.invalid")
        .arg("--keys")
        .arg(PHRASE)
        .assert()
        .failure();
    let stored = fs::read_to_string(&config).unwrap();
    assert!(!stored.contains("example.invalid"), "the url was saved by a command that failed");
    assert_no_secret_on_disk(&config);
}

/// Printing the config after the write failed shows settings that were never
/// saved, and under `--json` puts a second document on a stream that promises
/// one.
#[test]
fn a_refused_key_does_not_print_the_config_it_did_not_save() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("--json")
        .arg("config")
        .arg("--url")
        .arg("http://example.invalid")
        .arg("--keys")
        .arg(PHRASE)
        .assert()
        .failure()
        .stdout(predicate::str::contains("example.invalid").not())
        .stdout(predicate::str::contains(r#""keys_path""#).not());
}

/// Naming the keypair file before generating it is the usual order of work, so
/// the path is not required to exist.
#[test]
fn config_keys_accepts_a_path_that_does_not_exist_yet() {
    let config = empty_config();
    tvm_cli(&config).arg("config").arg("--keys").arg("./key.json").assert().success();
    assert!(fs::read_to_string(&config).unwrap().contains("./key.json"));
}

/// A directory name with a space in it makes the value look like several words,
/// which is all a seed phrase is. Words of a phrase are letters and nothing
/// else, so a path is still a path.
#[test]
fn config_keys_accepts_a_path_containing_a_space() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("config")
        .arg("--keys")
        .arg("./My Wallets/msig.keys.json")
        .assert()
        .success();
    assert!(fs::read_to_string(&config).unwrap().contains("./My Wallets/msig.keys.json"));
}

/// A keypair file named after its public key is 64 hex characters and nothing
/// else -- the shape of a raw secret key, and refused as one. Spelling it as a
/// path is the way through, and needs no guess about what exists on disk.
#[test]
fn config_keys_accepts_a_hex_file_name_spelled_as_a_path() {
    let config = empty_config();
    let path = format!("./{SECRET_KEY}");
    tvm_cli(&config).arg("config").arg("--keys").arg(&path).assert().success();
    assert!(fs::read_to_string(&config).unwrap().contains(&path));
}

/// What `keys_path` is allowed to hold cannot depend on where the tool happened
/// to run: the value is stored, the directory is not.
#[test]
fn the_refusal_does_not_depend_on_the_current_directory() {
    let dir = testdir!();
    let sub = dir.join("sub");
    fs::create_dir_all(&sub).unwrap();
    // A file whose name is exactly the value under test, to make sure its
    // presence is not what decides the answer.
    fs::write(dir.join(SECRET_KEY), "{}").unwrap();
    let config = dir.join("tvm-cli.conf.json");
    fs::write(&config, r#"{"retries": 1}"#).unwrap();

    for cwd in [&dir, &sub] {
        Command::cargo_bin(BIN_NAME)
            .unwrap()
            .current_dir(cwd)
            .arg("--config")
            .arg(&config)
            .arg("config")
            .arg("--keys")
            .arg(SECRET_KEY)
            .assert()
            .failure()
            .stdout(predicate::str::contains("would store a secret key"));
    }
}

#[test]
fn config_alias_add_refuses_a_seed_phrase() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("config")
        .arg("alias")
        .arg("add")
        .arg("msig")
        .arg("--addr")
        .arg(ADDRESS)
        .arg("--abi")
        .arg(ABI)
        .arg("--keys")
        .arg(PHRASE)
        .assert()
        .failure()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains("would store a seed phrase"));
    let stored = fs::read_to_string(&config).unwrap();
    assert!(!stored.contains("msig"), "the alias was saved by a command that failed");
    assert_no_secret_on_disk(&config);
}

#[test]
fn config_alias_add_accepts_a_path() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("config")
        .arg("alias")
        .arg("add")
        .arg("msig")
        .arg("--addr")
        .arg(ADDRESS)
        .arg("--abi")
        .arg(ABI)
        .arg("--keys")
        .arg("./msig.key.json")
        .assert()
        .success();
    assert!(fs::read_to_string(&config).unwrap().contains("./msig.key.json"));
}

/// `--alias` saves the signing key in the config file, and the key may come
/// from the config rather than the command line -- so the refusal has to happen
/// before the deploy is broadcast, not when the alias is written afterwards.
#[test]
fn deploy_refuses_to_store_a_secret_in_an_alias_before_deploying() {
    let config = config_with(&format!(r#"{{"retries": 1, "keys_path": "{PHRASE}"}}"#));
    tvm_cli(&config)
        .arg("deploy")
        .arg(TVC)
        .arg("{}")
        .arg("--abi")
        .arg(ABI)
        .arg("--alias")
        .arg("msig")
        .assert()
        .failure()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains("would store a seed phrase"))
        .stdout(predicate::str::contains("Deploying...").not());
}

/// A config written before the refusal existed still holds a phrase, and after
/// NODE-3871 printing it shows `<seed phrase>` -- so nothing on screen tells
/// its owner that the file on disk is a wallet. The warning is the only thing
/// that does, and its advice has to reproduce the same keypair: `getkeypair`
/// without `--phrase` invents a new one.
#[test]
fn a_stored_phrase_is_reported_on_stderr() {
    let config = config_with(&format!(r#"{{"retries": 1, "keys_path": "{PHRASE}"}}"#));
    tvm_cli(&config)
        .arg("config")
        .arg("--list")
        .assert()
        .success()
        .stderr(predicate::str::contains("seed phrase"))
        .stderr(predicate::str::contains("--phrase"))
        .stderr(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains(PHRASE).not());
}

#[test]
fn a_stored_alias_phrase_is_reported_on_stderr() {
    let config = config_with(&format!(
        r#"{{"config": {{"retries": 1}}, "aliases": {{"msig": {{"key_path": "{PHRASE}"}}}}}}"#
    ));
    tvm_cli(&config)
        .arg("config")
        .arg("--list")
        .assert()
        .success()
        .stderr(predicate::str::contains("msig"))
        .stderr(predicate::str::contains("seed phrase"))
        .stderr(predicate::str::contains(PHRASE).not());
}

/// A directory with no config of its own adopts the global config, and the
/// first write would either copy the secret into a file that did not exist
/// before or drop it and leave every later command in that directory running
/// unsigned. Neither is acceptable, so the write stops and names the file to
/// fix.
#[test]
fn writing_a_config_is_refused_while_the_global_one_holds_a_secret() {
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();
    let local = work.join("local.conf.json");

    let (mut cmd, _link) = cli_with_global_config(&dir, &global_config_holding_the_phrase());
    cmd.current_dir(&work)
        .arg("--config")
        .arg(&local)
        .arg("config")
        .arg("--url")
        .arg("main")
        .assert()
        .failure()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains("global config"))
        .stdout(predicate::str::contains("--global --keys"));

    assert!(!local.exists(), "a config file was written after the write was refused");
}

#[test]
fn writing_a_config_is_refused_while_a_global_alias_holds_a_secret() {
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();
    let local = work.join("local.conf.json");

    let (mut cmd, _link) = cli_with_global_config(
        &dir,
        &format!(
            r#"{{"config": {{"retries": 1}}, "aliases": {{"msig": {{"key_path": "{PHRASE}"}}}}}}"#
        ),
    );
    cmd.current_dir(&work)
        .arg("--config")
        .arg(&local)
        .arg("config")
        .arg("--url")
        .arg("main")
        .assert()
        .failure()
        .stdout(predicate::str::contains("msig"));

    assert!(!local.exists(), "a config file was written after the write was refused");
}

/// Naming a keypair file for this directory settles the question the refusal
/// asks, so the same command goes through.
#[test]
fn setting_a_key_for_this_directory_unblocks_the_write() {
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();
    let local = work.join("local.conf.json");

    let (mut cmd, _link) = cli_with_global_config(&dir, &global_config_holding_the_phrase());
    cmd.current_dir(&work)
        .arg("--config")
        .arg(&local)
        .arg("config")
        .arg("--url")
        .arg("main")
        .arg("--keys")
        .arg("./msig.keys.json")
        .assert()
        .success();

    let stored = fs::read_to_string(&local).unwrap();
    assert!(!stored.contains(PHRASE), "the phrase reached the new file");
    assert!(stored.contains("./msig.keys.json"));
}

/// The command that inherited the secret still signs with it: what is refused
/// is writing a second copy, not using the one that exists.
#[test]
fn an_inherited_secret_is_still_used_by_the_command_that_needs_it() {
    let tvc = fs::canonicalize(TVC).unwrap();
    let abi = fs::canonicalize(ABI).unwrap();
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();

    let (mut cmd, _link) = cli_with_global_config(&dir, &global_config_holding_the_phrase());
    cmd.current_dir(&work)
        .arg("--config")
        .arg(work.join("local.conf.json"))
        .arg("deploy_message")
        .arg(&tvc)
        .arg("{}")
        .arg("--abi")
        .arg(&abi)
        .assert()
        .stdout(predicate::str::contains("keys: <seed phrase>"))
        .stdout(predicate::str::contains(PHRASE).not());
}

/// Only a secret stops the write. A keypair path is what the field is for, in
/// the config and in an alias alike, and draws no warning either.
#[test]
fn an_inherited_alias_path_does_not_stop_the_write() {
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();
    let local = work.join("local.conf.json");

    let (mut cmd, _link) = cli_with_global_config(
        &dir,
        r#"{"config": {"retries": 1}, "aliases": {"msig": {"key_path": "./msig.keys.json"}}}"#,
    );
    cmd.current_dir(&work)
        .arg("--config")
        .arg(&local)
        .arg("config")
        .arg("--url")
        .arg("main")
        .assert()
        .success()
        .stderr(predicate::str::contains("Warning").not());

    assert!(fs::read_to_string(&local).unwrap().contains("./msig.keys.json"));
}

/// `--global` discards the config loaded at startup and reads the global one,
/// which is the file whose owner is least likely to look inside it. The advice
/// has to name that file: `config --keys` writes the local one and would leave
/// the phrase where it is, warning for ever.
#[test]
fn the_warning_for_the_global_config_points_at_the_global_config() {
    let dir = testdir!();
    let local = dir.join("local.conf.json");
    fs::write(&local, r#"{"retries": 1}"#).unwrap();

    let (mut cmd, _link) = cli_with_global_config(&dir, &global_config_holding_the_phrase());
    cmd.arg("--config")
        .arg(&local)
        .arg("config")
        .arg("--global")
        .arg("--list")
        .assert()
        .success()
        .stderr(predicate::str::contains("--global --keys"))
        .stderr(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains(PHRASE).not());
}

/// Whitespace around a key does not stop it being the key. Every one of these
/// used to be stored in full while the printed config claimed it had been
/// masked -- three mitigations missing the same value at once.
#[test]
fn config_keys_refuses_a_secret_key_padded_with_whitespace() {
    for padded in [format!("{SECRET_KEY} "), format!(" {SECRET_KEY}"), format!("\t{SECRET_KEY}\n")]
    {
        let config = empty_config();
        tvm_cli(&config)
            .arg("config")
            .arg("--keys")
            .arg(&padded)
            .assert()
            .failure()
            .stdout(predicate::str::contains("would store a secret key"));
        assert_no_secret_on_disk(&config);
    }
}

#[test]
fn config_keys_refuses_a_0x_prefixed_secret_key() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("config")
        .arg("--keys")
        .arg(format!("0x{SECRET_KEY}"))
        .assert()
        .failure()
        .stdout(predicate::str::contains(SECRET_KEY).not())
        .stdout(predicate::str::contains("would store a secret key"));
    assert_no_secret_on_disk(&config);
}

/// A phrase with one mistyped word is still nearly the whole wallet: the BIP-39
/// checksum narrows the missing word to a trivial search.
#[test]
fn config_keys_refuses_a_phrase_with_a_mistyped_word() {
    let config = empty_config();
    let typo = PHRASE.replace("enlist", "enl1st");
    tvm_cli(&config)
        .arg("config")
        .arg("--keys")
        .arg(&typo)
        .assert()
        .failure()
        .stdout(predicate::str::contains("would store a seed phrase"));
    assert_no_secret_on_disk(&config);
}

/// `deployx` is the other command that saves the signing key next to an alias,
/// and it reaches `add_alias` only after the deploy has been broadcast.
#[test]
fn deployx_refuses_to_store_a_secret_in_an_alias_before_deploying() {
    let config = config_with(&format!(r#"{{"retries": 1, "keys_path": "{PHRASE}"}}"#));
    tvm_cli(&config)
        .arg("deployx")
        .arg(TVC)
        .arg("--abi")
        .arg(ABI)
        .arg("--alias")
        .arg("msig")
        .assert()
        .failure()
        .stdout(predicate::str::contains(PHRASE).not())
        .stdout(predicate::str::contains("would store a seed phrase"))
        .stdout(predicate::str::contains("Deploying...").not());
}

/// Only a secret is dropped from an inherited config. A keypair path is what
/// the field is for, and a global config naming one has to keep working.
#[test]
fn a_legitimate_path_in_the_global_config_is_inherited_and_written() {
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();
    let local = work.join("local.conf.json");

    let (mut cmd, _link) = cli_with_global_config(
        &dir,
        r#"{"config": {"retries": 1, "keys_path": "./msig.keys.json"}}"#,
    );
    cmd.current_dir(&work)
        .arg("--config")
        .arg(&local)
        .arg("config")
        .arg("--url")
        .arg("main")
        .assert()
        .success();

    assert!(fs::read_to_string(&local).unwrap().contains("./msig.keys.json"));
}

/// The refusal accepts a path with a space in it, so the tool has to be able to
/// sign with one. `load_keypair` used to tell a path from a phrase by looking
/// for an ASCII space in it, so such a path was read as a mnemonic.
#[test]
fn a_keypair_path_with_a_space_can_actually_be_used() {
    let tvc = fs::canonicalize(TVC).unwrap();
    let abi = fs::canonicalize(ABI).unwrap();
    let dir = testdir!();
    let keys = "./My Wallets/msig.keys.json";

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .current_dir(&dir)
        .arg("getkeypair")
        .arg("--output")
        .arg(keys)
        .arg("--phrase")
        .arg(PHRASE)
        .assert()
        .success();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .current_dir(&dir)
        .arg("genaddr")
        .arg(&tvc)
        .arg("--abi")
        .arg(&abi)
        .arg("--setkey")
        .arg(keys)
        .assert()
        // The repository carries no valid contract image, so the run cannot
        // finish; reaching address generation is what shows the keypair file
        // was read rather than parsed as a mnemonic.
        .stdout(predicate::str::contains("Invalid bip39 phrase").not())
        .stdout(predicate::str::contains("cannot generate address"));
}

/// Every config field has a default and unknown keys are ignored, so any JSON
/// object parses into an all-default config -- which the next write puts over
/// the file. Pointing `--config` at a keypair file by mistake destroyed it.
#[test]
fn a_file_that_is_not_a_config_is_an_error_rather_than_a_fresh_start() {
    let dir = testdir!();
    let keypair = dir.join("msig.keys.json");
    let contents = format!(r#"{{"public": "04ad311d", "secret": "{SECRET_KEY}"}}"#);
    fs::write(&keypair, &contents).unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("--config")
        .arg(&keypair)
        .arg("config")
        .arg("--url")
        .arg("main")
        .assert()
        .failure()
        .stdout(predicate::str::contains("msig.keys.json"));

    assert_eq!(fs::read_to_string(&keypair).unwrap(), contents, "the keypair file was overwritten");
}

/// A field of the wrong type fails the config parse, and what is left ignores
/// every key in the file. The url and the wallet used to disappear in silence.
#[test]
fn a_config_whose_fields_do_not_fit_is_an_error_rather_than_a_fresh_start() {
    let dir = testdir!();
    let local = dir.join("typed.conf.json");
    let contents = r#"{"url": "http://mynode.example", "retries": "5", "wallet": "0:mywallet"}"#;
    fs::write(&local, contents).unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("--config")
        .arg(&local)
        .arg("config")
        .arg("--url")
        .arg("main")
        .assert()
        .failure()
        .stdout(predicate::str::contains("typed.conf.json"));

    assert_eq!(fs::read_to_string(&local).unwrap(), contents, "the config was overwritten");
}

/// A config that cannot be used stops every command, including the `getkeypair`
/// the remediation asks for, so the error has to say how to step around itself.
#[test]
fn the_error_for_an_unusable_config_offers_a_way_past_it() {
    let dir = testdir!();
    let local = dir.join("broken.conf.json");
    fs::write(&local, r#"{"config": {"retries": 7,}}"#).unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("--config")
        .arg(&local)
        .arg("getkeypair")
        .arg("--output")
        .arg(dir.join("out.json"))
        .arg("--phrase")
        .arg(PHRASE)
        .assert()
        .failure()
        .stdout(predicate::str::contains("--config"));
}

/// A file name with a space in it and no directory reads as several words. The
/// refusal cannot tell it from a phrase, so the message names the way through
/// instead of advising a conversion its owner cannot perform.
#[test]
fn refusing_a_bare_name_with_a_space_points_at_the_path_form() {
    let config = empty_config();
    tvm_cli(&config)
        .arg("config")
        .arg("--keys")
        .arg("my keys.json")
        .assert()
        .failure()
        .stdout(predicate::str::contains("./"));
}

/// `nodeid --keypair` reads the file through `load_keypair`, which used to
/// tell a path from a phrase by looking for an ASCII space in it, so such a
/// path was taken for a mnemonic. `genaddr` decides that for itself and does
/// not cover this.
#[test]
fn a_keypair_path_with_a_space_can_be_loaded() {
    let dir = testdir!();
    let keys = "./My Wallets/msig.keys.json";

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .current_dir(&dir)
        .arg("getkeypair")
        .arg("--output")
        .arg(keys)
        .arg("--phrase")
        .arg(PHRASE)
        .assert()
        .success();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .current_dir(&dir)
        .arg("nodeid")
        .arg("--keypair")
        .arg(keys)
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "a5d313e88f72d33995b120fc4f4dc3507f84a9f7290d95626abd42c80acf271f",
        ));
}

/// The refusal recognises a key with an `0x` prefix or whitespace around it, so
/// the `getkeypair` it advises has to accept the same values -- otherwise the
/// remediation fails on exactly the input that provoked it.
#[test]
fn getkeypair_accepts_the_forms_the_refusal_recognises() {
    const PUBLIC: &str = "04ad311dadcbf7fe4bc20d62e0fbfa195ab5f099009b40045632b997daf4b3b1";
    let dir = testdir!();

    for (name, value) in
        [("prefixed", format!("0x{SECRET_KEY}")), ("padded", format!(" {SECRET_KEY}\n"))]
    {
        let out = dir.join(format!("{name}.keys.json"));
        Command::cargo_bin(BIN_NAME)
            .unwrap()
            .arg("getkeypair")
            .arg("--output")
            .arg(&out)
            .arg("--phrase")
            .arg(&value)
            .assert()
            .success();
        assert!(fs::read_to_string(&out).unwrap().contains(PUBLIC), "{name} produced other keys");
    }
}

/// The accepting side of the guard, where the false positives live. Every one
/// of these is a config the tool must read and write again: the shapes it
/// writes itself, the bare `Config` object older versions wrote, and a file
/// whose only field happens to equal the default -- which used to decide the
/// answer.
#[test]
fn a_config_of_any_shape_is_read_and_written() {
    let dir = testdir!();
    let cases = [
        ("empty", "{}"),
        ("default_valued_field", r#"{"is_json": false}"#),
        ("another_default", r#"{"no_answer": true}"#),
        ("float_default", r#"{"depool_fee": 0.5}"#),
        ("non_default_field", r#"{"retries": 5}"#),
        ("bare_config", r#"{"url": "http://example.invalid", "retries": 3}"#),
        ("credentials", r#"{"project_id": "abc", "access_key": "def"}"#),
        ("full_config", r#"{"config": {"retries": 3}, "aliases": {}}"#),
    ];

    for (name, contents) in cases {
        let path = dir.join(format!("{name}.conf.json"));
        fs::write(&path, contents).unwrap();
        Command::cargo_bin(BIN_NAME)
            .unwrap()
            .arg("--config")
            .arg(&path)
            .arg("config")
            .arg("--wc")
            .arg("0")
            .assert()
            .success()
            .stderr(predicate::str::contains("Warning").not());
        assert!(
            fs::read_to_string(&path).unwrap().contains("\"wc\""),
            "{name} was not written back as a config"
        );
    }
}

/// What a bare `Config` file said has to survive being written back, or the
/// guard would be passing files it then empties.
#[test]
fn the_settings_of_a_bare_config_file_survive_the_first_write() {
    let dir = testdir!();
    let path = dir.join("bare.conf.json");
    fs::write(&path, r#"{"url": "http://example.invalid", "retries": 3, "wallet": "0:abc"}"#)
        .unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("--config")
        .arg(&path)
        .arg("config")
        .arg("--wc")
        .arg("0")
        .assert()
        .success();

    let stored = fs::read_to_string(&path).unwrap();
    for kept in ["http://example.invalid", r#""retries": 3"#, "0:abc"] {
        assert!(stored.contains(kept), "{kept} was lost");
    }
}

/// The invariant: a file the tool cannot account for is never replaced. Naming
/// one config field used to be enough to have the rest of the file thrown away.
#[test]
fn a_file_the_tool_cannot_account_for_is_never_overwritten() {
    let dir = testdir!();
    let cases = [
        ("rpc", r#"{"jsonrpc": "2.0", "method": "eth_call", "id": 1}"#),
        ("tsconfig", r#"{"path": "./out", "include": ["src"]}"#),
        ("path_only", r#"{"path": "./out"}"#),
        ("mixed", r#"{"local_run": true, "foo": "bar"}"#),
        ("keypair", r#"{"public": "04ad311d", "secret": "c4415c03"}"#),
    ];

    for (name, contents) in cases {
        let path = dir.join(format!("{name}.json"));
        fs::write(&path, contents).unwrap();
        Command::cargo_bin(BIN_NAME)
            .unwrap()
            .arg("--config")
            .arg(&path)
            .arg("config")
            .arg("--url")
            .arg("main")
            .assert()
            .failure();
        assert_eq!(fs::read_to_string(&path).unwrap(), contents, "{name} was overwritten");
    }
}

/// The global config is read by the same reader as any other, or a file in the
/// bare `Config` shape is silently taken for an empty one -- and the key in it
/// disappears without a word, which is the whole failure this guard exists to
/// stop.
#[test]
fn a_global_config_in_the_older_shape_is_not_discarded() {
    let tvc = fs::canonicalize(TVC).unwrap();
    let abi = fs::canonicalize(ABI).unwrap();
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();
    let global = format!(r#"{{"retries": 9, "keys_path": "{PHRASE}"}}"#);

    let (mut cmd, _link) = cli_with_global_config(&dir, &global);
    cmd.current_dir(&work)
        .arg("--config")
        .arg(work.join("local.conf.json"))
        .arg("deploy_message")
        .arg(&tvc)
        .arg("{}")
        .arg("--abi")
        .arg(&abi)
        .assert()
        .stdout(predicate::str::contains("keys: <seed phrase>"));

    let (mut cmd, _link) = cli_with_global_config(&dir, &global);
    cmd.current_dir(&work)
        .arg("--config")
        .arg(work.join("local.conf.json"))
        .arg("config")
        .arg("--url")
        .arg("main")
        .assert()
        .failure();
}

/// `--keys` means "clear this" to `config clear`, so advising it would tell the
/// owner to throw the key away. The advice names the file that actually holds
/// the value instead.
#[test]
fn the_refusal_advises_only_what_the_owner_can_act_on() {
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();

    let (mut cmd, _link) = cli_with_global_config(&dir, &global_config_holding_the_phrase());
    cmd.current_dir(&work)
        .arg("--config")
        .arg(work.join("local.conf.json"))
        .arg("config")
        .arg("clear")
        .arg("--url")
        .assert()
        .failure()
        .stdout(predicate::str::contains("config --global --keys"))
        .stdout(predicate::str::contains("to this command").not());
}

#[test]
fn the_refusal_for_an_alias_names_the_alias_command() {
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();

    let (mut cmd, _link) = cli_with_global_config(
        &dir,
        &format!(
            r#"{{"config": {{"retries": 1}}, "aliases": {{"msig": {{"key_path": "{PHRASE}"}}}}}}"#
        ),
    );
    cmd.current_dir(&work)
        .arg("--config")
        .arg(work.join("local.conf.json"))
        .arg("config")
        .arg("--url")
        .arg("main")
        .assert()
        .failure()
        .stdout(predicate::str::contains("alias add msig --keys"));
}

/// The check before a deploy has to be the one the write will make, not a guess
/// at it: with `--keys` of its own the alias value is fine, and the write still
/// fails on the inherited key. Both commands that save an alias are asked.
#[test]
fn a_deploy_with_its_own_key_still_stops_before_a_write_that_would_fail() {
    let tvc = fs::canonicalize(TVC).unwrap();
    let abi = fs::canonicalize(ABI).unwrap();

    for command in ["deploy", "deployx"] {
        let dir = testdir!().join(command);
        let work = dir.join("work");
        fs::create_dir_all(&work).unwrap();

        let (mut cmd, _link) = cli_with_global_config(&dir, &global_config_holding_the_phrase());
        cmd.current_dir(&work)
            .arg("--config")
            .arg(work.join("local.conf.json"))
            .arg(command)
            .arg(&tvc);
        if command == "deploy" {
            cmd.arg("{}");
        }
        cmd.arg("--abi")
            .arg(&abi)
            .arg("--alias")
            .arg("msig")
            .arg("--keys")
            .arg("./k.json")
            .assert()
            .failure()
            .stdout(predicate::str::contains("Deploying...").not());
    }
}

/// `getkeypair` and `nodeid` accept a value with whitespace around it, so
/// `genaddr` has to as well.
#[test]
fn genaddr_accepts_a_phrase_with_whitespace_around_it() {
    let padded = format!("  {PHRASE}  ");
    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("genaddr")
        .arg(TVC)
        .arg("--abi")
        .arg(ABI)
        .arg("--setkey")
        .arg(&padded)
        .assert()
        .stdout(predicate::str::contains("Invalid bip39 phrase").not())
        .stdout(predicate::str::contains("cannot generate address"));
}

/// A file may name fields of both shapes at once, and only one of them is the
/// shape it is read as. Checking against the union of the two took the fields
/// of the shape that was *not* chosen for recognised, and serde had already
/// ignored them -- the url and the wallet here went missing without a word.
#[test]
fn a_config_of_a_mixed_shape_is_not_silently_emptied() {
    let dir = testdir!();
    let path = dir.join("mixed.conf.json");
    let contents = r#"{"url": "http://example.invalid", "wallet": "0:abc", "aliases": {}}"#;
    fs::write(&path, contents).unwrap();

    Command::cargo_bin(BIN_NAME)
        .unwrap()
        .arg("--config")
        .arg(&path)
        .arg("config")
        .arg("--wc")
        .arg("0")
        .assert()
        .failure();

    assert_eq!(fs::read_to_string(&path).unwrap(), contents, "the config was overwritten");
}

/// Settings live inside `config` and inside each alias, so that is where a
/// field this version does not know will turn up -- checking only the top level
/// guarded the one place new fields are never added.
#[test]
fn an_unknown_field_nested_in_the_config_is_not_dropped() {
    let dir = testdir!();
    let cases = [
        ("in_config", r#"{"config": {"retries": 3, "my_note": "keep me"}}"#),
        ("in_alias", r#"{"aliases": {"msig": {"address": "a::b", "extra": "keep me"}}}"#),
    ];

    for (name, contents) in cases {
        let path = dir.join(format!("{name}.conf.json"));
        fs::write(&path, contents).unwrap();
        Command::cargo_bin(BIN_NAME)
            .unwrap()
            .arg("--config")
            .arg(&path)
            .arg("config")
            .arg("--wc")
            .arg("0")
            .assert()
            .failure();
        assert_eq!(fs::read_to_string(&path).unwrap(), contents, "{name} was overwritten");
    }
}

/// The global config sits next to the executable, so on a shared install one
/// unusable file there would take the tool down for everyone. It is a fallback:
/// a command that does not depend on it says so and carries on.
#[test]
fn an_unusable_global_config_does_not_stop_a_command_that_can_do_without_it() {
    let dir = testdir!();
    let (mut cmd, _link) = cli_with_global_config(&dir, r#"{"public": "aa", "secret": "bb"}"#);
    cmd.current_dir(&dir)
        .arg("genphrase")
        .assert()
        .success()
        .stderr(predicate::str::contains("Warning"));
}

/// Asked about the global config itself, though, the tool must not answer with
/// defaults -- the next write would put them over the file.
#[test]
fn the_global_config_is_not_replaced_when_it_holds_something_unknown() {
    let dir = testdir!();
    let global = r#"{"config": {"retries": 3}, "my_note": "keep me"}"#;
    let (mut cmd, _link) = cli_with_global_config(&dir, global);
    cmd.current_dir(&dir)
        .arg("config")
        .arg("--global")
        .arg("--url")
        .arg("main")
        .assert()
        .failure()
        // `--config` is no escape for the global config: the message has to
        // name what its owner can actually do.
        .stdout(predicate::str::contains("--config").not());

    assert_eq!(
        fs::read_to_string(dir.join(".tvm-cli.global.conf.json")).unwrap(),
        global,
        "the global config was overwritten"
    );
}

/// The command the refusal prints must not cost more than it fixes: setting the
/// key of an alias used to replace the whole entry, taking the address and the
/// ABI with it.
#[test]
fn setting_the_key_of_an_alias_keeps_its_address_and_abi() {
    let config = config_with(
        r#"{"config": {"retries": 1}, "aliases": {"msig": {"address": "abc::def", "abi_path": "./msig.abi.json", "key_path": "./old.keys.json"}}}"#,
    );

    tvm_cli(&config)
        .arg("config")
        .arg("alias")
        .arg("add")
        .arg("msig")
        .arg("--keys")
        .arg("./new.keys.json")
        .assert()
        .success();

    let stored = fs::read_to_string(&config).unwrap();
    for kept in ["abc::def", "./msig.abi.json", "./new.keys.json"] {
        assert!(stored.contains(kept), "{kept} was lost");
    }
}

/// `--keys` on `config clear` means "remove what is stored", like every other
/// option of that command. Taking a value made it look like the `--keys <file>`
/// of `config` itself, which sets one -- and the value was thrown away.
#[test]
fn config_clear_keys_does_not_pretend_to_take_a_path() {
    let config = config_with(r#"{"retries": 1, "keys_path": "./old.keys.json"}"#);
    tvm_cli(&config)
        .arg("config")
        .arg("clear")
        .arg("--keys")
        .arg("./new.keys.json")
        .assert()
        .failure();
    assert!(fs::read_to_string(&config).unwrap().contains("./old.keys.json"));

    let config = config_with(r#"{"retries": 1, "keys_path": "./old.keys.json"}"#);
    tvm_cli(&config).arg("config").arg("clear").arg("--keys").assert().success();
    assert!(fs::read_to_string(&config).unwrap().contains(r#""keys_path": null"#));
}

/// The whole point of the warning is that following it ends the warning. Run
/// the command it prints and see the phrase gone and the config quiet.
#[test]
fn the_remediation_the_warning_advises_finishes_the_job() {
    let dir = testdir!();
    let keys = dir.join("msig.keys.json");

    let (mut cmd, _link) = cli_with_global_config(&dir, &global_config_holding_the_phrase());
    cmd.current_dir(&dir)
        .arg("getkeypair")
        .arg("--output")
        .arg(&keys)
        .arg("--phrase")
        .arg(PHRASE)
        .assert()
        .success();

    let (mut cmd, _link) = cli_with_global_config(&dir, &global_config_holding_the_phrase());
    cmd.current_dir(&dir).arg("config").arg("--global").arg("--keys").arg(&keys).assert().success();

    let global = fs::read_to_string(dir.join(".tvm-cli.global.conf.json")).unwrap();
    assert!(!global.contains(PHRASE), "the phrase is still in the global config");

    Command::new(dir.join(BIN_NAME))
        .arg("config")
        .arg("--global")
        .arg("--list")
        .assert()
        .success()
        .stderr(predicate::str::contains("Warning").not());
}

/// What the global config says and what a directory inherits from it are two
/// different things when the file mixes the two shapes: serde drops the fields
/// of the shape it was not read as. Refusing the local write would punish this
/// directory for a file it does not own, so the run says what it could not
/// read and goes on.
#[test]
fn an_inherited_config_of_a_mixed_shape_says_what_it_could_not_read() {
    let dir = testdir!();
    let work = dir.join("work");
    fs::create_dir_all(&work).unwrap();

    let (mut cmd, _link) = cli_with_global_config(
        &dir,
        r#"{"url": "http://example.invalid", "wallet": "0:abc", "aliases": {}}"#,
    );
    cmd.current_dir(&work)
        .arg("--config")
        .arg(work.join("local.conf.json"))
        .arg("config")
        .arg("--wc")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("url"))
        .stderr(predicate::str::contains("wallet"));
}

/// A value the config file cannot hold must not be storable: json has no
/// infinity, so it would be written as `null` and the config would not parse
/// again -- the tool locking its own file with a valid command.
#[test]
fn config_refuses_a_depool_fee_that_json_cannot_hold() {
    for value in ["inf", "-inf", "NaN"] {
        let config = config_with(r#"{"retries": 1}"#);
        tvm_cli(&config).arg("config").arg("--depool_fee").arg(value).assert().failure();

        let stored = fs::read_to_string(&config).unwrap();
        assert!(!stored.contains("null"), "{value} was stored as null");

        tvm_cli(&config).arg("config").arg("--list").assert().success();
    }
}
