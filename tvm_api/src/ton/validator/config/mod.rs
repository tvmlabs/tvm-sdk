use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validator.config.Global`\n\n```text\nvalidator.config.global zero_state:tonNode.blockIdExt init_block:tonNode.blockIdExt hardforks:(vector tonNode.blockIdExt) = validator.config.Global;\n```\n"]
pub enum Global {
    Validator_Config_Global(crate::ton::validator::config::global::Global),
}
impl Global {
    pub fn hardforks(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Global::Validator_Config_Global(ref x) => &x.hardforks,
        }
    }

    pub fn init_block(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Global::Validator_Config_Global(ref x) => &x.init_block,
        }
    }

    pub fn zero_state(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            Global::Validator_Config_Global(ref x) => &x.zero_state,
        }
    }

    pub fn only(self) -> crate::ton::validator::config::global::Global {
        match self {
            Global::Validator_Config_Global(x) => x,
        }
    }
}
impl Eq for Global {}
impl Default for Global {
    fn default() -> Self {
        Global::Validator_Config_Global(crate::ton::validator::config::global::Global::default())
    }
}
impl crate::BoxedSerialize for Global {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Global::Validator_Config_Global(x) => (crate::ConstructorNumber(0x867dff6a), x),
        }
    }
}
impl crate::BoxedDeserialize for Global {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x867dff6a)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x867dff6a) => Ok(Global::Validator_Config_Global(
                _de.read_bare::<crate::ton::validator::config::global::Global>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `validator.config.Local`\n\n```text\nvalidator.config.local id:adnl.id.short = validator.config.Local;\n\nvalidator.config.random.local addr_list:adnl.addressList = validator.config.Local;\n```\n"]
pub enum Local {
    Validator_Config_Local(crate::ton::validator::config::local::Local),
    Validator_Config_Random_Local(crate::ton::validator::config::random::local::Local),
}
impl Local {
    pub fn addr_list(&self) -> Option<&crate::ton::adnl::addresslist::AddressList> {
        match self {
            Local::Validator_Config_Random_Local(ref x) => Some(&x.addr_list),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&crate::ton::adnl::id::short::Short> {
        match self {
            Local::Validator_Config_Local(ref x) => Some(&x.id),
            _ => None,
        }
    }
}
impl Eq for Local {}
impl Default for Local {
    fn default() -> Self {
        Local::Validator_Config_Local(crate::ton::validator::config::local::Local::default())
    }
}
impl crate::BoxedSerialize for Local {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Local::Validator_Config_Local(x) => (crate::ConstructorNumber(0x664bff68), x),
            Local::Validator_Config_Random_Local(x) => (crate::ConstructorNumber(0x59839462), x),
        }
    }
}
impl crate::BoxedDeserialize for Local {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x664bff68), crate::ConstructorNumber(0x59839462)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x664bff68) => Ok(Local::Validator_Config_Local(
                _de.read_bare::<crate::ton::validator::config::local::Local>()?,
            )),
            crate::ConstructorNumber(0x59839462) => Ok(Local::Validator_Config_Random_Local(
                _de.read_bare::<crate::ton::validator::config::random::local::Local>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod global;
pub mod local;
pub mod random;
