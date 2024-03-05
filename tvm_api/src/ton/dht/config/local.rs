use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.config.local`\n\n```text\ndht.config.local id:adnl.id.short = dht.config.Local;\n```\n"]
pub struct Local {
    pub id: crate::ton::adnl::id::short::Short,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x76204a6f)
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
    type Boxed = crate::ton::dht::config::Local;

    fn into_boxed(self) -> crate::ton::dht::config::Local {
        crate::ton::dht::config::Local::Dht_Config_Local(self)
    }
}
