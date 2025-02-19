use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.id.Short`\n\n```text\nadnl.id.short id:int256 = adnl.id.Short;\n```\n"]
pub enum Short {
    Adnl_Id_Short(crate::ton::adnl::id::short::Short),
}
impl Short {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            Short::Adnl_Id_Short(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::adnl::id::short::Short {
        match self {
            Short::Adnl_Id_Short(x) => x,
        }
    }
}
impl Eq for Short {}
impl Default for Short {
    fn default() -> Self {
        Short::Adnl_Id_Short(crate::ton::adnl::id::short::Short::default())
    }
}
impl crate::BoxedSerialize for Short {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Short::Adnl_Id_Short(x) => (crate::ConstructorNumber(0x3e3f654f), x),
        }
    }
}
impl crate::BoxedDeserialize for Short {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3e3f654f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x3e3f654f) => {
                Ok(Short::Adnl_Id_Short(_de.read_bare::<crate::ton::adnl::id::short::Short>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod short;
