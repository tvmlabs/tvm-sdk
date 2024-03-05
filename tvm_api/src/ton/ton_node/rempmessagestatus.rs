use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempAccepted`\n\n```text\ntonNode.rempAccepted level:tonNode.RempMessageLevel block_id:tonNode.blockIdExt master_id:tonNode.blockIdExt = tonNode.RempMessageStatus;\n```\n"]
pub struct RempAccepted {
    pub level: crate::ton::ton_node::RempMessageLevel,
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub master_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for RempAccepted {}
impl crate::BareSerialize for RempAccepted {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x30225e0e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempAccepted { level, block_id, master_id } = self;
        _ser.write_boxed::<crate::ton::ton_node::RempMessageLevel>(level)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(master_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempAccepted {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let level = _de.read_boxed::<crate::ton::ton_node::RempMessageLevel>()?;
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let master_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { level, block_id, master_id })
        }
    }
}
impl crate::IntoBoxed for RempAccepted {
    type Boxed = crate::ton::ton_node::RempMessageStatus;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatus {
        crate::ton::ton_node::RempMessageStatus::TonNode_RempAccepted(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempDuplicate`\n\n```text\ntonNode.rempDuplicate block_id:tonNode.blockIdExt = tonNode.RempMessageStatus;\n```\n"]
pub struct RempDuplicate {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for RempDuplicate {}
impl crate::BareSerialize for RempDuplicate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9ecd4334)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempDuplicate { block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempDuplicate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id })
        }
    }
}
impl crate::IntoBoxed for RempDuplicate {
    type Boxed = crate::ton::ton_node::RempMessageStatus;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatus {
        crate::ton::ton_node::RempMessageStatus::TonNode_RempDuplicate(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempIgnored`\n\n```text\ntonNode.rempIgnored level:tonNode.RempMessageLevel block_id:tonNode.blockIdExt = tonNode.RempMessageStatus;\n```\n"]
pub struct RempIgnored {
    pub level: crate::ton::ton_node::RempMessageLevel,
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for RempIgnored {}
impl crate::BareSerialize for RempIgnored {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x43bebb8b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempIgnored { level, block_id } = self;
        _ser.write_boxed::<crate::ton::ton_node::RempMessageLevel>(level)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempIgnored {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let level = _de.read_boxed::<crate::ton::ton_node::RempMessageLevel>()?;
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { level, block_id })
        }
    }
}
impl crate::IntoBoxed for RempIgnored {
    type Boxed = crate::ton::ton_node::RempMessageStatus;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatus {
        crate::ton::ton_node::RempMessageStatus::TonNode_RempIgnored(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempRejected`\n\n```text\ntonNode.rempRejected level:tonNode.RempMessageLevel block_id:tonNode.blockIdExt error:string = tonNode.RempMessageStatus;\n```\n"]
pub struct RempRejected {
    pub level: crate::ton::ton_node::RempMessageLevel,
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub error: crate::ton::string,
}
impl Eq for RempRejected {}
impl crate::BareSerialize for RempRejected {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb4e1ee77)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempRejected { level, block_id, error } = self;
        _ser.write_boxed::<crate::ton::ton_node::RempMessageLevel>(level)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        _ser.write_bare::<crate::ton::string>(error)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempRejected {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let level = _de.read_boxed::<crate::ton::ton_node::RempMessageLevel>()?;
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let error = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { level, block_id, error })
        }
    }
}
impl crate::IntoBoxed for RempRejected {
    type Boxed = crate::ton::ton_node::RempMessageStatus;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatus {
        crate::ton::ton_node::RempMessageStatus::TonNode_RempRejected(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempSentToValidators`\n\n```text\ntonNode.rempSentToValidators sent_to:int total_validators:int = tonNode.RempMessageStatus;\n```\n"]
pub struct RempSentToValidators {
    pub sent_to: crate::ton::int,
    pub total_validators: crate::ton::int,
}
impl Eq for RempSentToValidators {}
impl crate::BareSerialize for RempSentToValidators {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2ff6c87b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempSentToValidators { sent_to, total_validators } = self;
        _ser.write_bare::<crate::ton::int>(sent_to)?;
        _ser.write_bare::<crate::ton::int>(total_validators)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempSentToValidators {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let sent_to = _de.read_bare::<crate::ton::int>()?;
            let total_validators = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { sent_to, total_validators })
        }
    }
}
impl crate::IntoBoxed for RempSentToValidators {
    type Boxed = crate::ton::ton_node::RempMessageStatus;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatus {
        crate::ton::ton_node::RempMessageStatus::TonNode_RempSentToValidators(self)
    }
}
