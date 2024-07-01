use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.controlProcess`\n\n```text\nengine.controlProcess id:int256 permissions:int = engine.ControlProcess;\n```\n"]
pub struct ControlProcess {
    pub id: crate::ton::int256,
    pub permissions: crate::ton::int,
}
impl Eq for ControlProcess {}
impl crate::BareSerialize for ControlProcess {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6ac04817)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ControlProcess { id, permissions } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::int>(permissions)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ControlProcess {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let permissions = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, permissions })
        }
    }
}
impl crate::IntoBoxed for ControlProcess {
    type Boxed = crate::ton::engine::ControlProcess;
    fn into_boxed(self) -> crate::ton::engine::ControlProcess {
        crate::ton::engine::ControlProcess::Engine_ControlProcess(self)
    }
}
