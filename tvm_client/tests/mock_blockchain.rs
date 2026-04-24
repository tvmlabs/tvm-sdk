mod support;

use std::sync::Arc;

use serde_json::json;
use support::mock_blockchain::MockBlockchain;
use tvm_client::ClientConfig;
use tvm_client::ClientContext;
use tvm_client::net;
use tvm_client::net::NetworkConfig;
use tvm_client::net::NetworkQueriesProtocol;
use tvm_client::net::ParamsOfQuery;
use tvm_client::net::ParamsOfQueryCollection;

fn client_for(endpoint: String) -> Arc<ClientContext> {
    Arc::new(
        ClientContext::new(ClientConfig {
            network: NetworkConfig {
                endpoints: Some(vec![endpoint]),
                queries_protocol: NetworkQueriesProtocol::HTTP,
                ..Default::default()
            },
            ..Default::default()
        })
        .unwrap(),
    )
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
