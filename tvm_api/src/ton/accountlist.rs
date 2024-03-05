use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `accountList`\n\n```text\naccountList accounts:vector<fullAccountState> = AccountList;\n```\n"]
pub struct AccountList {
    pub accounts:
        crate::ton::vector<crate::ton::Bare, crate::ton::fullaccountstate::FullAccountState>,
}
impl Eq for AccountList {}
impl crate::BareSerialize for AccountList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x783eb255)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountList { accounts } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: fullaccountstate :: FullAccountState > > (accounts) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let accounts = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::fullaccountstate::FullAccountState,
            >>()?;
            Ok(Self { accounts })
        }
    }
}
impl crate::IntoBoxed for AccountList {
    type Boxed = crate::ton::AccountList;

    fn into_boxed(self) -> crate::ton::AccountList {
        crate::ton::AccountList::AccountList(self)
    }
}
