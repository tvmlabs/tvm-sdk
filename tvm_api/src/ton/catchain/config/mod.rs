use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.config.Global`\n\n```text\ncatchain.config.global tag:int256 nodes:(vector PublicKey) = catchain.config.Global;\n```\n"]
pub enum Global {
    Catchain_Config_Global(crate::ton::catchain::config::global::Global),
}
impl Global {
    pub fn nodes(&self) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::PublicKey> {
        match self {
            Global::Catchain_Config_Global(ref x) => &x.nodes,
        }
    }
    pub fn tag(&self) -> &crate::ton::int256 {
        match self {
            Global::Catchain_Config_Global(ref x) => &x.tag,
        }
    }
    pub fn only(self) -> crate::ton::catchain::config::global::Global {
        match self {
            Global::Catchain_Config_Global(x) => x,
        }
    }
}
impl Eq for Global {}
impl Default for Global {
    fn default() -> Self {
        Global::Catchain_Config_Global(crate::ton::catchain::config::global::Global::default())
    }
}
impl crate::BoxedSerialize for Global {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Global::Catchain_Config_Global(x) => (crate::ConstructorNumber(0x68c7b651), x),
        }
    }
}
impl crate::BoxedDeserialize for Global {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x68c7b651)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x68c7b651) => Ok(Global::Catchain_Config_Global(
                _de.read_bare::<crate::ton::catchain::config::global::Global>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod global;
