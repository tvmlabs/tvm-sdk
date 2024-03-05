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

pub mod contract;
pub mod error;
pub mod event;
pub mod function;
pub mod int;
pub mod json_abi;
pub mod param;
pub mod param_type;
pub mod token;

pub use contract::Contract;
pub use contract::DataItem;
pub use contract::PublicKeyData;
pub use contract::SignatureData;
pub use error::*;
pub use event::Event;
pub use function::Function;
pub use int::Int;
pub use int::Uint;
pub use json_abi::*;
pub use param::Param;
pub use param_type::ParamType;
pub use token::Token;
pub use token::TokenValue;

pub fn build_commit() -> Option<&'static str> {
    std::option_env!("BUILD_GIT_COMMIT")
}
