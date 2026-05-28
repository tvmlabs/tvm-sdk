use std::sync::Arc;

use serde::Deserialize;
use serde_json::Value;

use crate::ClientContext;
use crate::error::ClientError;
use crate::error::ClientResult;

mod validate;
pub use validate::validate_hex_id;

#[cfg(test)]
mod tests;

const API_VERSION: &str = "v2";

#[derive(Serialize, Deserialize, ApiType, Default, Clone, Debug)]
pub struct ParamsOfGetAccount {
    /// Account ID as a 64-character hex string (no 0x, no workchain).
    pub account_id: String,
    /// Dapp ID as a 64-character hex string (no 0x).
    /// Required when the server supports the v3 API (info.version >= "1.0.0").
    pub dapp_id: String,
}

#[derive(Serialize, Deserialize, ApiType, Default, Clone, Debug)]
pub struct ResultOfGetAccount {
    pub boc: String,
    pub dapp_id: String,
    pub state_timestamp: Option<u64>,
    pub account_id: String,
}

#[derive(Deserialize)]
struct RawAccountResponse {
    boc: String,
    #[serde(default)]
    dapp_id: Option<String>,
    #[serde(default)]
    state_timestamp: Option<u64>,
    #[serde(default)]
    account_id: Option<String>,
}

#[api_function]
pub async fn get_account(
    context: Arc<ClientContext>,
    params: ParamsOfGetAccount,
) -> ClientResult<ResultOfGetAccount> {
    validate_hex_id("account_id", &params.account_id)?;

    let server_link = context.get_server_link()?;
    let is_v3 = server_link.supports_dapp_id().await;

    // dapp_id is only required (and sent) in v3 mode.
    if is_v3 {
        validate_hex_id("dapp_id", &params.dapp_id)?;
    }

    let mut url = server_link.state().get_rest_api_endpoint().await;
    url.set_path(&format!("{API_VERSION}/account"));
    if is_v3 {
        url.set_query(Some(&format!("address={}&dapp_id={}", params.account_id, params.dapp_id,)));
    } else {
        url.set_query(Some(&format!("address=0:{}", params.account_id)));
    }

    let value = server_link.http_get(url).await?;
    parse_get_account_response(value, &params)
}

fn parse_get_account_response(
    value: Value,
    params: &ParamsOfGetAccount,
) -> ClientResult<ResultOfGetAccount> {
    let raw: RawAccountResponse = serde_json::from_value(value).map_err(|_| {
        ClientError::with_code_message(
            crate::net::ErrorCode::InvalidServerResponse as u32,
            "Server response can not be parsed".to_string(),
        )
    })?;

    Ok(ResultOfGetAccount {
        boc: raw.boc,
        state_timestamp: raw.state_timestamp,
        account_id: raw
            .account_id
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| params.account_id.clone()),
        dapp_id: raw.dapp_id.filter(|s| !s.is_empty()).unwrap_or_else(|| params.dapp_id.clone()),
    })
}
