use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.configInfo`\n\n```text\nliteServer.configInfo mode:# id:tonNode.blockIdExt state_proof:bytes config_proof:bytes = liteServer.ConfigInfo;\n```\n"]
pub struct ConfigInfo {
    pub mode: crate::ton::int,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub state_proof: crate::ton::bytes,
    pub config_proof: crate::ton::bytes,
}
impl Eq for ConfigInfo {}
impl crate::BareSerialize for ConfigInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xae7b272f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ConfigInfo { mode, id, state_proof, config_proof } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::bytes>(state_proof)?;
        _ser.write_bare::<crate::ton::bytes>(config_proof)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ConfigInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let state_proof = _de.read_bare::<crate::ton::bytes>()?;
            let config_proof = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { mode, id, state_proof, config_proof })
        }
    }
}
impl crate::IntoBoxed for ConfigInfo {
    type Boxed = crate::ton::lite_server::ConfigInfo;

    fn into_boxed(self) -> crate::ton::lite_server::ConfigInfo {
        crate::ton::lite_server::ConfigInfo::LiteServer_ConfigInfo(self)
    }
}
