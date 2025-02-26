use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `id.config.Local`\n\n```text\nid.config.local id:PrivateKey = id.config.Local;\n```\n"]
pub enum Local {
    Id_Config_Local(crate::ton::id::config::local::Local),
}
impl Local {
    pub fn id(&self) -> &crate::ton::PrivateKey {
        match self {
            Local::Id_Config_Local(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::id::config::local::Local {
        match self {
            Local::Id_Config_Local(x) => x,
        }
    }
}
impl Eq for Local {}
impl Default for Local {
    fn default() -> Self {
        Local::Id_Config_Local(crate::ton::id::config::local::Local::default())
    }
}
impl crate::BoxedSerialize for Local {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Local::Id_Config_Local(x) => (crate::ConstructorNumber(0x92a9c78e), x),
        }
    }
}
impl crate::BoxedDeserialize for Local {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x92a9c78e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x92a9c78e) => {
                Ok(Local::Id_Config_Local(_de.read_bare::<crate::ton::id::config::local::Local>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod local;
