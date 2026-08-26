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
    #[error("Fatal error: {0}")]
    FatalError(String),
    /// Invalid argument.
    #[error("Invalid argument: {0}")]
    InvalidArg(String),
    /// Invalid TL-B constructor tag.
    #[error("Invalid TL-B constructor tag `#{:x}` while parsing `{}` struct", t, s)]
    InvalidConstructorTag { t: u32, s: String },
    /// Invalid data.
    #[error("Invalid data: {0}")]
    InvalidData(String),
    /// Invalid index.
    #[error("Invalid index: {0}")]
    InvalidIndex(usize),
    /// Invalid operation.
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    /// Item is not found.
    #[error("{0} is not found")]
    NotFound(String),
    /// Other error.
    #[error("{0}")]
    Other(String),
    /// Attempting to read data from pruned branch cell.
    #[error("Attempting to read {0} from pruned branch cell")]
    PrunedCellAccess(String),
    /// Wrong hash.
    #[error("Wrong hash")]
    WrongHash,
    /// Wrong merkle proof.
    #[error("Wrong merkle proof: {0}")]
    WrongMerkleProof(String),
    /// Wrong merkle update.
    #[error("Wrong merkle update: {0}")]
    WrongMerkleUpdate(String),
    #[error("Bad signature")]
    BadSignature,
    #[error("External cell read")]
    ExternalCellRead,
}

#[cfg(test)]
mod tests {
    use super::BlockError;

    // A `#[error]` attribute whose trailing argument is a bare `0` interpolates
    // the integer literal, not the variant's field: it compiles, and every
    // reason the variant was built with is replaced by a zero on the way out.
    // These tests pin the payload to the rendered message so that shape cannot
    // come back.

    #[test]
    fn invalid_arg_error_carries_its_reason() {
        let error = BlockError::InvalidArg("workchain_id is not correct number".to_string());
        assert_eq!(error.to_string(), "Invalid argument: workchain_id is not correct number");
    }

    #[test]
    fn other_error_is_exactly_its_message() {
        let error = BlockError::Other("something went wrong".to_string());
        assert_eq!(error.to_string(), "something went wrong");
    }

    #[test]
    fn not_found_error_names_the_missing_item() {
        let error = BlockError::NotFound("config param 34".to_string());
        assert_eq!(error.to_string(), "config param 34 is not found");
    }
}
