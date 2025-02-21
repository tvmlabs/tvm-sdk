use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.db.node.value`\n\n```text\nadnl.db.node.value date:int id:PublicKey addr_list:adnl.addressList priority_addr_list:adnl.addressList = adnl.db.node.Value;\n```\n"]
pub struct Value {
    pub date: crate::ton::int,
    pub id: crate::ton::PublicKey,
    pub addr_list: crate::ton::adnl::addresslist::AddressList,
    pub priority_addr_list: crate::ton::adnl::addresslist::AddressList,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x545d2707)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { date, id, addr_list, priority_addr_list } = self;
        _ser.write_bare::<crate::ton::int>(date)?;
        _ser.write_boxed::<crate::ton::PublicKey>(id)?;
        _ser.write_bare::<crate::ton::adnl::addresslist::AddressList>(addr_list)?;
        _ser.write_bare::<crate::ton::adnl::addresslist::AddressList>(priority_addr_list)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let date = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_boxed::<crate::ton::PublicKey>()?;
            let addr_list = _de.read_bare::<crate::ton::adnl::addresslist::AddressList>()?;
            let priority_addr_list =
                _de.read_bare::<crate::ton::adnl::addresslist::AddressList>()?;
            Ok(Self { date, id, addr_list, priority_addr_list })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::adnl::db::node::Value;

    fn into_boxed(self) -> crate::ton::adnl::db::node::Value {
        crate::ton::adnl::db::node::Value::Adnl_Db_Node_Value(self)
    }
}
