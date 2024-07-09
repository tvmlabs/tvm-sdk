use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pk.aes`\n\n```text\npk.aes key:int256 = PrivateKey;\n```\n"]
pub struct Aes {
    pub key: crate::ton::int256,
}
impl Eq for Aes {}
impl crate::BareSerialize for Aes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa5e85137)
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
    type Boxed = crate::ton::PrivateKey;
    fn into_boxed(self) -> crate::ton::PrivateKey {
        crate::ton::PrivateKey::Pk_Aes(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pk.ed25519`\n\n```text\npk.ed25519 key:int256 = PrivateKey;\n```\n"]
pub struct Ed25519 {
    pub key: crate::ton::int256,
}
impl Eq for Ed25519 {}
impl crate::BareSerialize for Ed25519 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x49682317)
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
    type Boxed = crate::ton::PrivateKey;
    fn into_boxed(self) -> crate::ton::PrivateKey {
        crate::ton::PrivateKey::Pk_Ed25519(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pk.overlay`\n\n```text\npk.overlay name:bytes = PrivateKey;\n```\n"]
pub struct Overlay {
    pub name: crate::ton::bytes,
}
impl Eq for Overlay {}
impl crate::BareSerialize for Overlay {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x37a5f65b)
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
    type Boxed = crate::ton::PrivateKey;
    fn into_boxed(self) -> crate::ton::PrivateKey {
        crate::ton::PrivateKey::Pk_Overlay(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pk.unenc`\n\n```text\npk.unenc data:bytes = PrivateKey;\n```\n"]
pub struct Unenc {
    pub data: crate::ton::bytes,
}
impl Eq for Unenc {}
impl crate::BareSerialize for Unenc {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb1db9b30)
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
    type Boxed = crate::ton::PrivateKey;
    fn into_boxed(self) -> crate::ton::PrivateKey {
        crate::ton::PrivateKey::Pk_Unenc(self)
    }
}
