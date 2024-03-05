// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockError {
    /// Fatal error.
    #[error("Fatal error: {}", 0)]
    FatalError(String),
    /// Invalid argument.
    #[error("Invalid argument: {}", 0)]
    InvalidArg(String),
    /// Invalid TL-B constructor tag.
    #[error("Invalid TL-B constructor tag `#{:x}` while parsing `{}` struct", t, s)]
    InvalidConstructorTag { t: u32, s: String },
    /// Invalid data.
    #[error("Invalid data: {}", 0)]
    InvalidData(String),
    /// Invalid index.
    #[error("Invalid index: {}", 0)]
    InvalidIndex(usize),
    /// Invalid operation.
    #[error("Invalid operation: {}", 0)]
    InvalidOperation(String),
    /// Item is not found.
    #[error("{} is not found", 0)]
    NotFound(String),
    /// Other error.
    #[error("{}", 0)]
    Other(String),
    /// Attempting to read data from pruned branch cell.
    #[error("Attempting to read {} from pruned branch cell", 0)]
    PrunedCellAccess(String),
    /// Wrong hash.
    #[error("Wrong hash")]
    WrongHash,
    /// Wrong merkle proof.
    #[error("Wrong merkle proof: {}", 0)]
    WrongMerkleProof(String),
    /// Wrong merkle update.
    #[error("Wrong merkle update: {}", 0)]
    WrongMerkleUpdate(String),
    #[error("Bad signature")]
    BadSignature,
}
