use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.nodes`\n\n```text\ndht.nodes nodes:(vector dht.node) = dht.Nodes;\n```\n"]
pub struct Nodes {
    pub nodes: crate::ton::vector<crate::ton::dht::node::Node>,
}
impl Eq for Nodes {}
impl crate::BareSerialize for Nodes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7974a0be)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Nodes { nodes } = self;
        (nodes as &dyn crate::ton::VectoredBare<crate::ton::dht::node::Node>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Nodes {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nodes = <Vec<crate::ton::dht::node::Node> as crate::ton::VectoredBare<
                crate::ton::dht::node::Node,
            >>::deserialize(_de)?;
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
