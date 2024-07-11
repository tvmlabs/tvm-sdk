use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `options.configInfo`\n\n```text\noptions.configInfo default_wallet_id:int64 default_rwallet_init_public_key:string = options.ConfigInfo;\n```\n"]
pub struct ConfigInfo {
    pub default_wallet_id: crate::ton::int64,
    pub default_rwallet_init_public_key: crate::ton::string,
}
impl Eq for ConfigInfo {}
impl crate::BareSerialize for ConfigInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x07b75f16)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ConfigInfo { default_wallet_id, default_rwallet_init_public_key } = self;
        _ser.write_bare::<crate::ton::int64>(default_wallet_id)?;
        _ser.write_bare::<crate::ton::string>(default_rwallet_init_public_key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ConfigInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let default_wallet_id = _de.read_bare::<crate::ton::int64>()?;
            let default_rwallet_init_public_key = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { default_wallet_id, default_rwallet_init_public_key })
        }
    }
}
impl crate::IntoBoxed for ConfigInfo {
    type Boxed = crate::ton::options::ConfigInfo;

    fn into_boxed(self) -> crate::ton::options::ConfigInfo {
        crate::ton::options::ConfigInfo::Options_ConfigInfo(self)
    }
}
