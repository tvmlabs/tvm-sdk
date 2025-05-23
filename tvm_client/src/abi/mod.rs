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

#[cfg(test)]
mod tests;

pub(crate) mod decode_boc;
pub(crate) mod decode_data;
pub(crate) mod decode_message;
pub(crate) mod encode_account;
pub(crate) mod encode_boc;
pub(crate) mod encode_message;
pub(crate) mod function_id;
pub(crate) mod init_data;

mod errors;
mod internal;
mod signing;
mod types;

pub use decode_boc::ParamsOfDecodeBoc;
pub use decode_boc::ResultOfDecodeBoc;
pub use decode_boc::decode_boc;
pub use decode_data::ParamsOfDecodeAccountData;
pub use decode_data::ResultOfDecodeAccountData;
pub use decode_data::decode_account_data;
pub use decode_message::DataLayout;
pub use decode_message::DecodedMessageBody;
pub use decode_message::MessageBodyType;
pub use decode_message::ParamsOfDecodeMessage;
pub use decode_message::ParamsOfDecodeMessageBody;
pub use decode_message::ParamsOfGetSignatureData;
pub use decode_message::ResultOfGetSignatureData;
pub use decode_message::decode_message;
pub use decode_message::decode_message_body;
pub use decode_message::get_signature_data;
pub use encode_account::ParamsOfEncodeAccount;
pub use encode_account::ResultOfEncodeAccount;
pub use encode_account::encode_account;
pub use encode_boc::ParamsOfAbiEncodeBoc;
pub use encode_boc::ResultOfAbiEncodeBoc;
pub use encode_boc::encode_boc;
pub use encode_message::CallSet;
pub use encode_message::DeploySet;
pub use encode_message::ParamsOfAttachSignature;
pub use encode_message::ParamsOfAttachSignatureToMessageBody;
pub use encode_message::ParamsOfEncodeInternalMessage;
pub use encode_message::ParamsOfEncodeMessage;
pub use encode_message::ParamsOfEncodeMessageBody;
pub use encode_message::ResultOfAttachSignature;
pub use encode_message::ResultOfAttachSignatureToMessageBody;
pub use encode_message::ResultOfEncodeInternalMessage;
pub use encode_message::ResultOfEncodeMessage;
pub use encode_message::ResultOfEncodeMessageBody;
pub use encode_message::attach_signature;
pub use encode_message::attach_signature_to_message_body;
pub use encode_message::encode_internal_message;
pub use encode_message::encode_message;
pub use encode_message::encode_message_body;
pub use errors::Error;
pub use errors::ErrorCode;
pub use function_id::ParamsOfCalcFunctionId;
pub use function_id::ResultOfCalcFunctionId;
pub use function_id::calc_function_id;
pub use init_data::ParamsOfDecodeInitialData;
pub use init_data::ParamsOfEncodeInitialData;
pub use init_data::ParamsOfUpdateInitialData;
pub use init_data::ResultOfDecodeInitialData;
pub use init_data::ResultOfEncodeInitialData;
pub use init_data::ResultOfUpdateInitialData;
pub use init_data::decode_initial_data;
pub use init_data::encode_initial_data;
pub use init_data::update_initial_data;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
pub use signing::Signer;
pub use types::Abi;
pub use types::AbiContract;
pub use types::AbiData;
pub use types::AbiEvent;
pub use types::AbiFunction;
pub use types::AbiHandle;
pub use types::AbiParam;
pub use types::FunctionHeader;

pub fn default_workchain() -> i32 {
    0
}

pub fn default_message_expiration_timeout() -> u32 {
    40000
}

pub fn default_message_expiration_timeout_grow_factor() -> f32 {
    1.5
}

fn deserialize_workchain<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i32, D::Error> {
    Ok(Option::deserialize(deserializer)?.unwrap_or(default_workchain()))
}

fn deserialize_message_expiration_timeout<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u32, D::Error> {
    Ok(Option::deserialize(deserializer)?.unwrap_or(default_message_expiration_timeout()))
}

fn deserialize_message_expiration_timeout_grow_factor<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<f32, D::Error> {
    Ok(Option::deserialize(deserializer)?
        .unwrap_or(default_message_expiration_timeout_grow_factor()))
}

#[derive(Deserialize, Serialize, Debug, Clone, ApiType)]
pub struct AbiConfig {
    /// Workchain id that is used by default in DeploySet
    #[serde(default = "default_workchain", deserialize_with = "deserialize_workchain")]
    pub workchain: i32,

    /// Message lifetime for contracts which ABI includes "expire" header.
    ///
    /// Must be specified in milliseconds. Default is 40000 (40 sec).
    #[serde(
        default = "default_message_expiration_timeout",
        deserialize_with = "deserialize_message_expiration_timeout"
    )]
    pub message_expiration_timeout: u32,

    /// Factor that increases the expiration timeout for each retry
    ///
    /// Default is 1.5
    #[serde(
        default = "default_message_expiration_timeout_grow_factor",
        deserialize_with = "deserialize_message_expiration_timeout_grow_factor"
    )]
    pub message_expiration_timeout_grow_factor: f32,
}

impl Default for AbiConfig {
    fn default() -> Self {
        Self {
            workchain: default_workchain(),
            message_expiration_timeout: default_message_expiration_timeout(),
            message_expiration_timeout_grow_factor: default_message_expiration_timeout_grow_factor(
            ),
        }
    }
}
