use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.runMethodResult`\n\n```text\nliteServer.runMethodResult mode:# id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:mode.0?bytes proof:mode.0?bytes state_proof:mode.1?bytes init_c7:mode.3?bytes lib_extras:mode.4?bytes exit_code:int result:mode.2?bytes = liteServer.RunMethodResult;\n```\n"]
pub struct RunMethodResult {
    pub mode: crate::ton::int,
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub shardblk: crate::ton::ton_node::blockidext::BlockIdExt,
    pub shard_proof: Option<crate::ton::bytes>,
    pub proof: Option<crate::ton::bytes>,
    pub state_proof: Option<crate::ton::bytes>,
    pub init_c7: Option<crate::ton::bytes>,
    pub lib_extras: Option<crate::ton::bytes>,
    pub exit_code: crate::ton::int,
    pub result: Option<crate::ton::bytes>,
}
impl Eq for RunMethodResult {}
impl crate::BareSerialize for RunMethodResult {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa39a616b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunMethodResult {
            mode,
            id,
            shardblk,
            shard_proof,
            proof,
            state_proof,
            init_c7,
            lib_extras,
            exit_code,
            result,
        } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(shardblk)?;
        if let Some(inner) = shard_proof {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        if let Some(inner) = proof {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        if let Some(inner) = state_proof {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        if let Some(inner) = init_c7 {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        if let Some(inner) = lib_extras {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        _ser.write_bare::<crate::ton::int>(exit_code)?;
        if let Some(inner) = result {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for RunMethodResult {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let shardblk = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let shard_proof = if mode & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let proof = if mode & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let state_proof = if mode & (1 << 1u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let init_c7 = if mode & (1 << 3u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let lib_extras = if mode & (1 << 4u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let exit_code = _de.read_bare::<crate::ton::int>()?;
            let result = if mode & (1 << 2u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            Ok(Self {
                mode,
                id,
                shardblk,
                shard_proof,
                proof,
                state_proof,
                init_c7,
                lib_extras,
                exit_code,
                result,
            })
        }
    }
}
impl crate::IntoBoxed for RunMethodResult {
    type Boxed = crate::ton::lite_server::RunMethodResult;

    fn into_boxed(self) -> crate::ton::lite_server::RunMethodResult {
        crate::ton::lite_server::RunMethodResult::LiteServer_RunMethodResult(self)
    }
}
