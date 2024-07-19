use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.oneSessionStat`\n\n```text\nengine.validator.oneSessionStat session_id:string stats:(vector engine.validator.oneStat) = engine.OneSessionStat;\n```\n"]
pub struct OneSessionStat {
    pub session_id: crate::ton::string,
    pub stats: crate::ton::vector<crate::ton::engine::validator::onestat::OneStat>,
}
impl Eq for OneSessionStat {}
impl crate::BareSerialize for OneSessionStat {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xadf42035)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let OneSessionStat { session_id, stats } = self;
        _ser.write_bare::<crate::ton::string>(session_id)?;
        (stats as &dyn crate::ton::VectoredBare<crate::ton::engine::validator::onestat::OneStat>)
            .serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for OneSessionStat {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let session_id = _de.read_bare::<crate::ton::string>()?;
            let stats = < Vec < crate :: ton :: engine :: validator :: onestat :: OneStat > as crate :: ton :: VectoredBare < crate :: ton :: engine :: validator :: onestat :: OneStat >> :: deserialize (_de) ? ;
            Ok(Self { session_id, stats })
        }
    }
}
impl crate::IntoBoxed for OneSessionStat {
    type Boxed = crate::ton::engine::OneSessionStat;

    fn into_boxed(self) -> crate::ton::engine::OneSessionStat {
        crate::ton::engine::OneSessionStat::Engine_Validator_OneSessionStat(self)
    }
}
