use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempCombinedReceipt`\n\n```text\ntonNode.rempCombinedReceipt source_id:int256 ids:(vector tonNode.blockIdExt) \n        receipts:(vector tonNode.RempReceiptCompact) = tonNode.RempCombinedReceipt;\n```\n"]
pub struct RempCombinedReceipt {
    pub source_id: crate::ton::int256,
    pub ids: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
    pub receipts: crate::ton::vector<crate::ton::Boxed, crate::ton::ton_node::RempReceiptCompact>,
}
impl Eq for RempCombinedReceipt {}
impl crate::BareSerialize for RempCombinedReceipt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x98eb4db1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempCombinedReceipt { source_id, ids, receipts } = self;
        _ser.write_bare::<crate::ton::int256>(source_id)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (ids) ? ;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: ton_node :: RempReceiptCompact > > (receipts) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for RempCombinedReceipt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let source_id = _de.read_bare::<crate::ton::int256>()?;
            let ids = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            let receipts = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: ton_node :: RempReceiptCompact > > () ? ;
            Ok(Self { source_id, ids, receipts })
        }
    }
}
impl crate::IntoBoxed for RempCombinedReceipt {
    type Boxed = crate::ton::ton_node::RempCombinedReceipt;

    fn into_boxed(self) -> crate::ton::ton_node::RempCombinedReceipt {
        crate::ton::ton_node::RempCombinedReceipt::TonNode_RempCombinedReceipt(self)
    }
}
