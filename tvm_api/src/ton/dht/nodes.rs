use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.nodes`\n\n```text\ndht.nodes nodes:(vector dht.node) = dht.Nodes;\n```\n"]
pub struct Nodes {
    pub nodes: crate::ton::vector<crate::ton::Bare, crate::ton::dht::node::Node>,
}
impl Eq for Nodes {}
impl crate::BareSerialize for Nodes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7974a0be)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Nodes { nodes } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::dht::node::Node>>(
            nodes,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Nodes {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nodes = _de
                .read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::dht::node::Node>>()?;
            Ok(Self { nodes })
        }
    }
}
impl crate::IntoBoxed for Nodes {
    type Boxed = crate::ton::dht::Nodes;

    fn into_boxed(self) -> crate::ton::dht::Nodes {
        crate::ton::dht::Nodes::Dht_Nodes(self)
    }
}
