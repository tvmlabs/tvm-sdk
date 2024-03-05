use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.server.dnsEntry`\n\n```text\nhttp.server.dnsEntry domain:string addr:adnl.id.short = http.server.DnsEntry;\n```\n"]
pub struct DnsEntry {
    pub domain: crate::ton::string,
    pub addr: crate::ton::adnl::id::short::Short,
}
impl Eq for DnsEntry {}
impl crate::BareSerialize for DnsEntry {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd8726096)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DnsEntry { domain, addr } = self;
        _ser.write_bare::<crate::ton::string>(domain)?;
        _ser.write_bare::<crate::ton::adnl::id::short::Short>(addr)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DnsEntry {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let domain = _de.read_bare::<crate::ton::string>()?;
            let addr = _de.read_bare::<crate::ton::adnl::id::short::Short>()?;
            Ok(Self { domain, addr })
        }
    }
}
impl crate::IntoBoxed for DnsEntry {
    type Boxed = crate::ton::http::server::DnsEntry;

    fn into_boxed(self) -> crate::ton::http::server::DnsEntry {
        crate::ton::http::server::DnsEntry::Http_Server_DnsEntry(self)
    }
}
