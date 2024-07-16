use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `options.ConfigInfo`\n\n```text\noptions.configInfo default_wallet_id:int64 default_rwallet_init_public_key:string = options.ConfigInfo;\n```\n"]
pub enum ConfigInfo {
    Options_ConfigInfo(crate::ton::options::configinfo::ConfigInfo),
}
impl ConfigInfo {
    pub fn default_rwallet_init_public_key(&self) -> &crate::ton::string {
        match self {
            ConfigInfo::Options_ConfigInfo(ref x) => &x.default_rwallet_init_public_key,
        }
    }

    pub fn default_wallet_id(&self) -> &crate::ton::int64 {
        match self {
            ConfigInfo::Options_ConfigInfo(ref x) => &x.default_wallet_id,
        }
    }

    pub fn only(self) -> crate::ton::options::configinfo::ConfigInfo {
        match self {
            ConfigInfo::Options_ConfigInfo(x) => x,
        }
    }
}
impl Eq for ConfigInfo {}
impl Default for ConfigInfo {
    fn default() -> Self {
        ConfigInfo::Options_ConfigInfo(crate::ton::options::configinfo::ConfigInfo::default())
    }
}
impl crate::BoxedSerialize for ConfigInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ConfigInfo::Options_ConfigInfo(x) => (crate::ConstructorNumber(0x07b75f16), x),
        }
    }
}
impl crate::BoxedDeserialize for ConfigInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x07b75f16)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x07b75f16) => Ok(ConfigInfo::Options_ConfigInfo(
                _de.read_bare::<crate::ton::options::configinfo::ConfigInfo>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `options.Info`\n\n```text\noptions.info config_info:options.configInfo = options.Info;\n```\n"]
pub enum Info {
    Options_Info(crate::ton::options::info::Info),
}
impl Info {
    pub fn config_info(&self) -> &crate::ton::options::configinfo::ConfigInfo {
        match self {
            Info::Options_Info(ref x) => &x.config_info,
        }
    }

    pub fn only(self) -> crate::ton::options::info::Info {
        match self {
            Info::Options_Info(x) => x,
        }
    }
}
impl Eq for Info {}
impl Default for Info {
    fn default() -> Self {
        Info::Options_Info(crate::ton::options::info::Info::default())
    }
}
impl crate::BoxedSerialize for Info {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Info::Options_Info(x) => (crate::ConstructorNumber(0xfc251c80), x),
        }
    }
}
impl crate::BoxedDeserialize for Info {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfc251c80)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xfc251c80) => {
                Ok(Info::Options_Info(_de.read_bare::<crate::ton::options::info::Info>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `options`\n\n```text\noptions config:config keystore_type:KeyStoreType = Options;\n```\n"]
pub struct Options {
    pub config: crate::ton::config::Config,
    pub keystore_type: crate::ton::KeyStoreType,
}
impl Eq for Options {}
impl crate::BareSerialize for Options {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8d4c29f9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Options { config, keystore_type } = self;
        _ser.write_bare::<crate::ton::config::Config>(config)?;
        _ser.write_boxed::<crate::ton::KeyStoreType>(keystore_type)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Options {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let config = _de.read_bare::<crate::ton::config::Config>()?;
            let keystore_type = _de.read_boxed::<crate::ton::KeyStoreType>()?;
            Ok(Self { config, keystore_type })
        }
    }
}
impl crate::IntoBoxed for Options {
    type Boxed = crate::ton::Options;

    fn into_boxed(self) -> crate::ton::Options {
        crate::ton::Options::Options(self)
    }
}
pub mod configinfo;
pub mod info;
