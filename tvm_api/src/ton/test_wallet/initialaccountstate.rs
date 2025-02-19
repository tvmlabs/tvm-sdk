use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `testWallet.initialAccountState`\n\n```text\ntestWallet.initialAccountState public_key:string = InitialAccountState;\n```\n"]
pub struct InitialAccountState {
    pub public_key: crate::ton::string,
}
impl Eq for InitialAccountState {}
impl crate::BareSerialize for InitialAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x30d6bf64)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let InitialAccountState { public_key } = self;
        _ser.write_bare::<crate::ton::string>(public_key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for InitialAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let public_key = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { public_key })
        }
    }
}
impl crate::IntoBoxed for InitialAccountState {
    type Boxed = crate::ton::InitialAccountState;

    fn into_boxed(self) -> crate::ton::InitialAccountState {
        crate::ton::InitialAccountState::TestWallet_InitialAccountState(self)
    }
}
