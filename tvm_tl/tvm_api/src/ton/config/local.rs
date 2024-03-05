use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `config.local`\n\n```text\nconfig.local local_ids:(vector id.config.local) dht:(vector dht.config.Local) validators:(vector validator.config.Local) liteservers:(vector liteserver.config.Local) control:(vector control.config.local) = config.Local;\n```\n"]
pub struct Local {
    pub local_ids: crate::ton::vector<crate::ton::Bare, crate::ton::id::config::local::Local>,
    pub dht: crate::ton::vector<crate::ton::Boxed, crate::ton::dht::config::Local>,
    pub validators: crate::ton::vector<crate::ton::Boxed, crate::ton::validator::config::Local>,
    pub liteservers: crate::ton::vector<crate::ton::Boxed, crate::ton::liteserver::config::Local>,
    pub control: crate::ton::vector<crate::ton::Bare, crate::ton::control::config::local::Local>,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x789e915c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { local_ids, dht, validators, liteservers, control } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: id :: config :: local :: Local > > (local_ids) ? ;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::dht::config::Local>>(
            dht,
        )?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: validator :: config :: Local > > (validators) ? ;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: liteserver :: config :: Local > > (liteservers) ? ;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: control :: config :: local :: Local > > (control) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let local_ids = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: id :: config :: local :: Local > > () ? ;
            let dht = _de
                .read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::dht::config::Local>>(
                )?;
            let validators = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: validator :: config :: Local > > () ? ;
            let liteservers = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: liteserver :: config :: Local > > () ? ;
            let control = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: control :: config :: local :: Local > > () ? ;
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
