use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.gc`\n\n```text\nengine.gc ids:(vector int256) = engine.Gc;\n```\n"]
pub struct Gc {
    pub ids: crate::ton::vector<crate::ton::Bare, crate::ton::int256>,
}
impl Eq for Gc {}
impl crate::BareSerialize for Gc {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbfbd987b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Gc { ids } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>(ids)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Gc {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ids =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>()?;
            Ok(Self { ids })
        }
    }
}
impl crate::IntoBoxed for Gc {
    type Boxed = crate::ton::engine::Gc;

    fn into_boxed(self) -> crate::ton::engine::Gc {
        crate::ton::engine::Gc::Engine_Gc(self)
    }
}
