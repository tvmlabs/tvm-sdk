use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.adnlProxy.config`\n\n```text\nengine.adnlProxy.config ports:(vector engine.adnlProxy.port) = engine.adnlProxy.Config;\n```\n"]
pub struct Config {
    pub ports: crate::ton::vector<crate::ton::Bare, crate::ton::engine::adnl_proxy::port::Port>,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6e264101)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config { ports } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: engine :: adnl_proxy :: port :: Port > > (ports) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ports =
                _de.read_bare::<crate::ton::vector<
                    crate::ton::Bare,
                    crate::ton::engine::adnl_proxy::port::Port,
                >>()?;
            Ok(Self { ports })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::engine::adnl_proxy::Config;

    fn into_boxed(self) -> crate::ton::engine::adnl_proxy::Config {
        crate::ton::engine::adnl_proxy::Config::Engine_AdnlProxy_Config(self)
    }
}
