use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.oneSessionStat`\n\n```text\nengine.validator.oneSessionStat session_id:string stats:(vector engine.validator.oneStat) = engine.OneSessionStat;\n```\n"]
pub struct OneSessionStat {
    pub session_id: crate::ton::string,
    pub stats:
        crate::ton::vector<crate::ton::Bare, crate::ton::engine::validator::onestat::OneStat>,
}
impl Eq for OneSessionStat {}
impl crate::BareSerialize for OneSessionStat {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xadf42035)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let OneSessionStat { session_id, stats } = self;
        _ser.write_bare::<crate::ton::string>(session_id)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: engine :: validator :: onestat :: OneStat > > (stats) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for OneSessionStat {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let session_id = _de.read_bare::<crate::ton::string>()?;
            let stats = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validator::onestat::OneStat,
            >>()?;
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
