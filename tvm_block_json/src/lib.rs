// Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.  You may obtain a copy
// of the License at:
//
// https://www.ton.dev/licenses
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

mod serialize;
pub use self::serialize::*;
mod block_parser;
mod deserialize;

pub use block_parser::*;

pub use self::deserialize::*;

pub fn build_commit() -> Option<&'static str> {
    std::option_env!("BUILD_GIT_COMMIT")
}
