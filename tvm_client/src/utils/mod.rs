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
//

#[cfg(test)]
mod tests;

pub(crate) mod calc_storage_fee;
#[cfg(feature = "include-zstd")]
pub(crate) mod compression;
pub(crate) mod conversion;
mod errors;
pub(crate) mod json;

pub use calc_storage_fee::ParamsOfCalcStorageFee;
pub use calc_storage_fee::ResultOfCalcStorageFee;
pub use calc_storage_fee::calc_storage_fee;
#[cfg(feature = "include-zstd")]
pub use compression::compress_zstd;
#[cfg(feature = "include-zstd")]
pub use compression::decompress_zstd;
pub use conversion::AddressStringFormat;
pub use conversion::ParamsOfConvertAddress;
pub use conversion::ParamsOfGetAddressType;
pub use conversion::ResultOfConvertAddress;
pub use conversion::ResultOfGetAddressType;
pub use conversion::convert_address;
pub use conversion::get_address_type;
pub use errors::Error;
pub use errors::ErrorCode;

pub use crate::encoding::AccountAddressType;
