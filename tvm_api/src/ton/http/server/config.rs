use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.server.config`\n\n```text\nhttp.server.config dhs:(vector http.server.dnsEntry) local_hosts:(vector http.server.host) = http.server.Config;\n```\n"]
pub struct Config {
    pub dhs: crate::ton::vector<crate::ton::Bare, crate::ton::http::server::dnsentry::DnsEntry>,
    pub local_hosts: crate::ton::vector<crate::ton::Bare, crate::ton::http::server::host::Host>,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3a1477fc)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config { dhs, local_hosts } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: http :: server :: dnsentry :: DnsEntry > > (dhs) ? ;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: http :: server :: host :: Host > > (local_hosts) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let dhs = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::http::server::dnsentry::DnsEntry,
            >>()?;
            let local_hosts = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: http :: server :: host :: Host > > () ? ;
            Ok(Self { dhs, local_hosts })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::http::server::Config;
    fn into_boxed(self) -> crate::ton::http::server::Config {
        crate::ton::http::server::Config::Http_Server_Config(self)
    }
}
