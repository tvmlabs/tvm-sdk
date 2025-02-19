use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.transactionList`\n\n```text\nliteServer.transactionList ids:(vector tonNode.blockIdExt) transactions:bytes = liteServer.TransactionList;\n```\n"]
pub struct TransactionList {
    pub ids: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
    pub transactions: crate::ton::bytes,
}
impl Eq for TransactionList {}
impl crate::BareSerialize for TransactionList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6f26c60b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let TransactionList { ids, transactions } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (ids) ? ;
        _ser.write_bare::<crate::ton::bytes>(transactions)?;
        Ok(())
    }
}
impl crate::BareDeserialize for TransactionList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ids = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            let transactions = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { ids, transactions })
        }
    }
}
impl crate::IntoBoxed for TransactionList {
    type Boxed = crate::ton::lite_server::TransactionList;

    fn into_boxed(self) -> crate::ton::lite_server::TransactionList {
        crate::ton::lite_server::TransactionList::LiteServer_TransactionList(self)
    }
}
