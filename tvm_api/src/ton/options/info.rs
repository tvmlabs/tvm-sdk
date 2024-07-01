use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `options.info`\n\n```text\noptions.info config_info:options.configInfo = options.Info;\n```\n"]
pub struct Info {
    pub config_info: crate::ton::options::configinfo::ConfigInfo,
}
impl Eq for Info {}
impl crate::BareSerialize for Info {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfc251c80)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Info { config_info } = self;
        _ser.write_bare::<crate::ton::options::configinfo::ConfigInfo>(config_info)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Info {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let config_info = _de.read_bare::<crate::ton::options::configinfo::ConfigInfo>()?;
            Ok(Self { config_info })
        }
    }
}
impl crate::IntoBoxed for Info {
    type Boxed = crate::ton::options::Info;
    fn into_boxed(self) -> crate::ton::options::Info {
        crate::ton::options::Info::Options_Info(self)
    }
}
