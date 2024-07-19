use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.nodes`\n\n```text\nadnl.nodes nodes:(vector adnl.node) = adnl.Nodes;\n```\n"]
pub struct Nodes {
    pub nodes: crate::ton::vector<crate::ton::adnl::node::Node>,
}
impl Eq for Nodes {}
impl crate::BareSerialize for Nodes {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa209db56)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Nodes { nodes } = self;
        (nodes as &dyn crate::ton::VectoredBare<crate::ton::adnl::node::Node>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Nodes {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nodes = <Vec<crate::ton::adnl::node::Node> as crate::ton::VectoredBare<
                crate::ton::adnl::node::Node,
            >>::deserialize(_de)?;
            Ok(Self { nodes })
        }
    }
}
impl crate::IntoBoxed for Nodes {
    type Boxed = crate::ton::adnl::Nodes;

    fn into_boxed(self) -> crate::ton::adnl::Nodes {
        crate::ton::adnl::Nodes::Adnl_Nodes(self)
    }
}
