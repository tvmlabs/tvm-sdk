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

pub use batch::ParamsOfBatchQuery;
pub use batch::ResultOfBatchQuery;
pub use batch::batch_query;
pub(crate) use endpoint::Endpoint;
pub use errors::Error;
pub use errors::ErrorCode;
pub use iterators::ChainIterator;
pub use iterators::ParamsOfIteratorNext;
pub use iterators::RegisteredIterator;
pub use iterators::ResultOfIteratorNext;
pub use iterators::block_iterator::ParamsOfCreateBlockIterator;
pub use iterators::block_iterator::ParamsOfResumeBlockIterator;
pub use iterators::block_iterator::create_block_iterator;
pub use iterators::block_iterator::resume_block_iterator;
pub use iterators::iterator_next;
pub use iterators::remove_iterator;
pub use iterators::transaction_iterator::ParamsOfCreateTransactionIterator;
pub use iterators::transaction_iterator::ParamsOfResumeTransactionIterator;
pub use iterators::transaction_iterator::create_transaction_iterator;
pub use iterators::transaction_iterator::resume_transaction_iterator;
pub use network_params::ResultOfGetSignatureId;
pub use network_params::get_signature_id;
pub use queries::ParamsOfQuery;
pub use queries::ParamsOfWaitForCollection;
pub use queries::ResultOfAggregateCollection;
pub use queries::ResultOfQuery;
pub use queries::ResultOfQueryCollection;
pub use queries::ResultOfWaitForCollection;
pub use queries::aggregate_collection;
pub use queries::query;
pub use queries::query_collection;
pub use queries::query_counterparties;
pub use queries::wait_for_collection;
pub(crate) use server_link::EndpointStat;
pub(crate) use server_link::MAX_TIMEOUT;
pub(crate) use server_link::NetworkState;
pub use server_link::REST_API_PORT;
pub(crate) use server_link::ServerLink;
pub use server_link::construct_rest_api_endpoint;
pub use subscriptions::ParamsOfSubscribe;
pub use subscriptions::ParamsOfSubscribeCollection;
pub use subscriptions::ResultOfSubscribeCollection;
pub use subscriptions::ResultOfSubscription;
pub use subscriptions::SubscriptionResponseType;
pub use subscriptions::subscribe;
pub use subscriptions::subscribe_collection;
pub use subscriptions::unsubscribe;
pub use transaction_tree::MessageNode;
pub use transaction_tree::ParamsOfQueryTransactionTree;
pub use transaction_tree::ResultOfQueryTransactionTree;
pub use transaction_tree::TransactionNode;
pub use transaction_tree::query_transaction_tree;
pub use tvm_gql::AggregationFn;
pub use tvm_gql::FieldAggregation;
pub use tvm_gql::GraphQLQueryEvent;
pub use tvm_gql::OrderBy;
pub use tvm_gql::ParamsOfAggregateCollection;
pub use tvm_gql::ParamsOfQueryCollection;
pub use tvm_gql::ParamsOfQueryCounterparties;
pub use tvm_gql::ParamsOfQueryOperation;
pub use tvm_gql::PostRequest;
pub use tvm_gql::SortDirection;
pub use types::ACCOUNTS_COLLECTION;
pub use types::BLOCKS_COLLECTION;
pub use types::MESSAGES_COLLECTION;
pub use types::NetworkConfig;
pub use types::NetworkQueriesProtocol;
pub use types::TRANSACTIONS_COLLECTION;

use crate::client::ClientContext;
use crate::error::ClientResult;

pub(crate) mod batch;
mod endpoint;
mod errors;
mod gql;
pub(crate) mod iterators;
pub(crate) mod network_params;
pub(crate) mod queries;
mod server_link;
pub(crate) mod subscriptions;
pub(crate) mod transaction_tree;
mod tvm_gql;
pub(crate) mod types;
mod websocket_link;

mod acki_config;
mod network;
#[cfg(not(feature = "wasm-base"))]
#[cfg(test)]
mod tests;

pub(crate) use network::NetworkContext;

/// Suspends network module to stop any network activity
#[api_function]
pub async fn suspend(context: std::sync::Arc<ClientContext>) -> ClientResult<()> {
    context.get_server_link()?.suspend().await;
    Ok(())
}

/// Resumes network module to enable network activity
#[api_function]
pub async fn resume(context: std::sync::Arc<ClientContext>) -> ClientResult<()> {
    context.get_server_link()?.resume().await;
    Ok(())
}

#[derive(Serialize, Deserialize, ApiType, Default, Clone)]
pub struct ParamsOfFindLastShardBlock {
    /// Account address
    pub address: String,
}

#[derive(Serialize, Deserialize, ApiType, Default, Clone)]
pub struct ResultOfFindLastShardBlock {
    /// Account shard last block ID
    pub block_id: String,
}

/// Returns ID of the last block in a specified account shard
#[api_function]
pub async fn find_last_shard_block(
    context: std::sync::Arc<ClientContext>,
    params: ParamsOfFindLastShardBlock,
) -> ClientResult<ResultOfFindLastShardBlock> {
    let address = crate::encoding::account_decode(&params.address)?;

    let block_id =
        crate::processing::blocks_walking::find_last_shard_block(&context, &address, None).await?;

    Ok(ResultOfFindLastShardBlock { block_id: block_id.to_string() })
}

#[derive(Serialize, Deserialize, ApiType, Default, Clone)]
pub struct EndpointsSet {
    /// List of endpoints provided by server
    pub endpoints: Vec<String>,
}

/// Requests the list of alternative endpoints from server
#[api_function]
pub async fn fetch_endpoints(context: std::sync::Arc<ClientContext>) -> ClientResult<EndpointsSet> {
    let client = context.get_server_link()?;

    Ok(EndpointsSet { endpoints: client.fetch_endpoint_addresses().await? })
}

/// Sets the list of endpoints to use on reinit
#[api_function]
pub async fn set_endpoints(
    context: std::sync::Arc<ClientContext>,
    params: EndpointsSet,
) -> ClientResult<()> {
    if params.endpoints.is_empty() {
        return Err(Error::no_endpoints_provided());
    }

    context.get_server_link()?.set_endpoints(params.endpoints).await;

    Ok(())
}

#[derive(Serialize, Deserialize, ApiType, Default, Clone)]
pub struct ResultOfGetEndpoints {
    /// Current query endpoint
    pub query: String,
    /// List of all endpoints used by client
    pub endpoints: Vec<String>,
}

/// Requests the list of alternative endpoints from server
#[api_function]
pub async fn get_endpoints(
    context: std::sync::Arc<ClientContext>,
) -> ClientResult<ResultOfGetEndpoints> {
    let server_link = context.get_server_link()?;
    Ok(ResultOfGetEndpoints {
        query: server_link.get_query_endpoint().await?.query_url.clone(),
        endpoints: server_link.get_all_endpoint_addresses().await?,
    })
}
