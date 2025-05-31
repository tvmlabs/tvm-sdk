use std::sync::Arc;

use chacha20::Key;
use chacha20::Nonce;
use chacha20::cipher::NewStreamCipher;
use chacha20::cipher::SyncStreamCipher;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use tvm_types::base64_encode;
use zeroize::Zeroize;

use crate::ClientContext;
use crate::crypto::EncryptionBox;
use crate::crypto::EncryptionBoxInfo;
use crate::crypto::Error;
use crate::crypto::internal::SecretBuf;
use crate::crypto::internal::hex_decode_secret;
use crate::encoding::base64_decode;
use crate::encoding::hex_decode;
use crate::error::ClientResult;

#[derive(
    Serialize, Deserialize, Clone, Debug, ApiType, Default, PartialEq, Zeroize, ZeroizeOnDrop,
)]
pub struct ChaCha20ParamsEB {
    /// 256-bit key. Must be encoded with `hex`.
    pub key: String,
    /// 96-bit nonce. Must be encoded with `hex`.
    pub nonce: String,
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct ChaCha20EncryptionBox {
    key: Key,
    nonce: Nonce,
    hdpath: Option<String>,
}

impl ChaCha20EncryptionBox {
    pub fn new(params: ChaCha20ParamsEB, hdpath: Option<String>) -> ClientResult<Self> {
        let key = Key::clone_from_slice(&hex_decode_secret(&params.key)?);
        let nonce = Nonce::clone_from_slice(&hex_decode(&params.nonce)?);
        if key.len() != 32 {
            return Err(Error::invalid_key_size(key.len(), &[32]));
        }
        if nonce.len() != 12 {
            return Err(Error::invalid_nonce_size(nonce.len(), &[12]));
        }

        Ok(Self { key, nonce, hdpath })
    }

    fn chacha20(&self, data: &str) -> ClientResult<String> {
        let mut cipher = chacha20::ChaCha20::new(&self.key, &self.nonce);
        let mut data = SecretBuf(base64_decode(data)?);
        cipher.apply_keystream(&mut data.0);

        Ok(base64_encode(&data.0))
    }
}

#[async_trait::async_trait]
impl EncryptionBox for ChaCha20EncryptionBox {
    async fn get_info(&self, _context: Arc<ClientContext>) -> ClientResult<EncryptionBoxInfo> {
        Ok(EncryptionBoxInfo {
            algorithm: Some("ChaCha20".to_owned()),
            hdpath: self.hdpath.clone(),
            public: None,
            options: Some(json!({ "nonce": hex::encode(self.nonce) })),
        })
    }

    async fn encrypt(&self, _context: Arc<ClientContext>, data: &String) -> ClientResult<String> {
        self.chacha20(data)
    }

    async fn decrypt(&self, _context: Arc<ClientContext>, data: &String) -> ClientResult<String> {
        self.chacha20(data)
    }
}
