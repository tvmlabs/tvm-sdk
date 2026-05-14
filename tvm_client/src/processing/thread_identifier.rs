// 2022-2024 (c) Copyright Contributors to the GOSH DAO. All rights reserved.
//

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ThreadIdentifier([u8; 34]);

impl Serialize for ThreadIdentifier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for ThreadIdentifier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct BytesVisitor;
        impl<'de> serde::de::Visitor<'de> for BytesVisitor {
            type Value = ThreadIdentifier;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "34 bytes")
            }

            fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                let arr: [u8; 34] =
                    v.try_into().map_err(|_| E::invalid_length(v.len(), &"34 bytes"))?;
                Ok(ThreadIdentifier(arr))
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let mut arr = [0u8; 34];
                for (i, byte) in arr.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &"34 bytes"))?;
                }
                Ok(ThreadIdentifier(arr))
            }
        }
        deserializer.deserialize_bytes(BytesVisitor)
    }
}

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

impl TryFrom<String> for ThreadIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let array: [u8; 34] = hex::decode(value)
            .map_err(|_| anyhow::anyhow!("Failed to decode ThreadIdentifier from hex string"))?
            .try_into()
            .map_err(|v: Vec<u8>| {
                anyhow::anyhow!("Expected a Vec of length 34 but got length {}", v.len())
            })?;
        Ok(Self(array))
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
