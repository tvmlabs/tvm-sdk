use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.config.global`\n\n```text\nadnl.config.global static_nodes:adnl.nodes = adnl.config.Global;\n```\n"]
pub struct Global {
    pub static_nodes: crate::ton::adnl::nodes::Nodes,
}
impl Eq for Global {}
impl crate::BareSerialize for Global {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbe6f80d0)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Global { static_nodes } = self;
        _ser.write_bare::<crate::ton::adnl::nodes::Nodes>(static_nodes)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Global {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let static_nodes = _de.read_bare::<crate::ton::adnl::nodes::Nodes>()?;
            Ok(Self { static_nodes })
        }
    }
}
impl crate::IntoBoxed for Global {
    type Boxed = crate::ton::adnl::config::Global;
    fn into_boxed(self) -> crate::ton::adnl::config::Global {
        crate::ton::adnl::config::Global::Adnl_Config_Global(self)
    }
}
