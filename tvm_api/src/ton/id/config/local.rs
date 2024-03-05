use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `id.config.local`\n\n```text\nid.config.local id:PrivateKey = id.config.Local;\n```\n"]
pub struct Local {
    pub id: crate::ton::PrivateKey,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x92a9c78e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { id } = self;
        _ser.write_boxed::<crate::ton::PrivateKey>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_boxed::<crate::ton::PrivateKey>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Local {
    type Boxed = crate::ton::id::config::Local;

    fn into_boxed(self) -> crate::ton::id::config::Local {
        crate::ton::id::config::Local::Id_Config_Local(self)
    }
}
