use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.sendMsgStatus`\n\n```text\nliteServer.sendMsgStatus status:int = liteServer.SendMsgStatus;\n```\n"]
pub struct SendMsgStatus {
    pub status: crate::ton::int,
}
impl Eq for SendMsgStatus {}
impl crate::BareSerialize for SendMsgStatus {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3950e597)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SendMsgStatus { status } = self;
        _ser.write_bare::<crate::ton::int>(status)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SendMsgStatus {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let status = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { status })
        }
    }
}
impl crate::IntoBoxed for SendMsgStatus {
    type Boxed = crate::ton::lite_server::SendMsgStatus;
    fn into_boxed(self) -> crate::ton::lite_server::SendMsgStatus {
        crate::ton::lite_server::SendMsgStatus::LiteServer_SendMsgStatus(self)
    }
}
