use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.db.key.nodes`\n\n```text\noverlay.db.key.nodes local_id:int256 overlay:int256 = overlay.db.Key;\n```\n"]
pub struct Nodes {
    pub local_id: crate::ton::int256,
    pub overlay: crate::ton::int256,
}
impl Eq for Nodes {}
impl crate::BareSerialize for Nodes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc4d07316)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Nodes { local_id, overlay } = self;
        _ser.write_bare::<crate::ton::int256>(local_id)?;
        _ser.write_bare::<crate::ton::int256>(overlay)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Nodes {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_id = _de.read_bare::<crate::ton::int256>()?;
            let overlay = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { local_id, overlay })
        }
    }
}
impl crate::IntoBoxed for Nodes {
    type Boxed = crate::ton::overlay::db::Key;

    fn into_boxed(self) -> crate::ton::overlay::db::Key {
        crate::ton::overlay::db::Key::Overlay_Db_Key_Nodes(self)
    }
}
