use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `mbpp.newMcBlockSome`\n\n```text\nmbpp.newMcBlockSome block:tonNode.blockBroadcast = mbpp.NewMcBlock;\n```\n"]
pub struct NewMcBlockSome {
    pub block: crate::ton::ton_node::broadcast::BlockBroadcast,
}
impl Eq for NewMcBlockSome {}
impl crate::BareSerialize for NewMcBlockSome {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xde670c49)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let NewMcBlockSome { block } = self;
        _ser.write_bare::<crate::ton::ton_node::broadcast::BlockBroadcast>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for NewMcBlockSome {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::broadcast::BlockBroadcast>()?;
            Ok(Self { block })
        }
    }
}
impl crate::IntoBoxed for NewMcBlockSome {
    type Boxed = crate::ton::mbpp::NewMcBlock;
    fn into_boxed(self) -> crate::ton::mbpp::NewMcBlock {
        crate::ton::mbpp::NewMcBlock::Mbpp_NewMcBlockSome(self)
    }
}
