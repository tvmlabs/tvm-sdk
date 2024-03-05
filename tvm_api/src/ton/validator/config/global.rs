use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validator.config.global`\n\n```text\nvalidator.config.global zero_state:tonNode.blockIdExt init_block:tonNode.blockIdExt hardforks:(vector tonNode.blockIdExt) = validator.config.Global;\n```\n"]
pub struct Global {
    pub zero_state: crate::ton::ton_node::blockidext::BlockIdExt,
    pub init_block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub hardforks:
        crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for Global {}
impl crate::BareSerialize for Global {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x867dff6a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Global { zero_state, init_block, hardforks } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(zero_state)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(init_block)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (hardforks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for Global {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let zero_state = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let init_block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let hardforks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { zero_state, init_block, hardforks })
        }
    }
}
impl crate::IntoBoxed for Global {
    type Boxed = crate::ton::validator::config::Global;

    fn into_boxed(self) -> crate::ton::validator::config::Global {
        crate::ton::validator::config::Global::Validator_Config_Global(self)
    }
}
