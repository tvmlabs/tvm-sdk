use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `control.config.local`\n\n```text\ncontrol.config.local priv:PrivateKey pub:int256 port:int = control.config.Local;\n```\n"]
pub struct Local {
    pub priv_: crate::ton::PrivateKey,
    pub pub_: crate::ton::int256,
    pub port: crate::ton::int,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x751deced)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { priv_, pub_, port } = self;
        _ser.write_boxed::<crate::ton::PrivateKey>(priv_)?;
        _ser.write_bare::<crate::ton::int256>(pub_)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let priv_ = _de.read_boxed::<crate::ton::PrivateKey>()?;
            let pub_ = _de.read_bare::<crate::ton::int256>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { priv_, pub_, port })
        }
    }
}
impl crate::IntoBoxed for Local {
    type Boxed = crate::ton::control::config::Local;
    fn into_boxed(self) -> crate::ton::control::config::Local {
        crate::ton::control::config::Local::Control_Config_Local(self)
    }
}
