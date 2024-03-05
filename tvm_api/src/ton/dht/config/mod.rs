use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.config.Global`\n\n```text\ndht.config.global static_nodes:dht.nodes k:int a:int = dht.config.Global;\n```\n"]
pub enum Global {
    Dht_Config_Global(crate::ton::dht::config::global::Global),
}
impl Global {
    pub fn a(&self) -> &crate::ton::int {
        match self {
            Global::Dht_Config_Global(ref x) => &x.a,
        }
    }

    pub fn k(&self) -> &crate::ton::int {
        match self {
            Global::Dht_Config_Global(ref x) => &x.k,
        }
    }

    pub fn static_nodes(&self) -> &crate::ton::dht::nodes::Nodes {
        match self {
            Global::Dht_Config_Global(ref x) => &x.static_nodes,
        }
    }

    pub fn only(self) -> crate::ton::dht::config::global::Global {
        match self {
            Global::Dht_Config_Global(x) => x,
        }
    }
}
impl Eq for Global {}
impl Default for Global {
    fn default() -> Self {
        Global::Dht_Config_Global(crate::ton::dht::config::global::Global::default())
    }
}
impl crate::BoxedSerialize for Global {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Global::Dht_Config_Global(x) => (crate::ConstructorNumber(0x84ceca07), x),
        }
    }
}
impl crate::BoxedDeserialize for Global {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x84ceca07)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x84ceca07) => Ok(Global::Dht_Config_Global(
                _de.read_bare::<crate::ton::dht::config::global::Global>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.config.Local`\n\n```text\ndht.config.local id:adnl.id.short = dht.config.Local;\n\ndht.config.random.local cnt:int = dht.config.Local;\n```\n"]
pub enum Local {
    Dht_Config_Local(crate::ton::dht::config::local::Local),
    Dht_Config_Random_Local(crate::ton::dht::config::random::local::Local),
}
impl Local {
    pub fn cnt(&self) -> Option<&crate::ton::int> {
        match self {
            Local::Dht_Config_Random_Local(ref x) => Some(&x.cnt),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&crate::ton::adnl::id::short::Short> {
        match self {
            Local::Dht_Config_Local(ref x) => Some(&x.id),
            _ => None,
        }
    }
}
impl Eq for Local {}
impl Default for Local {
    fn default() -> Self {
        Local::Dht_Config_Local(crate::ton::dht::config::local::Local::default())
    }
}
impl crate::BoxedSerialize for Local {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Local::Dht_Config_Local(x) => (crate::ConstructorNumber(0x76204a6f), x),
            Local::Dht_Config_Random_Local(x) => (crate::ConstructorNumber(0x9beb2577), x),
        }
    }
}
impl crate::BoxedDeserialize for Local {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x76204a6f), crate::ConstructorNumber(0x9beb2577)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x76204a6f) => Ok(Local::Dht_Config_Local(
                _de.read_bare::<crate::ton::dht::config::local::Local>()?,
            )),
            crate::ConstructorNumber(0x9beb2577) => Ok(Local::Dht_Config_Random_Local(
                _de.read_bare::<crate::ton::dht::config::random::local::Local>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod global;
pub mod local;
pub mod random;
