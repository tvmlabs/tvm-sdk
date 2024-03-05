use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.masterchainInfo`\n\n```text\nliteServer.masterchainInfo last:tonNode.blockIdExt state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfo;\n```\n"]
pub struct MasterchainInfo {
    pub last: crate::ton::ton_node::blockidext::BlockIdExt,
    pub state_root_hash: crate::ton::int256,
    pub init: crate::ton::ton_node::zerostateidext::ZeroStateIdExt,
}
impl Eq for MasterchainInfo {}
impl crate::BareSerialize for MasterchainInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x85832881)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MasterchainInfo { last, state_root_hash, init } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(last)?;
        _ser.write_bare::<crate::ton::int256>(state_root_hash)?;
        _ser.write_bare::<crate::ton::ton_node::zerostateidext::ZeroStateIdExt>(init)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MasterchainInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let last = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let state_root_hash = _de.read_bare::<crate::ton::int256>()?;
            let init = _de.read_bare::<crate::ton::ton_node::zerostateidext::ZeroStateIdExt>()?;
            Ok(Self { last, state_root_hash, init })
        }
    }
}
impl crate::IntoBoxed for MasterchainInfo {
    type Boxed = crate::ton::lite_server::MasterchainInfo;

    fn into_boxed(self) -> crate::ton::lite_server::MasterchainInfo {
        crate::ton::lite_server::MasterchainInfo::LiteServer_MasterchainInfo(self)
    }
}
