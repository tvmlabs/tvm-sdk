use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `raw.initialAccountState`\n\n```text\nraw.initialAccountState code:bytes data:bytes = InitialAccountState;\n```\n"]
pub struct InitialAccountState {
    pub code: crate::ton::bytes,
    pub data: crate::ton::bytes,
}
impl Eq for InitialAccountState {}
impl crate::BareSerialize for InitialAccountState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xebdb5c47)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let InitialAccountState { code, data } = self;
        _ser.write_bare::<crate::ton::bytes>(code)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for InitialAccountState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let code = _de.read_bare::<crate::ton::bytes>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { code, data })
        }
    }
}
impl crate::IntoBoxed for InitialAccountState {
    type Boxed = crate::ton::InitialAccountState;
    fn into_boxed(self) -> crate::ton::InitialAccountState {
        crate::ton::InitialAccountState::Raw_InitialAccountState(self)
    }
}
