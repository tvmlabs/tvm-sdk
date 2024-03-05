use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.keyDescription`\n\n```text\ndht.keyDescription key:dht.key id:PublicKey update_rule:dht.UpdateRule signature:bytes = dht.KeyDescription;\n```\n"]
pub struct KeyDescription {
    pub key: crate::ton::dht::key::Key,
    pub id: crate::ton::PublicKey,
    pub update_rule: crate::ton::dht::UpdateRule,
    pub signature: crate::ton::bytes,
}
impl Eq for KeyDescription {}
impl crate::Signing for KeyDescription {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for KeyDescription {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x281d4e05)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let KeyDescription { key, id, update_rule, signature } = self;
        _ser.write_bare::<crate::ton::dht::key::Key>(key)?;
        _ser.write_boxed::<crate::ton::PublicKey>(id)?;
        _ser.write_boxed::<crate::ton::dht::UpdateRule>(update_rule)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for KeyDescription {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_bare::<crate::ton::dht::key::Key>()?;
            let id = _de.read_boxed::<crate::ton::PublicKey>()?;
            let update_rule = _de.read_boxed::<crate::ton::dht::UpdateRule>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { key, id, update_rule, signature })
        }
    }
}
impl crate::IntoBoxed for KeyDescription {
    type Boxed = crate::ton::dht::KeyDescription;

    fn into_boxed(self) -> crate::ton::dht::KeyDescription {
        crate::ton::dht::KeyDescription::Dht_KeyDescription(self)
    }
}
