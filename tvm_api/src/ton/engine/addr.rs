use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.addr`\n\n```text\nengine.addr ip:int port:int categories:(vector int) priority_categories:(vector int) = engine.Addr;\n```\n"]
pub struct Addr {
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
    pub categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
    pub priority_categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for Addr {}
impl crate::BareSerialize for Addr {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xef311fec)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Addr { ip, port, categories, priority_categories } = self;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(categories)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(
            priority_categories,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Addr {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            let priority_categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { ip, port, categories, priority_categories })
        }
    }
}
impl crate::IntoBoxed for Addr {
    type Boxed = crate::ton::engine::Addr;

    fn into_boxed(self) -> crate::ton::engine::Addr {
        crate::ton::engine::Addr::Engine_Addr(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.addrProxy`\n\n```text\nengine.addrProxy in_ip:int in_port:int out_ip:int out_port:int \n          proxy_type:adnl.Proxy categories:(vector int) priority_categories:(vector int) = engine.Addr;\n```\n"]
pub struct AddrProxy {
    pub in_ip: crate::ton::int,
    pub in_port: crate::ton::int,
    pub out_ip: crate::ton::int,
    pub out_port: crate::ton::int,
    pub proxy_type: crate::ton::adnl::Proxy,
    pub categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
    pub priority_categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for AddrProxy {}
impl crate::BareSerialize for AddrProxy {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8adf6549)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddrProxy {
            in_ip,
            in_port,
            out_ip,
            out_port,
            proxy_type,
            categories,
            priority_categories,
        } = self;
        _ser.write_bare::<crate::ton::int>(in_ip)?;
        _ser.write_bare::<crate::ton::int>(in_port)?;
        _ser.write_bare::<crate::ton::int>(out_ip)?;
        _ser.write_bare::<crate::ton::int>(out_port)?;
        _ser.write_boxed::<crate::ton::adnl::Proxy>(proxy_type)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(categories)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(
            priority_categories,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddrProxy {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let in_ip = _de.read_bare::<crate::ton::int>()?;
            let in_port = _de.read_bare::<crate::ton::int>()?;
            let out_ip = _de.read_bare::<crate::ton::int>()?;
            let out_port = _de.read_bare::<crate::ton::int>()?;
            let proxy_type = _de.read_boxed::<crate::ton::adnl::Proxy>()?;
            let categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            let priority_categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self {
                in_ip,
                in_port,
                out_ip,
                out_port,
                proxy_type,
                categories,
                priority_categories,
            })
        }
    }
}
impl crate::IntoBoxed for AddrProxy {
    type Boxed = crate::ton::engine::Addr;

    fn into_boxed(self) -> crate::ton::engine::Addr {
        crate::ton::engine::Addr::Engine_AddrProxy(self)
    }
}
