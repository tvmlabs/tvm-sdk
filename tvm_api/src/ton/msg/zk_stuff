use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `msg.Data`\n\n```text\nmsg.dataDecryptedText text:bytes = msg.Data;\n\nmsg.dataEncryptedText text:bytes = msg.Data;\n\nmsg.dataRaw body:bytes init_state:bytes = msg.Data;\n\nmsg.dataText text:bytes = msg.Data;\n```\n"]
pub enum Data {
    Msg_DataDecryptedText(crate::ton::msg::data::DataDecryptedText),
    Msg_DataEncryptedText(crate::ton::msg::data::DataEncryptedText),
    Msg_DataRaw(crate::ton::msg::data::DataRaw),
    Msg_DataText(crate::ton::msg::data::DataText),
}
impl Data {
    pub fn body(&self) -> Option<&crate::ton::bytes> {
        match self {
            Data::Msg_DataRaw(ref x) => Some(&x.body),
            _ => None,
        }
    }

    pub fn init_state(&self) -> Option<&crate::ton::bytes> {
        match self {
            Data::Msg_DataRaw(ref x) => Some(&x.init_state),
            _ => None,
        }
    }

    pub fn text(&self) -> Option<&crate::ton::bytes> {
        match self {
            Data::Msg_DataDecryptedText(ref x) => Some(&x.text),
            Data::Msg_DataEncryptedText(ref x) => Some(&x.text),
            Data::Msg_DataText(ref x) => Some(&x.text),
            _ => None,
        }
    }
}
impl Eq for Data {}
impl Default for Data {
    fn default() -> Self {
        Data::Msg_DataDecryptedText(crate::ton::msg::data::DataDecryptedText::default())
    }
}
impl crate::BoxedSerialize for Data {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Data::Msg_DataDecryptedText(x) => (crate::ConstructorNumber(0xb32960b9), x),
            Data::Msg_DataEncryptedText(x) => (crate::ConstructorNumber(0xee520bda), x),
            Data::Msg_DataRaw(x) => (crate::ConstructorNumber(0x8d065d76), x),
            Data::Msg_DataText(x) => (crate::ConstructorNumber(0xeba43290), x),
        }
    }
}
impl crate::BoxedDeserialize for Data {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xb32960b9),
            crate::ConstructorNumber(0xee520bda),
            crate::ConstructorNumber(0x8d065d76),
            crate::ConstructorNumber(0xeba43290),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb32960b9) => Ok(Data::Msg_DataDecryptedText(
                _de.read_bare::<crate::ton::msg::data::DataDecryptedText>()?,
            )),
            crate::ConstructorNumber(0xee520bda) => Ok(Data::Msg_DataEncryptedText(
                _de.read_bare::<crate::ton::msg::data::DataEncryptedText>()?,
            )),
            crate::ConstructorNumber(0x8d065d76) => {
                Ok(Data::Msg_DataRaw(_de.read_bare::<crate::ton::msg::data::DataRaw>()?))
            }
            crate::ConstructorNumber(0xeba43290) => {
                Ok(Data::Msg_DataText(_de.read_bare::<crate::ton::msg::data::DataText>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `msg.DataDecrypted`\n\n```text\nmsg.dataDecrypted proof:bytes data:msg.Data = msg.DataDecrypted;\n```\n"]
pub enum DataDecrypted {
    Msg_DataDecrypted(crate::ton::msg::datadecrypted::DataDecrypted),
}
impl DataDecrypted {
    pub fn data(&self) -> &crate::ton::msg::Data {
        match self {
            DataDecrypted::Msg_DataDecrypted(ref x) => &x.data,
        }
    }

    pub fn proof(&self) -> &crate::ton::bytes {
        match self {
            DataDecrypted::Msg_DataDecrypted(ref x) => &x.proof,
        }
    }

    pub fn only(self) -> crate::ton::msg::datadecrypted::DataDecrypted {
        match self {
            DataDecrypted::Msg_DataDecrypted(x) => x,
        }
    }
}
impl Eq for DataDecrypted {}
impl Default for DataDecrypted {
    fn default() -> Self {
        DataDecrypted::Msg_DataDecrypted(crate::ton::msg::datadecrypted::DataDecrypted::default())
    }
}
impl crate::BoxedSerialize for DataDecrypted {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DataDecrypted::Msg_DataDecrypted(x) => (crate::ConstructorNumber(0x0ba960e9), x),
        }
    }
}
impl crate::BoxedDeserialize for DataDecrypted {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0ba960e9)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0ba960e9) => Ok(DataDecrypted::Msg_DataDecrypted(
                _de.read_bare::<crate::ton::msg::datadecrypted::DataDecrypted>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `msg.DataDecryptedArray`\n\n```text\nmsg.dataDecryptedArray elements:vector<msg.dataDecrypted> = msg.DataDecryptedArray;\n```\n"]
pub enum DataDecryptedArray {
    Msg_DataDecryptedArray(crate::ton::msg::datadecryptedarray::DataDecryptedArray),
}
impl DataDecryptedArray {
    pub fn elements(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::msg::datadecrypted::DataDecrypted> {
        match self {
            DataDecryptedArray::Msg_DataDecryptedArray(ref x) => &x.elements,
        }
    }

    pub fn only(self) -> crate::ton::msg::datadecryptedarray::DataDecryptedArray {
        match self {
            DataDecryptedArray::Msg_DataDecryptedArray(x) => x,
        }
    }
}
impl Eq for DataDecryptedArray {}
impl Default for DataDecryptedArray {
    fn default() -> Self {
        DataDecryptedArray::Msg_DataDecryptedArray(
            crate::ton::msg::datadecryptedarray::DataDecryptedArray::default(),
        )
    }
}
impl crate::BoxedSerialize for DataDecryptedArray {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DataDecryptedArray::Msg_DataDecryptedArray(x) => {
                (crate::ConstructorNumber(0xe35c4709), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for DataDecryptedArray {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe35c4709)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe35c4709) => Ok(DataDecryptedArray::Msg_DataDecryptedArray(
                _de.read_bare::<crate::ton::msg::datadecryptedarray::DataDecryptedArray>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `msg.DataEncrypted`\n\n```text\nmsg.dataEncrypted source:accountAddress data:msg.Data = msg.DataEncrypted;\n```\n"]
pub enum DataEncrypted {
    Msg_DataEncrypted(crate::ton::msg::dataencrypted::DataEncrypted),
}
impl DataEncrypted {
    pub fn data(&self) -> &crate::ton::msg::Data {
        match self {
            DataEncrypted::Msg_DataEncrypted(ref x) => &x.data,
        }
    }

    pub fn source(&self) -> &crate::ton::accountaddress::AccountAddress {
        match self {
            DataEncrypted::Msg_DataEncrypted(ref x) => &x.source,
        }
    }

    pub fn only(self) -> crate::ton::msg::dataencrypted::DataEncrypted {
        match self {
            DataEncrypted::Msg_DataEncrypted(x) => x,
        }
    }
}
impl Eq for DataEncrypted {}
impl Default for DataEncrypted {
    fn default() -> Self {
        DataEncrypted::Msg_DataEncrypted(crate::ton::msg::dataencrypted::DataEncrypted::default())
    }
}
impl crate::BoxedSerialize for DataEncrypted {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DataEncrypted::Msg_DataEncrypted(x) => (crate::ConstructorNumber(0x21a13d51), x),
        }
    }
}
impl crate::BoxedDeserialize for DataEncrypted {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x21a13d51)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x21a13d51) => Ok(DataEncrypted::Msg_DataEncrypted(
                _de.read_bare::<crate::ton::msg::dataencrypted::DataEncrypted>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `msg.DataEncryptedArray`\n\n```text\nmsg.dataEncryptedArray elements:vector<msg.dataEncrypted> = msg.DataEncryptedArray;\n```\n"]
pub enum DataEncryptedArray {
    Msg_DataEncryptedArray(crate::ton::msg::dataencryptedarray::DataEncryptedArray),
}
impl DataEncryptedArray {
    pub fn elements(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::msg::dataencrypted::DataEncrypted> {
        match self {
            DataEncryptedArray::Msg_DataEncryptedArray(ref x) => &x.elements,
        }
    }

    pub fn only(self) -> crate::ton::msg::dataencryptedarray::DataEncryptedArray {
        match self {
            DataEncryptedArray::Msg_DataEncryptedArray(x) => x,
        }
    }
}
impl Eq for DataEncryptedArray {}
impl Default for DataEncryptedArray {
    fn default() -> Self {
        DataEncryptedArray::Msg_DataEncryptedArray(
            crate::ton::msg::dataencryptedarray::DataEncryptedArray::default(),
        )
    }
}
impl crate::BoxedSerialize for DataEncryptedArray {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DataEncryptedArray::Msg_DataEncryptedArray(x) => {
                (crate::ConstructorNumber(0x244759b2), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for DataEncryptedArray {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x244759b2)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x244759b2) => Ok(DataEncryptedArray::Msg_DataEncryptedArray(
                _de.read_bare::<crate::ton::msg::dataencryptedarray::DataEncryptedArray>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `msg.Message`\n\n```text\nmsg.message destination:accountAddress public_key:string amount:int64 data:msg.Data = msg.Message;\n```\n"]
pub enum Message {
    Msg_Message(crate::ton::msg::message::Message),
}
impl Message {
    pub fn amount(&self) -> &crate::ton::int64 {
        match self {
            Message::Msg_Message(ref x) => &x.amount,
        }
    }

    pub fn data(&self) -> &crate::ton::msg::Data {
        match self {
            Message::Msg_Message(ref x) => &x.data,
        }
    }

    pub fn destination(&self) -> &crate::ton::accountaddress::AccountAddress {
        match self {
            Message::Msg_Message(ref x) => &x.destination,
        }
    }

    pub fn public_key(&self) -> &crate::ton::string {
        match self {
            Message::Msg_Message(ref x) => &x.public_key,
        }
    }

    pub fn only(self) -> crate::ton::msg::message::Message {
        match self {
            Message::Msg_Message(x) => x,
        }
    }
}
impl Eq for Message {}
impl Default for Message {
    fn default() -> Self {
        Message::Msg_Message(crate::ton::msg::message::Message::default())
    }
}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::Msg_Message(x) => (crate::ConstructorNumber(0x8233d034), x),
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8233d034)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8233d034) => {
                Ok(Message::Msg_Message(_de.read_bare::<crate::ton::msg::message::Message>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod data;
pub mod datadecrypted;
pub mod datadecryptedarray;
pub mod dataencrypted;
pub mod dataencryptedarray;
pub mod message;
