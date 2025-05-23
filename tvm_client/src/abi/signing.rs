use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use tvm_types::base64_encode;

use crate::ClientContext;
use crate::crypto::KeyPair;
use crate::crypto::SigningBoxHandle;
use crate::error::ClientResult;

#[derive(Serialize, Deserialize, Clone, Debug, ApiType, PartialEq)]
#[serde(tag = "type")]
#[derive(Default)]
pub enum Signer {
    /// No keys are provided. Creates an unsigned message.
    #[default]
    None,
    /// Only public key is provided in unprefixed hex string format to generate
    /// unsigned message and `data_to_sign` which can be signed later.
    External { public_key: String },
    /// Key pair is provided for signing
    Keys { keys: KeyPair },
    /// Signing Box interface is provided for signing, allows Dapps to sign
    /// messages using external APIs, such as HSM, cold wallet, etc.
    SigningBox { handle: SigningBoxHandle },
}

impl Signer {
    pub async fn sign(
        &self,
        context: Arc<ClientContext>,
        data_to_sign: &[u8],
    ) -> ClientResult<Option<Vec<u8>>> {
        match self {
            Signer::None => Ok(None),
            Signer::Keys { keys } => {
                crate::crypto::internal::sign_using_keys(data_to_sign, &keys.decode()?)
                    .map(|(_, sign)| Some(sign))
            }
            Signer::External { .. } => Ok(None),
            Signer::SigningBox { handle } => {
                let result = crate::crypto::signing_box_sign(
                    context,
                    crate::crypto::ParamsOfSigningBoxSign {
                        signing_box: handle.clone(),
                        unsigned: base64_encode(data_to_sign),
                    },
                )
                .await?;

                Some(crate::encoding::hex_decode(&result.signature)).transpose()
            }
        }
    }

    pub async fn resolve_public_key(
        &self,
        context: Arc<ClientContext>,
    ) -> ClientResult<Option<String>> {
        match self {
            Signer::None => Ok(None),
            Signer::Keys { keys } => Ok(Some(keys.public.clone())),
            Signer::External { public_key } => Ok(Some(public_key.clone())),
            Signer::SigningBox { handle } => crate::crypto::signing_box_get_public_key(
                context,
                crate::crypto::RegisteredSigningBox { handle: handle.clone() },
            )
            .await
            .map(|result| Some(result.pubkey)),
        }
    }
}
