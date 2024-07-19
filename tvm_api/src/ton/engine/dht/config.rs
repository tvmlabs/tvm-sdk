use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.dht.config`\n\n```text\nengine.dht.config dht:(vector engine.dht) gc:engine.gc = engine.dht.Config;\n```\n"]
pub struct Config {
    pub dht: crate::ton::vector<crate::ton::engine::dht::Dht>,
    pub gc: crate::ton::engine::gc::Gc,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf43d80c6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config { dht, gc } = self;
        (dht as &dyn crate::ton::VectoredBare<crate::ton::engine::dht::Dht>).serialize(_ser)?;
        _ser.write_bare::<crate::ton::engine::gc::Gc>(gc)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let dht = <Vec<crate::ton::engine::dht::Dht> as crate::ton::VectoredBare<
                crate::ton::engine::dht::Dht,
            >>::deserialize(_de)?;
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
