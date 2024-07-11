use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.blockTransactions`\n\n```text\nliteServer.blockTransactions id:tonNode.blockIdExt req_count:# incomplete:Bool ids:(vector liteServer.transactionId) proof:bytes = liteServer.BlockTransactions;\n```\n"]
pub struct BlockTransactions {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub req_count: crate::ton::int,
    pub incomplete: crate::ton::Bool,
    pub ids:
        crate::ton::vector<crate::ton::Bare, crate::ton::lite_server::transactionid::TransactionId>,
    pub proof: crate::ton::bytes,
}
impl Eq for BlockTransactions {}
impl crate::BareSerialize for BlockTransactions {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbd8cad2b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockTransactions { id, req_count, incomplete, ids, proof } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(req_count)?;
        _ser.write_boxed::<crate::ton::Bool>(incomplete)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::lite_server::transactionid::TransactionId,
        >>(ids)?;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockTransactions {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let req_count = _de.read_bare::<crate::ton::int>()?;
            let incomplete = _de.read_boxed::<crate::ton::Bool>()?;
            let ids = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::lite_server::transactionid::TransactionId,
            >>()?;
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, req_count, incomplete, ids, proof })
        }
    }
}
impl crate::IntoBoxed for BlockTransactions {
    type Boxed = crate::ton::lite_server::BlockTransactions;

    fn into_boxed(self) -> crate::ton::lite_server::BlockTransactions {
        crate::ton::lite_server::BlockTransactions::LiteServer_BlockTransactions(self)
    }
}
