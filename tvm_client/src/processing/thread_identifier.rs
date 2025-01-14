// 2022-2024 (c) Copyright Contributors to the GOSH DAO. All rights reserved.
//

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;
use serde_with::Bytes;
use serde_with::serde_as;

#[serde_as]
#[derive(Copy, Clone, Eq, Hash, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ThreadIdentifier(#[serde_as(as = "Bytes")] [u8; 34]);

impl Default for ThreadIdentifier {
    fn default() -> Self {
        Self([0; 34])
    }
}

impl From<[u8; 34]> for ThreadIdentifier {
    fn from(array: [u8; 34]) -> Self {
        Self(array)
    }
}

impl TryFrom<std::string::String> for ThreadIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match hex::decode(value) {
            Ok(array) => {
                let boxed_slice = array.into_boxed_slice();
                let boxed_array: Box<[u8; 34]> = match boxed_slice.try_into() {
                    Ok(array) => array,
                    Err(e) => anyhow::bail!("Expected a Vec of length 34 but it was {}", e.len()),
                };
                Ok(Self(*boxed_array))
            }
            Err(_) => anyhow::bail!("Failed to convert to ThreadIdentifier"),
        }
    }
}

impl Display for ThreadIdentifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", hex::encode(self.0))
    }
}
impl Debug for ThreadIdentifier {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "ThreadIdentifier<{}>", hex::encode(self.0))
    }
}

impl AsRef<[u8]> for ThreadIdentifier {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
