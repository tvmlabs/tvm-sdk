use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `control.config.Local`\n\n```text\ncontrol.config.local priv:PrivateKey pub:int256 port:int = control.config.Local;\n```\n"]
pub enum Local {
    Control_Config_Local(crate::ton::control::config::local::Local),
}
impl Local {
    pub fn port(&self) -> &crate::ton::int {
        match self {
            Local::Control_Config_Local(ref x) => &x.port,
        }
    }

    pub fn priv_(&self) -> &crate::ton::PrivateKey {
        match self {
            Local::Control_Config_Local(ref x) => &x.priv_,
        }
    }

    pub fn pub_(&self) -> &crate::ton::int256 {
        match self {
            Local::Control_Config_Local(ref x) => &x.pub_,
        }
    }

    pub fn only(self) -> crate::ton::control::config::local::Local {
        match self {
            Local::Control_Config_Local(x) => x,
        }
    }
}
impl Eq for Local {}
impl Default for Local {
    fn default() -> Self {
        Local::Control_Config_Local(crate::ton::control::config::local::Local::default())
    }
}
impl crate::BoxedSerialize for Local {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Local::Control_Config_Local(x) => (crate::ConstructorNumber(0x751deced), x),
        }
    }
}
impl crate::BoxedDeserialize for Local {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x751deced)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x751deced) => Ok(Local::Control_Config_Local(
                _de.read_bare::<crate::ton::control::config::local::Local>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod local;
