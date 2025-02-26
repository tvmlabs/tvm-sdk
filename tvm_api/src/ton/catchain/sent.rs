use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.sent`\n\n```text\ncatchain.sent cnt:int = catchain.Sent;\n```\n"]
pub struct Sent {
    pub cnt: crate::ton::int,
}
impl Eq for Sent {}
impl crate::BareSerialize for Sent {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfaf751af)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Sent { cnt } = self;
        _ser.write_bare::<crate::ton::int>(cnt)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Sent {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let cnt = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { cnt })
        }
    }
}
impl crate::IntoBoxed for Sent {
    type Boxed = crate::ton::catchain::Sent;

    fn into_boxed(self) -> crate::ton::catchain::Sent {
        crate::ton::catchain::Sent::Catchain_Sent(self)
    }
}
