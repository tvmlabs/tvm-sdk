use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.dataEncrypted`\n\n```text\nmsg.dataEncrypted source:accountAddress data:msg.Data = msg.DataEncrypted;\n```\n"]
pub struct DataEncrypted {
    pub source: crate::ton::accountaddress::AccountAddress,
    pub data: crate::ton::msg::Data,
}
impl Eq for DataEncrypted {}
impl crate::BareSerialize for DataEncrypted {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x21a13d51)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataEncrypted { source, data } = self;
        _ser.write_bare::<crate::ton::accountaddress::AccountAddress>(source)?;
        _ser.write_boxed::<crate::ton::msg::Data>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DataEncrypted {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let source = _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?;
            let data = _de.read_boxed::<crate::ton::msg::Data>()?;
            Ok(Self { source, data })
        }
    }
}
impl crate::IntoBoxed for DataEncrypted {
    type Boxed = crate::ton::msg::DataEncrypted;

    fn into_boxed(self) -> crate::ton::msg::DataEncrypted {
        crate::ton::msg::DataEncrypted::Msg_DataEncrypted(self)
    }
}
