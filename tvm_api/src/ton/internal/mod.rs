use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `internal.BlockId`\n\n```text\nton.blockId workchain:int32 shard:int64 seqno:int32 = internal.BlockId;\n```\n"]
pub enum BlockId {
    Ton_BlockId(crate::ton::internal::ton::blockid::BlockId),
}
impl BlockId {
    pub fn seqno(&self) -> &crate::ton::int32 {
        match self {
            BlockId::Ton_BlockId(ref x) => &x.seqno,
        }
    }

    pub fn shard(&self) -> &crate::ton::int64 {
        match self {
            BlockId::Ton_BlockId(ref x) => &x.shard,
        }
    }

    pub fn workchain(&self) -> &crate::ton::int32 {
        match self {
            BlockId::Ton_BlockId(ref x) => &x.workchain,
        }
    }

    pub fn only(self) -> crate::ton::internal::ton::blockid::BlockId {
        match self {
            BlockId::Ton_BlockId(x) => x,
        }
    }
}
impl Eq for BlockId {}
impl Default for BlockId {
    fn default() -> Self {
        BlockId::Ton_BlockId(crate::ton::internal::ton::blockid::BlockId::default())
    }
}
impl crate::BoxedSerialize for BlockId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockId::Ton_BlockId(x) => (crate::ConstructorNumber(0xb9587fa2), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb9587fa2)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb9587fa2) => Ok(BlockId::Ton_BlockId(
                _de.read_bare::<crate::ton::internal::ton::blockid::BlockId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `internal.TransactionId`\n\n```text\ninternal.transactionId lt:int64 hash:bytes = internal.TransactionId;\n```\n"]
pub enum TransactionId {
    Internal_TransactionId(crate::ton::internal::transactionid::TransactionId),
}
impl TransactionId {
    pub fn hash(&self) -> &crate::ton::bytes {
        match self {
            TransactionId::Internal_TransactionId(ref x) => &x.hash,
        }
    }

    pub fn lt(&self) -> &crate::ton::int64 {
        match self {
            TransactionId::Internal_TransactionId(ref x) => &x.lt,
        }
    }

    pub fn only(self) -> crate::ton::internal::transactionid::TransactionId {
        match self {
            TransactionId::Internal_TransactionId(x) => x,
        }
    }
}
impl Eq for TransactionId {}
impl Default for TransactionId {
    fn default() -> Self {
        TransactionId::Internal_TransactionId(
            crate::ton::internal::transactionid::TransactionId::default(),
        )
    }
}
impl crate::BoxedSerialize for TransactionId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            TransactionId::Internal_TransactionId(x) => (crate::ConstructorNumber(0xc5050322), x),
        }
    }
}
impl crate::BoxedDeserialize for TransactionId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc5050322)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc5050322) => Ok(TransactionId::Internal_TransactionId(
                _de.read_bare::<crate::ton::internal::transactionid::TransactionId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod ton;
pub mod transactionid;
