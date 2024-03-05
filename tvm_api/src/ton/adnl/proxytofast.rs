use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.proxyToFast`\n\n```text\nadnl.proxyToFast ip:int port:int date:int signature:int256 = adnl.ProxyToSign;\n```\n"]
pub struct ProxyToFast {
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
    pub date: crate::ton::int,
    pub signature: crate::ton::int256,
}
impl Eq for ProxyToFast {}
impl crate::BareSerialize for ProxyToFast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb4ee21d6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ProxyToFast { ip, port, date, signature } = self;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_bare::<crate::ton::int>(date)?;
        _ser.write_bare::<crate::ton::int256>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ProxyToFast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let date = _de.read_bare::<crate::ton::int>()?;
            let signature = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { ip, port, date, signature })
        }
    }
}
impl crate::IntoBoxed for ProxyToFast {
    type Boxed = crate::ton::adnl::ProxyToSign;

    fn into_boxed(self) -> crate::ton::adnl::ProxyToSign {
        crate::ton::adnl::ProxyToSign::Adnl_ProxyToFast(self)
    }
}
