use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.controlQueryError`\n\n```text\nengine.validator.controlQueryError code:int message:string = engine.validator.ControlQueryError;\n```\n"]
pub struct ControlQueryError {
    pub code: crate::ton::int,
    pub message: crate::ton::string,
}
impl Eq for ControlQueryError {}
impl crate::BareSerialize for ControlQueryError {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x77269a1f)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ControlQueryError { code, message } = self;
        _ser.write_bare::<crate::ton::int>(code)?;
        _ser.write_bare::<crate::ton::string>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ControlQueryError {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let code = _de.read_bare::<crate::ton::int>()?;
            let message = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { code, message })
        }
    }
}
impl crate::IntoBoxed for ControlQueryError {
    type Boxed = crate::ton::engine::validator::ControlQueryError;
    fn into_boxed(self) -> crate::ton::engine::validator::ControlQueryError {
        crate::ton::engine::validator::ControlQueryError::Engine_Validator_ControlQueryError(self)
    }
}
