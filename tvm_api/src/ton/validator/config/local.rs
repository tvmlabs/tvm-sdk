use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validator.config.local`\n\n```text\nvalidator.config.local id:adnl.id.short = validator.config.Local;\n```\n"]
pub struct Local {
    pub id: crate::ton::adnl::id::short::Short,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x664bff68)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { id } = self;
        _ser.write_bare::<crate::ton::adnl::id::short::Short>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::adnl::id::short::Short>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Local {
    type Boxed = crate::ton::validator::config::Local;

    fn into_boxed(self) -> crate::ton::validator::config::Local {
        crate::ton::validator::config::Local::Validator_Config_Local(self)
    }
}
