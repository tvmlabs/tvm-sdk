use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rwallet.config`\n\n```text\nrwallet.config start_at:int53 limits:vector<rwallet.limit> = rwallet.Config;\n```\n"]
pub struct Config {
    pub start_at: crate::ton::int53,
    pub limits: crate::ton::vector<crate::ton::Bare, crate::ton::rwallet::limit::Limit>,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfae7849a)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config { start_at, limits } = self;
        _ser.write_bare::<crate::ton::int53>(start_at)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::rwallet::limit::Limit>>(
            limits,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let start_at = _de.read_bare::<crate::ton::int53>()?;
            let limits = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: rwallet :: limit :: Limit > > () ? ;
            Ok(Self { start_at, limits })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::rwallet::Config;
    fn into_boxed(self) -> crate::ton::rwallet::Config {
        crate::ton::rwallet::Config::Rwallet_Config(self)
    }
}
