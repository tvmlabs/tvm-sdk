use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempAcceptedCompact`\n\n```text\ntonNode.rempAcceptedCompact level:byte block_id_index:byte master_id_index:byte = tonNode.RempMessageStatusCompact;\n```\n"]
pub struct RempAcceptedCompact {
    pub level: crate::ton::byte,
    pub block_id_index: crate::ton::byte,
    pub master_id_index: crate::ton::byte,
}
impl Eq for RempAcceptedCompact {}
impl crate::BareSerialize for RempAcceptedCompact {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8f092f0f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempAcceptedCompact { level, block_id_index, master_id_index } = self;
        _ser.write_bare::<crate::ton::byte>(level)?;
        _ser.write_bare::<crate::ton::byte>(block_id_index)?;
        _ser.write_bare::<crate::ton::byte>(master_id_index)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempAcceptedCompact {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let level = _de.read_bare::<crate::ton::byte>()?;
            let block_id_index = _de.read_bare::<crate::ton::byte>()?;
            let master_id_index = _de.read_bare::<crate::ton::byte>()?;
            Ok(Self { level, block_id_index, master_id_index })
        }
    }
}
impl crate::IntoBoxed for RempAcceptedCompact {
    type Boxed = crate::ton::ton_node::RempMessageStatusCompact;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatusCompact {
        crate::ton::ton_node::RempMessageStatusCompact::TonNode_RempAcceptedCompact(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempDuplicateCompact`\n\n```text\ntonNode.rempDuplicateCompact block_id_index:byte = tonNode.RempMessageStatusCompact;\n```\n"]
pub struct RempDuplicateCompact {
    pub block_id_index: crate::ton::byte,
}
impl Eq for RempDuplicateCompact {}
impl crate::BareSerialize for RempDuplicateCompact {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x16b33de4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempDuplicateCompact { block_id_index } = self;
        _ser.write_bare::<crate::ton::byte>(block_id_index)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempDuplicateCompact {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id_index = _de.read_bare::<crate::ton::byte>()?;
            Ok(Self { block_id_index })
        }
    }
}
impl crate::IntoBoxed for RempDuplicateCompact {
    type Boxed = crate::ton::ton_node::RempMessageStatusCompact;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatusCompact {
        crate::ton::ton_node::RempMessageStatusCompact::TonNode_RempDuplicateCompact(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempIgnoredCompact`\n\n```text\ntonNode.rempIgnoredCompact level:byte block_id_index:byte = tonNode.RempMessageStatusCompact;\n```\n"]
pub struct RempIgnoredCompact {
    pub level: crate::ton::byte,
    pub block_id_index: crate::ton::byte,
}
impl Eq for RempIgnoredCompact {}
impl crate::BareSerialize for RempIgnoredCompact {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbd26c204)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempIgnoredCompact { level, block_id_index } = self;
        _ser.write_bare::<crate::ton::byte>(level)?;
        _ser.write_bare::<crate::ton::byte>(block_id_index)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempIgnoredCompact {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let level = _de.read_bare::<crate::ton::byte>()?;
            let block_id_index = _de.read_bare::<crate::ton::byte>()?;
            Ok(Self { level, block_id_index })
        }
    }
}
impl crate::IntoBoxed for RempIgnoredCompact {
    type Boxed = crate::ton::ton_node::RempMessageStatusCompact;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatusCompact {
        crate::ton::ton_node::RempMessageStatusCompact::TonNode_RempIgnoredCompact(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempRejectedCompact`\n\n```text\ntonNode.rempRejectedCompact level:byte block_id_index:byte error:string = tonNode.RempMessageStatusCompact;\n```\n"]
pub struct RempRejectedCompact {
    pub level: crate::ton::byte,
    pub block_id_index: crate::ton::byte,
    pub error: crate::ton::string,
}
impl Eq for RempRejectedCompact {}
impl crate::BareSerialize for RempRejectedCompact {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x15a2b254)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempRejectedCompact { level, block_id_index, error } = self;
        _ser.write_bare::<crate::ton::byte>(level)?;
        _ser.write_bare::<crate::ton::byte>(block_id_index)?;
        _ser.write_bare::<crate::ton::string>(error)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempRejectedCompact {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let level = _de.read_bare::<crate::ton::byte>()?;
            let block_id_index = _de.read_bare::<crate::ton::byte>()?;
            let error = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { level, block_id_index, error })
        }
    }
}
impl crate::IntoBoxed for RempRejectedCompact {
    type Boxed = crate::ton::ton_node::RempMessageStatusCompact;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatusCompact {
        crate::ton::ton_node::RempMessageStatusCompact::TonNode_RempRejectedCompact(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempSentToValidatorsCompact`\n\n```text\ntonNode.rempSentToValidatorsCompact sent_to:byte total_validators:byte = tonNode.RempMessageStatusCompact;\n```\n"]
pub struct RempSentToValidatorsCompact {
    pub sent_to: crate::ton::byte,
    pub total_validators: crate::ton::byte,
}
impl Eq for RempSentToValidatorsCompact {}
impl crate::BareSerialize for RempSentToValidatorsCompact {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3f854835)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempSentToValidatorsCompact { sent_to, total_validators } = self;
        _ser.write_bare::<crate::ton::byte>(sent_to)?;
        _ser.write_bare::<crate::ton::byte>(total_validators)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempSentToValidatorsCompact {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let sent_to = _de.read_bare::<crate::ton::byte>()?;
            let total_validators = _de.read_bare::<crate::ton::byte>()?;
            Ok(Self { sent_to, total_validators })
        }
    }
}
impl crate::IntoBoxed for RempSentToValidatorsCompact {
    type Boxed = crate::ton::ton_node::RempMessageStatusCompact;

    fn into_boxed(self) -> crate::ton::ton_node::RempMessageStatusCompact {
        crate::ton::ton_node::RempMessageStatusCompact::TonNode_RempSentToValidatorsCompact(self)
    }
}
