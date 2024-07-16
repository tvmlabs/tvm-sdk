use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.config.global`\n\n```text\ncatchain.config.global tag:int256 nodes:(vector PublicKey) = catchain.config.Global;\n```\n"]
pub struct Global {
    pub tag: crate::ton::int256,
    pub nodes: crate::ton::vector<crate::ton::Boxed, crate::ton::PublicKey>,
}
impl Eq for Global {}
impl crate::BareSerialize for Global {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x68c7b651)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Global { tag, nodes } = self;
        _ser.write_bare::<crate::ton::int256>(tag)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::PublicKey>>(nodes)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Global {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let tag = _de.read_bare::<crate::ton::int256>()?;
            let nodes =
                _de.read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::PublicKey>>()?;
            Ok(Self { tag, nodes })
        }
    }
}
impl crate::IntoBoxed for Global {
    type Boxed = crate::ton::catchain::config::Global;

    fn into_boxed(self) -> crate::ton::catchain::config::Global {
        crate::ton::catchain::config::Global::Catchain_Config_Global(self)
    }
}
