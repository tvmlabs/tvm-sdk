use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.data`\n\n```text\ntonNode.data data:bytes = tonNode.Data;\n```\n"]
pub struct Data {
    pub data: crate::ton::bytes,
}
impl Eq for Data {}
impl crate::BareSerialize for Data {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x560a2484)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Data { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Data {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for Data {
    type Boxed = crate::ton::ton_node::Data;

    fn into_boxed(self) -> crate::ton::ton_node::Data {
        crate::ton::ton_node::Data::TonNode_Data(self)
    }
}
