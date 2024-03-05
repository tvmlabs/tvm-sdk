use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.db.Key`\n\n```text\noverlay.db.key.nodes local_id:int256 overlay:int256 = overlay.db.Key;\n```\n"]
pub enum Key {
    Overlay_Db_Key_Nodes(crate::ton::overlay::db::key::nodes::Nodes),
}
impl Key {
    pub fn local_id(&self) -> &crate::ton::int256 {
        match self {
            Key::Overlay_Db_Key_Nodes(ref x) => &x.local_id,
        }
    }

    pub fn overlay(&self) -> &crate::ton::int256 {
        match self {
            Key::Overlay_Db_Key_Nodes(ref x) => &x.overlay,
        }
    }

    pub fn only(self) -> crate::ton::overlay::db::key::nodes::Nodes {
        match self {
            Key::Overlay_Db_Key_Nodes(x) => x,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Overlay_Db_Key_Nodes(crate::ton::overlay::db::key::nodes::Nodes::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Overlay_Db_Key_Nodes(x) => (crate::ConstructorNumber(0xc4d07316), x),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc4d07316)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc4d07316) => Ok(Key::Overlay_Db_Key_Nodes(
                _de.read_bare::<crate::ton::overlay::db::key::nodes::Nodes>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.db.Nodes`\n\n```text\noverlay.db.nodes nodes:overlay.nodes = overlay.db.Nodes;\n```\n"]
pub enum Nodes {
    Overlay_Db_Nodes(crate::ton::overlay::db::nodes::Nodes),
}
impl Nodes {
    pub fn nodes(&self) -> &crate::ton::overlay::nodes::Nodes {
        match self {
            Nodes::Overlay_Db_Nodes(ref x) => &x.nodes,
        }
    }

    pub fn only(self) -> crate::ton::overlay::db::nodes::Nodes {
        match self {
            Nodes::Overlay_Db_Nodes(x) => x,
        }
    }
}
impl Eq for Nodes {}
impl Default for Nodes {
    fn default() -> Self {
        Nodes::Overlay_Db_Nodes(crate::ton::overlay::db::nodes::Nodes::default())
    }
}
impl crate::BoxedSerialize for Nodes {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Nodes::Overlay_Db_Nodes(x) => (crate::ConstructorNumber(0xd588ce1a), x),
        }
    }
}
impl crate::BoxedDeserialize for Nodes {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd588ce1a)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd588ce1a) => Ok(Nodes::Overlay_Db_Nodes(
                _de.read_bare::<crate::ton::overlay::db::nodes::Nodes>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod key;
pub mod nodes;
