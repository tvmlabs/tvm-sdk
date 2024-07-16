use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.transactions`\n\n```text\nraw.transactions transactions:vector<raw.transaction> previous_transaction_id:internal.transactionId = raw.Transactions;\n```\n"]
pub struct Transactions {
    pub transactions:
        crate::ton::vector<crate::ton::Bare, crate::ton::raw::transaction::Transaction>,
    pub previous_transaction_id: crate::ton::internal::transactionid::TransactionId,
}
impl Eq for Transactions {}
impl crate::BareSerialize for Transactions {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x84fae8ed)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Transactions { transactions, previous_transaction_id } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: raw :: transaction :: Transaction > > (transactions) ? ;
        _ser.write_bare::<crate::ton::internal::transactionid::TransactionId>(
            previous_transaction_id,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Transactions {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let transactions = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: raw :: transaction :: Transaction > > () ? ;
            let previous_transaction_id =
                _de.read_bare::<crate::ton::internal::transactionid::TransactionId>()?;
            Ok(Self { transactions, previous_transaction_id })
        }
    }
}
impl crate::IntoBoxed for Transactions {
    type Boxed = crate::ton::raw::Transactions;

    fn into_boxed(self) -> crate::ton::raw::Transactions {
        crate::ton::raw::Transactions::Raw_Transactions(self)
    }
}
