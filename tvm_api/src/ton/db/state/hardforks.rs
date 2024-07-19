use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.hardforks`\n\n```text\ndb.state.hardforks blocks:(vector tonNode.blockIdExt) = db.state.Hardforks;\n```\n"]
pub struct Hardforks {
    pub blocks: crate::ton::vector<crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for Hardforks {}
impl crate::BareSerialize for Hardforks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x85f30d04)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Hardforks { blocks } = self;
        (blocks as &dyn crate::ton::VectoredBare<crate::ton::ton_node::blockidext::BlockIdExt>)
            .serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Hardforks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks =
                <Vec<crate::ton::ton_node::blockidext::BlockIdExt> as crate::ton::VectoredBare<
                    crate::ton::ton_node::blockidext::BlockIdExt,
                >>::deserialize(_de)?;
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
