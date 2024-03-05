use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rwallet.actionInit`\n\n```text\nrwallet.actionInit config:rwallet.config = rwallet.Action;\n```\n"]
pub struct ActionInit {
    pub config: crate::ton::rwallet::config::Config,
}
impl Eq for ActionInit {}
impl crate::BareSerialize for ActionInit {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2533bd6b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionInit { config } = self;
        _ser.write_bare::<crate::ton::rwallet::config::Config>(config)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionInit {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let config = _de.read_bare::<crate::ton::rwallet::config::Config>()?;
            Ok(Self { config })
        }
    }
}
impl crate::IntoBoxed for ActionInit {
    type Boxed = crate::ton::rwallet::Action;

    fn into_boxed(self) -> crate::ton::rwallet::Action {
        crate::ton::rwallet::Action::Rwallet_ActionInit(self)
    }
}
