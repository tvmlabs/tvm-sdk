use std::sync::Arc;

use crate::ClientContext;
use crate::error::ClientError;
use crate::error::ClientResult;

#[derive(Serialize, Deserialize, ApiType, Default, Clone)]
pub struct ParamsOfGetAccount {
    pub address: String,
}

#[derive(Serialize, Deserialize, ApiType, Default, Clone)]
pub struct ResultOfGetAccount {
    pub boc: String,
    pub dapp_id: Option<String>,
}

#[cfg(test)]
mod tests;

const API_VERSION: &str = "v2";

#[api_function]
pub async fn get_account(
    context: Arc<ClientContext>,
    params: ParamsOfGetAccount,
) -> ClientResult<ResultOfGetAccount> {
    let server_link = context.get_server_link()?;
    let mut url = server_link.state().get_rest_api_endpoint().await;

    url.set_path(&format!("{API_VERSION}/account"));
    url.set_query(Some(&format!("address={}", params.address)));

    let value = server_link.http_get(url).await?;

    serde_json::from_value::<ResultOfGetAccount>(value).map_err(|_| {
        ClientError::with_code_message(
            crate::net::ErrorCode::InvalidServerResponse as u32,
            "Server response can not be parsed".to_string(),
        )
    })
}
