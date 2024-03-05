use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.sessionStats`\n\n```text\nengine.validator.sessionStats stats:(vector engine.validator.oneSessionStat) = engine.validator.SessionStats;\n```\n"]
pub struct SessionStats {
    pub stats: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::onesessionstat::OneSessionStat,
    >,
}
impl Eq for SessionStats {}
impl crate::BareSerialize for SessionStats {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8a0adbde)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SessionStats { stats } = self;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::validator::onesessionstat::OneSessionStat,
        >>(stats)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SessionStats {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let stats = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validator::onesessionstat::OneSessionStat,
            >>()?;
            Ok(Self { stats })
        }
    }
}
impl crate::IntoBoxed for SessionStats {
    type Boxed = crate::ton::engine::validator::SessionStats;

    fn into_boxed(self) -> crate::ton::engine::validator::SessionStats {
        crate::ton::engine::validator::SessionStats::Engine_Validator_SessionStats(self)
    }
}
