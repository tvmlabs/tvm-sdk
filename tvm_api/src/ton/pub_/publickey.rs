use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pub.aes`\n\n```text\npub.aes key:int256 = PublicKey;\n```\n"]
pub struct Aes {
    pub key: crate::ton::int256,
}
impl Eq for Aes {}
impl crate::BareSerialize for Aes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2dbcadd4)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Aes { key } = self;
        _ser.write_bare::<crate::ton::int256>(key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Aes {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key })
        }
    }
}
impl crate::IntoBoxed for Aes {
    type Boxed = crate::ton::PublicKey;
    fn into_boxed(self) -> crate::ton::PublicKey {
        crate::ton::PublicKey::Pub_Aes(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pub.bls`\n\n```text\npub.bls bls_key:bytes = PublicKey;\n```\n"]
pub struct Bls {
    pub bls_key: crate::ton::bytes,
}
impl Eq for Bls {}
impl crate::BareSerialize for Bls {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x99d1d8e9)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Bls { bls_key } = self;
        _ser.write_bare::<crate::ton::bytes>(bls_key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Bls {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let bls_key = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { bls_key })
        }
    }
}
impl crate::IntoBoxed for Bls {
    type Boxed = crate::ton::PublicKey;
    fn into_boxed(self) -> crate::ton::PublicKey {
        crate::ton::PublicKey::Pub_Bls(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pub.ed25519`\n\n```text\npub.ed25519 key:int256 = PublicKey;\n```\n"]
pub struct Ed25519 {
    pub key: crate::ton::int256,
}
impl Eq for Ed25519 {}
impl crate::BareSerialize for Ed25519 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4813b4c6)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Ed25519 { key } = self;
        _ser.write_bare::<crate::ton::int256>(key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Ed25519 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key })
        }
    }
}
impl crate::IntoBoxed for Ed25519 {
    type Boxed = crate::ton::PublicKey;
    fn into_boxed(self) -> crate::ton::PublicKey {
        crate::ton::PublicKey::Pub_Ed25519(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pub.overlay`\n\n```text\npub.overlay name:bytes = PublicKey;\n```\n"]
pub struct Overlay {
    pub name: crate::ton::bytes,
}
impl Eq for Overlay {}
impl crate::BareSerialize for Overlay {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x34ba45cb)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Overlay { name } = self;
        _ser.write_bare::<crate::ton::bytes>(name)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Overlay {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let name = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { name })
        }
    }
}
impl crate::IntoBoxed for Overlay {
    type Boxed = crate::ton::PublicKey;
    fn into_boxed(self) -> crate::ton::PublicKey {
        crate::ton::PublicKey::Pub_Overlay(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pub.unenc`\n\n```text\npub.unenc data:bytes = PublicKey;\n```\n"]
pub struct Unenc {
    pub data: crate::ton::bytes,
}
impl Eq for Unenc {}
impl crate::BareSerialize for Unenc {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb61f450a)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Unenc { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Unenc {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for Unenc {
    type Boxed = crate::ton::PublicKey;
    fn into_boxed(self) -> crate::ton::PublicKey {
        crate::ton::PublicKey::Pub_Unenc(self)
    }
}
