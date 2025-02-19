use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.dataDecryptedArray`\n\n```text\nmsg.dataDecryptedArray elements:vector<msg.dataDecrypted> = msg.DataDecryptedArray;\n```\n"]
pub struct DataDecryptedArray {
    pub elements:
        crate::ton::vector<crate::ton::Bare, crate::ton::msg::datadecrypted::DataDecrypted>,
}
impl Eq for DataDecryptedArray {}
impl crate::BareSerialize for DataDecryptedArray {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe35c4709)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataDecryptedArray { elements } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: msg :: datadecrypted :: DataDecrypted > > (elements) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for DataDecryptedArray {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let elements = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::msg::datadecrypted::DataDecrypted,
            >>()?;
            Ok(Self { elements })
        }
    }
}
impl crate::IntoBoxed for DataDecryptedArray {
    type Boxed = crate::ton::msg::DataDecryptedArray;

    fn into_boxed(self) -> crate::ton::msg::DataDecryptedArray {
        crate::ton::msg::DataDecryptedArray::Msg_DataDecryptedArray(self)
    }
}
