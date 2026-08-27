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
    /// Dapp ID as a 64-character hex string (no 0x). Required.
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
    let base = server_link.state().get_rest_api_endpoint().await;
    let account_url = |query: String| {
        let mut url = base.clone();
        url.set_path(&format!("{API_VERSION}/account"));
        url.set_query(Some(&query));
        url
    };

    validate_hex_id("dapp_id", &params.dapp_id)?;
    let value = server_link
        .http_get(account_url(format!(
            "account_id={}&dapp_id={}",
            params.account_id, params.dapp_id
        )))
        .await?;
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

    // Defensive: a v3 server is expected to echo `account_id` and `dapp_id`.
    // If it does not, fill from `params` rather than returning empty strings
    // inside a successful result. Covered by
    // `get_account_fills_ids_when_server_omits_them`.
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
