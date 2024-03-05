use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dns.Action`\n\n```text\ndns.actionDelete name:string category:int32 = dns.Action;\n\ndns.actionDeleteAll = dns.Action;\n\ndns.actionSet entry:dns.entry = dns.Action;\n```\n"]
pub enum Action {
    Dns_ActionDelete(crate::ton::dns::action::ActionDelete),
    Dns_ActionDeleteAll,
    Dns_ActionSet(crate::ton::dns::action::ActionSet),
}
impl Action {
    pub fn category(&self) -> Option<&crate::ton::int32> {
        match self {
            Action::Dns_ActionDelete(ref x) => Some(&x.category),
            _ => None,
        }
    }

    pub fn entry(&self) -> Option<&crate::ton::dns::entry::Entry> {
        match self {
            Action::Dns_ActionSet(ref x) => Some(&x.entry),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&crate::ton::string> {
        match self {
            Action::Dns_ActionDelete(ref x) => Some(&x.name),
            _ => None,
        }
    }
}
impl Eq for Action {}
impl Default for Action {
    fn default() -> Self {
        Action::Dns_ActionDelete(crate::ton::dns::action::ActionDelete::default())
    }
}
impl crate::BoxedSerialize for Action {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Action::Dns_ActionDelete(x) => (crate::ConstructorNumber(0x2e34b7e2), x),
            Action::Dns_ActionDeleteAll => (crate::ConstructorNumber(0x3f9e909e), &()),
            Action::Dns_ActionSet(x) => (crate::ConstructorNumber(0xae0bb1c3), x),
        }
    }
}
impl crate::BoxedDeserialize for Action {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x2e34b7e2),
            crate::ConstructorNumber(0x3f9e909e),
            crate::ConstructorNumber(0xae0bb1c3),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x2e34b7e2) => Ok(Action::Dns_ActionDelete(
                _de.read_bare::<crate::ton::dns::action::ActionDelete>()?,
            )),
            crate::ConstructorNumber(0x3f9e909e) => Ok(Action::Dns_ActionDeleteAll),
            crate::ConstructorNumber(0xae0bb1c3) => {
                Ok(Action::Dns_ActionSet(_de.read_bare::<crate::ton::dns::action::ActionSet>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dns.Entry`\n\n```text\ndns.entry name:string category:int32 entry:dns.EntryData = dns.Entry;\n```\n"]
pub enum Entry {
    Dns_Entry(crate::ton::dns::entry::Entry),
}
impl Entry {
    pub fn category(&self) -> &crate::ton::int32 {
        match self {
            Entry::Dns_Entry(ref x) => &x.category,
        }
    }

    pub fn entry(&self) -> &crate::ton::dns::EntryData {
        match self {
            Entry::Dns_Entry(ref x) => &x.entry,
        }
    }

    pub fn name(&self) -> &crate::ton::string {
        match self {
            Entry::Dns_Entry(ref x) => &x.name,
        }
    }

    pub fn only(self) -> crate::ton::dns::entry::Entry {
        match self {
            Entry::Dns_Entry(x) => x,
        }
    }
}
impl Eq for Entry {}
impl Default for Entry {
    fn default() -> Self {
        Entry::Dns_Entry(crate::ton::dns::entry::Entry::default())
    }
}
impl crate::BoxedSerialize for Entry {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Entry::Dns_Entry(x) => (crate::ConstructorNumber(0x922eaab8), x),
        }
    }
}
impl crate::BoxedDeserialize for Entry {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x922eaab8)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x922eaab8) => {
                Ok(Entry::Dns_Entry(_de.read_bare::<crate::ton::dns::entry::Entry>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dns.EntryData`\n\n```text\ndns.entryDataAdnlAddress adnl_address:AdnlAddress = dns.EntryData;\n\ndns.entryDataNextResolver resolver:AccountAddress = dns.EntryData;\n\ndns.entryDataSmcAddress smc_address:AccountAddress = dns.EntryData;\n\ndns.entryDataText text:string = dns.EntryData;\n\ndns.entryDataUnknown bytes:bytes = dns.EntryData;\n```\n"]
pub enum EntryData {
    Dns_EntryDataAdnlAddress(crate::ton::dns::entrydata::EntryDataAdnlAddress),
    Dns_EntryDataNextResolver(crate::ton::dns::entrydata::EntryDataNextResolver),
    Dns_EntryDataSmcAddress(crate::ton::dns::entrydata::EntryDataSmcAddress),
    Dns_EntryDataText(crate::ton::dns::entrydata::EntryDataText),
    Dns_EntryDataUnknown(crate::ton::dns::entrydata::EntryDataUnknown),
}
impl EntryData {
    pub fn adnl_address(&self) -> Option<&crate::ton::AdnlAddress> {
        match self {
            EntryData::Dns_EntryDataAdnlAddress(ref x) => Some(&x.adnl_address),
            _ => None,
        }
    }

    pub fn bytes(&self) -> Option<&crate::ton::bytes> {
        match self {
            EntryData::Dns_EntryDataUnknown(ref x) => Some(&x.bytes),
            _ => None,
        }
    }

    pub fn resolver(&self) -> Option<&crate::ton::AccountAddress> {
        match self {
            EntryData::Dns_EntryDataNextResolver(ref x) => Some(&x.resolver),
            _ => None,
        }
    }

    pub fn smc_address(&self) -> Option<&crate::ton::AccountAddress> {
        match self {
            EntryData::Dns_EntryDataSmcAddress(ref x) => Some(&x.smc_address),
            _ => None,
        }
    }

    pub fn text(&self) -> Option<&crate::ton::string> {
        match self {
            EntryData::Dns_EntryDataText(ref x) => Some(&x.text),
            _ => None,
        }
    }
}
impl Eq for EntryData {}
impl Default for EntryData {
    fn default() -> Self {
        EntryData::Dns_EntryDataAdnlAddress(
            crate::ton::dns::entrydata::EntryDataAdnlAddress::default(),
        )
    }
}
impl crate::BoxedSerialize for EntryData {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            EntryData::Dns_EntryDataAdnlAddress(x) => (crate::ConstructorNumber(0xbd98ba10), x),
            EntryData::Dns_EntryDataNextResolver(x) => (crate::ConstructorNumber(0x13b13dc8), x),
            EntryData::Dns_EntryDataSmcAddress(x) => (crate::ConstructorNumber(0x97197a42), x),
            EntryData::Dns_EntryDataText(x) => (crate::ConstructorNumber(0xd0c3a112), x),
            EntryData::Dns_EntryDataUnknown(x) => (crate::ConstructorNumber(0xb35ad380), x),
        }
    }
}
impl crate::BoxedDeserialize for EntryData {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xbd98ba10),
            crate::ConstructorNumber(0x13b13dc8),
            crate::ConstructorNumber(0x97197a42),
            crate::ConstructorNumber(0xd0c3a112),
            crate::ConstructorNumber(0xb35ad380),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbd98ba10) => Ok(EntryData::Dns_EntryDataAdnlAddress(
                _de.read_bare::<crate::ton::dns::entrydata::EntryDataAdnlAddress>()?,
            )),
            crate::ConstructorNumber(0x13b13dc8) => Ok(EntryData::Dns_EntryDataNextResolver(
                _de.read_bare::<crate::ton::dns::entrydata::EntryDataNextResolver>()?,
            )),
            crate::ConstructorNumber(0x97197a42) => Ok(EntryData::Dns_EntryDataSmcAddress(
                _de.read_bare::<crate::ton::dns::entrydata::EntryDataSmcAddress>()?,
            )),
            crate::ConstructorNumber(0xd0c3a112) => Ok(EntryData::Dns_EntryDataText(
                _de.read_bare::<crate::ton::dns::entrydata::EntryDataText>()?,
            )),
            crate::ConstructorNumber(0xb35ad380) => Ok(EntryData::Dns_EntryDataUnknown(
                _de.read_bare::<crate::ton::dns::entrydata::EntryDataUnknown>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dns.Resolved`\n\n```text\ndns.resolved entries:vector<dns.entry> = dns.Resolved;\n```\n"]
pub enum Resolved {
    Dns_Resolved(crate::ton::dns::resolved::Resolved),
}
impl Resolved {
    pub fn entries(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::dns::entry::Entry> {
        match self {
            Resolved::Dns_Resolved(ref x) => &x.entries,
        }
    }

    pub fn only(self) -> crate::ton::dns::resolved::Resolved {
        match self {
            Resolved::Dns_Resolved(x) => x,
        }
    }
}
impl Eq for Resolved {}
impl Default for Resolved {
    fn default() -> Self {
        Resolved::Dns_Resolved(crate::ton::dns::resolved::Resolved::default())
    }
}
impl crate::BoxedSerialize for Resolved {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Resolved::Dns_Resolved(x) => (crate::ConstructorNumber(0x7c970596), x),
        }
    }
}
impl crate::BoxedDeserialize for Resolved {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7c970596)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7c970596) => {
                Ok(Resolved::Dns_Resolved(_de.read_bare::<crate::ton::dns::resolved::Resolved>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod accountstate;
pub mod action;
pub mod entry;
pub mod entrydata;
pub mod initialaccountstate;
pub mod resolved;
