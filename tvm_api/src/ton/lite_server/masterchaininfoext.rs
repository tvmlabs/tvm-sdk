use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.masterchainInfoExt`\n\n```text\nliteServer.masterchainInfoExt mode:# version:int capabilities:long last:tonNode.blockIdExt last_utime:int now:int state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfoExt;\n```\n"]
pub struct MasterchainInfoExt {
    pub mode: crate::ton::int,
    pub version: crate::ton::int,
    pub capabilities: crate::ton::long,
    pub last: crate::ton::ton_node::blockidext::BlockIdExt,
    pub last_utime: crate::ton::int,
    pub now: crate::ton::int,
    pub state_root_hash: crate::ton::int256,
    pub init: crate::ton::ton_node::zerostateidext::ZeroStateIdExt,
}
impl Eq for MasterchainInfoExt {}
impl crate::BareSerialize for MasterchainInfoExt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa8cce0f5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MasterchainInfoExt {
            mode,
            version,
            capabilities,
            last,
            last_utime,
            now,
            state_root_hash,
            init,
        } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::int>(version)?;
        _ser.write_bare::<crate::ton::long>(capabilities)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(last)?;
        _ser.write_bare::<crate::ton::int>(last_utime)?;
        _ser.write_bare::<crate::ton::int>(now)?;
        _ser.write_bare::<crate::ton::int256>(state_root_hash)?;
        _ser.write_bare::<crate::ton::ton_node::zerostateidext::ZeroStateIdExt>(init)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MasterchainInfoExt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let version = _de.read_bare::<crate::ton::int>()?;
            let capabilities = _de.read_bare::<crate::ton::long>()?;
            let last = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let last_utime = _de.read_bare::<crate::ton::int>()?;
            let now = _de.read_bare::<crate::ton::int>()?;
            let state_root_hash = _de.read_bare::<crate::ton::int256>()?;
            let init = _de.read_bare::<crate::ton::ton_node::zerostateidext::ZeroStateIdExt>()?;
            Ok(Self { mode, version, capabilities, last, last_utime, now, state_root_hash, init })
        }
    }
}
impl crate::IntoBoxed for MasterchainInfoExt {
    type Boxed = crate::ton::lite_server::MasterchainInfoExt;

    fn into_boxed(self) -> crate::ton::lite_server::MasterchainInfoExt {
        crate::ton::lite_server::MasterchainInfoExt::LiteServer_MasterchainInfoExt(self)
    }
}
