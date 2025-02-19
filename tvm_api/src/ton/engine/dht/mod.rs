use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.dht.Config`\n\n```text\nengine.dht.config dht:(vector engine.dht) gc:engine.gc = engine.dht.Config;\n```\n"]
pub enum Config {
    Engine_Dht_Config(crate::ton::engine::dht::config::Config),
}
impl Config {
    pub fn dht(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::dht::Dht> {
        match self {
            Config::Engine_Dht_Config(ref x) => &x.dht,
        }
    }

    pub fn gc(&self) -> &crate::ton::engine::gc::Gc {
        match self {
            Config::Engine_Dht_Config(ref x) => &x.gc,
        }
    }

    pub fn only(self) -> crate::ton::engine::dht::config::Config {
        match self {
            Config::Engine_Dht_Config(x) => x,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::Engine_Dht_Config(crate::ton::engine::dht::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::Engine_Dht_Config(x) => (crate::ConstructorNumber(0xf43d80c6), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf43d80c6)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf43d80c6) => Ok(Config::Engine_Dht_Config(
                _de.read_bare::<crate::ton::engine::dht::config::Config>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.dht`\n\n```text\nengine.dht id:int256 = engine.Dht;\n```\n"]
pub struct Dht {
    pub id: crate::ton::int256,
}
impl Eq for Dht {}
impl crate::BareSerialize for Dht {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5de9f2fa)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Dht { id } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Dht {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Dht {
    type Boxed = crate::ton::engine::Dht;

    fn into_boxed(self) -> crate::ton::engine::Dht {
        crate::ton::engine::Dht::Engine_Dht(self)
    }
}
pub mod config;
