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

use serde::Deserialize;
use serde::Deserializer;

pub(crate) mod blockchain_config;
pub(crate) mod cache;
pub(crate) mod common;
pub(crate) mod encode;
mod errors;
pub mod internal;
pub(crate) mod parse;
pub(crate) mod state_init;

pub(crate) mod encode_external_in_message;
#[cfg(test)]
pub(crate) mod tests;
pub(crate) mod tvc;

pub use blockchain_config::get_blockchain_config;
pub use blockchain_config::ParamsOfGetBlockchainConfig;
pub use blockchain_config::ResultOfGetBlockchainConfig;
pub use cache::cache_get;
pub use cache::cache_set;
pub use cache::cache_unpin;
pub use cache::BocCacheType;
pub use cache::CachedBoc;
pub use cache::ParamsOfBocCacheGet;
pub use cache::ParamsOfBocCacheSet;
pub use cache::ParamsOfBocCacheUnpin;
pub use cache::ResultOfBocCacheGet;
pub use cache::ResultOfBocCacheSet;
pub use common::get_boc_depth;
pub use common::get_boc_hash;
pub use common::ParamsOfGetBocDepth;
pub use common::ParamsOfGetBocHash;
pub use common::ResultOfGetBocDepth;
pub use common::ResultOfGetBocHash;
pub use encode::encode_boc;
pub use encode::BuilderOp;
pub use encode::ParamsOfEncodeBoc;
pub use encode::ResultOfEncodeBoc;
pub use encode_external_in_message::encode_external_in_message;
pub use encode_external_in_message::ParamsOfEncodeExternalInMessage;
pub use encode_external_in_message::ResultOfEncodeExternalInMessage;
pub use errors::Error;
pub use errors::ErrorCode;
pub use parse::parse_account;
pub use parse::parse_block;
pub use parse::parse_message;
pub use parse::parse_shardstate;
pub use parse::parse_transaction;
pub use parse::required_boc;
pub use parse::source_boc;
pub use parse::ParamsOfParse;
pub use parse::ParamsOfParseShardstate;
pub use parse::ResultOfParse;
pub use state_init::decode_state_init;
pub use state_init::encode_state_init;
pub use state_init::get_code_from_tvc;
pub use state_init::get_code_salt;
pub use state_init::get_compiler_version;
pub use state_init::get_compiler_version_from_cell;
pub use state_init::set_code_salt;
pub use state_init::ParamsOfDecodeStateInit;
pub use state_init::ParamsOfEncodeStateInit;
pub use state_init::ParamsOfGetCodeFromTvc;
pub use state_init::ParamsOfGetCodeSalt;
pub use state_init::ParamsOfGetCompilerVersion;
pub use state_init::ParamsOfSetCodeSalt;
pub use state_init::ResultOfDecodeStateInit;
pub use state_init::ResultOfEncodeStateInit;
pub use state_init::ResultOfGetCodeFromTvc;
pub use state_init::ResultOfGetCodeSalt;
pub use state_init::ResultOfGetCompilerVersion;
pub use state_init::ResultOfSetCodeSalt;
pub use tvc::decode_tvc;
pub use tvc::Tvc;
pub use tvc::TvcV1;

pub fn default_cache_max_size() -> u32 {
    10 * 1024 // * 1024 = 10 MB
}

fn deserialize_cache_max_size<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    Ok(Option::deserialize(deserializer)?.unwrap_or(default_cache_max_size()))
}

#[derive(Deserialize, Serialize, Debug, Clone, ApiType)]
pub struct BocConfig {
    /// Maximum BOC cache size in kilobytes. Default is 10 MB
    #[serde(default = "default_cache_max_size", deserialize_with = "deserialize_cache_max_size")]
    pub cache_max_size: u32,
}

impl Default for BocConfig {
    fn default() -> Self {
        Self { cache_max_size: default_cache_max_size() }
    }
}
