use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.dataDecrypted`\n\n```text\nmsg.dataDecrypted proof:bytes data:msg.Data = msg.DataDecrypted;\n```\n"]
pub struct DataDecrypted {
    pub proof: crate::ton::bytes,
    pub data: crate::ton::msg::Data,
}
impl Eq for DataDecrypted {}
impl crate::BareSerialize for DataDecrypted {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0ba960e9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataDecrypted { proof, data } = self;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        _ser.write_boxed::<crate::ton::msg::Data>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DataDecrypted {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            let data = _de.read_boxed::<crate::ton::msg::Data>()?;
            Ok(Self { proof, data })
        }
    }
}
impl crate::IntoBoxed for DataDecrypted {
    type Boxed = crate::ton::msg::DataDecrypted;

    fn into_boxed(self) -> crate::ton::msg::DataDecrypted {
        crate::ton::msg::DataDecrypted::Msg_DataDecrypted(self)
    }
}
