use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteclient.config.Global`\n\n```text\nliteclient.config.global liteservers:(vector liteserver.desc) validator:validator.config.global = liteclient.config.Global;\n```\n"]
pub enum Global {
    Liteclient_Config_Global(crate::ton::liteclient::config::global::Global),
}
impl Global {
    pub fn liteservers(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::liteserver::desc::Desc> {
        match self {
            Global::Liteclient_Config_Global(ref x) => &x.liteservers,
        }
    }

    pub fn validator(&self) -> &crate::ton::validator::config::global::Global {
        match self {
            Global::Liteclient_Config_Global(ref x) => &x.validator,
        }
    }

    pub fn only(self) -> crate::ton::liteclient::config::global::Global {
        match self {
            Global::Liteclient_Config_Global(x) => x,
        }
    }
}
impl Eq for Global {}
impl Default for Global {
    fn default() -> Self {
        Global::Liteclient_Config_Global(crate::ton::liteclient::config::global::Global::default())
    }
}
impl crate::BoxedSerialize for Global {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Global::Liteclient_Config_Global(x) => (crate::ConstructorNumber(0x088dc0f8), x),
        }
    }
}
impl crate::BoxedDeserialize for Global {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x088dc0f8)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x088dc0f8) => Ok(Global::Liteclient_Config_Global(
                _de.read_bare::<crate::ton::liteclient::config::global::Global>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod global;
