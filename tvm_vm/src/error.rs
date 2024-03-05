// Copyright (C) 2019-2022 TON Labs. All Rights Reserved.
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
use tvm_types::error;
use tvm_types::fail;
use tvm_types::ExceptionCode;
use tvm_types::Result;

use crate::types::Exception;

#[derive(Debug, Error)]
pub enum TvmError {
    /// Fatal error.
    #[error("Fatal error: {}", 0)]
    FatalError(String),
    /// Invalid argument.
    #[error("Invalid argument: {}", 0)]
    InvalidArg(usize),
    /// Invalid data.
    #[error("Invalid data: {}", 0)]
    InvalidData(String),
    /// TVM Exception description
    #[error("VM Exception: {} {}", 0, 1)]
    TvmExceptionFull(Exception, String),
}

pub fn tvm_exception(err: anyhow::Error) -> Result<Exception> {
    match err.downcast::<TvmError>() {
        Ok(TvmError::TvmExceptionFull(err, _)) => Ok(err),
        Ok(err) => fail!(err),
        Err(err) => {
            if let Some(err) = err.downcast_ref::<tvm_types::types::ExceptionCode>() {
                Ok(Exception::from(*err))
            } else {
                Err(err)
            }
        }
    }
}

pub fn tvm_exception_code(err: &anyhow::Error) -> Option<ExceptionCode> {
    match err.downcast_ref::<TvmError>() {
        Some(TvmError::TvmExceptionFull(err, _)) => err.exception_code(),
        Some(_) => None,
        None => err.downcast_ref::<tvm_types::types::ExceptionCode>().cloned(),
    }
}

pub fn tvm_exception_or_custom_code(err: &anyhow::Error) -> i32 {
    match err.downcast_ref::<TvmError>() {
        Some(TvmError::TvmExceptionFull(err, _)) => err.exception_or_custom_code(),
        Some(_) => ExceptionCode::UnknownError as i32,
        None => {
            if let Some(err) = err.downcast_ref::<tvm_types::types::ExceptionCode>() {
                *err as i32
            } else {
                ExceptionCode::UnknownError as i32
            }
        }
    }
}

pub fn tvm_exception_full(err: &anyhow::Error) -> Option<Exception> {
    match err.downcast_ref::<TvmError>() {
        Some(TvmError::TvmExceptionFull(err, _)) => Some(err.clone()),
        Some(_) => None,
        None => err
            .downcast_ref::<tvm_types::types::ExceptionCode>()
            .map(|err| Exception::from_code(*err, file!(), line!())),
    }
}

pub fn update_error_description(
    mut err: anyhow::Error,
    f: impl FnOnce(&str) -> String,
) -> anyhow::Error {
    match err.downcast_mut::<TvmError>() {
        Some(TvmError::TvmExceptionFull(_err, descr)) => *descr = f(descr.as_str()),
        Some(_) => (),
        None => {
            if let Some(code) = err.downcast_ref::<tvm_types::ExceptionCode>() {
                // TODO: it is wrong, need to modify current backtrace
                err = TvmError::TvmExceptionFull(
                    Exception::from_code(*code, file!(), line!()),
                    f(&format!("{:?}", err)),
                )
                .into()
            }
        }
    }
    err
}

#[cfg(test)]
#[path = "tests/test_error.rs"]
mod tests;
