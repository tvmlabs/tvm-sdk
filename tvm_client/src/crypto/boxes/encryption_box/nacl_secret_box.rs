use std::sync::Arc;

use zeroize::Zeroize;

use crate::crypto::nacl_secret_box;
use crate::crypto::nacl_secret_box_open;
use crate::crypto::EncryptionBox;
use crate::crypto::EncryptionBoxInfo;
use crate::crypto::ParamsOfNaclSecretBox;
use crate::crypto::ParamsOfNaclSecretBoxOpen;
use crate::error::ClientResult;
use crate::ClientContext;

#[derive(
    Serialize, Deserialize, Clone, Debug, ApiType, Default, PartialEq, Zeroize, ZeroizeOnDrop,
)]
pub struct NaclSecretBoxParamsEB {
    /// Secret key - unprefixed 0-padded to 64 symbols hex string
    pub key: String,
    /// Nonce in `hex`
    pub nonce: String,
}

#[derive(Debug)]
pub struct NaclSecretEncryptionBox {
    params: NaclSecretBoxParamsEB,
    hdpath: Option<String>,
}

impl NaclSecretEncryptionBox {
    pub fn new(params: NaclSecretBoxParamsEB, hdpath: Option<String>) -> Self {
        Self { params, hdpath }
    }
}

#[async_trait::async_trait]
impl EncryptionBox for NaclSecretEncryptionBox {
    async fn get_info(&self, _context: Arc<ClientContext>) -> ClientResult<EncryptionBoxInfo> {
        Ok(EncryptionBoxInfo {
            algorithm: Some("NaclSecretBox".to_owned()),
            hdpath: self.hdpath.clone(),
            public: None,
            options: Some(json!({ "nonce": &self.params.nonce })),
        })
    }

    async fn encrypt(&self, context: Arc<ClientContext>, data: &String) -> ClientResult<String> {
        nacl_secret_box(
            context,
            ParamsOfNaclSecretBox {
                decrypted: data.clone(),
                nonce: self.params.nonce.clone(),
                key: self.params.key.clone(),
            },
        )
        .map(|result| result.encrypted)
    }

    async fn decrypt(&self, context: Arc<ClientContext>, data: &String) -> ClientResult<String> {
        nacl_secret_box_open(
            context,
            ParamsOfNaclSecretBoxOpen {
                encrypted: data.clone(),
                nonce: self.params.nonce.clone(),
                key: self.params.key.to_string(),
            },
        )
        .map(|result| result.decrypted)
    }
}
