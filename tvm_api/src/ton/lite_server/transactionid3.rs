use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.transactionId3`\n\n```text\nliteServer.transactionId3 account:int256 lt:long = liteServer.TransactionId3;\n```\n"]
pub struct TransactionId3 {
    pub account: crate::ton::int256,
    pub lt: crate::ton::long,
}
impl Eq for TransactionId3 {}
impl crate::BareSerialize for TransactionId3 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2c81da77)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let TransactionId3 { account, lt } = self;
        _ser.write_bare::<crate::ton::int256>(account)?;
        _ser.write_bare::<crate::ton::long>(lt)?;
        Ok(())
    }
}
impl crate::BareDeserialize for TransactionId3 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account = _de.read_bare::<crate::ton::int256>()?;
            let lt = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { account, lt })
        }
    }
}
impl crate::IntoBoxed for TransactionId3 {
    type Boxed = crate::ton::lite_server::TransactionId3;
    fn into_boxed(self) -> crate::ton::lite_server::TransactionId3 {
        crate::ton::lite_server::TransactionId3::LiteServer_TransactionId3(self)
    }
}
