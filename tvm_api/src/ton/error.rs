use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `error`\n\n```text\nerror code:int32 message:string = Error;\n```\n"]
pub struct Error {
    pub code: crate::ton::int32,
    pub message: crate::ton::string,
}
impl Eq for Error {}
impl crate::BareSerialize for Error {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9bdd8f1a)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Error { code, message } = self;
        _ser.write_bare::<crate::ton::int32>(code)?;
        _ser.write_bare::<crate::ton::string>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Error {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let code = _de.read_bare::<crate::ton::int32>()?;
            let message = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { code, message })
        }
    }
}
impl crate::IntoBoxed for Error {
    type Boxed = crate::ton::Error;
    fn into_boxed(self) -> crate::ton::Error {
        crate::ton::Error::Error(self)
    }
}
