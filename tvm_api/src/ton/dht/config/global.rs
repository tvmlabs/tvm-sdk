use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.config.global`\n\n```text\ndht.config.global static_nodes:dht.nodes k:int a:int = dht.config.Global;\n```\n"]
pub struct Global {
    pub static_nodes: crate::ton::dht::nodes::Nodes,
    pub k: crate::ton::int,
    pub a: crate::ton::int,
}
impl Eq for Global {}
impl crate::BareSerialize for Global {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x84ceca07)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Global { static_nodes, k, a } = self;
        _ser.write_bare::<crate::ton::dht::nodes::Nodes>(static_nodes)?;
        _ser.write_bare::<crate::ton::int>(k)?;
        _ser.write_bare::<crate::ton::int>(a)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Global {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let static_nodes = _de.read_bare::<crate::ton::dht::nodes::Nodes>()?;
            let k = _de.read_bare::<crate::ton::int>()?;
            let a = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { static_nodes, k, a })
        }
    }
}
impl crate::IntoBoxed for Global {
    type Boxed = crate::ton::dht::config::Global;

    fn into_boxed(self) -> crate::ton::dht::config::Global {
        crate::ton::dht::config::Global::Dht_Config_Global(self)
    }
}
