use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.fullNodeMaster`\n\n```text\nengine.validator.fullNodeMaster port:int adnl:int256 = engine.validator.FullNodeMaster;\n```\n"]
pub struct FullNodeMaster {
    pub port: crate::ton::int,
    pub adnl: crate::ton::int256,
}
impl Eq for FullNodeMaster {}
impl crate::BareSerialize for FullNodeMaster {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8485f668)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let FullNodeMaster { port, adnl } = self;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_bare::<crate::ton::int256>(adnl)?;
        Ok(())
    }
}
impl crate::BareDeserialize for FullNodeMaster {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let port = _de.read_bare::<crate::ton::int>()?;
            let adnl = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { port, adnl })
        }
    }
}
impl crate::IntoBoxed for FullNodeMaster {
    type Boxed = crate::ton::engine::validator::FullNodeMaster;

    fn into_boxed(self) -> crate::ton::engine::validator::FullNodeMaster {
        crate::ton::engine::validator::FullNodeMaster::Engine_Validator_FullNodeMaster(self)
    }
}
