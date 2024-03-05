use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.id.short`\n\n```text\nadnl.id.short id:int256 = adnl.id.Short;\n```\n"]
pub struct Short {
    pub id: crate::ton::int256,
}
impl Eq for Short {}
impl crate::BareSerialize for Short {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3e3f654f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Short { id } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Short {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Short {
    type Boxed = crate::ton::adnl::id::Short;

    fn into_boxed(self) -> crate::ton::adnl::id::Short {
        crate::ton::adnl::id::Short::Adnl_Id_Short(self)
    }
}
