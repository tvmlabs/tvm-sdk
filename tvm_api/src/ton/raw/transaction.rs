use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.transaction`\n\n```text\nraw.transaction utime:int53 data:bytes transaction_id:internal.transactionId fee:int64 storage_fee:int64 other_fee:int64 in_msg:raw.message out_msgs:vector<raw.message> = raw.Transaction;\n```\n"]
pub struct Transaction {
    pub utime: crate::ton::int53,
    pub data: crate::ton::bytes,
    pub transaction_id: crate::ton::internal::transactionid::TransactionId,
    pub fee: crate::ton::int64,
    pub storage_fee: crate::ton::int64,
    pub other_fee: crate::ton::int64,
    pub in_msg: crate::ton::raw::message::Message,
    pub out_msgs: crate::ton::vector<crate::ton::Bare, crate::ton::raw::message::Message>,
}
impl Eq for Transaction {}
impl crate::BareSerialize for Transaction {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x70828481)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Transaction {
            utime,
            data,
            transaction_id,
            fee,
            storage_fee,
            other_fee,
            in_msg,
            out_msgs,
        } = self;
        _ser.write_bare::<crate::ton::int53>(utime)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::internal::transactionid::TransactionId>(transaction_id)?;
        _ser.write_bare::<crate::ton::int64>(fee)?;
        _ser.write_bare::<crate::ton::int64>(storage_fee)?;
        _ser.write_bare::<crate::ton::int64>(other_fee)?;
        _ser.write_bare::<crate::ton::raw::message::Message>(in_msg)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::raw::message::Message>>(
            out_msgs,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Transaction {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let utime = _de.read_bare::<crate::ton::int53>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let transaction_id =
                _de.read_bare::<crate::ton::internal::transactionid::TransactionId>()?;
            let fee = _de.read_bare::<crate::ton::int64>()?;
            let storage_fee = _de.read_bare::<crate::ton::int64>()?;
            let other_fee = _de.read_bare::<crate::ton::int64>()?;
            let in_msg = _de.read_bare::<crate::ton::raw::message::Message>()?;
            let out_msgs = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: raw :: message :: Message > > () ? ;
            Ok(Self { utime, data, transaction_id, fee, storage_fee, other_fee, in_msg, out_msgs })
        }
    }
}
impl crate::IntoBoxed for Transaction {
    type Boxed = crate::ton::raw::Transaction;

    fn into_boxed(self) -> crate::ton::raw::Transaction {
        crate::ton::raw::Transaction::Raw_Transaction(self)
    }
}
