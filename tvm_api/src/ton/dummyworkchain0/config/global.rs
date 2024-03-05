use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dummyworkchain0.config.global`\n\n```text\ndummyworkchain0.config.global zero_state_hash:int256 = dummyworkchain0.config.Global;\n```\n"]
pub struct Global {
    pub zero_state_hash: crate::ton::int256,
}
impl Eq for Global {}
impl crate::BareSerialize for Global {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xda616ed3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Global { zero_state_hash } = self;
        _ser.write_bare::<crate::ton::int256>(zero_state_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Global {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let zero_state_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { zero_state_hash })
        }
    }
}
impl crate::IntoBoxed for Global {
    type Boxed = crate::ton::dummyworkchain0::config::Global;

    fn into_boxed(self) -> crate::ton::dummyworkchain0::config::Global {
        crate::ton::dummyworkchain0::config::Global::Dummyworkchain0_Config_Global(self)
    }
}
