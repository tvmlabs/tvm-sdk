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

#![cfg_attr(feature = "ci_run", deny(warnings))]

pub mod transaction_executor;
pub use transaction_executor::*;

pub mod ordinary_transaction;
pub use ordinary_transaction::OrdinaryTransactionExecutor;

pub mod tick_tock_transaction;
pub use tick_tock_transaction::TickTockTransactionExecutor;

#[macro_use]
pub mod error;
pub use error::*;

pub mod vmsetup;
pub use vmsetup::*;

pub mod blockchain_config;
pub use blockchain_config::*;

pub fn build_commit() -> Option<&'static str> {
    std::option_env!("BUILD_GIT_COMMIT")
}
