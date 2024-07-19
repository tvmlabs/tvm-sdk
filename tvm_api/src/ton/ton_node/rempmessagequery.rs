use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempMessageQuery`\n\n```text\ntonNode.rempMessageQuery message_id:int256 = tonNode.rempMessageQuery;\n```\n"]
pub struct RempMessageQuery {
    pub message_id: crate::ton::int256,
}
impl Eq for RempMessageQuery {}
impl crate::BareSerialize for RempMessageQuery {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x862a665a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempMessageQuery { message_id } = self;
        _ser.write_bare::<crate::ton::int256>(message_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempMessageQuery {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message_id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { message_id })
        }
    }
}
impl crate::IntoBoxed for RempMessageQuery {
    type Boxed = crate::ton::ton_node::RempMessageQuery;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageQuery {
        crate::ton::ton_node::RempMessageQuery::TonNode_RempMessageQuery(self)
    }
}
