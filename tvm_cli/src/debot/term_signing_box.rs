use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::io::{self};

use tvm_client::crypto::KeyPair;
use tvm_client::crypto::RegisteredSigningBox;
use tvm_client::crypto::SigningBoxHandle;
use tvm_client::crypto::get_signing_box;
use tvm_client::crypto::remove_signing_box;

use super::term_browser::input;
use crate::crypto::load_keypair;
use crate::helpers::TonClient;
use crate::helpers::read_keys;

pub(super) struct TerminalSigningBox {
    handle: SigningBoxHandle,
    client: TonClient,
}

impl TerminalSigningBox {
    pub async fn new<R: Read>(
        client: TonClient,
        possible_keys: Vec<String>,
        reader: Option<BufReader<R>>,
    ) -> Result<Self, String> {
        let keys = {
            if let Some(mut reader) = reader {
                let mut writer = io::stdout();
                input_keys(None, possible_keys, &mut reader, &mut writer, 3)?
            } else {
                let stdio = std::io::stdin();
                let mut reader = stdio.lock();
                let mut writer = io::stdout();
                input_keys(None, possible_keys, &mut reader, &mut writer, 3)?
            }
        };
        let handle = get_signing_box(client.clone(), keys)
            .await
            .map(|r| r.handle)
            .map_err(|e| e.to_string())?;

        Ok(Self { handle, client })
    }

    pub async fn new_with_keypath(client: TonClient, keys_path: String) -> Result<Self, String> {
        let keys = read_keys(&keys_path).unwrap_or_default();
        let handle = get_signing_box(client.clone(), keys)
            .await
            .map(|r| r.handle)
            .map_err(|e| e.to_string())?;

        Ok(Self { handle, client })
    }

    pub fn handle(&self) -> SigningBoxHandle {
        self.handle.clone()
    }

    pub fn leak(&mut self) -> SigningBoxHandle {
        let handle = self.handle.clone();
        self.handle = SigningBoxHandle(0);
        handle
    }
}

impl Drop for TerminalSigningBox {
    fn drop(&mut self) {
        if self.handle.0 != 0 {
            let _ = remove_signing_box(
                self.client.clone(),
                RegisteredSigningBox { handle: self.handle.clone() },
            );
        }
    }
}

pub(super) fn input_keys<R, W>(
    prompt: Option<&str>,
    possible_keys: Vec<String>,
    reader: &mut R,
    writer: &mut W,
    tries: u8,
) -> Result<KeyPair, String>
where
    R: BufRead,
    W: Write,
{
    let enter_str = prompt.unwrap_or_default();
    let mut pair = Err("no keypair".to_string());
    let mut format_pubkeys = String::new();
    possible_keys.iter().for_each(|x| format_pubkeys += &format!(" {},", x));
    for _ in 0..tries {
        let value = input(enter_str, reader, writer);
        pair = load_keypair(&value).map_err(|e| {
            println!("Invalid keys: {}. Try again.", e);
            e
        });
        if let Ok(ref keys) = pair {
            if !possible_keys.is_empty() {
                if !possible_keys.iter().any(|x| x.get(2..).unwrap() == keys.public.as_str()) {
                    println!("Unexpected keys.");
                    println!(
                        "Hint: enter keypair which contains one of the following public keys: {}",
                        format_pubkeys
                    );
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    pair
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use testdir::testdir;

    use super::*;

    const PUBLIC: &str = "1a45739a421eada273512fbbd6dc9dd813f1eb8f06260c3102286827a1e3c267";
    const PRIVATE: &str = "0900aa3a7e5c37fe7ee8b85c34f11353537f6f2ff60cce88e17fac65ad8725b9";
    const SEED: &str =
        "episode polar pistol excite essence van cover fox visual gown yellow minute";

    fn create_keypair_file(name: &str) {
        let mut file = File::create(name).unwrap();
        file.write_all(
            format!(
                r#"{{
            "public": "{}",
            "secret": "{}"
        }}"#,
                PUBLIC, PRIVATE
            )
            .as_bytes(),
        )
        .unwrap();
    }

    #[test]
    fn load_key_from_file() {
        let keys_file = testdir!().join("keys.json");
        let keys_file = keys_file.to_str().unwrap();
        create_keypair_file(keys_file);

        let mut in_data = keys_file.as_bytes();
        let mut out_data = vec![];
        let keys = input_keys(None, vec![], &mut in_data, &mut out_data, 1).unwrap();
        assert_eq!(keys.public, PUBLIC);
        assert_eq!(keys.secret, PRIVATE);
    }

    #[test]
    fn load_key_from_seed() {
        let mut in_data = SEED.as_bytes();
        let mut out_data = vec![];

        let keys = input_keys(None, vec![], &mut in_data, &mut out_data, 1).unwrap();
        assert_eq!(keys.public, PUBLIC);
        assert_eq!(keys.secret, PRIVATE);
    }
}
