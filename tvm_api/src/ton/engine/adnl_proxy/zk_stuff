use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.adnlProxy.Config`\n\n```text\nengine.adnlProxy.config ports:(vector engine.adnlProxy.port) = engine.adnlProxy.Config;\n```\n"]
pub enum Config {
    Engine_AdnlProxy_Config(crate::ton::engine::adnl_proxy::config::Config),
}
impl Config {
    pub fn ports(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::adnl_proxy::port::Port> {
        match self {
            Config::Engine_AdnlProxy_Config(ref x) => &x.ports,
        }
    }
    pub fn only(self) -> crate::ton::engine::adnl_proxy::config::Config {
        match self {
            Config::Engine_AdnlProxy_Config(x) => x,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::Engine_AdnlProxy_Config(crate::ton::engine::adnl_proxy::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::Engine_AdnlProxy_Config(x) => (crate::ConstructorNumber(0x6e264101), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6e264101)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x6e264101) => Ok(Config::Engine_AdnlProxy_Config(
                _de.read_bare::<crate::ton::engine::adnl_proxy::config::Config>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.adnlProxy.Port`\n\n```text\nengine.adnlProxy.port in_port:int out_port:int dst_ip:int dst_port:int proxy_type:adnl.Proxy = engine.adnlProxy.Port;\n```\n"]
pub enum Port {
    Engine_AdnlProxy_Port(crate::ton::engine::adnl_proxy::port::Port),
}
impl Port {
    pub fn dst_ip(&self) -> &crate::ton::int {
        match self {
            Port::Engine_AdnlProxy_Port(ref x) => &x.dst_ip,
        }
    }
    pub fn dst_port(&self) -> &crate::ton::int {
        match self {
            Port::Engine_AdnlProxy_Port(ref x) => &x.dst_port,
        }
    }
    pub fn in_port(&self) -> &crate::ton::int {
        match self {
            Port::Engine_AdnlProxy_Port(ref x) => &x.in_port,
        }
    }
    pub fn out_port(&self) -> &crate::ton::int {
        match self {
            Port::Engine_AdnlProxy_Port(ref x) => &x.out_port,
        }
    }
    pub fn proxy_type(&self) -> &crate::ton::adnl::Proxy {
        match self {
            Port::Engine_AdnlProxy_Port(ref x) => &x.proxy_type,
        }
    }
    pub fn only(self) -> crate::ton::engine::adnl_proxy::port::Port {
        match self {
            Port::Engine_AdnlProxy_Port(x) => x,
        }
    }
}
impl Eq for Port {}
impl Default for Port {
    fn default() -> Self {
        Port::Engine_AdnlProxy_Port(crate::ton::engine::adnl_proxy::port::Port::default())
    }
}
impl crate::BoxedSerialize for Port {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Port::Engine_AdnlProxy_Port(x) => (crate::ConstructorNumber(0xf901754a), x),
        }
    }
}
impl crate::BoxedDeserialize for Port {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf901754a)]
    }
    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf901754a) => Ok(Port::Engine_AdnlProxy_Port(
                _de.read_bare::<crate::ton::engine::adnl_proxy::port::Port>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod config;
pub mod port;
