use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.server.host`\n\n```text\nhttp.server.host domains:(vector string) ip:int port:int adnl_id:adnl.id.short = http.server.Host;\n```\n"]
pub struct Host {
    pub domains: crate::ton::vector<crate::ton::Bare, crate::ton::string>,
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
    pub adnl_id: crate::ton::adnl::id::short::Short,
}
impl Eq for Host {}
impl crate::BareSerialize for Host {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc57de2a7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Host { domains, ip, port, adnl_id } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::string>>(domains)?;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_bare::<crate::ton::adnl::id::short::Short>(adnl_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Host {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let domains =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::string>>()?;
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let adnl_id = _de.read_bare::<crate::ton::adnl::id::short::Short>()?;
            Ok(Self { domains, ip, port, adnl_id })
        }
    }
}
impl crate::IntoBoxed for Host {
    type Boxed = crate::ton::http::server::Host;

    fn into_boxed(self) -> crate::ton::http::server::Host {
        crate::ton::http::server::Host::Http_Server_Host(self)
    }
}
