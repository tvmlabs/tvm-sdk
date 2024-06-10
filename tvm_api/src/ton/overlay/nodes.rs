use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.nodes`\n\n```text\noverlay.nodes nodes:(vector overlay.node) = overlay.Nodes;\n```\n"]
pub struct Nodes {
    pub nodes: crate::ton::vector<crate::ton::Bare, crate::ton::overlay::node::Node>,
}
impl Eq for Nodes {}
impl crate::BareSerialize for Nodes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe487290e)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Nodes { nodes } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::overlay::node::Node>>(
            nodes,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Nodes {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nodes = _de
                .read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::overlay::node::Node>>(
                )?;
            Ok(Self { nodes })
        }
    }
}
impl crate::IntoBoxed for Nodes {
    type Boxed = crate::ton::overlay::Nodes;
    fn into_boxed(self) -> crate::ton::overlay::Nodes {
        crate::ton::overlay::Nodes::Overlay_Nodes(self)
    }
}
