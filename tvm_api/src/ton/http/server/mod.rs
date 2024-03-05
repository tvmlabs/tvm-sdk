use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `http.server.Config`\n\n```text\nhttp.server.config dhs:(vector http.server.dnsEntry) local_hosts:(vector http.server.host) = http.server.Config;\n```\n"]
pub enum Config {
    Http_Server_Config(crate::ton::http::server::config::Config),
}
impl Config {
    pub fn dhs(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::http::server::dnsentry::DnsEntry> {
        match self {
            Config::Http_Server_Config(ref x) => &x.dhs,
        }
    }

    pub fn local_hosts(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::http::server::host::Host> {
        match self {
            Config::Http_Server_Config(ref x) => &x.local_hosts,
        }
    }

    pub fn only(self) -> crate::ton::http::server::config::Config {
        match self {
            Config::Http_Server_Config(x) => x,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::Http_Server_Config(crate::ton::http::server::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::Http_Server_Config(x) => (crate::ConstructorNumber(0x3a1477fc), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3a1477fc)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x3a1477fc) => Ok(Config::Http_Server_Config(
                _de.read_bare::<crate::ton::http::server::config::Config>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `http.server.DnsEntry`\n\n```text\nhttp.server.dnsEntry domain:string addr:adnl.id.short = http.server.DnsEntry;\n```\n"]
pub enum DnsEntry {
    Http_Server_DnsEntry(crate::ton::http::server::dnsentry::DnsEntry),
}
impl DnsEntry {
    pub fn addr(&self) -> &crate::ton::adnl::id::short::Short {
        match self {
            DnsEntry::Http_Server_DnsEntry(ref x) => &x.addr,
        }
    }

    pub fn domain(&self) -> &crate::ton::string {
        match self {
            DnsEntry::Http_Server_DnsEntry(ref x) => &x.domain,
        }
    }

    pub fn only(self) -> crate::ton::http::server::dnsentry::DnsEntry {
        match self {
            DnsEntry::Http_Server_DnsEntry(x) => x,
        }
    }
}
impl Eq for DnsEntry {}
impl Default for DnsEntry {
    fn default() -> Self {
        DnsEntry::Http_Server_DnsEntry(crate::ton::http::server::dnsentry::DnsEntry::default())
    }
}
impl crate::BoxedSerialize for DnsEntry {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DnsEntry::Http_Server_DnsEntry(x) => (crate::ConstructorNumber(0xd8726096), x),
        }
    }
}
impl crate::BoxedDeserialize for DnsEntry {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd8726096)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd8726096) => Ok(DnsEntry::Http_Server_DnsEntry(
                _de.read_bare::<crate::ton::http::server::dnsentry::DnsEntry>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `http.server.Host`\n\n```text\nhttp.server.host domains:(vector string) ip:int port:int adnl_id:adnl.id.short = http.server.Host;\n```\n"]
pub enum Host {
    Http_Server_Host(crate::ton::http::server::host::Host),
}
impl Host {
    pub fn adnl_id(&self) -> &crate::ton::adnl::id::short::Short {
        match self {
            Host::Http_Server_Host(ref x) => &x.adnl_id,
        }
    }

    pub fn domains(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::string> {
        match self {
            Host::Http_Server_Host(ref x) => &x.domains,
        }
    }

    pub fn ip(&self) -> &crate::ton::int {
        match self {
            Host::Http_Server_Host(ref x) => &x.ip,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            Host::Http_Server_Host(ref x) => &x.port,
        }
    }

    pub fn only(self) -> crate::ton::http::server::host::Host {
        match self {
            Host::Http_Server_Host(x) => x,
        }
    }
}
impl Eq for Host {}
impl Default for Host {
    fn default() -> Self {
        Host::Http_Server_Host(crate::ton::http::server::host::Host::default())
    }
}
impl crate::BoxedSerialize for Host {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Host::Http_Server_Host(x) => (crate::ConstructorNumber(0xc57de2a7), x),
        }
    }
}
impl crate::BoxedDeserialize for Host {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc57de2a7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc57de2a7) => {
                Ok(Host::Http_Server_Host(_de.read_bare::<crate::ton::http::server::host::Host>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod config;
pub mod dnsentry;
pub mod host;
