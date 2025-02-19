use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.debug.Verbosity`\n\n```text\nliteServer.debug.verbosity value:int = liteServer.debug.Verbosity;\n```\n"]
pub enum Verbosity {
    LiteServer_Debug_Verbosity(crate::ton::lite_server::debug::verbosity::Verbosity),
}
impl Verbosity {
    pub fn value(&self) -> &crate::ton::int {
        match self {
            Verbosity::LiteServer_Debug_Verbosity(ref x) => &x.value,
        }
    }

    pub fn only(self) -> crate::ton::lite_server::debug::verbosity::Verbosity {
        match self {
            Verbosity::LiteServer_Debug_Verbosity(x) => x,
        }
    }
}
impl Eq for Verbosity {}
impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::LiteServer_Debug_Verbosity(
            crate::ton::lite_server::debug::verbosity::Verbosity::default(),
        )
    }
}
impl crate::BoxedSerialize for Verbosity {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Verbosity::LiteServer_Debug_Verbosity(x) => (crate::ConstructorNumber(0x5d404733), x),
        }
    }
}
impl crate::BoxedDeserialize for Verbosity {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5d404733)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5d404733) => Ok(Verbosity::LiteServer_Debug_Verbosity(
                _de.read_bare::<crate::ton::lite_server::debug::verbosity::Verbosity>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod verbosity;
