use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.server.config`\n\n```text\nhttp.server.config dhs:(vector http.server.dnsEntry) local_hosts:(vector http.server.host) = http.server.Config;\n```\n"]
pub struct Config {
    pub dhs: crate::ton::vector<crate::ton::http::server::dnsentry::DnsEntry>,
    pub local_hosts: crate::ton::vector<crate::ton::http::server::host::Host>,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3a1477fc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config { dhs, local_hosts } = self;
        (dhs as &dyn crate::ton::VectoredBare<crate::ton::http::server::dnsentry::DnsEntry>)
            .serialize(_ser)?;
        (local_hosts as &dyn crate::ton::VectoredBare<crate::ton::http::server::host::Host>)
            .serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let dhs =
                <Vec<crate::ton::http::server::dnsentry::DnsEntry> as crate::ton::VectoredBare<
                    crate::ton::http::server::dnsentry::DnsEntry,
                >>::deserialize(_de)?;
            let local_hosts =
                <Vec<crate::ton::http::server::host::Host> as crate::ton::VectoredBare<
                    crate::ton::http::server::host::Host,
                >>::deserialize(_de)?;
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
