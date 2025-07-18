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

// 2022-2025 (c) Copyright Contributors to the GOSH DAO. All rights reserved.
//

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

use serde_json::Value;

use crate::client::ClientEnv;
use crate::client::FetchMethod;
use crate::client::binding_config;
use crate::client::core_version;
use crate::error::ClientResult;
use crate::net::Error;
use crate::net::NetworkConfig;

pub const BOC_VERSION: &str = "2";

#[derive(Debug)]
pub(crate) struct Endpoint {
    pub query_url: String,
    pub subscription_url: String,
    pub ip_address: Option<String>,
    pub server_version: AtomicU32,
    pub server_time_delta: AtomicI64,
    pub server_latency: AtomicU64,
    pub next_latency_detection_time: AtomicU64,
    pub remp_enabled: AtomicBool,
}

impl Clone for Endpoint {
    fn clone(&self) -> Self {
        Self {
            query_url: self.query_url.clone(),
            subscription_url: self.subscription_url.clone(),
            ip_address: self.ip_address.clone(),
            server_version: AtomicU32::new(self.server_version.load(Ordering::Relaxed)),
            server_time_delta: AtomicI64::new(self.server_time_delta.load(Ordering::Relaxed)),
            server_latency: AtomicU64::new(self.server_latency.load(Ordering::Relaxed)),
            next_latency_detection_time: AtomicU64::new(
                self.next_latency_detection_time.load(Ordering::Relaxed),
            ),
            remp_enabled: AtomicBool::new(self.remp_enabled.load(Ordering::Relaxed)),
        }
    }
}

const QUERY_INFO: &str = "?query=%7Binfo%7Bversion%20time%20latency%20rempEnabled%7D%7D";

const HTTP_PROTOCOL: &str = "http://";
const HTTPS_PROTOCOL: &str = "https://";

impl Endpoint {
    pub fn http_headers(config: &NetworkConfig) -> Vec<(String, String)> {
        let mut headers = vec![
            ("tvmclient-core-version".to_string(), core_version()),
            ("X-AckiNacki-Expected-Account-Boc-Version".to_string(), BOC_VERSION.to_owned()),
        ];
        if let Some(binding) = binding_config() {
            headers.push(("tvmclient-binding-library".to_string(), binding.library));
            headers.push(("tvmclient-binding-version".to_string(), binding.version));
        }
        if let Some(auth) = config.get_auth_header() {
            headers.push(auth);
        }

        if let Some(rest_api_auth) = config.get_rest_api_header() {
            headers.push(rest_api_auth);
        }

        headers
    }

    fn expand_address(base_url: &str) -> String {
        let mut base_url = base_url.trim_end_matches('/').to_lowercase();
        if !base_url.starts_with(HTTP_PROTOCOL) && !base_url.starts_with(HTTPS_PROTOCOL) {
            let stripped_url = base_url.split_once(['/', ':']).map(|x| x.0).unwrap_or(&base_url);
            let protocol = if stripped_url == "localhost"
                || stripped_url == "127.0.0.1"
                || stripped_url == "0.0.0.0"
            {
                HTTP_PROTOCOL
            } else {
                HTTPS_PROTOCOL
            };
            base_url = format!("{}{}", protocol, base_url);
        };
        if base_url.ends_with("/graphql") { base_url } else { format!("{}/graphql", base_url) }
    }

    async fn fetch_info_with_url(
        client_env: &ClientEnv,
        query_url: &str,
        query: &str,
        timeout: u32,
        config: &NetworkConfig,
    ) -> ClientResult<(Value, String, Option<String>)> {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_owned(), "application/json".to_owned());
        for (name, value) in Self::http_headers(config) {
            headers.insert(name, value);
        }
        let response = client_env
            .fetch(
                &format!("{}{}", query_url, query),
                FetchMethod::Get,
                Some(headers),
                None,
                timeout,
            )
            .await?;
        if response.status == 401 {
            return Err(Error::unauthorized(&response));
        }
        let query_url = response.url.trim_end_matches(query).to_owned();
        let info = response.body_as_json(false)?["data"]["info"].to_owned();
        Ok((info, query_url, response.remote_address))
    }

    pub async fn resolve(
        client_env: &ClientEnv,
        config: &NetworkConfig,
        address: &str,
    ) -> ClientResult<Self> {
        let address = Self::expand_address(address);
        let info_request_time = client_env.now_ms();
        let (info, query_url, ip_address) = Self::fetch_info_with_url(
            client_env,
            &address,
            QUERY_INFO,
            config.query_timeout,
            config,
        )
        .await?;
        let subscription_url = query_url.replace("https://", "wss://").replace("http://", "ws://");
        let endpoint = Self {
            query_url,
            subscription_url,
            ip_address,
            server_time_delta: AtomicI64::default(),
            server_version: AtomicU32::default(),
            server_latency: AtomicU64::default(),
            next_latency_detection_time: AtomicU64::default(),
            remp_enabled: AtomicBool::default(),
        };
        endpoint.apply_server_info(client_env, config, info_request_time, &info)?;
        Ok(endpoint)
    }

    pub async fn refresh(
        &self,
        client_env: &ClientEnv,
        config: &NetworkConfig,
    ) -> ClientResult<()> {
        let info_request_time = client_env.now_ms();
        let (info, _, _) = Self::fetch_info_with_url(
            client_env,
            &self.query_url,
            QUERY_INFO,
            config.query_timeout,
            config,
        )
        .await?;
        self.apply_server_info(client_env, config, info_request_time, &info)?;
        Ok(())
    }

    pub fn apply_server_info(
        &self,
        client_env: &ClientEnv,
        config: &NetworkConfig,
        info_request_time: u64,
        info: &Value,
    ) -> ClientResult<()> {
        if let Some(version) = info["version"].as_str() {
            let mut parts: Vec<&str> = version.split('.').collect();
            parts.resize(3, "0");
            let parse_part = |i: usize| {
                parts[i].parse::<u32>().map_err(|err| {
                    Error::invalid_server_response(format!(
                        "Can not parse version {}: {}",
                        version, err
                    ))
                })
            };
            self.server_version.store(
                parse_part(0)? * 1000000 + parse_part(1)? * 1000 + parse_part(2)?,
                Ordering::Relaxed,
            );
        }
        if let Some(server_time) = info["time"].as_i64() {
            let now = client_env.now_ms();
            self.server_time_delta
                .store(server_time - ((info_request_time + now) / 2) as i64, Ordering::Relaxed);
            if let Some(latency) = info["latency"].as_i64() {
                self.server_latency.store(latency.unsigned_abs(), Ordering::Relaxed);
                self.next_latency_detection_time
                    .store(now + config.latency_detection_interval as u64, Ordering::Relaxed);
            }
        }
        self.remp_enabled
            .store(info["rempEnabled"].as_bool().unwrap_or_default(), Ordering::Relaxed);
        Ok(())
    }

    pub fn latency(&self) -> u64 {
        self.server_latency.load(Ordering::Relaxed)
    }

    pub fn next_latency_detection_time(&self) -> u64 {
        self.next_latency_detection_time.load(Ordering::Relaxed)
    }

    pub fn remp_enabled(&self) -> bool {
        self.remp_enabled.load(Ordering::Relaxed)
    }
}

#[test]
fn test_expand_address() {
    assert_eq!(Endpoint::expand_address("localhost"), "http://localhost/graphql");
    assert_eq!(Endpoint::expand_address("localhost:8033"), "http://localhost:8033/graphql");
    assert_eq!(Endpoint::expand_address("0.0.0.0:8033/graphql"), "http://0.0.0.0:8033/graphql");
    assert_eq!(Endpoint::expand_address("127.0.0.1/graphql"), "http://127.0.0.1/graphql");
    assert_eq!(Endpoint::expand_address("http://localhost/graphql"), "http://localhost/graphql");
    assert_eq!(Endpoint::expand_address("https://localhost"), "https://localhost/graphql");

    assert_eq!(
        Endpoint::expand_address("shellnet.ackinacki.org"),
        "https://shellnet.ackinacki.org/graphql"
    );
    assert_eq!(
        Endpoint::expand_address("shellnet.ackinacki.org:8033"),
        "https://shellnet.ackinacki.org:8033/graphql"
    );
    assert_eq!(
        Endpoint::expand_address("shellnet.ackinacki.org:8033/graphql"),
        "https://shellnet.ackinacki.org:8033/graphql"
    );
    assert_eq!(
        Endpoint::expand_address("shellnet.ackinacki.org/graphql"),
        "https://shellnet.ackinacki.org/graphql"
    );
    assert_eq!(
        Endpoint::expand_address("http://shellnet.ackinacki.org/graphql"),
        "http://shellnet.ackinacki.org/graphql"
    );
    assert_eq!(
        Endpoint::expand_address("https://shellnet.ackinacki.org"),
        "https://shellnet.ackinacki.org/graphql"
    );
}
