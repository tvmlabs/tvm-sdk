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

// 2022-2025 (c) Copyright Contributors to the GOSH DAO. All rights reserved.
//

use std::collections::HashMap;
use std::collections::HashSet;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use futures::Future;
use futures::Stream;
use futures::StreamExt;
use rand::seq::SliceRandom;
use reqwest::Url;
use serde_json::Value;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio::sync::watch;
use tvm_block::MsgAddressInt;
use tvm_types::UInt256;
use tvm_types::base64_encode;

use super::ErrorCode;
use super::tvm_gql::ExtMessage;
use crate::client::ClientEnv;
use crate::client::FetchMethod;
use crate::error::AddNetworkUrl;
use crate::error::ClientError;
use crate::error::ClientResult;
use crate::net::Error;
use crate::net::GraphQLQueryEvent;
use crate::net::NetworkConfig;
use crate::net::ParamsOfAggregateCollection;
use crate::net::ParamsOfQueryCollection;
use crate::net::ParamsOfQueryCounterparties;
use crate::net::ParamsOfQueryOperation;
use crate::net::ParamsOfWaitForCollection;
use crate::net::PostRequest;
use crate::net::endpoint::Endpoint;
use crate::net::tvm_gql::GraphQLQuery;
use crate::net::types::NetworkQueriesProtocol;
use crate::net::websocket_link::WebsocketLink;
use crate::processing::ThreadIdentifier;

pub const MAX_TIMEOUT: u32 = i32::MAX as u32;
pub const MIN_RESUME_TIMEOUT: u32 = 500;
pub const MAX_RESUME_TIMEOUT: u32 = 3000;
pub const ENDPOINT_CACHE_TIMEOUT: u64 = 10 * 60 * 1000;
pub const API_VERSION: &str = "v2";
pub const REST_API_PORT: u16 = 8600;
pub const ENDPOINT_MESSAGES: &str = "messages";

pub(crate) struct Subscription {
    pub unsubscribe: Pin<Box<dyn Future<Output = ()> + Send>>,
    pub data_stream: Pin<Box<dyn Stream<Item = ClientResult<Value>> + Send>>,
}

struct SuspendRegulation {
    sender: watch::Sender<bool>,
    internal_suspend: bool,
    external_suspend: bool,
}

pub(crate) enum EndpointStat {
    MessageDelivered,
    MessageUndelivered,
}

#[allow(dead_code)]
pub(crate) struct ResolvedEndpoint {
    pub endpoint: Arc<Endpoint>,
    pub time_added: u64,
}

pub(crate) struct NetworkState {
    client_env: Arc<ClientEnv>,
    config: NetworkConfig,
    endpoint_addresses: RwLock<Vec<String>>,
    has_multiple_endpoints: AtomicBool,
    bad_delivery_addresses: RwLock<HashSet<String>>,
    suspended: watch::Receiver<bool>,
    suspend_regulation: Arc<Mutex<SuspendRegulation>>,
    resume_timeout: AtomicU32,
    query_endpoint: RwLock<Option<Arc<Endpoint>>>,
    resolved_endpoints: RwLock<HashMap<String, ResolvedEndpoint>>,
    bk_send_message_endpoint: RwLock<Option<Url>>,
    rest_api_endpoint: RwLock<Url>,
    bm_issuer_pubkey: RwLock<Option<String>>,
    bm_token: RwLock<Option<Value>>,
    use_https_for_rest_api: bool,
}

async fn query_by_url(
    client_env: &ClientEnv,
    address: &str,
    query: &str,
    timeout: u32,
) -> ClientResult<Value> {
    let response = client_env
        .fetch(&format!("{}?query={}", address, query), FetchMethod::Get, None, None, timeout)
        .await?;

    response.body_as_json(false)
}

impl NetworkState {
    pub fn new(
        client_env: Arc<ClientEnv>,
        config: NetworkConfig,
        endpoint_addresses: Vec<String>,
        rest_api_endpoint: Url,
        use_https_for_rest_api: bool,
    ) -> Self {
        let (sender, receiver) = watch::channel(false);
        let regulation =
            SuspendRegulation { sender, internal_suspend: false, external_suspend: false };
        let has_multiple_endpoints = AtomicBool::new(endpoint_addresses.len() > 1);
        Self {
            client_env,
            config,
            endpoint_addresses: RwLock::new(endpoint_addresses),
            has_multiple_endpoints,
            bad_delivery_addresses: RwLock::new(HashSet::new()),
            suspended: receiver,
            suspend_regulation: Arc::new(Mutex::new(regulation)),
            resume_timeout: AtomicU32::new(0),
            query_endpoint: RwLock::new(None),
            resolved_endpoints: Default::default(),
            bk_send_message_endpoint: RwLock::new(None),
            rest_api_endpoint: RwLock::new(rest_api_endpoint),
            bm_issuer_pubkey: RwLock::new(None),
            bm_token: RwLock::new(None),
            use_https_for_rest_api,
        }
    }

    pub fn has_multiple_endpoints(&self) -> bool {
        self.has_multiple_endpoints.load(Ordering::Relaxed)
    }

    async fn suspend(&self, sender: &watch::Sender<bool>) {
        if !*self.suspended.borrow() {
            let _ = sender.send(true);
            *self.query_endpoint.write().await = None;
        }
    }

    async fn resume(sender: &watch::Sender<bool>) {
        let _ = sender.send(false);
    }

    pub async fn external_suspend(&self) {
        let mut regulation = self.suspend_regulation.lock().await;
        regulation.external_suspend = true;
        self.suspend(&regulation.sender).await;
    }

    pub async fn external_resume(&self) {
        let mut regulation = self.suspend_regulation.lock().await;
        regulation.external_suspend = false;
        if !regulation.internal_suspend {
            Self::resume(&regulation.sender).await;
        }
    }

    pub fn next_resume_timeout(&self) -> u32 {
        let timeout = self.resume_timeout.load(Ordering::Relaxed);
        let next_timeout = (timeout * 2).clamp(MIN_RESUME_TIMEOUT, MAX_RESUME_TIMEOUT); // 0, 0.5, 1, 2, 3, 3, 3...
        self.resume_timeout.store(next_timeout, Ordering::Relaxed);
        timeout
    }

    pub fn reset_resume_timeout(&self) {
        self.resume_timeout.store(0, Ordering::Relaxed);
    }

    pub async fn internal_suspend(&self) {
        let mut regulation = self.suspend_regulation.lock().await;
        if regulation.internal_suspend {
            return;
        }

        regulation.internal_suspend = true;
        self.suspend(&regulation.sender).await;

        let timeout = self.next_resume_timeout();
        log::debug!("Internal resume timeout {}", timeout);

        let env = self.client_env.clone();
        let regulation = self.suspend_regulation.clone();

        self.client_env.spawn(async move {
            let _ = env.set_timer(timeout as u64).await;
            let mut regulation = regulation.lock().await;
            regulation.internal_suspend = false;
            if !regulation.external_suspend {
                Self::resume(&regulation.sender).await;
            }
        });
    }

    pub async fn set_endpoint_addresses(&self, addresses: Vec<String>) {
        self.has_multiple_endpoints.store(addresses.len() > 1, Ordering::Relaxed);
        *self.endpoint_addresses.write().await = addresses;
    }

    #[allow(dead_code)]
    pub async fn get_addresses_for_sending(&self) -> Vec<String> {
        let mut addresses = self.endpoint_addresses.read().await.clone();
        addresses.shuffle(&mut rand::thread_rng());
        let bad_delivery = self.bad_delivery_addresses.read().await.clone();
        if !bad_delivery.is_empty() {
            let mut i = 0;
            let mut processed = 0;
            while processed < addresses.len() {
                if bad_delivery.contains(&addresses[i]) {
                    let address = addresses.remove(i);
                    addresses.push(address);
                } else {
                    i += 1;
                }
                processed += 1;
            }
        }
        addresses
    }

    pub async fn update_stat(&self, addresses: &[String], stat: EndpointStat) {
        let bad_delivery = self.bad_delivery_addresses.read().await.clone();
        let addresses: HashSet<_> = addresses.iter().cloned().collect();
        let new_bad_delivery = match stat {
            EndpointStat::MessageDelivered => &bad_delivery - &addresses,
            EndpointStat::MessageUndelivered => &bad_delivery | &addresses,
        };
        if new_bad_delivery != bad_delivery {
            *self.bad_delivery_addresses.write().await = new_bad_delivery;
        }
    }

    pub async fn invalidate_querying_endpoint(&self) {
        *self.query_endpoint.write().await = None
    }

    pub async fn refresh_query_endpoint(&self) -> ClientResult<()> {
        let endpoint_guard = self.query_endpoint.write().await;
        if let Some(endpoint) = endpoint_guard.as_ref() {
            endpoint.refresh(&self.client_env, &self.config).await
        } else {
            Ok(())
        }
    }

    pub async fn config_servers(&self) -> Vec<String> {
        self.endpoint_addresses.read().await.clone()
    }

    pub async fn query_endpoint(&self) -> Option<Arc<Endpoint>> {
        self.query_endpoint.read().await.clone()
    }

    pub async fn resolve_endpoint(&self, address: &str) -> ClientResult<Arc<Endpoint>> {
        let endpoint = Endpoint::resolve(&self.client_env, &self.config, address).await?;
        let endpoint = Arc::new(endpoint);
        self.add_resolved_endpoint(address.to_owned(), endpoint.clone()).await;
        Ok(endpoint)
    }

    async fn select_querying_endpoint(self: &Arc<NetworkState>) -> ClientResult<Arc<Endpoint>> {
        let is_better =
            |a: &ClientResult<Arc<Endpoint>>, b: &ClientResult<Arc<Endpoint>>| match (a, b) {
                (Ok(a), Ok(b)) => a.latency() < b.latency(),
                (Ok(_), Err(_)) => true,
                (Err(_), Err(_)) => true,
                _ => false,
            };
        let start = self.client_env.now_ms();
        loop {
            let mut futures = vec![];
            for address in self.endpoint_addresses.read().await.iter() {
                let address = address.clone();
                let self_copy = self.clone();
                futures.push(Box::pin(async move { self_copy.resolve_endpoint(&address).await }));
            }
            let mut selected = Err(crate::client::Error::net_module_not_init());
            let mut unauthorised = None;
            while !futures.is_empty() {
                let (result, _, remain_futures) = futures::future::select_all(futures).await;
                if let Ok(endpoint) = &result {
                    if endpoint.latency() <= self.config.max_latency as u64 {
                        if !remain_futures.is_empty() {
                            self.client_env.spawn(async move {
                                futures::future::join_all(remain_futures).await;
                            });
                        }
                        return result;
                    }
                }
                futures = remain_futures;
                if let Err(err) = &result {
                    if err.is_unauthorized() {
                        unauthorised = Some(err.clone());
                    }
                }
                if is_better(&result, &selected) {
                    selected = result;
                }
            }
            if selected.is_ok() {
                return selected;
            }
            if let Some(unauthorised) = unauthorised {
                return Err(unauthorised);
            }
            if !self.can_retry_network_error(start) {
                return selected;
            }
            let _ = self.client_env.set_timer(self.next_resume_timeout() as u64).await;
        }
    }

    pub async fn get_query_endpoint(self: &Arc<NetworkState>) -> ClientResult<Arc<Endpoint>> {
        // wait for resume
        let mut suspended = self.suspended.clone();
        while *suspended.borrow() {
            let _ = suspended.changed().await;
        }

        if let Some(endpoint) = &*self.query_endpoint.read().await {
            return Ok(endpoint.clone());
        }

        let mut locked_query_endpoint = self.query_endpoint.write().await;
        if let Some(endpoint) = &*locked_query_endpoint {
            return Ok(endpoint.clone());
        }
        let fastest = self.select_querying_endpoint().await?;
        *locked_query_endpoint = Some(fastest.clone());
        Ok(fastest)
    }

    pub async fn get_all_endpoint_addresses(&self) -> ClientResult<Vec<String>> {
        Ok(self.endpoint_addresses.read().await.clone())
    }

    pub async fn add_resolved_endpoint(&self, address: String, endpoint: Arc<Endpoint>) {
        let mut lock = self.resolved_endpoints.write().await;
        lock.insert(address, ResolvedEndpoint { endpoint, time_added: self.client_env.now_ms() });
    }

    #[allow(dead_code)]
    pub async fn get_resolved_endpoint(&self, address: &str) -> Option<Arc<Endpoint>> {
        let lock = self.resolved_endpoints.read().await;
        lock.get(address).and_then(|endpoint| {
            if endpoint.time_added + ENDPOINT_CACHE_TIMEOUT > self.client_env.now_ms() {
                Some(endpoint.endpoint.clone())
            } else {
                None
            }
        })
    }

    pub async fn select_send_message_endpoint(&self) -> Url {
        let guarded_bk_endpoint = self.bk_send_message_endpoint.read().await;
        if let Some(bk_endpoint) = guarded_bk_endpoint.as_ref() {
            bk_endpoint.clone()
        } else {
            self.rest_api_endpoint.read().await.clone()
        }
    }

    pub async fn get_rest_api_endpoint(&self) -> Url {
        self.rest_api_endpoint.read().await.clone()
    }

    pub async fn update_bk_send_message_endpoint(&self, endpoint: Option<Url>) {
        *self.bk_send_message_endpoint.write().await = endpoint
    }

    pub async fn get_bm_issuer_pubkey(&self) -> Option<String> {
        self.bm_issuer_pubkey.read().await.clone()
    }

    pub async fn update_bm_issuer_pubkey(&self, address: Option<String>) {
        *self.bm_issuer_pubkey.write().await = address
    }

    pub async fn get_bm_token(&self) -> Option<Value> {
        self.bm_token.read().await.clone()
    }

    pub async fn update_bm_token(&self, token: &Value) {
        *self.bm_token.write().await = Some(token.clone())
    }

    pub async fn update_bm_data(&self, token: &Value) {
        let issuer_pubkey =
            token.get("issuer").and_then(|issuer| issuer.get("bm").or_else(|| issuer.get("bk")));

        if let Some(Value::String(pubkey)) = issuer_pubkey {
            self.update_bm_issuer_pubkey(Some(pubkey.to_string())).await;
        }

        self.update_bm_token(token).await;
    }

    pub fn can_retry_network_error(&self, start: u64) -> bool {
        self.client_env.now_ms() < start + self.config.max_reconnect_timeout as u64
    }

    pub fn env(&self) -> &Arc<ClientEnv> {
        &self.client_env
    }
}

pub(crate) struct ServerLink {
    config: NetworkConfig,
    pub(crate) client_env: Arc<ClientEnv>,
    websocket_link: Arc<WebsocketLink>,
    state: Arc<NetworkState>,
}

fn strip_endpoint(endpoint: &str) -> String {
    endpoint
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .trim_end_matches('\\')
        .to_string()
}

fn replace_endpoints(endpoints: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    for endpoint in endpoints {
        let norm = strip_endpoint(&endpoint);
        if seen.insert(norm) {
            result.push(endpoint);
        }
    }

    result
}

pub fn construct_rest_api_endpoint(original: &str, use_https: bool) -> ClientResult<Url> {
    // Set HTTP if scheme is not presented
    let original = if original.contains("://") {
        original.to_string()
    } else {
        format!("{}://{}", if use_https { "https" } else { "http" }, original)
    };

    let mut url = Url::parse(&original).map_err(Error::parse_url_failed)?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| Error::parse_url_failed("Missing port in URL"))?;

    // Set the port for the REST API if no specific port is specified
    if (url.scheme() == "http" && port == 80) || (url.scheme() == "https" && port == 443) {
        url.set_port(Some(REST_API_PORT)).map_err(|_| Error::parse_url_failed("Can't set port"))?;
    }
    if url.scheme() == "https" {
        url.set_port(None).map_err(|_| Error::parse_url_failed("Can't set port"))?;
    }
    url.set_path(&format!("{API_VERSION}/"));
    Ok(url)
}

fn get_redirection_data(data: &Value, use_https: bool) -> (Option<String>, Option<Url>) {
    // TODO: Add type RedirectionData
    let producers_opt = data.get("result").and_then(|res| res.get("producers")).or_else(|| {
        data.get("node_error")
            .and_then(|ne| ne.get("extensions"))
            .and_then(|ext| ext.get("details"))
            .and_then(|details| details.get("producers"))
    });

    let redirection_url = producers_opt
        .and_then(|val| val.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.as_str())
        .and_then(|s| construct_rest_api_endpoint(s, use_https).ok());

    let thread_id = data
        .get("node_error")
        .and_then(|ne| ne.get("extensions"))
        .and_then(|ext| ext.get("details"))
        .and_then(|details| details.get("thread_id"))
        .and_then(Value::as_str)
        .map(String::from);

    (thread_id, redirection_url)
}

impl ServerLink {
    pub fn new(config: NetworkConfig, client_env: Arc<ClientEnv>) -> ClientResult<Self> {
        let endpoint_addresses = config.endpoints.clone().unwrap_or(vec!["localhost".to_string()]);
        if endpoint_addresses.is_empty() {
            return Err(crate::client::Error::net_module_not_init());
        }
        let rest_api_addr = endpoint_addresses[0].clone();

        let endpoint_addresses = replace_endpoints(endpoint_addresses);

        let use_https_for_rest_api =
            endpoint_addresses.first().is_some_and(|s| s.starts_with("https://"));
        let rest_api_endpoint =
            construct_rest_api_endpoint(&rest_api_addr, use_https_for_rest_api)?;

        let state = Arc::new(NetworkState::new(
            client_env.clone(),
            config.clone(),
            endpoint_addresses,
            rest_api_endpoint,
            use_https_for_rest_api,
        ));

        Ok(ServerLink {
            config: config.clone(),
            client_env: client_env.clone(),
            state: state.clone(),
            websocket_link: Arc::new(WebsocketLink::new(client_env, state, config)),
        })
    }

    pub fn config(&self) -> &NetworkConfig {
        &self.config
    }

    pub async fn config_servers(&self) -> Vec<String> {
        self.state.config_servers().await
    }

    pub fn state(&self) -> Arc<NetworkState> {
        self.state.clone()
    }

    // Returns Stream with updates database fields by provided filter
    pub async fn subscribe_collection(
        &self,
        table: &str,
        filter: &Value,
        fields: &str,
    ) -> ClientResult<Subscription> {
        self.subscribe_operation(
            GraphQLQuery::with_collection_subscription(table, filter, fields),
            format!("/{}", table),
        )
        .await
    }

    pub async fn subscribe(
        &self,
        subscription: String,
        variables: Option<Value>,
    ) -> ClientResult<Subscription> {
        self.subscribe_operation(
            GraphQLQuery::with_subscription(subscription.trim().to_string(), variables),
            String::new(),
        )
        .await
    }

    pub async fn subscribe_operation(
        &self,
        operation: GraphQLQuery,
        result_path: String,
    ) -> ClientResult<Subscription> {
        let mut event_receiver = self.websocket_link.start_operation(operation).await?;

        let mut id = None;
        let start = self.client_env.now_ms();
        loop {
            match event_receiver.recv().await {
                Some(GraphQLQueryEvent::Id(received_id)) => id = Some(received_id),
                Some(GraphQLQueryEvent::Data(_)) => {
                    return Err(Error::wrong_ws_protocol_sequence(
                        "data received before operation started",
                    ));
                }
                Some(GraphQLQueryEvent::Complete) => {
                    return Err(Error::wrong_ws_protocol_sequence(
                        "operation completed before started",
                    ));
                }
                Some(GraphQLQueryEvent::Error(err)) => {
                    if err.code == ErrorCode::NetworkModuleSuspended as u32
                        || err.code == ErrorCode::NetworkModuleResumed as u32
                    {
                        continue;
                    }
                    let is_retryable = err.code != ErrorCode::GraphqlWebsocketInitError as u32
                        && crate::client::Error::is_network_error(&err);
                    if !is_retryable || !self.state.can_retry_network_error(start) {
                        return Err(err);
                    }
                }
                Some(GraphQLQueryEvent::Started) => break,
                None => {
                    return Err(Error::wrong_ws_protocol_sequence(
                        "receiver stream is closed before operation started",
                    ));
                }
            }
        }

        let id =
            id.ok_or_else(|| Error::wrong_ws_protocol_sequence("operation ID is not provided"))?;
        let result_path = Arc::new(result_path);
        let event_receiver = tokio_stream::wrappers::ReceiverStream::new(event_receiver);
        let data_receiver = event_receiver.filter_map(move |event| {
            let result_path = result_path.clone();
            async move {
                match event {
                    GraphQLQueryEvent::Data(mut value) => Some(Ok(value
                        .pointer_mut(&result_path)
                        .map(|val| val.take())
                        .unwrap_or_default())),
                    GraphQLQueryEvent::Error(error) => Some(Err(error)),
                    GraphQLQueryEvent::Complete => Some(Ok(Value::Null)),
                    GraphQLQueryEvent::Id(_) => Some(Err(Error::wrong_ws_protocol_sequence(
                        "ID has changed after operation started",
                    ))),
                    GraphQLQueryEvent::Started => None,
                }
            }
        });

        let link = self.websocket_link.clone();
        let unsubscribe = async move {
            link.stop_operation(id).await;
        };

        Ok(Subscription {
            data_stream: Box::pin(data_receiver),
            unsubscribe: Box::pin(unsubscribe),
        })
    }

    pub(crate) async fn query_graphql(
        &self,
        query: &GraphQLQuery,
        endpoint: Option<&Endpoint>,
    ) -> ClientResult<Value> {
        let request = json!({
            "query": query.query,
            "variables": query.variables,
        })
        .to_string();

        let mut headers = HashMap::new();
        headers.insert("content-type".to_owned(), "application/json".to_owned());
        for (name, value) in Endpoint::http_headers(&self.config) {
            headers.insert(name, value);
        }

        let mut current_endpoint: Option<Arc<Endpoint>>;
        let start = self.client_env.now_ms();
        loop {
            let endpoint = if let Some(endpoint) = endpoint {
                endpoint
            } else {
                current_endpoint = Some(self.state.get_query_endpoint().await?.clone());
                current_endpoint.as_ref().unwrap()
            };
            let result = self
                .client_env
                .fetch(
                    &endpoint.query_url,
                    FetchMethod::Post,
                    Some(headers.clone()),
                    Some(request.clone()),
                    query.timeout.unwrap_or(self.config.query_timeout),
                )
                .await;
            let result = match result {
                Err(err) => Err(err),
                Ok(response) => {
                    self.state.reset_resume_timeout();
                    if response.status == 401 {
                        Err(Error::unauthorized(&response))
                    } else {
                        match response.body_as_json(false) {
                            Err(err) => Err(err),
                            Ok(value) => match Error::try_extract_graphql_error(&value) {
                                Some(err) => Err(err),
                                None => Ok(value),
                            },
                        }
                    }
                }
            };

            if let Err(err) = &result {
                if crate::client::Error::is_network_error(err) {
                    let multiple_endpoints = self.state.has_multiple_endpoints();
                    if multiple_endpoints {
                        self.state.internal_suspend().await;
                        self.websocket_link.suspend().await;
                        self.websocket_link.resume().await;
                    }
                    if self.state.can_retry_network_error(start) {
                        if !multiple_endpoints {
                            let _ = self
                                .client_env
                                .set_timer(self.state.next_resume_timeout() as u64)
                                .await;
                        }
                        continue;
                    }
                }
            }

            return result;
        }
    }

    pub(crate) async fn query_http(&self, request: String, endpoint: &Url) -> ClientResult<Value> {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_owned(), "application/json".to_owned());
        for (name, value) in Endpoint::http_headers(&self.config) {
            headers.insert(name, value);
        }

        let start = self.client_env.now_ms();

        let mut last_error = None;
        for _ in 0..1 {
            let result = self
                .client_env
                .fetch(
                    endpoint.as_str(),
                    FetchMethod::Post,
                    Some(headers.clone()),
                    Some(request.clone()),
                    self.config.query_timeout,
                )
                .await;

            let result = match result {
                Err(err) => Err(err),
                Ok(response) => {
                    self.state.reset_resume_timeout();
                    if response.status == 401 {
                        Err(Error::unauthorized(&response))
                    } else if response.status == 404 {
                        Err(Error::not_found("The requested resource could not be found"))
                    } else {
                        match response.body_as_json(true) {
                            Err(err) => Err(err),
                            Ok(value) => match Error::try_extract_send_messages_error(&value) {
                                Some(err) => Err(err),
                                None => Ok(value),
                            },
                        }
                    }
                }
            };

            if let Err(err) = &result {
                if crate::client::Error::is_network_error(err)
                    && self.state.can_retry_network_error(start)
                {
                    let _ =
                        self.client_env.set_timer(self.state.next_resume_timeout() as u64).await;
                    last_error = Some(err.clone());
                    continue;
                }
            }
            return result;
        }
        Err(Error::all_attempts_failed(last_error))
    }

    pub(crate) async fn http_get(&self, url: Url) -> ClientResult<Value> {
        let mut headers = HashMap::new();
        for (name, value) in Endpoint::http_headers(&self.config) {
            headers.insert(name, value);
        }
        let result = self
            .client_env
            .fetch(url.as_ref(), FetchMethod::Get, Some(headers), None, self.config.query_timeout)
            .await;

        match result {
            Err(err) => Err(err),
            Ok(response) => {
                if response.status == 200 {
                    response.body_as_json(true)
                } else if response.status == 401 {
                    Err(Error::unauthorized(&response))
                } else if response.status == 404 {
                    Err(Error::not_found(&format!("Resource not found: {url}")))
                } else {
                    // HTTP_CODE 500 and any other unhandled codes
                    Err(Error::invalid_server_response(response.body))
                }
            }
        }
    }

    pub(crate) async fn query_ws(&self, query: &GraphQLQuery) -> ClientResult<Value> {
        let mut receiver = self.websocket_link.start_operation(query.clone()).await?;
        let mut id = None::<u32>;
        let mut result = Ok(Value::Null);
        let start = self.client_env.now_ms();
        loop {
            match receiver.recv().await {
                Some(GraphQLQueryEvent::Id(received_id)) => id = Some(received_id),
                Some(GraphQLQueryEvent::Data(data)) => {
                    result = Ok(json!({ "data": data }));
                    break;
                }
                Some(GraphQLQueryEvent::Complete) => break,
                Some(GraphQLQueryEvent::Error(err)) => {
                    if err.code == ErrorCode::NetworkModuleSuspended as u32
                        || err.code == ErrorCode::NetworkModuleResumed as u32
                    {
                        continue;
                    }
                    let is_retryable = err.code != ErrorCode::GraphqlWebsocketInitError as u32
                        && crate::client::Error::is_network_error(&err);
                    result = Err(err);
                    if !is_retryable || !self.state.can_retry_network_error(start) {
                        break;
                    }
                }
                Some(GraphQLQueryEvent::Started) => {}
                None => break,
            }
        }
        if let Some(id) = id {
            self.websocket_link.stop_operation(id).await;
        }
        result
    }

    pub(crate) async fn query(
        &self,
        query: &GraphQLQuery,
        endpoint: Option<&Endpoint>,
    ) -> ClientResult<Value> {
        match self.config.queries_protocol {
            NetworkQueriesProtocol::HTTP => self.query_graphql(query, endpoint).await,
            NetworkQueriesProtocol::WS => self.query_ws(query).await,
        }
    }

    pub async fn batch_query(
        &self,
        params: &[ParamsOfQueryOperation],
        endpoint: Option<Endpoint>,
    ) -> ClientResult<Vec<Value>> {
        let latency_detection_required = if endpoint.is_some() {
            false
        } else if self.state.has_multiple_endpoints() {
            let endpoint = self.state.get_query_endpoint().await?;
            self.client_env.now_ms() > endpoint.next_latency_detection_time()
        } else {
            false
        };
        let mut query =
            GraphQLQuery::build(params, latency_detection_required, self.config.wait_for_timeout);
        let info_request_time = self.client_env.now_ms();
        let mut result = self.query(&query, endpoint.as_ref()).await?;
        if latency_detection_required {
            let current_endpoint = self.state.get_query_endpoint().await?;
            let server_info = query.get_server_info(params, &result)?;
            current_endpoint.apply_server_info(
                &self.client_env,
                &self.config,
                info_request_time,
                &server_info,
            )?;
            if current_endpoint.latency() > self.config.max_latency as u64 {
                self.invalidate_querying_endpoint().await;
                query = GraphQLQuery::build(params, false, self.config.wait_for_timeout);
                result = self.query(&query, endpoint.as_ref()).await?;
            }
        }
        query.get_results(params, &result)
    }

    pub async fn query_collection(
        &self,
        params: ParamsOfQueryCollection,
        endpoint: Option<Endpoint>,
    ) -> ClientResult<Value> {
        Ok(self
            .batch_query(&[ParamsOfQueryOperation::QueryCollection(params)], endpoint)
            .await?
            .remove(0))
    }

    pub async fn wait_for_collection(
        &self,
        params: ParamsOfWaitForCollection,
        endpoint: Option<Endpoint>,
    ) -> ClientResult<Value> {
        Ok(self
            .batch_query(&[ParamsOfQueryOperation::WaitForCollection(params)], endpoint)
            .await?
            .remove(0))
    }

    pub async fn aggregate_collection(
        &self,
        params: ParamsOfAggregateCollection,
        endpoint: Option<Endpoint>,
    ) -> ClientResult<Value> {
        Ok(self
            .batch_query(&[ParamsOfQueryOperation::AggregateCollection(params)], endpoint)
            .await?
            .remove(0))
    }

    pub async fn query_counterparties(
        &self,
        params: ParamsOfQueryCounterparties,
    ) -> ClientResult<Value> {
        Ok(self
            .batch_query(&[ParamsOfQueryOperation::QueryCounterparties(params)], None)
            .await?
            .remove(0))
    }

    // Sends message to blockchain
    pub async fn send_message(
        &self,
        msg_id: &str,
        msg_body: &[u8],
        thread_id: ThreadIdentifier,
        dst: MsgAddressInt,
    ) -> ClientResult<Value> {
        // This helper function adds "resource" part to the URL
        fn ensure_resource(url: &Url) -> Url {
            let resource = ENDPOINT_MESSAGES;
            if url.path().ends_with(resource) {
                url.clone()
            } else {
                url.join(ENDPOINT_MESSAGES).unwrap_or(url.clone())
            }
        }
        let mut attempts = 0;

        let network_state = self.state();
        let mut message = ExtMessage {
            id: msg_id.to_string(),
            body: base64_encode(msg_body),
            expire_at: None,
            thread_id: Some(thread_id.to_string()),
            ext_message_token: network_state.get_bm_token().await,
        };

        let mut endpoint = network_state.select_send_message_endpoint().await;

        let query = json!([message]).to_string();
        let mut result = self.query_http(query, &ensure_resource(&endpoint)).await;
        while attempts < self.config.message_retries_count {
            attempts += 1;
            if let Ok(ref data) = result {
                let (_, bk_url) = get_redirection_data(data, self.state.use_https_for_rest_api);
                if bk_url.is_some() {
                    network_state.update_bk_send_message_endpoint(bk_url).await;
                }
                if let Some(Value::Object(_)) = data.get("ext_message_token") {
                    network_state.update_bm_data(&data["ext_message_token"]).await;
                }
            }

            let Err(err) = result.as_mut() else { return result };

            if let Some(Value::Object(_)) = err.data.get("ext_message_token") {
                network_state.update_bm_data(&err.data["ext_message_token"]).await;
                message.ext_message_token = network_state.get_bm_token().await;
            }

            let Some(ext) = err.data.get("node_error").and_then(|e| e.get("extensions")) else {
                return result;
            };

            let Some(code) = ext.get("code").and_then(Value::as_str) else {
                return result;
            };

            if !["WRONG_PRODUCER", "THREAD_MISMATCH", "TOKEN_EXPIRED"].contains(&code) {
                ensure_address(&mut err.data, Value::String(dst.to_string()));
                return result;
            }

            if code == "TOKEN_EXPIRED" {
                endpoint = network_state.get_rest_api_endpoint().await.clone();
                network_state.update_bk_send_message_endpoint(None).await;
            }

            let (real_thread_id, redirect_url) =
                get_redirection_data(&err.data, self.state.use_https_for_rest_api);

            if let Some(thread_id) = real_thread_id {
                message.set_thread_id(Some(thread_id));
            }

            if let Some(bk_endpoint) = redirect_url {
                network_state.update_bk_send_message_endpoint(Some(bk_endpoint)).await;
                endpoint = network_state.select_send_message_endpoint().await;
            }
            result =
                self.query_http(json!([message]).to_string(), &ensure_resource(&endpoint)).await;
        }

        result
    }

    pub async fn send_messages(
        &self,
        messages: Vec<(UInt256, String)>,
        endpoint: Option<&Endpoint>,
    ) -> ClientResult<Option<ClientError>> {
        let mut requests = Vec::with_capacity(messages.len());
        for (hash, boc) in messages {
            requests.push(PostRequest { id: base64_encode(hash.as_slice()), body: boc })
        }
        let result = self.query(&GraphQLQuery::with_post_requests(&requests), endpoint).await;

        // Send messages is always successful in order to process case when server
        // received message but client didn't receive response
        if let Err(err) = &result {
            log::warn!("Send messages error: {}", err.message);
        }

        Ok(result.err())
    }

    pub async fn suspend(&self) {
        self.state.external_suspend().await;
        self.websocket_link.suspend().await;
    }

    pub async fn resume(&self) {
        self.state.external_resume().await;
        self.websocket_link.resume().await;
    }

    pub async fn fetch_endpoint_addresses(&self) -> ClientResult<Vec<String>> {
        let endpoint = self.state.get_query_endpoint().await?;

        let result = query_by_url(
            &self.client_env,
            &endpoint.query_url,
            "%7Binfo%7Bendpoints%7D%7D",
            self.config.query_timeout,
        )
        .await
        .add_network_url(self)
        .await?;

        serde_json::from_value(result["data"]["info"]["endpoints"].clone()).map_err(|_| {
            Error::invalid_server_response(format!(
                "Can not parse endpoints from response: {}",
                result
            ))
        })
    }

    pub async fn set_endpoints(&self, endpoints: Vec<String>) {
        self.state.set_endpoint_addresses(endpoints).await;
    }

    #[allow(dead_code)]
    pub async fn get_addresses_for_sending(&self) -> Vec<String> {
        self.state.get_addresses_for_sending().await
    }

    pub async fn get_query_endpoint(&self) -> ClientResult<Arc<Endpoint>> {
        self.state.get_query_endpoint().await
    }

    pub async fn get_all_endpoint_addresses(&self) -> ClientResult<Vec<String>> {
        self.state.get_all_endpoint_addresses().await
    }

    pub async fn update_stat(&self, addresses: &[String], stat: EndpointStat) {
        self.state.update_stat(addresses, stat).await
    }

    pub async fn invalidate_querying_endpoint(&self) {
        self.state.invalidate_querying_endpoint().await
    }
}

fn ensure_address(err_data: &mut Value, dst: Value) {
    if let Some(addr) = err_data.pointer_mut("/node_error/extensions/details/address") {
        if addr.is_null() {
            *addr = dst;
        }
    } else if let Some(details) =
        err_data.pointer_mut("/node_error/extensions/details").and_then(Value::as_object_mut)
    {
        details.insert("address".to_string(), dst);
    }
}

#[cfg(test)]
#[test]
fn test_construct_rest_endpoint() {
    fn rest_url(origin: &str, use_https: bool) -> String {
        construct_rest_api_endpoint(origin, use_https).unwrap().to_string()
    }
    assert_eq!(rest_url("a.b.c", false), "http://a.b.c:8600/v2/");
    assert_eq!(rest_url("a.b.c", true), "https://a.b.c/v2/");
    assert_eq!(rest_url("a.b.c:1234", false), "http://a.b.c:1234/v2/");
    assert_eq!(rest_url("a.b.c:1234", true), "https://a.b.c/v2/");
    assert_eq!(rest_url("http://a.b.c", false), "http://a.b.c:8600/v2/");
    assert_eq!(rest_url("http://a.b.c", true), "http://a.b.c:8600/v2/");
    assert_eq!(rest_url("http://a.b.c:1234", false), "http://a.b.c:1234/v2/");
    assert_eq!(rest_url("http://a.b.c:1234", true), "http://a.b.c:1234/v2/");
    assert_eq!(rest_url("https://a.b.c", false), "https://a.b.c/v2/");
    assert_eq!(rest_url("https://a.b.c", true), "https://a.b.c/v2/");
    assert_eq!(rest_url("https://a.b.c:1234", false), "https://a.b.c/v2/");
    assert_eq!(rest_url("https://a.b.c:1234", true), "https://a.b.c/v2/");
}

// #[cfg(test)]
// #[path = "tests/test_server_link.rs"]
// mod tests;
