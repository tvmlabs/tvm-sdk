#[cfg(feature = "acki-nacki")]
use chitchat::{ChitchatId, ClusterStateSnapshot};
use serde_derive::{Deserialize, Serialize};
use crate::config::Config;

#[cfg(feature = "acki-nacki")]
const ADDRESS_VALUE: &str = "public_endpoint";

#[cfg(feature = "acki-nacki")]
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub cluster_id: String,
    pub cluster_state: ClusterStateSnapshot,
    pub live_nodes: Vec<ChitchatId>,
    pub dead_nodes: Vec<ChitchatId>,
}

#[cfg(feature = "acki-nacki")]
pub async fn resolve_gossip_to_endpoints(config: &mut Config) -> anyhow::Result<()> {
    if config.gossip_seeds.is_empty() {
        return Ok(());
    }

    let mut endpoints = vec![];
    for gossip_seed in &config.gossip_seeds {
        eprintln!("resolve_gossip_to_endpoints {gossip_seed}");
        let network_response = match reqwest::get(gossip_seed).await {
            Ok(response) => match response.text().await {
                Ok(text) => text,
                _ => continue,
            },
            _ => continue,
        };
        let decoded: ApiResponse = match serde_json::from_str(&network_response) {
            Ok(value) => value,
            _ => continue,
        };
        eprintln!("resolve_gossip_to_endpoints {gossip_seed} successfully decoded response");
        for node_state in decoded.cluster_state.node_state_snapshots {
            for (key, value) in node_state.node_state.key_values() {
                if key == ADDRESS_VALUE {
                    endpoints.push(value.value.clone());
                }
            }
        }
        if !endpoints.is_empty() {
            eprintln!("set config endpoints: {endpoints:?}");
            config.url = "".to_string();
            config.endpoints = endpoints;
            return Ok(());
        }
    }
    Ok(())
}