use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rwallet.limit`\n\n```text\nrwallet.limit seconds:int32 value:int64 = rwallet.Limit;\n```\n"]
pub struct Limit {
    pub seconds: crate::ton::int32,
    pub value: crate::ton::int64,
}
impl Eq for Limit {}
impl crate::BareSerialize for Limit {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x48def67e)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Limit { seconds, value } = self;
        _ser.write_bare::<crate::ton::int32>(seconds)?;
        _ser.write_bare::<crate::ton::int64>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Limit {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let seconds = _de.read_bare::<crate::ton::int32>()?;
            let value = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { seconds, value })
        }
    }
}
impl crate::IntoBoxed for Limit {
    type Boxed = crate::ton::rwallet::Limit;
    fn into_boxed(self) -> crate::ton::rwallet::Limit {
        crate::ton::rwallet::Limit::Rwallet_Limit(self)
    }
}
