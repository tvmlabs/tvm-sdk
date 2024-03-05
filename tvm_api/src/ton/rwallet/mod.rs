use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `rwallet.Action`\n\n```text\nrwallet.actionInit config:rwallet.config = rwallet.Action;\n```\n"]
pub enum Action {
    Rwallet_ActionInit(crate::ton::rwallet::actioninit::ActionInit),
}
impl Action {
    pub fn config(&self) -> &crate::ton::rwallet::config::Config {
        match self {
            Action::Rwallet_ActionInit(ref x) => &x.config,
        }
    }

    pub fn only(self) -> crate::ton::rwallet::actioninit::ActionInit {
        match self {
            Action::Rwallet_ActionInit(x) => x,
        }
    }
}
impl Eq for Action {}
impl Default for Action {
    fn default() -> Self {
        Action::Rwallet_ActionInit(crate::ton::rwallet::actioninit::ActionInit::default())
    }
}
impl crate::BoxedSerialize for Action {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Action::Rwallet_ActionInit(x) => (crate::ConstructorNumber(0x2533bd6b), x),
        }
    }
}
impl crate::BoxedDeserialize for Action {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2533bd6b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x2533bd6b) => Ok(Action::Rwallet_ActionInit(
                _de.read_bare::<crate::ton::rwallet::actioninit::ActionInit>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `rwallet.Config`\n\n```text\nrwallet.config start_at:int53 limits:vector<rwallet.limit> = rwallet.Config;\n```\n"]
pub enum Config {
    Rwallet_Config(crate::ton::rwallet::config::Config),
}
impl Config {
    pub fn limits(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::rwallet::limit::Limit> {
        match self {
            Config::Rwallet_Config(ref x) => &x.limits,
        }
    }

    pub fn start_at(&self) -> &crate::ton::int53 {
        match self {
            Config::Rwallet_Config(ref x) => &x.start_at,
        }
    }

    pub fn only(self) -> crate::ton::rwallet::config::Config {
        match self {
            Config::Rwallet_Config(x) => x,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::Rwallet_Config(crate::ton::rwallet::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::Rwallet_Config(x) => (crate::ConstructorNumber(0xfae7849a), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfae7849a)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xfae7849a) => {
                Ok(Config::Rwallet_Config(_de.read_bare::<crate::ton::rwallet::config::Config>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `rwallet.Limit`\n\n```text\nrwallet.limit seconds:int32 value:int64 = rwallet.Limit;\n```\n"]
pub enum Limit {
    Rwallet_Limit(crate::ton::rwallet::limit::Limit),
}
impl Limit {
    pub fn seconds(&self) -> &crate::ton::int32 {
        match self {
            Limit::Rwallet_Limit(ref x) => &x.seconds,
        }
    }

    pub fn value(&self) -> &crate::ton::int64 {
        match self {
            Limit::Rwallet_Limit(ref x) => &x.value,
        }
    }

    pub fn only(self) -> crate::ton::rwallet::limit::Limit {
        match self {
            Limit::Rwallet_Limit(x) => x,
        }
    }
}
impl Eq for Limit {}
impl Default for Limit {
    fn default() -> Self {
        Limit::Rwallet_Limit(crate::ton::rwallet::limit::Limit::default())
    }
}
impl crate::BoxedSerialize for Limit {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Limit::Rwallet_Limit(x) => (crate::ConstructorNumber(0x48def67e), x),
        }
    }
}
impl crate::BoxedDeserialize for Limit {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x48def67e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x48def67e) => {
                Ok(Limit::Rwallet_Limit(_de.read_bare::<crate::ton::rwallet::limit::Limit>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod accountstate;
pub mod actioninit;
pub mod config;
pub mod initialaccountstate;
pub mod limit;
