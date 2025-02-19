use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.capabilities`\n\n```text\ntonNode.capabilities version:int capabilities:long = tonNode.Capabilities;\n```\n"]
pub struct Capabilities {
    pub version: crate::ton::int,
    pub capabilities: crate::ton::long,
}
impl Eq for Capabilities {}
impl crate::BareSerialize for Capabilities {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf5bf60c0)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Capabilities { version, capabilities } = self;
        _ser.write_bare::<crate::ton::int>(version)?;
        _ser.write_bare::<crate::ton::long>(capabilities)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Capabilities {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let version = _de.read_bare::<crate::ton::int>()?;
            let capabilities = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { version, capabilities })
        }
    }
}
impl crate::IntoBoxed for Capabilities {
    type Boxed = crate::ton::ton_node::Capabilities;

    fn into_boxed(self) -> crate::ton::ton_node::Capabilities {
        crate::ton::ton_node::Capabilities::TonNode_Capabilities(self)
    }
}
