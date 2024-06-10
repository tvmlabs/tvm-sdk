use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.info`\n\n```text\nsmc.info id:int53 = smc.Info;\n```\n"]
pub struct Info {
    pub id: crate::ton::int53,
}
impl Eq for Info {}
impl crate::BareSerialize for Info {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x439b963c)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Info { id } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Info {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Info {
    type Boxed = crate::ton::smc::Info;
    fn into_boxed(self) -> crate::ton::smc::Info {
        crate::ton::smc::Info::Smc_Info(self)
    }
}
