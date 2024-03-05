use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.entryDataAdnlAddress`\n\n```text\ndns.entryDataAdnlAddress adnl_address:AdnlAddress = dns.EntryData;\n```\n"]
pub struct EntryDataAdnlAddress {
    pub adnl_address: crate::ton::AdnlAddress,
}
impl Eq for EntryDataAdnlAddress {}
impl crate::BareSerialize for EntryDataAdnlAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbd98ba10)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let EntryDataAdnlAddress { adnl_address } = self;
        _ser.write_boxed::<crate::ton::AdnlAddress>(adnl_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for EntryDataAdnlAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let adnl_address = _de.read_boxed::<crate::ton::AdnlAddress>()?;
            Ok(Self { adnl_address })
        }
    }
}
impl crate::IntoBoxed for EntryDataAdnlAddress {
    type Boxed = crate::ton::dns::EntryData;

    fn into_boxed(self) -> crate::ton::dns::EntryData {
        crate::ton::dns::EntryData::Dns_EntryDataAdnlAddress(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.entryDataNextResolver`\n\n```text\ndns.entryDataNextResolver resolver:AccountAddress = dns.EntryData;\n```\n"]
pub struct EntryDataNextResolver {
    pub resolver: crate::ton::AccountAddress,
}
impl Eq for EntryDataNextResolver {}
impl crate::BareSerialize for EntryDataNextResolver {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x13b13dc8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let EntryDataNextResolver { resolver } = self;
        _ser.write_boxed::<crate::ton::AccountAddress>(resolver)?;
        Ok(())
    }
}
impl crate::BareDeserialize for EntryDataNextResolver {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let resolver = _de.read_boxed::<crate::ton::AccountAddress>()?;
            Ok(Self { resolver })
        }
    }
}
impl crate::IntoBoxed for EntryDataNextResolver {
    type Boxed = crate::ton::dns::EntryData;

    fn into_boxed(self) -> crate::ton::dns::EntryData {
        crate::ton::dns::EntryData::Dns_EntryDataNextResolver(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.entryDataSmcAddress`\n\n```text\ndns.entryDataSmcAddress smc_address:AccountAddress = dns.EntryData;\n```\n"]
pub struct EntryDataSmcAddress {
    pub smc_address: crate::ton::AccountAddress,
}
impl Eq for EntryDataSmcAddress {}
impl crate::BareSerialize for EntryDataSmcAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x97197a42)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let EntryDataSmcAddress { smc_address } = self;
        _ser.write_boxed::<crate::ton::AccountAddress>(smc_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for EntryDataSmcAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let smc_address = _de.read_boxed::<crate::ton::AccountAddress>()?;
            Ok(Self { smc_address })
        }
    }
}
impl crate::IntoBoxed for EntryDataSmcAddress {
    type Boxed = crate::ton::dns::EntryData;

    fn into_boxed(self) -> crate::ton::dns::EntryData {
        crate::ton::dns::EntryData::Dns_EntryDataSmcAddress(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.entryDataText`\n\n```text\ndns.entryDataText text:string = dns.EntryData;\n```\n"]
pub struct EntryDataText {
    pub text: crate::ton::string,
}
impl Eq for EntryDataText {}
impl crate::BareSerialize for EntryDataText {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd0c3a112)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let EntryDataText { text } = self;
        _ser.write_bare::<crate::ton::string>(text)?;
        Ok(())
    }
}
impl crate::BareDeserialize for EntryDataText {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let text = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { text })
        }
    }
}
impl crate::IntoBoxed for EntryDataText {
    type Boxed = crate::ton::dns::EntryData;

    fn into_boxed(self) -> crate::ton::dns::EntryData {
        crate::ton::dns::EntryData::Dns_EntryDataText(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.entryDataUnknown`\n\n```text\ndns.entryDataUnknown bytes:bytes = dns.EntryData;\n```\n"]
pub struct EntryDataUnknown {
    pub bytes: crate::ton::bytes,
}
impl Eq for EntryDataUnknown {}
impl crate::BareSerialize for EntryDataUnknown {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb35ad380)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let EntryDataUnknown { bytes: ref bytes_ } = self;
        _ser.write_bare::<crate::ton::bytes>(bytes_)?;
        Ok(())
    }
}
impl crate::BareDeserialize for EntryDataUnknown {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let bytes = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { bytes })
        }
    }
}
impl crate::IntoBoxed for EntryDataUnknown {
    type Boxed = crate::ton::dns::EntryData;

    fn into_boxed(self) -> crate::ton::dns::EntryData {
        crate::ton::dns::EntryData::Dns_EntryDataUnknown(self)
    }
}
