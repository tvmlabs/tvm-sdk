use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.adnl`\n\n```text\nengine.adnl id:int256 category:int = engine.Adnl;\n```\n"]
pub struct Adnl {
    pub id: crate::ton::int256,
    pub category: crate::ton::int,
}
impl Eq for Adnl {}
impl crate::BareSerialize for Adnl {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x62d76550)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Adnl { id, category } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::int>(category)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Adnl {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let category = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, category })
        }
    }
}
impl crate::IntoBoxed for Adnl {
    type Boxed = crate::ton::engine::Adnl;
    fn into_boxed(self) -> crate::ton::engine::Adnl {
        crate::ton::engine::Adnl::Engine_Adnl(self)
    }
}
