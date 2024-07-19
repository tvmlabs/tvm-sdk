use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.dataList`\n\n```text\ntonNode.dataList data:(vector bytes) = tonNode.DataList;\n```\n"]
pub struct DataList {
    pub data: crate::ton::vector<crate::ton::bytes>,
}
impl Eq for DataList {}
impl crate::BareSerialize for DataList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x14f43313)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataList { data } = self;
        (data as &dyn crate::ton::VectoredBare<crate::ton::bytes>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DataList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = < Vec < crate :: ton :: bytes > as crate :: ton :: VectoredBare < crate :: ton :: bytes >> :: deserialize (_de) ? ;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for DataList {
    type Boxed = crate::ton::ton_node::DataList;

    fn into_boxed(self) -> crate::ton::ton_node::DataList {
        crate::ton::ton_node::DataList::TonNode_DataList(self)
    }
}
