use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.transactionId`\n\n```text\nliteServer.transactionId mode:# account:mode.0?int256 lt:mode.1?long hash:mode.2?int256 = liteServer.TransactionId;\n```\n"]
pub struct TransactionId {
    pub mode: crate::ton::int,
    pub account: Option<crate::ton::int256>,
    pub lt: Option<crate::ton::long>,
    pub hash: Option<crate::ton::int256>,
}
impl Eq for TransactionId {}
impl crate::BareSerialize for TransactionId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb12f65af)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let TransactionId { mode, account, lt, hash } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        if let Some(inner) = account {
            _ser.write_bare::<crate::ton::int256>(inner)?;
        }
        if let Some(inner) = lt {
            _ser.write_bare::<crate::ton::long>(inner)?;
        }
        if let Some(inner) = hash {
            _ser.write_bare::<crate::ton::int256>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for TransactionId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let account = if mode & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::int256>()?)
            } else {
                None
            };
            let lt = if mode & (1 << 1u32) != 0 {
                Some(_de.read_bare::<crate::ton::long>()?)
            } else {
                None
            };
            let hash = if mode & (1 << 2u32) != 0 {
                Some(_de.read_bare::<crate::ton::int256>()?)
            } else {
                None
            };
            Ok(Self { mode, account, lt, hash })
        }
    }
}
impl crate::IntoBoxed for TransactionId {
    type Boxed = crate::ton::lite_server::TransactionId;

    fn into_boxed(self) -> crate::ton::lite_server::TransactionId {
        crate::ton::lite_server::TransactionId::LiteServer_TransactionId(self)
    }
}
