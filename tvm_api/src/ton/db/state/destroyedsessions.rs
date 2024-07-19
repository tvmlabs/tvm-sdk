use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.destroyedSessions`\n\n```text\ndb.state.destroyedSessions sessions:(vector int256) = db.state.DestroyedSessions;\n```\n"]
pub struct DestroyedSessions {
    pub sessions: crate::ton::vector<crate::ton::int256>,
}
impl Eq for DestroyedSessions {}
impl crate::BareSerialize for DestroyedSessions {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xada8d984)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DestroyedSessions { sessions } = self;
        (sessions as &dyn crate::ton::VectoredBare<crate::ton::int256>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DestroyedSessions {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let sessions = <Vec<crate::ton::int256> as crate::ton::VectoredBare<
                crate::ton::int256,
            >>::deserialize(_de)?;
            Ok(Self { sessions })
        }
    }
}
impl crate::IntoBoxed for DestroyedSessions {
    type Boxed = crate::ton::db::state::DestroyedSessions;

    fn into_boxed(self) -> crate::ton::db::state::DestroyedSessions {
        crate::ton::db::state::DestroyedSessions::Db_State_DestroyedSessions(self)
    }
}
