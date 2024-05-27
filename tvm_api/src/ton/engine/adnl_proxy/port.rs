use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.adnlProxy.port`\n\n```text\nengine.adnlProxy.port in_port:int out_port:int dst_ip:int dst_port:int proxy_type:adnl.Proxy = engine.adnlProxy.Port;\n```\n"]
pub struct Port {
    pub in_port: crate::ton::int,
    pub out_port: crate::ton::int,
    pub dst_ip: crate::ton::int,
    pub dst_port: crate::ton::int,
    pub proxy_type: crate::ton::adnl::Proxy,
}
impl Eq for Port {}
impl crate::BareSerialize for Port {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf901754a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Port { in_port, out_port, dst_ip, dst_port, proxy_type } = self;
        _ser.write_bare::<crate::ton::int>(in_port)?;
        _ser.write_bare::<crate::ton::int>(out_port)?;
        _ser.write_bare::<crate::ton::int>(dst_ip)?;
        _ser.write_bare::<crate::ton::int>(dst_port)?;
        _ser.write_boxed::<crate::ton::adnl::Proxy>(proxy_type)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Port {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let in_port = _de.read_bare::<crate::ton::int>()?;
            let out_port = _de.read_bare::<crate::ton::int>()?;
            let dst_ip = _de.read_bare::<crate::ton::int>()?;
            let dst_port = _de.read_bare::<crate::ton::int>()?;
            let proxy_type = _de.read_boxed::<crate::ton::adnl::Proxy>()?;
            Ok(Self { in_port, out_port, dst_ip, dst_port, proxy_type })
        }
    }
}
impl crate::IntoBoxed for Port {
    type Boxed = crate::ton::engine::adnl_proxy::Port;

    fn into_boxed(self) -> crate::ton::engine::adnl_proxy::Port {
        crate::ton::engine::adnl_proxy::Port::Engine_AdnlProxy_Port(self)
    }
}
