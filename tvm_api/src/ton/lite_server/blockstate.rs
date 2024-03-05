use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.blockState`\n\n```text\nliteServer.blockState id:tonNode.blockIdExt root_hash:int256 file_hash:int256 data:bytes = liteServer.BlockState;\n```\n"]
pub struct BlockState {
    pub id: crate::ton::ton_node::blockidext::BlockIdExt,
    pub root_hash: crate::ton::int256,
    pub file_hash: crate::ton::int256,
    pub data: crate::ton::bytes,
}
impl Eq for BlockState {}
impl crate::BareSerialize for BlockState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xabaddc0c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockState { id, root_hash, file_hash, data } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(id)?;
        _ser.write_bare::<crate::ton::int256>(root_hash)?;
        _ser.write_bare::<crate::ton::int256>(file_hash)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let root_hash = _de.read_bare::<crate::ton::int256>()?;
            let file_hash = _de.read_bare::<crate::ton::int256>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, root_hash, file_hash, data })
        }
    }
}
impl crate::IntoBoxed for BlockState {
    type Boxed = crate::ton::lite_server::BlockState;

    fn into_boxed(self) -> crate::ton::lite_server::BlockState {
        crate::ton::lite_server::BlockState::LiteServer_BlockState(self)
    }
}
