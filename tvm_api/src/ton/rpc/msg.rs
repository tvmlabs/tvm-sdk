use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.decrypt`\n\n```text\nmsg.decrypt input_key:InputKey data:msg.dataEncryptedArray = msg.DataDecryptedArray;\n```\n"]
pub struct Decrypt {
    pub input_key: crate::ton::InputKey,
    pub data: crate::ton::msg::dataencryptedarray::DataEncryptedArray,
}
impl Eq for Decrypt {}
impl crate::BareSerialize for Decrypt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0d53cf09)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Decrypt { input_key, data } = self;
        _ser.write_boxed::<crate::ton::InputKey>(input_key)?;
        _ser.write_bare::<crate::ton::msg::dataencryptedarray::DataEncryptedArray>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Decrypt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let input_key = _de.read_boxed::<crate::ton::InputKey>()?;
            let data =
                _de.read_bare::<crate::ton::msg::dataencryptedarray::DataEncryptedArray>()?;
            Ok(Self { input_key, data })
        }
    }
}
impl crate::BoxedDeserialize for Decrypt {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0d53cf09)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x0d53cf09) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Decrypt {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x0d53cf09), self)
    }
}
impl crate::Function for Decrypt {
    type Reply = crate::ton::msg::DataDecryptedArray;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `msg.decryptWithProof`\n\n```text\nmsg.decryptWithProof proof:bytes data:msg.dataEncrypted = msg.Data;\n```\n"]
pub struct DecryptWithProof {
    pub proof: crate::ton::bytes,
    pub data: crate::ton::msg::dataencrypted::DataEncrypted,
}
impl Eq for DecryptWithProof {}
impl crate::BareSerialize for DecryptWithProof {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8222c881)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DecryptWithProof { proof, data } = self;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        _ser.write_bare::<crate::ton::msg::dataencrypted::DataEncrypted>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DecryptWithProof {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            let data = _de.read_bare::<crate::ton::msg::dataencrypted::DataEncrypted>()?;
            Ok(Self { proof, data })
        }
    }
}
impl crate::BoxedDeserialize for DecryptWithProof {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8222c881)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x8222c881) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DecryptWithProof {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x8222c881), self)
    }
}
impl crate::Function for DecryptWithProof {
    type Reply = crate::ton::msg::Data;
}
