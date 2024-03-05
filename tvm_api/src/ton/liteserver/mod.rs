use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `liteserver.Desc`\n\n```text\nliteserver.desc id:PublicKey ip:int port:int = liteserver.Desc;\n```\n"]
pub enum Desc {
    Liteserver_Desc(crate::ton::liteserver::desc::Desc),
}
impl Desc {
    pub fn id(&self) -> &crate::ton::PublicKey {
        match self {
            Desc::Liteserver_Desc(ref x) => &x.id,
        }
    }

    pub fn ip(&self) -> &crate::ton::int {
        match self {
            Desc::Liteserver_Desc(ref x) => &x.ip,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            Desc::Liteserver_Desc(ref x) => &x.port,
        }
    }

    pub fn only(self) -> crate::ton::liteserver::desc::Desc {
        match self {
            Desc::Liteserver_Desc(x) => x,
        }
    }
}
impl Eq for Desc {}
impl Default for Desc {
    fn default() -> Self {
        Desc::Liteserver_Desc(crate::ton::liteserver::desc::Desc::default())
    }
}
impl crate::BoxedSerialize for Desc {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Desc::Liteserver_Desc(x) => (crate::ConstructorNumber(0xc449a474), x),
        }
    }
}
impl crate::BoxedDeserialize for Desc {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc449a474)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc449a474) => {
                Ok(Desc::Liteserver_Desc(_de.read_bare::<crate::ton::liteserver::desc::Desc>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod config;
pub mod desc;
