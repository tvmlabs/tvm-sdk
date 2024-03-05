use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.db.node.key`\n\n```text\nadnl.db.node.key local_id:int256 peer_id:int256 = adnl.db.Key;\n```\n"]
pub struct Key {
    pub local_id: crate::ton::int256,
    pub peer_id: crate::ton::int256,
}
impl Eq for Key {}
impl crate::BareSerialize for Key {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc5a3e42e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Key { local_id, peer_id } = self;
        _ser.write_bare::<crate::ton::int256>(local_id)?;
        _ser.write_bare::<crate::ton::int256>(peer_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Key {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_id = _de.read_bare::<crate::ton::int256>()?;
            let peer_id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { local_id, peer_id })
        }
    }
}
impl crate::IntoBoxed for Key {
    type Boxed = crate::ton::adnl::db::Key;

    fn into_boxed(self) -> crate::ton::adnl::db::Key {
        crate::ton::adnl::db::Key::Adnl_Db_Node_Key(self)
    }
}
