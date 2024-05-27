use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.dataFull`\n\n```text\ntonNode.dataFull id:tonNode.blockIdExt proof:bytes block:bytes is_link:Bool = tonNode.DataFull;\n```\n"]
pub struct DataFull {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub proof: crate::ton::bytes,
    pub block: crate::ton::bytes,
    pub is_link: crate::ton::Bool,
}
impl Eq for DataFull {}
impl crate::BareSerialize for DataFull {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbe589f93)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataFull { id, proof, block, is_link } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        _ser.write_bare::<crate::ton::bytes>(block)?;
        _ser.write_boxed::<crate::ton::Bool>(is_link)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DataFull {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            let block = _de.read_bare::<crate::ton::bytes>()?;
            let is_link = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { id, proof, block, is_link })
        }
    }
}
impl crate::IntoBoxed for DataFull {
    type Boxed = crate::ton::ton_node::DataFull;

    fn into_boxed(self) -> crate::ton::ton_node::DataFull {
        crate::ton::ton_node::DataFull::TonNode_DataFull(self)
    }
}
