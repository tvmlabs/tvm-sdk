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
    let base = server_link.state().get_rest_api_endpoint().await;
    let account_url = |query: String| {
        let mut url = base.clone();
        url.set_path(&format!("{API_VERSION}/account"));
        url.set_query(Some(&query));
        url
    };

    // Primary signal: the GraphQL `info.version` probe. It is best-effort and
    // fail-fast (single attempt, no reconnect retry loop) — if GraphQL is
    // unavailable we do NOT hang or fail; we fall back to the legacy v2 REST
    // form and, if the server rejects it, retry the v3 form. This keeps working
    // both on legacy v2 nodes and on REST-only v3 nodes that expose no GraphQL.
    // (Transitional: drop the v2->v3 fallback once all nodes serve v3.)
    let graphql_ok = server_link.state().try_resolve_query_endpoint().await.is_ok();
    let graphql_says_v3 = graphql_ok && server_link.supports_dapp_id().await;

    if !graphql_says_v3 {
        // GraphQL reported a v2 (< 1.0.0) server, or GraphQL is unavailable.
        match server_link.http_get(account_url(format!("address=0:{}", params.account_id))).await {
            Ok(value) => return parse_get_account_response(value, &params),
            Err(v2_err) => {
                // An authoritative v2 verdict is trusted: the error is genuine.
                if graphql_ok {
                    return Err(v2_err);
                }
                // GraphQL was unavailable — the node may actually be v3, so
                // fall through and retry with the v3 form
                // below.
            }
        }
    }

    // v3 form: GraphQL says v3, or GraphQL was unavailable and v2 was rejected.
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
