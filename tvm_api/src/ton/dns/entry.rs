use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.entry`\n\n```text\ndns.entry name:string category:int32 entry:dns.EntryData = dns.Entry;\n```\n"]
pub struct Entry {
    pub name: crate::ton::string,
    pub category: crate::ton::int32,
    pub entry: crate::ton::dns::EntryData,
}
impl Eq for Entry {}
impl crate::BareSerialize for Entry {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x922eaab8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Entry { name, category, entry } = self;
        _ser.write_bare::<crate::ton::string>(name)?;
        _ser.write_bare::<crate::ton::int32>(category)?;
        _ser.write_boxed::<crate::ton::dns::EntryData>(entry)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Entry {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let name = _de.read_bare::<crate::ton::string>()?;
            let category = _de.read_bare::<crate::ton::int32>()?;
            let entry = _de.read_boxed::<crate::ton::dns::EntryData>()?;
            Ok(Self { name, category, entry })
        }
    }
}
impl crate::IntoBoxed for Entry {
    type Boxed = crate::ton::dns::Entry;

    fn into_boxed(self) -> crate::ton::dns::Entry {
        crate::ton::dns::Entry::Dns_Entry(self)
    }
}
