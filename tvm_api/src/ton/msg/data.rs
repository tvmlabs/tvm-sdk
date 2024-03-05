use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.dataDecryptedText`\n\n```text\nmsg.dataDecryptedText text:bytes = msg.Data;\n```\n"]
pub struct DataDecryptedText {
    pub text: crate::ton::bytes,
}
impl Eq for DataDecryptedText {}
impl crate::BareSerialize for DataDecryptedText {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb32960b9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataDecryptedText { text } = self;
        _ser.write_bare::<crate::ton::bytes>(text)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DataDecryptedText {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let text = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { text })
        }
    }
}
impl crate::IntoBoxed for DataDecryptedText {
    type Boxed = crate::ton::msg::Data;

    fn into_boxed(self) -> crate::ton::msg::Data {
        crate::ton::msg::Data::Msg_DataDecryptedText(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.dataEncryptedText`\n\n```text\nmsg.dataEncryptedText text:bytes = msg.Data;\n```\n"]
pub struct DataEncryptedText {
    pub text: crate::ton::bytes,
}
impl Eq for DataEncryptedText {}
impl crate::BareSerialize for DataEncryptedText {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xee520bda)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataEncryptedText { text } = self;
        _ser.write_bare::<crate::ton::bytes>(text)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DataEncryptedText {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let text = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { text })
        }
    }
}
impl crate::IntoBoxed for DataEncryptedText {
    type Boxed = crate::ton::msg::Data;

    fn into_boxed(self) -> crate::ton::msg::Data {
        crate::ton::msg::Data::Msg_DataEncryptedText(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.dataRaw`\n\n```text\nmsg.dataRaw body:bytes init_state:bytes = msg.Data;\n```\n"]
pub struct DataRaw {
    pub body: crate::ton::bytes,
    pub init_state: crate::ton::bytes,
}
impl Eq for DataRaw {}
impl crate::BareSerialize for DataRaw {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8d065d76)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataRaw { body, init_state } = self;
        _ser.write_bare::<crate::ton::bytes>(body)?;
        _ser.write_bare::<crate::ton::bytes>(init_state)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DataRaw {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let body = _de.read_bare::<crate::ton::bytes>()?;
            let init_state = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { body, init_state })
        }
    }
}
impl crate::IntoBoxed for DataRaw {
    type Boxed = crate::ton::msg::Data;

    fn into_boxed(self) -> crate::ton::msg::Data {
        crate::ton::msg::Data::Msg_DataRaw(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.dataText`\n\n```text\nmsg.dataText text:bytes = msg.Data;\n```\n"]
pub struct DataText {
    pub text: crate::ton::bytes,
}
impl Eq for DataText {}
impl crate::BareSerialize for DataText {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xeba43290)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DataText { text } = self;
        _ser.write_bare::<crate::ton::bytes>(text)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DataText {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let text = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { text })
        }
    }
}
impl crate::IntoBoxed for DataText {
    type Boxed = crate::ton::msg::Data;

    fn into_boxed(self) -> crate::ton::msg::Data {
        crate::ton::msg::Data::Msg_DataText(self)
    }
}
