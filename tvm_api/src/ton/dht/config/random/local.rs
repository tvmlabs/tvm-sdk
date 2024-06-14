use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.config.random.local`\n\n```text\ndht.config.random.local cnt:int = dht.config.Local;\n```\n"]
pub struct Local {
    pub cnt: crate::ton::int,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9beb2577)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { cnt } = self;
        _ser.write_bare::<crate::ton::int>(cnt)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let cnt = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { cnt })
        }
    }
}
impl crate::IntoBoxed for Local {
    type Boxed = crate::ton::dht::config::Local;
    fn into_boxed(self) -> crate::ton::dht::config::Local {
        crate::ton::dht::config::Local::Dht_Config_Random_Local(self)
    }
}
