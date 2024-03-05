use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.resolved`\n\n```text\ndns.resolved entries:vector<dns.entry> = dns.Resolved;\n```\n"]
pub struct Resolved {
    pub entries: crate::ton::vector<crate::ton::Bare, crate::ton::dns::entry::Entry>,
}
impl Eq for Resolved {}
impl crate::BareSerialize for Resolved {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7c970596)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Resolved { entries } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::dns::entry::Entry>>(
            entries,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Resolved {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let entries = _de
                .read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::dns::entry::Entry>>(
                )?;
            Ok(Self { entries })
        }
    }
}
impl crate::IntoBoxed for Resolved {
    type Boxed = crate::ton::dns::Resolved;

    fn into_boxed(self) -> crate::ton::dns::Resolved {
        crate::ton::dns::Resolved::Dns_Resolved(self)
    }
}
