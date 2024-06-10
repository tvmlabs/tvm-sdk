use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.hardforks`\n\n```text\ndb.state.hardforks blocks:(vector tonNode.blockIdExt) = db.state.Hardforks;\n```\n"]
pub struct Hardforks {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for Hardforks {}
impl crate::BareSerialize for Hardforks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x85f30d04)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Hardforks { blocks } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for Hardforks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { blocks })
        }
    }
}
impl crate::IntoBoxed for Hardforks {
    type Boxed = crate::ton::db::state::Hardforks;
    fn into_boxed(self) -> crate::ton::db::state::Hardforks {
        crate::ton::db::state::Hardforks::Db_State_Hardforks(self)
    }
}
