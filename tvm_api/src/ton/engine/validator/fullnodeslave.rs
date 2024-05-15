use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.fullNodeSlave`\n\n```text\nengine.validator.fullNodeSlave ip:int port:int adnl:PublicKey = engine.validator.FullNodeSlave;\n```\n"]
pub struct FullNodeSlave {
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
    pub adnl: crate::ton::PublicKey,
}
impl Eq for FullNodeSlave {}
impl crate::BareSerialize for FullNodeSlave {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x88256b79)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let FullNodeSlave { ip, port, adnl } = self;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_boxed::<crate::ton::PublicKey>(adnl)?;
        Ok(())
    }
}
impl crate::BareDeserialize for FullNodeSlave {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let adnl = _de.read_boxed::<crate::ton::PublicKey>()?;
            Ok(Self { ip, port, adnl })
        }
    }
}
impl crate::IntoBoxed for FullNodeSlave {
    type Boxed = crate::ton::engine::validator::FullNodeSlave;
    fn into_boxed(self) -> crate::ton::engine::validator::FullNodeSlave {
        crate::ton::engine::validator::FullNodeSlave::Engine_Validator_FullNodeSlave(self)
    }
}
