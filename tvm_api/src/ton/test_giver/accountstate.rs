use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `testGiver.accountState`\n\n```text\ntestGiver.accountState seqno:int32 = AccountState;\n```\n"]
pub struct AccountState {
    pub seqno: crate::ton::int32,
}
impl Eq for AccountState {}
impl crate::BareSerialize for AccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd67779aa)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountState { seqno } = self;
        _ser.write_bare::<crate::ton::int32>(seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let seqno = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { seqno })
        }
    }
}
impl crate::IntoBoxed for AccountState {
    type Boxed = crate::ton::AccountState;

    fn into_boxed(self) -> crate::ton::AccountState {
        crate::ton::AccountState::TestGiver_AccountState(self)
    }
}
