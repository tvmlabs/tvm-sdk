use std::sync::Arc;

use httpmock::prelude::*;

use crate::ClientConfig;
use crate::ClientContext;
use crate::account;
use crate::account::ParamsOfGetAccount;
use crate::account::ResultOfGetAccount;
use crate::error::ClientResult;
use crate::net::NetworkConfig;

#[tokio::test]
#[ignore = "This test stopped working when the SDK started using a non-configurable port for the REST API. TODO: use another mocking librarary"]
async fn test_get_account() -> ClientResult<()> {
    let server = MockServer::start();

    let contract_addr = "0:11111";
    let contract_boc = "te6ccAAS";
    let not_found_addr = "0:55555";
    let error_addr = "0:66666";

    let mock_200 = server.mock(|when, then| {
        when.method(GET)
            .path("/v2/account")
            .query_param("address", contract_addr)
            .header("Authorization", "Bearer secret");
        then.status(200).header("content-type", "application/json").body(
            serde_json::to_string(&ResultOfGetAccount { boc: contract_boc.to_string() }).unwrap(),
        );
    });

    let mock_404 = server.mock(|when, then| {
        when.method(GET)
            .path("/v2/account")
            .query_param("address", not_found_addr)
            .header("Authorization", "Bearer secret");
        then.status(404).body("not found".to_string());
    });
    let mock_500 = server.mock(|when, then| {
        when.method(GET)
            .path("/v2/account")
            .query_param("address", error_addr)
            .header("Authorization", "Bearer secret");
        then.status(500).body("Internal Server Error".to_string());
    });

    let mock_401 = server.mock(|when, then| {
        when.method(GET).path("/v2/account");
        then.status(401).body("Unauthorized");
    });

    let mut config = ClientConfig {
        network: NetworkConfig {
            endpoints: Some(vec![server.url("/")]),
            api_token: Some("my_secret_token".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let client = Arc::new(ClientContext::new(config.clone()).unwrap());
    // Correct request
    let params = ParamsOfGetAccount { address: contract_addr.to_string() };
    let boc = account::get_account(client.clone(), params).await?.boc;
    assert_eq!(boc, contract_boc);
    mock_200.assert();

    // Not found address
    let params = ParamsOfGetAccount { address: not_found_addr.to_string() };
    match account::get_account(client.clone(), params).await {
        Ok(_) => panic!("Expected an error but got Ok"),
        Err(e) => {
            assert_eq!(e.code, crate::net::ErrorCode::NotFound as u32);
        }
    }
    mock_404.assert();

    // Internal error
    let params = ParamsOfGetAccount { address: error_addr.to_string() };
    match account::get_account(client.clone(), params).await {
        Ok(_) => panic!("Expected an error but got Ok"),
        Err(e) => {
            assert_eq!(e.code, crate::net::ErrorCode::InvalidServerResponse as u32);
        }
    }
    mock_500.assert();

    // Request without authorization
    config.network.api_token = None;
    let client = Arc::new(ClientContext::new(config).unwrap());
    let params = ParamsOfGetAccount { address: contract_addr.to_string() };
    let response: Result<ResultOfGetAccount, crate::error::ClientError> =
        account::get_account(client.clone(), params).await;

    match response {
        Ok(_) => panic!("Expected an error but got Ok"),
        Err(e) => {
            assert_eq!(e.code, crate::net::ErrorCode::Unauthorized as u32);
        }
    };
    mock_401.assert();

    Ok(())
}
