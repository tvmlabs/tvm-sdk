use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.db.node.Value`\n\n```text\nadnl.db.node.value date:int id:PublicKey addr_list:adnl.addressList priority_addr_list:adnl.addressList = adnl.db.node.Value;\n```\n"]
pub enum Value {
    Adnl_Db_Node_Value(crate::ton::adnl::db::node::value::Value),
}
impl Value {
    pub fn addr_list(&self) -> &crate::ton::adnl::addresslist::AddressList {
        match self {
            Value::Adnl_Db_Node_Value(ref x) => &x.addr_list,
        }
    }
    pub fn date(&self) -> &crate::ton::int {
        match self {
            Value::Adnl_Db_Node_Value(ref x) => &x.date,
        }
    }
    pub fn id(&self) -> &crate::ton::PublicKey {
        match self {
            Value::Adnl_Db_Node_Value(ref x) => &x.id,
        }
    }
    pub fn priority_addr_list(&self) -> &crate::ton::adnl::addresslist::AddressList {
        match self {
            Value::Adnl_Db_Node_Value(ref x) => &x.priority_addr_list,
        }
    }
    pub fn only(self) -> crate::ton::adnl::db::node::value::Value {
        match self {
            Value::Adnl_Db_Node_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Adnl_Db_Node_Value(crate::ton::adnl::db::node::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Adnl_Db_Node_Value(x) => (crate::ConstructorNumber(0x545d2707), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x545d2707)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x545d2707) => Ok(Value::Adnl_Db_Node_Value(
                _de.read_bare::<crate::ton::adnl::db::node::value::Value>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod value;
