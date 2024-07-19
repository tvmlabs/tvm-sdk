// Copyright (C) 2019-2023 TON. All Rights Reserved.
//
// Licensed under the SOFTWARE EVALUATION License (the "License"); you may not
// use this file except in compliance with the License.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific TON DEV software governing permissions and
// limitations under the License.

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use ::secstr::*;

use crate::BareDeserialize;
use crate::BareSerialize;

/// SecureBytes built-in type.
#[derive(Clone, PartialEq)]
pub struct SecureBytes(SecVec<u8>);

impl SecureBytes {
    pub fn new(cont: Vec<u8>) -> Self {
        Self(SecVec::<u8>::new(cont))
    }

    /// Borrow the contents of the string.
    pub fn unsecure(&self) -> &[u8] {
        self.0.unsecure()
    }

    /// Mutably borrow the contents of the string.
    pub fn unsecure_mut(&mut self) -> &mut [u8] {
        self.0.unsecure_mut()
    }

    /// Overwrite the string with zeros. This is automatically called in the
    /// destructor.
    pub fn zero_out(&mut self) {
        self.0.zero_out()
    }
}

impl Default for SecureBytes {
    fn default() -> Self {
        Self::new(Vec::default())
    }
}

impl Debug for SecureBytes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for SecureBytes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Hash for SecureBytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.unsecure().hash(state);
    }
}

impl BareSerialize for SecureBytes {
    fn constructor(&self) -> crate::ConstructorNumber {
        unreachable!()
    }

    fn serialize_bare(&self, ser: &mut crate::Serializer) -> crate::Result<()> {
        self.0.unsecure().serialize_bare(ser)
    }
}

impl BareDeserialize for SecureBytes {
    fn deserialize_bare(de: &mut crate::Deserializer) -> crate::Result<Self> {
        Vec::<u8>::deserialize_bare(de).map(|vec| SecureBytes(SecVec::<u8>::new(vec)))
    }
}

/// SecureString built-in type.
pub type SecureString = SecureBytes;
