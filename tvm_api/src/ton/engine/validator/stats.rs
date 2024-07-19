use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.stats`\n\n```text\nengine.validator.stats stats:(vector engine.validator.oneStat) = engine.validator.Stats;\n```\n"]
pub struct Stats {
    pub stats: crate::ton::vector<crate::ton::engine::validator::onestat::OneStat>,
}
impl Eq for Stats {}
impl crate::BareSerialize for Stats {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5d49d36f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Stats { stats } = self;
        (stats as &dyn crate::ton::VectoredBare<crate::ton::engine::validator::onestat::OneStat>)
            .serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Stats {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let stats = < Vec < crate :: ton :: engine :: validator :: onestat :: OneStat > as crate :: ton :: VectoredBare < crate :: ton :: engine :: validator :: onestat :: OneStat >> :: deserialize (_de) ? ;
            Ok(Self { stats })
        }
    }
}
impl crate::IntoBoxed for Stats {
    type Boxed = crate::ton::engine::validator::Stats;

    fn into_boxed(self) -> crate::ton::engine::validator::Stats {
        crate::ton::engine::validator::Stats::Engine_Validator_Stats(self)
    }
}
