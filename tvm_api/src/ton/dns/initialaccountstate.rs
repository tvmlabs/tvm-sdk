use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.initialAccountState`\n\n```text\ndns.initialAccountState public_key:string wallet_id:int64 = InitialAccountState;\n```\n"]
pub struct InitialAccountState {
    pub public_key: crate::ton::string,
    pub wallet_id: crate::ton::int64,
}
impl Eq for InitialAccountState {}
impl crate::BareSerialize for InitialAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6dcba4bf)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let InitialAccountState { public_key, wallet_id } = self;
        _ser.write_bare::<crate::ton::string>(public_key)?;
        _ser.write_bare::<crate::ton::int64>(wallet_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for InitialAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let public_key = _de.read_bare::<crate::ton::string>()?;
            let wallet_id = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { public_key, wallet_id })
        }
    }
}
impl crate::IntoBoxed for InitialAccountState {
    type Boxed = crate::ton::InitialAccountState;

    fn into_boxed(self) -> crate::ton::InitialAccountState {
        crate::ton::InitialAccountState::Dns_InitialAccountState(self)
    }
}
