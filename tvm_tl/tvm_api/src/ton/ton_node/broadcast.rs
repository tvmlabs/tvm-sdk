use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.blockBroadcast`\n\n```text\ntonNode.blockBroadcast id:tonNode.blockIdExt catchain_seqno:int validator_set_hash:int \n              signatures:(vector tonNode.blockSignature) \n              proof:bytes data:bytes = tonNode.Broadcast;\n```\n"]
pub struct BlockBroadcast {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub catchain_seqno: crate::ton::int,
    pub validator_set_hash: crate::ton::int,
    pub signatures:
        crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blocksignature::BlockSignature>,
    pub proof: crate::ton::bytes,
    pub data: crate::ton::bytes,
}
impl Eq for BlockBroadcast {}
impl crate::BareSerialize for BlockBroadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xae2e1105)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockBroadcast { id, catchain_seqno, validator_set_hash, signatures, proof, data } =
            self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(catchain_seqno)?;
        _ser.write_bare::<crate::ton::int>(validator_set_hash)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::ton_node::blocksignature::BlockSignature,
        >>(signatures)?;
        _ser.write_bare::<crate::ton::bytes>(proof)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockBroadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let catchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let validator_set_hash = _de.read_bare::<crate::ton::int>()?;
            let signatures = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blocksignature::BlockSignature,
            >>()?;
            let proof = _de.read_bare::<crate::ton::bytes>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, catchain_seqno, validator_set_hash, signatures, proof, data })
        }
    }
}
impl crate::IntoBoxed for BlockBroadcast {
    type Boxed = crate::ton::ton_node::Broadcast;

    fn into_boxed(self) -> crate::ton::ton_node::Broadcast {
        crate::ton::ton_node::Broadcast::TonNode_BlockBroadcast(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.blockCandidateBroadcast`\n\n```text\ntonNode.blockCandidateBroadcast id:tonNode.blockIdExt data:bytes collated_data:bytes collated_data_file_hash:int256 created_by:int256 created_timestamp:long = tonNode.Broadcast;\n```\n"]
pub struct BlockCandidateBroadcast {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub data: crate::ton::bytes,
    pub collated_data: crate::ton::bytes,
    pub collated_data_file_hash: crate::ton::int256,
    pub created_by: crate::ton::int256,
    pub created_timestamp: crate::ton::long,
}
impl Eq for BlockCandidateBroadcast {}
impl crate::BareSerialize for BlockCandidateBroadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2f8cdf1d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockCandidateBroadcast {
            id,
            data,
            collated_data,
            collated_data_file_hash,
            created_by,
            created_timestamp,
        } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::bytes>(collated_data)?;
        _ser.write_bare::<crate::ton::int256>(collated_data_file_hash)?;
        _ser.write_bare::<crate::ton::int256>(created_by)?;
        _ser.write_bare::<crate::ton::long>(created_timestamp)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockCandidateBroadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let collated_data = _de.read_bare::<crate::ton::bytes>()?;
            let collated_data_file_hash = _de.read_bare::<crate::ton::int256>()?;
            let created_by = _de.read_bare::<crate::ton::int256>()?;
            let created_timestamp = _de.read_bare::<crate::ton::long>()?;
            Ok(Self {
                id,
                data,
                collated_data,
                collated_data_file_hash,
                created_by,
                created_timestamp,
            })
        }
    }
}
impl crate::IntoBoxed for BlockCandidateBroadcast {
    type Boxed = crate::ton::ton_node::Broadcast;

    fn into_boxed(self) -> crate::ton::ton_node::Broadcast {
        crate::ton::ton_node::Broadcast::TonNode_BlockCandidateBroadcast(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.connectivityCheckBroadcast`\n\n```text\ntonNode.connectivityCheckBroadcast pub_key:int256 padding:bytes = tonNode.Broadcast;\n```\n"]
pub struct ConnectivityCheckBroadcast {
    pub pub_key: crate::ton::int256,
    pub padding: crate::ton::bytes,
}
impl Eq for ConnectivityCheckBroadcast {}
impl crate::BareSerialize for ConnectivityCheckBroadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x336c53d9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ConnectivityCheckBroadcast { pub_key, padding } = self;
        _ser.write_bare::<crate::ton::int256>(pub_key)?;
        _ser.write_bare::<crate::ton::bytes>(padding)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ConnectivityCheckBroadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let pub_key = _de.read_bare::<crate::ton::int256>()?;
            let padding = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { pub_key, padding })
        }
    }
}
impl crate::IntoBoxed for ConnectivityCheckBroadcast {
    type Boxed = crate::ton::ton_node::Broadcast;

    fn into_boxed(self) -> crate::ton::ton_node::Broadcast {
        crate::ton::ton_node::Broadcast::TonNode_ConnectivityCheckBroadcast(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.externalMessageBroadcast`\n\n```text\ntonNode.externalMessageBroadcast message:tonNode.externalMessage = tonNode.Broadcast;\n```\n"]
pub struct ExternalMessageBroadcast {
    pub message: crate::ton::ton_node::externalmessage::ExternalMessage,
}
impl Eq for ExternalMessageBroadcast {}
impl crate::BareSerialize for ExternalMessageBroadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3d1b1867)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExternalMessageBroadcast { message } = self;
        _ser.write_bare::<crate::ton::ton_node::externalmessage::ExternalMessage>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExternalMessageBroadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message =
                _de.read_bare::<crate::ton::ton_node::externalmessage::ExternalMessage>()?;
            Ok(Self { message })
        }
    }
}
impl crate::IntoBoxed for ExternalMessageBroadcast {
    type Boxed = crate::ton::ton_node::Broadcast;

    fn into_boxed(self) -> crate::ton::ton_node::Broadcast {
        crate::ton::ton_node::Broadcast::TonNode_ExternalMessageBroadcast(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.ihrMessageBroadcast`\n\n```text\ntonNode.ihrMessageBroadcast message:tonNode.ihrMessage = tonNode.Broadcast;\n```\n"]
pub struct IhrMessageBroadcast {
    pub message: crate::ton::ton_node::ihrmessage::IhrMessage,
}
impl Eq for IhrMessageBroadcast {}
impl crate::BareSerialize for IhrMessageBroadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x525da4b3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let IhrMessageBroadcast { message } = self;
        _ser.write_bare::<crate::ton::ton_node::ihrmessage::IhrMessage>(message)?;
        Ok(())
    }
}
impl crate::BareDeserialize for IhrMessageBroadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let message = _de.read_bare::<crate::ton::ton_node::ihrmessage::IhrMessage>()?;
            Ok(Self { message })
        }
    }
}
impl crate::IntoBoxed for IhrMessageBroadcast {
    type Boxed = crate::ton::ton_node::Broadcast;

    fn into_boxed(self) -> crate::ton::ton_node::Broadcast {
        crate::ton::ton_node::Broadcast::TonNode_IhrMessageBroadcast(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.newShardBlockBroadcast`\n\n```text\ntonNode.newShardBlockBroadcast block:tonNode.newShardBlock = tonNode.Broadcast;\n```\n"]
pub struct NewShardBlockBroadcast {
    pub block: crate::ton::ton_node::newshardblock::NewShardBlock,
}
impl Eq for NewShardBlockBroadcast {}
impl crate::BareSerialize for NewShardBlockBroadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0af2fabc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let NewShardBlockBroadcast { block } = self;
        _ser.write_bare::<crate::ton::ton_node::newshardblock::NewShardBlock>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for NewShardBlockBroadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::newshardblock::NewShardBlock>()?;
            Ok(Self { block })
        }
    }
}
impl crate::IntoBoxed for NewShardBlockBroadcast {
    type Boxed = crate::ton::ton_node::Broadcast;

    fn into_boxed(self) -> crate::ton::ton_node::Broadcast {
        crate::ton::ton_node::Broadcast::TonNode_NewShardBlockBroadcast(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.queueUpdateBroadcast`\n\n```text\ntonNode.queueUpdateBroadcast id:tonNode.blockIdExt catchain_seqno:int validator_set_hash:int \n              signatures:(vector tonNode.blockSignature) target_wc:int\n              data:bytes = tonNode.Broadcast;\n```\n"]
pub struct QueueUpdateBroadcast {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub catchain_seqno: crate::ton::int,
    pub validator_set_hash: crate::ton::int,
    pub signatures:
        crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blocksignature::BlockSignature>,
    pub target_wc: crate::ton::int,
    pub data: crate::ton::bytes,
}
impl Eq for QueueUpdateBroadcast {}
impl crate::BareSerialize for QueueUpdateBroadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9b00bad5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let QueueUpdateBroadcast {
            id,
            catchain_seqno,
            validator_set_hash,
            signatures,
            target_wc,
            data,
        } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(catchain_seqno)?;
        _ser.write_bare::<crate::ton::int>(validator_set_hash)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::ton_node::blocksignature::BlockSignature,
        >>(signatures)?;
        _ser.write_bare::<crate::ton::int>(target_wc)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for QueueUpdateBroadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let catchain_seqno = _de.read_bare::<crate::ton::int>()?;
            let validator_set_hash = _de.read_bare::<crate::ton::int>()?;
            let signatures = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blocksignature::BlockSignature,
            >>()?;
            let target_wc = _de.read_bare::<crate::ton::int>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, catchain_seqno, validator_set_hash, signatures, target_wc, data })
        }
    }
}
impl crate::IntoBoxed for QueueUpdateBroadcast {
    type Boxed = crate::ton::ton_node::Broadcast;

    fn into_boxed(self) -> crate::ton::ton_node::Broadcast {
        crate::ton::ton_node::Broadcast::TonNode_QueueUpdateBroadcast(self)
    }
}
