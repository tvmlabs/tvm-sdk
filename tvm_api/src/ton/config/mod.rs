use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `config`\n\n```text\nconfig config:string blockchain_name:string use_callbacks_for_network:Bool ignore_cache:Bool = Config;\n```\n"]
pub struct Config {
    pub config: crate::ton::string,
    pub blockchain_name: crate::ton::string,
    pub use_callbacks_for_network: crate::ton::Bool,
    pub ignore_cache: crate::ton::Bool,
}
impl Eq for Config {}
impl crate::BareSerialize for Config {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa44e0238)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Config { config, blockchain_name, use_callbacks_for_network, ignore_cache } = self;
        _ser.write_bare::<crate::ton::string>(config)?;
        _ser.write_bare::<crate::ton::string>(blockchain_name)?;
        _ser.write_boxed::<crate::ton::Bool>(use_callbacks_for_network)?;
        _ser.write_boxed::<crate::ton::Bool>(ignore_cache)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Config {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let config = _de.read_bare::<crate::ton::string>()?;
            let blockchain_name = _de.read_bare::<crate::ton::string>()?;
            let use_callbacks_for_network = _de.read_boxed::<crate::ton::Bool>()?;
            let ignore_cache = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { config, blockchain_name, use_callbacks_for_network, ignore_cache })
        }
    }
}
impl crate::IntoBoxed for Config {
    type Boxed = crate::ton::Config;

    fn into_boxed(self) -> crate::ton::Config {
        crate::ton::Config::Config(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `config.Global`\n\n```text\nconfig.global adnl:adnl.config.global dht:dht.config.global validator:validator.config.global = config.Global;\n```\n"]
pub enum Global {
    Config_Global(crate::ton::config::global::Global),
}
impl Global {
    pub fn adnl(&self) -> &crate::ton::adnl::config::global::Global {
        match self {
            Global::Config_Global(ref x) => &x.adnl,
        }
    }

    pub fn dht(&self) -> &crate::ton::dht::config::global::Global {
        match self {
            Global::Config_Global(ref x) => &x.dht,
        }
    }

    pub fn validator(&self) -> &crate::ton::validator::config::global::Global {
        match self {
            Global::Config_Global(ref x) => &x.validator,
        }
    }

    pub fn only(self) -> crate::ton::config::global::Global {
        match self {
            Global::Config_Global(x) => x,
        }
    }
}
impl Eq for Global {}
impl Default for Global {
    fn default() -> Self {
        Global::Config_Global(crate::ton::config::global::Global::default())
    }
}
impl crate::BoxedSerialize for Global {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Global::Config_Global(x) => (crate::ConstructorNumber(0xf4269fd2), x),
        }
    }
}
impl crate::BoxedDeserialize for Global {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf4269fd2)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf4269fd2) => {
                Ok(Global::Config_Global(_de.read_bare::<crate::ton::config::global::Global>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `config.Local`\n\n```text\nconfig.local local_ids:(vector id.config.local) dht:(vector dht.config.Local) validators:(vector validator.config.Local) liteservers:(vector liteserver.config.Local) control:(vector control.config.local) = config.Local;\n```\n"]
pub enum Local {
    Config_Local(crate::ton::config::local::Local),
}
impl Local {
    pub fn control(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::control::config::local::Local> {
        match self {
            Local::Config_Local(ref x) => &x.control,
        }
    }

    pub fn dht(&self) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::dht::config::Local> {
        match self {
            Local::Config_Local(ref x) => &x.dht,
        }
    }

    pub fn liteservers(
        &self,
    ) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::liteserver::config::Local> {
        match self {
            Local::Config_Local(ref x) => &x.liteservers,
        }
    }

    pub fn local_ids(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::id::config::local::Local> {
        match self {
            Local::Config_Local(ref x) => &x.local_ids,
        }
    }

    pub fn validators(
        &self,
    ) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::validator::config::Local> {
        match self {
            Local::Config_Local(ref x) => &x.validators,
        }
    }

    pub fn only(self) -> crate::ton::config::local::Local {
        match self {
            Local::Config_Local(x) => x,
        }
    }
}
impl Eq for Local {}
impl Default for Local {
    fn default() -> Self {
        Local::Config_Local(crate::ton::config::local::Local::default())
    }
}
impl crate::BoxedSerialize for Local {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Local::Config_Local(x) => (crate::ConstructorNumber(0x789e915c), x),
        }
    }
}
impl crate::BoxedDeserialize for Local {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x789e915c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x789e915c) => {
                Ok(Local::Config_Local(_de.read_bare::<crate::ton::config::local::Local>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod global;
pub mod local;
