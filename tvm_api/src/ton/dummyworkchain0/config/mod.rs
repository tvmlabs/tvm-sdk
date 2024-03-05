use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dummyworkchain0.config.Global`\n\n```text\ndummyworkchain0.config.global zero_state_hash:int256 = dummyworkchain0.config.Global;\n```\n"]
pub enum Global {
    Dummyworkchain0_Config_Global(crate::ton::dummyworkchain0::config::global::Global),
}
impl Global {
    pub fn zero_state_hash(&self) -> &crate::ton::int256 {
        match self {
            Global::Dummyworkchain0_Config_Global(ref x) => &x.zero_state_hash,
        }
    }

    pub fn only(self) -> crate::ton::dummyworkchain0::config::global::Global {
        match self {
            Global::Dummyworkchain0_Config_Global(x) => x,
        }
    }
}
impl Eq for Global {}
impl Default for Global {
    fn default() -> Self {
        Global::Dummyworkchain0_Config_Global(
            crate::ton::dummyworkchain0::config::global::Global::default(),
        )
    }
}
impl crate::BoxedSerialize for Global {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Global::Dummyworkchain0_Config_Global(x) => (crate::ConstructorNumber(0xda616ed3), x),
        }
    }
}
impl crate::BoxedDeserialize for Global {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xda616ed3)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xda616ed3) => Ok(Global::Dummyworkchain0_Config_Global(
                _de.read_bare::<crate::ton::dummyworkchain0::config::global::Global>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod global;
