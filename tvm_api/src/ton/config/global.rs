use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `config.global`\n\n```text\nconfig.global adnl:adnl.config.global dht:dht.config.global validator:validator.config.global = config.Global;\n```\n"]
pub struct Global {
    pub adnl: crate::ton::adnl::config::global::Global,
    pub dht: crate::ton::dht::config::global::Global,
    pub validator: crate::ton::validator::config::global::Global,
}
impl Eq for Global {}
impl crate::BareSerialize for Global {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf4269fd2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Global { adnl, dht, validator } = self;
        _ser.write_bare::<crate::ton::adnl::config::global::Global>(adnl)?;
        _ser.write_bare::<crate::ton::dht::config::global::Global>(dht)?;
        _ser.write_bare::<crate::ton::validator::config::global::Global>(validator)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Global {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let adnl = _de.read_bare::<crate::ton::adnl::config::global::Global>()?;
            let dht = _de.read_bare::<crate::ton::dht::config::global::Global>()?;
            let validator = _de.read_bare::<crate::ton::validator::config::global::Global>()?;
            Ok(Self { adnl, dht, validator })
        }
    }
}
impl crate::IntoBoxed for Global {
    type Boxed = crate::ton::config::Global;

    fn into_boxed(self) -> crate::ton::config::Global {
        crate::ton::config::Global::Config_Global(self)
    }
}
