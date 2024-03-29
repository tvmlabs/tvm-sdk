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

use sha2::Digest;

use crate::client::ClientContext;
use crate::encoding::base64_decode;
use crate::error::ClientResult;

//--------------------------------------------------------------------------------------------- sha

#[derive(Serialize, Deserialize, ApiType, Default)]
pub struct ParamsOfHash {
    /// Input data for hash calculation. Encoded with `base64`.
    pub data: String,
}

#[derive(Serialize, Deserialize, ApiType, Default)]
pub struct ResultOfHash {
    /// Hash of input `data`. Encoded with 'hex'.
    pub hash: String,
}

/// Calculates SHA256 hash of the specified data.
#[api_function]
pub fn sha256(
    _context: std::sync::Arc<ClientContext>,
    params: ParamsOfHash,
) -> ClientResult<ResultOfHash> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(base64_decode(&params.data)?);
    Ok(ResultOfHash { hash: hex::encode(hasher.finalize()) })
}

/// Calculates SHA512 hash of the specified data.
#[api_function]
pub fn sha512(
    _context: std::sync::Arc<ClientContext>,
    params: ParamsOfHash,
) -> ClientResult<ResultOfHash> {
    let mut hasher = sha2::Sha512::new();
    hasher.update(base64_decode(&params.data)?);
    Ok(ResultOfHash { hash: hex::encode(hasher.finalize()) })
}
