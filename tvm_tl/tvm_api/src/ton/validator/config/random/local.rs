use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validator.config.random.local`\n\n```text\nvalidator.config.random.local addr_list:adnl.addressList = validator.config.Local;\n```\n"]
pub struct Local {
    pub addr_list: crate::ton::adnl::addresslist::AddressList,
}
impl Eq for Local {}
impl crate::BareSerialize for Local {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x59839462)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Local { addr_list } = self;
        _ser.write_bare::<crate::ton::adnl::addresslist::AddressList>(addr_list)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Local {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let addr_list = _de.read_bare::<crate::ton::adnl::addresslist::AddressList>()?;
            Ok(Self { addr_list })
        }
    }
}
impl crate::IntoBoxed for Local {
    type Boxed = crate::ton::validator::config::Local;

    fn into_boxed(self) -> crate::ton::validator::config::Local {
        crate::ton::validator::config::Local::Validator_Config_Random_Local(self)
    }
}
