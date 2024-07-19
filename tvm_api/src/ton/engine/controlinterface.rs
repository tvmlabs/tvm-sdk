use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.controlInterface`\n\n```text\nengine.controlInterface id:int256 port:int allowed:(vector engine.controlProcess) = engine.ControlInterface;\n```\n"]
pub struct ControlInterface {
    pub id: crate::ton::int256,
    pub port: crate::ton::int,
    pub allowed: crate::ton::vector<crate::ton::engine::controlprocess::ControlProcess>,
}
impl Eq for ControlInterface {}
impl crate::BareSerialize for ControlInterface {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x31816fab)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ControlInterface { id, port, allowed } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        (allowed
            as &dyn crate::ton::VectoredBare<crate::ton::engine::controlprocess::ControlProcess>)
            .serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ControlInterface {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let allowed = < Vec < crate :: ton :: engine :: controlprocess :: ControlProcess > as crate :: ton :: VectoredBare < crate :: ton :: engine :: controlprocess :: ControlProcess >> :: deserialize (_de) ? ;
            Ok(Self { id, port, allowed })
        }
    }
}
impl crate::IntoBoxed for ControlInterface {
    type Boxed = crate::ton::engine::ControlInterface;

    fn into_boxed(self) -> crate::ton::engine::ControlInterface {
        crate::ton::engine::ControlInterface::Engine_ControlInterface(self)
    }
}
