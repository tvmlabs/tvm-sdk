use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.shardPublicOverlayId`\n\n```text\ntonNode.shardPublicOverlayId workchain:int shard:long zero_state_file_hash:int256 = tonNode.ShardPublicOverlayId;\n```\n"]
pub struct ShardPublicOverlayId {
    pub workchain: crate::ton::int,
    pub shard: crate::ton::long,
    pub zero_state_file_hash: crate::ton::int256,
}
impl Eq for ShardPublicOverlayId {}
impl crate::BareSerialize for ShardPublicOverlayId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4d9ed329)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ShardPublicOverlayId { workchain, shard, zero_state_file_hash } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::long>(shard)?;
        _ser.write_bare::<crate::ton::int256>(zero_state_file_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ShardPublicOverlayId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let shard = _de.read_bare::<crate::ton::long>()?;
            let zero_state_file_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { workchain, shard, zero_state_file_hash })
        }
    }
}
impl crate::IntoBoxed for ShardPublicOverlayId {
    type Boxed = crate::ton::ton_node::ShardPublicOverlayId;

    fn into_boxed(self) -> crate::ton::ton_node::ShardPublicOverlayId {
        crate::ton::ton_node::ShardPublicOverlayId::TonNode_ShardPublicOverlayId(self)
    }
}
