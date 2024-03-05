use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `internal.transactionId`\n\n```text\ninternal.transactionId lt:int64 hash:bytes = internal.TransactionId;\n```\n"]
pub struct TransactionId {
    pub lt: crate::ton::int64,
    pub hash: crate::ton::bytes,
}
impl Eq for TransactionId {}
impl crate::BareSerialize for TransactionId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc5050322)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let TransactionId { lt, hash } = self;
        _ser.write_bare::<crate::ton::int64>(lt)?;
        _ser.write_bare::<crate::ton::bytes>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for TransactionId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let lt = _de.read_bare::<crate::ton::int64>()?;
            let hash = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { lt, hash })
        }
    }
}
impl crate::IntoBoxed for TransactionId {
    type Boxed = crate::ton::internal::TransactionId;

    fn into_boxed(self) -> crate::ton::internal::TransactionId {
        crate::ton::internal::TransactionId::Internal_TransactionId(self)
    }
}
