mod support;

use std::sync::Arc;

use serde_json::json;
use support::mock_blockchain::MockBlockchain;
use tvm_client::ClientConfig;
use tvm_client::ClientContext;
use tvm_client::account;
use tvm_client::account::ParamsOfGetAccount;
use tvm_client::client::ErrorCode as ClientErrorCode;
use tvm_client::error::ClientError;
use tvm_client::net;
use tvm_client::net::AggregationFn;
use tvm_client::net::ErrorCode;
use tvm_client::net::FieldAggregation;
use tvm_client::net::NetworkConfig;
use tvm_client::net::NetworkQueriesProtocol;
use tvm_client::net::ParamsOfAggregateCollection;
use tvm_client::net::ParamsOfQuery;
use tvm_client::net::ParamsOfQueryCollection;
use tvm_client::net::ParamsOfQueryCounterparties;
use tvm_client::net::ParamsOfWaitForCollection;

fn client_for(endpoint: String) -> Arc<ClientContext> {
    client_for_with_token(endpoint, None)
}

fn client_for_with_token(endpoint: String, api_token: Option<&str>) -> Arc<ClientContext> {
    Arc::new(
        ClientContext::new(ClientConfig {
            network: NetworkConfig {
                endpoints: Some(vec![endpoint]),
                queries_protocol: NetworkQueriesProtocol::HTTP,
                api_token: api_token.map(str::to_owned),
                max_reconnect_timeout: 1,
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap(),
    )
}

fn expect_client_err<T>(result: Result<T, ClientError>) -> ClientError {
    match result {
        Ok(_) => panic!("expected client error"),
        Err(err) => err,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_uses_mock_blockchain_http_endpoint() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let response = net::query(
        client,
        ParamsOfQuery { query: "query{info{version}}".to_owned(), variables: None },
    )
    .await
    .unwrap();

    assert_eq!(response.result["data"]["info"]["version"], "1.2.3");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_collection_uses_mock_blockchain_http_endpoint() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let response = net::query_collection(
        client,
        ParamsOfQueryCollection {
            collection: "blocks".to_owned(),
            filter: Some(json!({ "seq_no": { "eq": 1 } })),
            result: "id seq_no".to_owned(),
            order: None,
            limit: Some(1),
        },
    )
    .await
    .unwrap();

    assert_eq!(response.result.len(), 1);
    assert_eq!(response.result[0]["id"], "mock-block-1");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_collection_maps_empty_and_malformed_mock_responses() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let empty = net::query_collection(
        client.clone(),
        ParamsOfQueryCollection {
            collection: "blocks".to_owned(),
            filter: Some(json!({ "seq_no": { "eq": 999 } })),
            result: "id seq_no".to_owned(),
            order: None,
            limit: Some(1),
        },
    )
    .await
    .unwrap();
    assert!(empty.result.is_empty());

    let err = net::query_collection(
        client.clone(),
        ParamsOfQueryCollection {
            collection: "blocks".to_owned(),
            filter: Some(json!({ "seq_no": { "eq": 13 } })),
            result: "id seq_no".to_owned(),
            order: None,
            limit: Some(1),
        },
    )
    .await;
    let err = expect_client_err(err);
    assert_eq!(err.code(), ErrorCode::InvalidServerResponse as u32);

    let missing_data = net::query_collection(
        client,
        ParamsOfQueryCollection {
            collection: "blocks".to_owned(),
            filter: Some(json!({ "seq_no": { "eq": 42 } })),
            result: "id seq_no".to_owned(),
            order: None,
            limit: Some(1),
        },
    )
    .await;
    let missing_data = expect_client_err(missing_data);
    assert_eq!(missing_data.code(), ErrorCode::QueryFailed as u32);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_maps_graphql_errors_from_mock_blockchain() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let err = net::query(
        client,
        ParamsOfQuery { query: "query{forceGraphqlError}".to_owned(), variables: None },
    )
    .await;
    let err = expect_client_err(err);

    assert_eq!(err.code(), ErrorCode::QueryFailed as u32);
    assert!(err.message().contains("forced graphql failure"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_maps_mock_blockchain_transport_errors() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let unauthorized = net::query(
        client.clone(),
        ParamsOfQuery { query: "query{forceUnauthorized}".to_owned(), variables: None },
    )
    .await;
    let unauthorized = expect_client_err(unauthorized);
    assert_eq!(unauthorized.code(), ErrorCode::Unauthorized as u32);

    let not_found = net::query(
        client.clone(),
        ParamsOfQuery { query: "query{forceHttp404}".to_owned(), variables: None },
    )
    .await;
    let not_found = expect_client_err(not_found);
    assert_eq!(not_found.code(), ErrorCode::QueryFailed as u32);

    let malformed = net::query(
        client,
        ParamsOfQuery { query: "query{forceMalformedJson}".to_owned(), variables: None },
    )
    .await;
    let malformed = expect_client_err(malformed);
    assert_eq!(malformed.code(), ErrorCode::QueryFailed as u32);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_maps_payload_graphql_errors_from_mock_blockchain() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let err = net::query(
        client,
        ParamsOfQuery { query: "query{forcePayloadGraphqlError}".to_owned(), variables: None },
    )
    .await;
    let err = expect_client_err(err);

    assert_eq!(err.code(), ErrorCode::QueryFailed as u32);
    assert!(err.message().contains("forced payload graphql failure"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_account_uses_mock_blockchain_rest_endpoint() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for_with_token(blockchain.endpoint(), Some("secret"));

    let account =
        account::get_account(client, ParamsOfGetAccount { address: "0:11111".to_owned() })
            .await
            .unwrap();

    assert_eq!(account.boc, "te6ccAAS");
    assert_eq!(account.dapp_id.as_deref(), Some("mock-dapp"));
    assert_eq!(account.state_timestamp, Some(1_700_000_001));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_account_maps_mock_blockchain_rest_errors() {
    let blockchain = MockBlockchain::start().await;
    let authorized = client_for_with_token(blockchain.endpoint(), Some("secret"));

    let not_found = account::get_account(
        authorized.clone(),
        ParamsOfGetAccount { address: "0:missing".to_owned() },
    )
    .await;
    let not_found = expect_client_err(not_found);
    assert_eq!(not_found.code(), ErrorCode::NotFound as u32);

    let invalid_response = account::get_account(
        authorized.clone(),
        ParamsOfGetAccount { address: "0:55555".to_owned() },
    )
    .await;
    let invalid_response = expect_client_err(invalid_response);
    assert_eq!(invalid_response.code(), ErrorCode::InvalidServerResponse as u32);

    let invalid_shape = account::get_account(
        authorized.clone(),
        ParamsOfGetAccount { address: "0:bad-json".to_owned() },
    )
    .await;
    let invalid_shape = expect_client_err(invalid_shape);
    assert_eq!(invalid_shape.code(), ErrorCode::InvalidServerResponse as u32);

    let invalid_json =
        account::get_account(authorized, ParamsOfGetAccount { address: "0:not-json".to_owned() })
            .await;
    let invalid_json = expect_client_err(invalid_json);
    assert_eq!(invalid_json.code(), ClientErrorCode::HttpRequestParseError as u32);

    let unauthorized = account::get_account(
        client_for_with_token(blockchain.endpoint(), Some("wrong")),
        ParamsOfGetAccount { address: "0:11111".to_owned() },
    )
    .await;
    let unauthorized = expect_client_err(unauthorized);
    assert_eq!(unauthorized.code(), ErrorCode::Unauthorized as u32);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn wait_for_collection_uses_mock_blockchain_response_and_timeout_path() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let found = net::wait_for_collection(
        client.clone(),
        ParamsOfWaitForCollection {
            collection: "blocks".to_owned(),
            filter: Some(json!({ "seq_no": { "eq": 2 } })),
            result: "id seq_no".to_owned(),
            timeout: Some(10),
        },
    )
    .await
    .unwrap();
    assert_eq!(found.result["id"], "mock-wait-block");

    let timed_out = net::wait_for_collection(
        client,
        ParamsOfWaitForCollection {
            collection: "blocks".to_owned(),
            filter: Some(json!({ "seq_no": { "eq": 777 } })),
            result: "id seq_no".to_owned(),
            timeout: Some(10),
        },
    )
    .await;
    let timed_out = expect_client_err(timed_out);
    assert_eq!(timed_out.code(), ErrorCode::WaitForTimeout as u32);
    assert!(timed_out.message().contains("wait_for operation"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn aggregate_collection_uses_mock_blockchain_response() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let aggregated = net::aggregate_collection(
        client,
        ParamsOfAggregateCollection {
            collection: "blocks".to_owned(),
            filter: Some(json!({})),
            fields: Some(vec![FieldAggregation {
                field: "".to_owned(),
                aggregation_fn: AggregationFn::COUNT,
            }]),
        },
    )
    .await
    .unwrap();

    assert_eq!(aggregated.values[0], "2");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn aggregate_collection_preserves_unexpected_mock_value_shape() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let aggregated = net::aggregate_collection(
        client,
        ParamsOfAggregateCollection {
            collection: "blocks".to_owned(),
            filter: Some(json!({ "tag": { "eq": "bad-aggregate" } })),
            fields: Some(vec![FieldAggregation {
                field: "".to_owned(),
                aggregation_fn: AggregationFn::COUNT,
            }]),
        },
    )
    .await
    .unwrap();

    assert_eq!(aggregated.values["count"], "2");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn query_counterparties_uses_mock_blockchain_pagination() {
    let blockchain = MockBlockchain::start().await;
    let client = client_for(blockchain.endpoint());

    let first_page = net::query_counterparties(
        client.clone(),
        ParamsOfQueryCounterparties {
            account: "0:account".to_owned(),
            result: "counterparty last_message_id cursor".to_owned(),
            first: Some(2),
            after: None,
        },
    )
    .await
    .unwrap();

    assert_eq!(first_page.result.len(), 2);
    assert_eq!(first_page.result[1]["cursor"], "cursor-2");

    let second_page = net::query_counterparties(
        client,
        ParamsOfQueryCounterparties {
            account: "0:account".to_owned(),
            result: "counterparty last_message_id cursor".to_owned(),
            first: Some(2),
            after: Some("cursor-2".to_owned()),
        },
    )
    .await
    .unwrap();

    assert_eq!(second_page.result.len(), 1);
    assert_eq!(second_page.result[0]["counterparty"], "0:counterparty-3");
}
