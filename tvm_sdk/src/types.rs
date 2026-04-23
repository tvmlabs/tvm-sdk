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

use std::fmt;

use num_traits::cast::ToPrimitive;
use tvm_types::Result;
use tvm_types::UInt256;
use tvm_types::base64_encode;

use crate::error::SdkError;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct StringId(String);

pub type BlockId = StringId;

impl From<UInt256> for StringId {
    fn from(id: UInt256) -> Self {
        StringId(id.as_hex_string())
    }
}

impl From<String> for StringId {
    fn from(id: String) -> Self {
        StringId(id)
    }
}

impl From<&str> for StringId {
    fn from(id: &str) -> Self {
        StringId(id.to_owned())
    }
}

impl From<Vec<u8>> for StringId {
    fn from(id: Vec<u8>) -> Self {
        StringId(hex::encode(id))
    }
}

impl From<&[u8]> for StringId {
    fn from(id: &[u8]) -> Self {
        StringId(hex::encode(id))
    }
}

impl fmt::Display for StringId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StringId {
    pub fn to_base64(&self) -> Result<String> {
        let bytes = self.to_bytes()?;
        Ok(base64_encode(bytes))
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        hex::decode(&self.0).map_err(Into::into)
    }
}

pub fn grams_to_u64(grams: &tvm_block::types::Grams) -> Result<u64> {
    grams.as_u128().to_u64().ok_or_else(|| {
        SdkError::InvalidData { msg: format!("Cannot convert grams value {}", grams) }.into()
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tvm_block::types::Grams;
    use tvm_types::UInt256;
    use tvm_types::base64_encode;

    use super::*;

    #[test]
    fn string_id_converts_from_strings_and_bytes() {
        assert_eq!(StringId::from("abcd").to_string(), "abcd");
        assert_eq!(StringId::from("abcd".to_owned()).to_string(), "abcd");
        assert_eq!(StringId::from(vec![0xab, 0xcd]).to_string(), "abcd");
        assert_eq!(StringId::from(&[0xab, 0xcd][..]).to_string(), "abcd");
    }

    #[test]
    fn string_id_decodes_hex_and_encodes_base64() {
        let id = StringId::from("abcd");

        assert_eq!(id.to_bytes().unwrap(), vec![0xab, 0xcd]);
        assert_eq!(id.to_base64().unwrap(), base64_encode([0xab, 0xcd]));
    }

    #[test]
    fn string_id_formats_uint256_as_hex() {
        let id = StringId::from(UInt256::from([0x11; 32]));

        assert_eq!(id.to_string(), "11".repeat(32));
    }

    #[test]
    fn string_id_reports_invalid_hex() {
        assert!(StringId::from("not-hex").to_bytes().is_err());
        assert!(StringId::from("not-hex").to_base64().is_err());
    }

    #[test]
    fn grams_to_u64_accepts_boundary_and_rejects_overflow() {
        assert_eq!(grams_to_u64(&Grams::from(u64::MAX)).unwrap(), u64::MAX);

        let overflow = Grams::from_str("18446744073709551616").unwrap();
        let err = grams_to_u64(&overflow).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid data: Cannot convert grams value 18446744073709551616"
        );
    }
}
