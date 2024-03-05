use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteserver.config.Local`\n\n```text\nliteserver.config.local id:PrivateKey port:int = liteserver.config.Local;\n\nliteserver.config.random.local port:int = liteserver.config.Local;\n```\n"]
pub enum Local {
    Liteserver_Config_Local(crate::ton::liteserver::config::local::Local),
    Liteserver_Config_Random_Local(crate::ton::liteserver::config::random::local::Local),
}
impl Local {
    pub fn id(&self) -> Option<&crate::ton::PrivateKey> {
        match self {
            Local::Liteserver_Config_Local(ref x) => Some(&x.id),
            _ => None,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            Local::Liteserver_Config_Local(ref x) => &x.port,
            Local::Liteserver_Config_Random_Local(ref x) => &x.port,
        }
    }
}
impl Eq for Local {}
impl Default for Local {
    fn default() -> Self {
        Local::Liteserver_Config_Local(crate::ton::liteserver::config::local::Local::default())
    }
}
impl crate::BoxedSerialize for Local {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Local::Liteserver_Config_Local(x) => (crate::ConstructorNumber(0x4673eb8f), x),
            Local::Liteserver_Config_Random_Local(x) => (crate::ConstructorNumber(0x7cc9453b), x),
        }
    }
}
impl crate::BoxedDeserialize for Local {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x4673eb8f), crate::ConstructorNumber(0x7cc9453b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x4673eb8f) => Ok(Local::Liteserver_Config_Local(
                _de.read_bare::<crate::ton::liteserver::config::local::Local>()?,
            )),
            crate::ConstructorNumber(0x7cc9453b) => Ok(Local::Liteserver_Config_Random_Local(
                _de.read_bare::<crate::ton::liteserver::config::random::local::Local>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod local;
pub mod random;
