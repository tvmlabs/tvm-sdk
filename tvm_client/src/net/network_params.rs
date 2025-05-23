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
//

use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use tvm_block::GlobalCapabilities;
use tvm_executor::BlockchainConfig;

use super::acki_config;
use crate::client::ClientContext;
use crate::client::NetworkParams;
use crate::error::ClientResult;

const DEFAULT_ACKI_GLOBAL_ID: i32 = 100;

#[derive(Serialize, Deserialize, ApiType, Default, Clone)]
pub struct ResultOfGetSignatureId {
    /// Signature ID for configured network if it should be used in messages
    /// signature
    pub signature_id: Option<i32>,
}

/// Returns signature ID for configured network if it should be used in messages
/// signature
#[api_function]
pub async fn get_signature_id(
    context: std::sync::Arc<ClientContext>,
) -> ClientResult<ResultOfGetSignatureId> {
    let params = get_default_params(&context).await?;
    if params.blockchain_config.has_capability(GlobalCapabilities::CapSignatureWithId) {
        Ok(ResultOfGetSignatureId { signature_id: Some(params.global_id) })
    } else {
        Ok(ResultOfGetSignatureId { signature_id: None })
    }
}

pub(crate) async fn get_default_params(
    _context: &Arc<ClientContext>,
) -> ClientResult<NetworkParams> {
    ackinacki_network().map(|(blockchain_config, global_id)| NetworkParams {
        blockchain_config: Arc::new(blockchain_config),
        global_id,
    })
}

/// TODO: make it more generic
pub(crate) fn ackinacki_network() -> ClientResult<(BlockchainConfig, i32)> {
    let global_id = DEFAULT_ACKI_GLOBAL_ID;
    let config = blockchain_config_from_json(&acki_config::get_config()?)?;
    Ok((config, global_id))
}

fn blockchain_config_from_json(json: &str) -> ClientResult<BlockchainConfig> {
    let map = serde_json::from_str::<serde_json::Map<String, Value>>(json)
        .map_err(crate::tvm::Error::json_deserialization_failed)?;
    let config_params =
        tvm_block_json::parse_config(&map).map_err(crate::tvm::Error::can_not_parse_config)?;
    BlockchainConfig::with_config(config_params).map_err(crate::tvm::Error::can_not_convert_config)
}
