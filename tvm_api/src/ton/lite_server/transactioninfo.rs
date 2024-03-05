use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.transactionInfo`\n\n```text\nliteServer.transactionInfo id:tonNode.blockIdExt proof:bytes transaction:bytes = liteServer.TransactionInfo;\n```\n"]
pub struct TransactionInfo {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub proof: crate::ton::bytes,
    pub transaction: crate::ton::bytes,
}
impl Eq for TransactionInfo {}
impl crate::BareSerialize for TransactionInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0edeed47)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let TransactionInfo { id, proof, transaction } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        _ser.write_bare::<crate::ton::bytes>(transaction)?;
        Ok(())
    }
}
impl crate::BareDeserialize for TransactionInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            let transaction = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, proof, transaction })
        }
    }
}
impl crate::IntoBoxed for TransactionInfo {
    type Boxed = crate::ton::lite_server::TransactionInfo;

    fn into_boxed(self) -> crate::ton::lite_server::TransactionInfo {
        crate::ton::lite_server::TransactionInfo::LiteServer_TransactionInfo(self)
    }
}
