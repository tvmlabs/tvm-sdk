use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.actionDelete`\n\n```text\ndns.actionDelete name:string category:int32 = dns.Action;\n```\n"]
pub struct ActionDelete {
    pub name: crate::ton::string,
    pub category: crate::ton::int32,
}
impl Eq for ActionDelete {}
impl crate::BareSerialize for ActionDelete {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2e34b7e2)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionDelete { name, category } = self;
        _ser.write_bare::<crate::ton::string>(name)?;
        _ser.write_bare::<crate::ton::int32>(category)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionDelete {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let name = _de.read_bare::<crate::ton::string>()?;
            let category = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { name, category })
        }
    }
}
impl crate::IntoBoxed for ActionDelete {
    type Boxed = crate::ton::dns::Action;
    fn into_boxed(self) -> crate::ton::dns::Action {
        crate::ton::dns::Action::Dns_ActionDelete(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dns.actionSet`\n\n```text\ndns.actionSet entry:dns.entry = dns.Action;\n```\n"]
pub struct ActionSet {
    pub entry: crate::ton::dns::entry::Entry,
}
impl Eq for ActionSet {}
impl crate::BareSerialize for ActionSet {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xae0bb1c3)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionSet { entry } = self;
        _ser.write_bare::<crate::ton::dns::entry::Entry>(entry)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionSet {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let entry = _de.read_bare::<crate::ton::dns::entry::Entry>()?;
            Ok(Self { entry })
        }
    }
}
impl crate::IntoBoxed for ActionSet {
    type Boxed = crate::ton::dns::Action;
    fn into_boxed(self) -> crate::ton::dns::Action {
        crate::ton::dns::Action::Dns_ActionSet(self)
    }
}
