use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.validatorStats`\n\n```text\nliteServer.validatorStats mode:# id:tonNode.blockIdExt count:int complete:Bool state_proof:bytes data_proof:bytes = liteServer.ValidatorStats;\n```\n"]
pub struct ValidatorStats {
    pub mode: crate::ton::int,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub count: crate::ton::int,
    pub complete: crate::ton::Bool,
    pub state_proof: crate::ton::bytes,
    pub data_proof: crate::ton::bytes,
}
impl Eq for ValidatorStats {}
impl crate::BareSerialize for ValidatorStats {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb9f796d8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidatorStats { mode, id, count, complete, state_proof, data_proof } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int>(count)?;
        _ser.write_boxed::<crate::ton::Bool>(complete)?;
        _ser.write_bare::<crate::ton::bytes>(state_proof)?;
        _ser.write_bare::<crate::ton::bytes>(data_proof)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidatorStats {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let count = _de.read_bare::<crate::ton::int>()?;
            let complete = _de.read_boxed::<crate::ton::Bool>()?;
            let state_proof = _de.read_bare::<crate::ton::bytes>()?;
            let data_proof = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { mode, id, count, complete, state_proof, data_proof })
        }
    }
}
impl crate::IntoBoxed for ValidatorStats {
    type Boxed = crate::ton::lite_server::ValidatorStats;

    fn into_boxed(self) -> crate::ton::lite_server::ValidatorStats {
        crate::ton::lite_server::ValidatorStats::LiteServer_ValidatorStats(self)
    }
}
