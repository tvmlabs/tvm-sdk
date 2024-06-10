use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.dataEncryptedArray`\n\n```text\nmsg.dataEncryptedArray elements:vector<msg.dataEncrypted> = msg.DataEncryptedArray;\n```\n"]
pub struct DataEncryptedArray {
    pub elements:
        crate::ton::vector<crate::ton::Bare, crate::ton::msg::dataencrypted::DataEncrypted>,
}
impl Eq for DataEncryptedArray {}
impl crate::BareSerialize for DataEncryptedArray {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x244759b2)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataEncryptedArray { elements } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: msg :: dataencrypted :: DataEncrypted > > (elements) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for DataEncryptedArray {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let elements = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::msg::dataencrypted::DataEncrypted,
            >>()?;
            Ok(Self { elements })
        }
    }
}
impl crate::IntoBoxed for DataEncryptedArray {
    type Boxed = crate::ton::msg::DataEncryptedArray;
    fn into_boxed(self) -> crate::ton::msg::DataEncryptedArray {
        crate::ton::msg::DataEncryptedArray::Msg_DataEncryptedArray(self)
    }
}
