use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.dht.config`\n\n```text\nengine.dht.config dht:(vector engine.dht) gc:engine.gc = engine.dht.Config;\n```\n"]
pub struct Config {
    pub dht: crate::ton::vector<crate::ton::Bare, crate::ton::engine::dht::Dht>,
    pub gc: crate::ton::engine::gc::Gc,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf43d80c6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config { dht, gc } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::engine::dht::Dht>>(dht)?;
        _ser.write_bare::<crate::ton::engine::gc::Gc>(gc)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let dht = _de
                .read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::engine::dht::Dht>>(
                )?;
            let gc = _de.read_bare::<crate::ton::engine::gc::Gc>()?;
            Ok(Self { dht, gc })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::engine::dht::Config;

    fn into_boxed(self) -> crate::ton::engine::dht::Config {
        crate::ton::engine::dht::Config::Engine_Dht_Config(self)
    }
}
