use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `config.local`\n\n```text\nconfig.local local_ids:(vector id.config.local) dht:(vector dht.config.Local) validators:(vector validator.config.Local) liteservers:(vector liteserver.config.Local) control:(vector control.config.local) = config.Local;\n```\n"]
pub struct Local {
    pub local_ids: crate::ton::vector<crate::ton::id::config::local::Local>,
    pub dht: crate::ton::vector<crate::ton::dht::config::Local>,
    pub validators: crate::ton::vector<crate::ton::validator::config::Local>,
    pub liteservers: crate::ton::vector<crate::ton::liteserver::config::Local>,
    pub control: crate::ton::vector<crate::ton::control::config::local::Local>,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x789e915c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { local_ids, dht, validators, liteservers, control } = self;
        (local_ids as &dyn crate::ton::VectoredBare<crate::ton::id::config::local::Local>)
            .serialize(_ser)?;
        (dht as &dyn crate::ton::VectoredBoxed<crate::ton::dht::config::Local>).serialize(_ser)?;
        (validators as &dyn crate::ton::VectoredBoxed<crate::ton::validator::config::Local>)
            .serialize(_ser)?;
        (liteservers as &dyn crate::ton::VectoredBoxed<crate::ton::liteserver::config::Local>)
            .serialize(_ser)?;
        (control as &dyn crate::ton::VectoredBare<crate::ton::control::config::local::Local>)
            .serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_ids =
                <Vec<crate::ton::id::config::local::Local> as crate::ton::VectoredBare<
                    crate::ton::id::config::local::Local,
                >>::deserialize(_de)?;
            let dht = <Vec<crate::ton::dht::config::Local> as crate::ton::VectoredBoxed<
                crate::ton::dht::config::Local,
            >>::deserialize(_de)?;
            let validators =
                <Vec<crate::ton::validator::config::Local> as crate::ton::VectoredBoxed<
                    crate::ton::validator::config::Local,
                >>::deserialize(_de)?;
            let liteservers =
                <Vec<crate::ton::liteserver::config::Local> as crate::ton::VectoredBoxed<
                    crate::ton::liteserver::config::Local,
                >>::deserialize(_de)?;
            let control =
                <Vec<crate::ton::control::config::local::Local> as crate::ton::VectoredBare<
                    crate::ton::control::config::local::Local,
                >>::deserialize(_de)?;
            Ok(Self { local_ids, dht, validators, liteservers, control })
        }
    }
}
impl crate::IntoBoxed for Local {
    type Boxed = crate::ton::config::Local;

    fn into_boxed(self) -> crate::ton::config::Local {
        crate::ton::config::Local::Config_Local(self)
    }
}
