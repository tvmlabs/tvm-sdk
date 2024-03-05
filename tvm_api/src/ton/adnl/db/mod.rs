use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.db.Key`\n\n```text\nadnl.db.node.key local_id:int256 peer_id:int256 = adnl.db.Key;\n```\n"]
pub enum Key {
    Adnl_Db_Node_Key(crate::ton::adnl::db::node::key::Key),
}
impl Key {
    pub fn local_id(&self) -> &crate::ton::int256 {
        match self {
            Key::Adnl_Db_Node_Key(ref x) => &x.local_id,
        }
    }

    pub fn peer_id(&self) -> &crate::ton::int256 {
        match self {
            Key::Adnl_Db_Node_Key(ref x) => &x.peer_id,
        }
    }

    pub fn only(self) -> crate::ton::adnl::db::node::key::Key {
        match self {
            Key::Adnl_Db_Node_Key(x) => x,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Adnl_Db_Node_Key(crate::ton::adnl::db::node::key::Key::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Adnl_Db_Node_Key(x) => (crate::ConstructorNumber(0xc5a3e42e), x),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc5a3e42e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc5a3e42e) => {
                Ok(Key::Adnl_Db_Node_Key(_de.read_bare::<crate::ton::adnl::db::node::key::Key>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod node;
