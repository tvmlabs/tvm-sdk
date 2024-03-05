use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteclient.config.global`\n\n```text\nliteclient.config.global liteservers:(vector liteserver.desc) validator:validator.config.global = liteclient.config.Global;\n```\n"]
pub struct Global {
    pub liteservers: crate::ton::vector<crate::ton::Bare, crate::ton::liteserver::desc::Desc>,
    pub validator: crate::ton::validator::config::global::Global,
}
impl Eq for Global {}
impl crate::BareSerialize for Global {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x088dc0f8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Global { liteservers, validator } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: liteserver :: desc :: Desc > > (liteservers) ? ;
        _ser.write_bare::<crate::ton::validator::config::global::Global>(validator)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Global {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let liteservers = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: liteserver :: desc :: Desc > > () ? ;
            let validator = _de.read_bare::<crate::ton::validator::config::global::Global>()?;
            Ok(Self { liteservers, validator })
        }
    }
}
impl crate::IntoBoxed for Global {
    type Boxed = crate::ton::liteclient::config::Global;

    fn into_boxed(self) -> crate::ton::liteclient::config::Global {
        crate::ton::liteclient::config::Global::Liteclient_Config_Global(self)
    }
}
