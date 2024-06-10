use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.db.nodes`\n\n```text\noverlay.db.nodes nodes:overlay.nodes = overlay.db.Nodes;\n```\n"]
pub struct Nodes {
    pub nodes: crate::ton::overlay::nodes::Nodes,
}
impl Eq for Nodes {}
impl crate::BareSerialize for Nodes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd588ce1a)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Nodes { nodes } = self;
        _ser.write_bare::<crate::ton::overlay::nodes::Nodes>(nodes)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Nodes {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nodes = _de.read_bare::<crate::ton::overlay::nodes::Nodes>()?;
            Ok(Self { nodes })
        }
    }
}
impl crate::IntoBoxed for Nodes {
    type Boxed = crate::ton::overlay::db::Nodes;
    fn into_boxed(self) -> crate::ton::overlay::db::Nodes {
        crate::ton::overlay::db::Nodes::Overlay_Db_Nodes(self)
    }
}
