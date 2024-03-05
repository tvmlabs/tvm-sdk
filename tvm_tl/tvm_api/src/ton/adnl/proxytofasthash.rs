use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.proxyToFastHash`\n\n```text\nadnl.proxyToFastHash ip:int port:int date:int data_hash:int256 shared_secret:int256 = adnl.ProxyTo;\n```\n"]
pub struct ProxyToFastHash {
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
    pub date: crate::ton::int,
    pub data_hash: crate::ton::int256,
    pub shared_secret: crate::ton::int256,
}
impl Eq for ProxyToFastHash {}
impl crate::BareSerialize for ProxyToFastHash {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xddbdf85e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ProxyToFastHash { ip, port, date, data_hash, shared_secret } = self;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_bare::<crate::ton::int>(date)?;
        _ser.write_bare::<crate::ton::int256>(data_hash)?;
        _ser.write_bare::<crate::ton::int256>(shared_secret)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ProxyToFastHash {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let date = _de.read_bare::<crate::ton::int>()?;
            let data_hash = _de.read_bare::<crate::ton::int256>()?;
            let shared_secret = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { ip, port, date, data_hash, shared_secret })
        }
    }
}
impl crate::IntoBoxed for ProxyToFastHash {
    type Boxed = crate::ton::adnl::ProxyTo;

    fn into_boxed(self) -> crate::ton::adnl::ProxyTo {
        crate::ton::adnl::ProxyTo::Adnl_ProxyToFastHash(self)
    }
}
