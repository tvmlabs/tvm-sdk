use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.config.Global`\n\n```text\nadnl.config.global static_nodes:adnl.nodes = adnl.config.Global;\n```\n"]
pub enum Global {
    Adnl_Config_Global(crate::ton::adnl::config::global::Global),
}
impl Global {
    pub fn static_nodes(&self) -> &crate::ton::adnl::nodes::Nodes {
        match self {
            Global::Adnl_Config_Global(ref x) => &x.static_nodes,
        }
    }
    pub fn only(self) -> crate::ton::adnl::config::global::Global {
        match self {
            Global::Adnl_Config_Global(x) => x,
        }
    }
}
impl Eq for Global {}
impl Default for Global {
    fn default() -> Self {
        Global::Adnl_Config_Global(crate::ton::adnl::config::global::Global::default())
    }
}
impl crate::BoxedSerialize for Global {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Global::Adnl_Config_Global(x) => (crate::ConstructorNumber(0xbe6f80d0), x),
        }
    }
}
impl crate::BoxedDeserialize for Global {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbe6f80d0)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbe6f80d0) => Ok(Global::Adnl_Config_Global(
                _de.read_bare::<crate::ton::adnl::config::global::Global>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod global;
