use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `accountRevisionList`\n\n```text\naccountRevisionList revisions:vector<fullAccountState> = AccountRevisionList;\n```\n"]
pub struct AccountRevisionList {
    pub revisions:
        crate::ton::vector<crate::ton::Bare, crate::ton::fullaccountstate::FullAccountState>,
}
impl Eq for AccountRevisionList {}
impl crate::BareSerialize for AccountRevisionList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1f6c64ca)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountRevisionList { revisions } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: fullaccountstate :: FullAccountState > > (revisions) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountRevisionList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let revisions = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::fullaccountstate::FullAccountState,
            >>()?;
            Ok(Self { revisions })
        }
    }
}
impl crate::IntoBoxed for AccountRevisionList {
    type Boxed = crate::ton::AccountRevisionList;

    fn into_boxed(self) -> crate::ton::AccountRevisionList {
        crate::ton::AccountRevisionList::AccountRevisionList(self)
    }
}
