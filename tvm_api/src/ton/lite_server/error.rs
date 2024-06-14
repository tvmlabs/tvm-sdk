use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.error`\n\n```text\nliteServer.error code:int message:string = liteServer.Error;\n```\n"]
pub struct Error {
    pub code: crate::ton::int,
    pub message: crate::ton::string,
}
impl Eq for Error {}
impl crate::BareSerialize for Error {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbba9e148)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Error { code, message } = self;
        _ser.write_bare::<crate::ton::int>(code)?;
        _ser.write_bare::<crate::ton::string>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Error {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let code = _de.read_bare::<crate::ton::int>()?;
            let message = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { code, message })
        }
    }
}
impl crate::IntoBoxed for Error {
    type Boxed = crate::ton::lite_server::Error;
    fn into_boxed(self) -> crate::ton::lite_server::Error {
        crate::ton::lite_server::Error::LiteServer_Error(self)
    }
}
