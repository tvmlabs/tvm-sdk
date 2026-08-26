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
use tvm_block::ComputeSkipReason;
use tvm_types::types::ExceptionCode;
use tvm_vm::stack::StackItem;

#[derive(Debug, Error, PartialEq)]
pub enum ExecutorError {
    #[error("Invalid external message")]
    InvalidExtMessage,
    #[error("Transaction executor internal error: {0}")]
    TrExecutorError(String),
    #[error("VM Exception, code: {0}")]
    TvmExceptionCode(ExceptionCode),
    #[error("Contract did not accept message, exit code: {0}")]
    NoAcceptError(i32, Option<StackItem>),
    #[error("Cannot pay for importing this external message")]
    NoFundsToImportMsg,
    #[error("Compute phase skipped while processing external inbound message with reason {0:?}")]
    ExtMsgComputeSkipped(ComputeSkipReason),
    #[error(
        "Compute phase terminated due to deadline reached (transaction execution takes too long time)"
    )]
    TerminationDeadlineReached,
}

#[cfg(test)]
mod tests {
    use tvm_block::ComputeSkipReason;
    use tvm_types::types::ExceptionCode;

    use super::ExecutorError;

    // See the note in `tvm_block::error`: a `#[error]` attribute whose trailing
    // argument is a bare `0` interpolates that integer literal, not the
    // variant's field.
    // Here that hid every exit code a contract ever refused a message with.

    #[test]
    fn no_accept_error_carries_the_exit_code() {
        let error = ExecutorError::NoAcceptError(60, None);
        assert_eq!(error.to_string(), "Contract did not accept message, exit code: 60");
    }

    #[test]
    fn tvm_exception_code_error_names_the_exception() {
        let error = ExecutorError::TvmExceptionCode(ExceptionCode::StackUnderflow);
        assert_eq!(
            error.to_string(),
            format!("VM Exception, code: {}", ExceptionCode::StackUnderflow)
        );
    }

    #[test]
    fn compute_skipped_error_names_the_reason() {
        let error = ExecutorError::ExtMsgComputeSkipped(ComputeSkipReason::NoGas);
        assert_eq!(
            error.to_string(),
            "Compute phase skipped while processing external inbound message with reason NoGas"
        );
    }
}
