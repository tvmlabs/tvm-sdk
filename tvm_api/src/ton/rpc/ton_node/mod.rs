use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadBlock`\n\n```text\ntonNode.downloadBlock block:tonNode.blockIdExt = tonNode.Data;\n```\n"]
pub struct DownloadBlock {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadBlock {}
impl crate::BareSerialize for DownloadBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe27279c3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadBlock { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe27279c3)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xe27279c3) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xe27279c3), self)
    }
}
impl crate::Function for DownloadBlock {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadBlockFull`\n\n```text\ntonNode.downloadBlockFull block:tonNode.blockIdExt = tonNode.DataFull;\n```\n"]
pub struct DownloadBlockFull {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadBlockFull {}
impl crate::BareSerialize for DownloadBlockFull {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6a27c49d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadBlockFull { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadBlockFull {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadBlockFull {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6a27c49d)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6a27c49d) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadBlockFull {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6a27c49d), self)
    }
}
impl crate::Function for DownloadBlockFull {
    type Reply = crate::ton::ton_node::DataFull;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadBlockProof`\n\n```text\ntonNode.downloadBlockProof block:tonNode.blockIdExt = tonNode.Data;\n```\n"]
pub struct DownloadBlockProof {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadBlockProof {}
impl crate::BareSerialize for DownloadBlockProof {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4bd6478a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadBlockProof { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadBlockProof {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadBlockProof {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x4bd6478a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x4bd6478a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadBlockProof {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x4bd6478a), self)
    }
}
impl crate::Function for DownloadBlockProof {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadBlockProofLink`\n\n```text\ntonNode.downloadBlockProofLink block:tonNode.blockIdExt = tonNode.Data;\n```\n"]
pub struct DownloadBlockProofLink {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadBlockProofLink {}
impl crate::BareSerialize for DownloadBlockProofLink {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x25b300c6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadBlockProofLink { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadBlockProofLink {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadBlockProofLink {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x25b300c6)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x25b300c6) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadBlockProofLink {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x25b300c6), self)
    }
}
impl crate::Function for DownloadBlockProofLink {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadBlockProofLinks`\n\n```text\ntonNode.downloadBlockProofLinks blocks:(vector tonNode.blockIdExt) = tonNode.DataList;\n```\n"]
pub struct DownloadBlockProofLinks {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for DownloadBlockProofLinks {}
impl crate::BareSerialize for DownloadBlockProofLinks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x28d12b63)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadBlockProofLinks { blocks } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadBlockProofLinks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { blocks })
        }
    }
}
impl crate::BoxedDeserialize for DownloadBlockProofLinks {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x28d12b63)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x28d12b63) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadBlockProofLinks {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x28d12b63), self)
    }
}
impl crate::Function for DownloadBlockProofLinks {
    type Reply = crate::ton::ton_node::DataList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadBlockProofs`\n\n```text\ntonNode.downloadBlockProofs blocks:(vector tonNode.blockIdExt) = tonNode.DataList;\n```\n"]
pub struct DownloadBlockProofs {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for DownloadBlockProofs {}
impl crate::BareSerialize for DownloadBlockProofs {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa5b053f5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadBlockProofs { blocks } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadBlockProofs {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { blocks })
        }
    }
}
impl crate::BoxedDeserialize for DownloadBlockProofs {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa5b053f5)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xa5b053f5) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadBlockProofs {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xa5b053f5), self)
    }
}
impl crate::Function for DownloadBlockProofs {
    type Reply = crate::ton::ton_node::DataList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadBlocks`\n\n```text\ntonNode.downloadBlocks blocks:(vector tonNode.blockIdExt) = tonNode.DataList;\n```\n"]
pub struct DownloadBlocks {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for DownloadBlocks {}
impl crate::BareSerialize for DownloadBlocks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7659c57d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadBlocks { blocks } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadBlocks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { blocks })
        }
    }
}
impl crate::BoxedDeserialize for DownloadBlocks {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7659c57d)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x7659c57d) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadBlocks {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x7659c57d), self)
    }
}
impl crate::Function for DownloadBlocks {
    type Reply = crate::ton::ton_node::DataList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadKeyBlockProof`\n\n```text\ntonNode.downloadKeyBlockProof block:tonNode.blockIdExt = tonNode.Data;\n```\n"]
pub struct DownloadKeyBlockProof {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadKeyBlockProof {}
impl crate::BareSerialize for DownloadKeyBlockProof {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xec23483a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadKeyBlockProof { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadKeyBlockProof {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadKeyBlockProof {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xec23483a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xec23483a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadKeyBlockProof {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xec23483a), self)
    }
}
impl crate::Function for DownloadKeyBlockProof {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadKeyBlockProofLink`\n\n```text\ntonNode.downloadKeyBlockProofLink block:tonNode.blockIdExt = tonNode.Data;\n```\n"]
pub struct DownloadKeyBlockProofLink {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadKeyBlockProofLink {}
impl crate::BareSerialize for DownloadKeyBlockProofLink {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x12e42ad2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadKeyBlockProofLink { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadKeyBlockProofLink {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadKeyBlockProofLink {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x12e42ad2)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x12e42ad2) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadKeyBlockProofLink {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x12e42ad2), self)
    }
}
impl crate::Function for DownloadKeyBlockProofLink {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadKeyBlockProofLinks`\n\n```text\ntonNode.downloadKeyBlockProofLinks blocks:(vector tonNode.blockIdExt) = tonNode.DataList;\n```\n"]
pub struct DownloadKeyBlockProofLinks {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for DownloadKeyBlockProofLinks {}
impl crate::BareSerialize for DownloadKeyBlockProofLinks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x75c38550)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadKeyBlockProofLinks { blocks } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadKeyBlockProofLinks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { blocks })
        }
    }
}
impl crate::BoxedDeserialize for DownloadKeyBlockProofLinks {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x75c38550)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x75c38550) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadKeyBlockProofLinks {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x75c38550), self)
    }
}
impl crate::Function for DownloadKeyBlockProofLinks {
    type Reply = crate::ton::ton_node::DataList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadKeyBlockProofs`\n\n```text\ntonNode.downloadKeyBlockProofs blocks:(vector tonNode.blockIdExt) = tonNode.DataList;\n```\n"]
pub struct DownloadKeyBlockProofs {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for DownloadKeyBlockProofs {}
impl crate::BareSerialize for DownloadKeyBlockProofs {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc327de3a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadKeyBlockProofs { blocks } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadKeyBlockProofs {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { blocks })
        }
    }
}
impl crate::BoxedDeserialize for DownloadKeyBlockProofs {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc327de3a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xc327de3a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadKeyBlockProofs {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xc327de3a), self)
    }
}
impl crate::Function for DownloadKeyBlockProofs {
    type Reply = crate::ton::ton_node::DataList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadNextBlockFull`\n\n```text\ntonNode.downloadNextBlockFull prev_block:tonNode.blockIdExt = tonNode.DataFull;\n```\n"]
pub struct DownloadNextBlockFull {
    pub prev_block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadNextBlockFull {}
impl crate::BareSerialize for DownloadNextBlockFull {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6ea0374a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadNextBlockFull { prev_block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(prev_block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadNextBlockFull {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let prev_block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { prev_block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadNextBlockFull {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6ea0374a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6ea0374a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadNextBlockFull {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6ea0374a), self)
    }
}
impl crate::Function for DownloadNextBlockFull {
    type Reply = crate::ton::ton_node::DataFull;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadPersistentMsgQueueSlice`\n\n```text\ntonNode.downloadPersistentMsgQueueSlice block:tonNode.blockIdExt masterchain_block:tonNode.blockIdExt target_wc:int offset:long max_size:long = tonNode.Data;\n```\n"]
pub struct DownloadPersistentMsgQueueSlice {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub masterchain_block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub target_wc: crate::ton::int,
    pub offset: crate::ton::long,
    pub max_size: crate::ton::long,
}
impl Eq for DownloadPersistentMsgQueueSlice {}
impl crate::BareSerialize for DownloadPersistentMsgQueueSlice {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x32fb50b1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadPersistentMsgQueueSlice {
            block,
            masterchain_block,
            target_wc,
            offset,
            max_size,
        } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(masterchain_block)?;
        _ser.write_bare::<crate::ton::int>(target_wc)?;
        _ser.write_bare::<crate::ton::long>(offset)?;
        _ser.write_bare::<crate::ton::long>(max_size)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadPersistentMsgQueueSlice {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let masterchain_block =
                _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let target_wc = _de.read_bare::<crate::ton::int>()?;
            let offset = _de.read_bare::<crate::ton::long>()?;
            let max_size = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { block, masterchain_block, target_wc, offset, max_size })
        }
    }
}
impl crate::BoxedDeserialize for DownloadPersistentMsgQueueSlice {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x32fb50b1)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x32fb50b1) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadPersistentMsgQueueSlice {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x32fb50b1), self)
    }
}
impl crate::Function for DownloadPersistentMsgQueueSlice {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadPersistentState`\n\n```text\ntonNode.downloadPersistentState block:tonNode.blockIdExt masterchain_block:tonNode.blockIdExt = tonNode.Data;\n```\n"]
pub struct DownloadPersistentState {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub masterchain_block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadPersistentState {}
impl crate::BareSerialize for DownloadPersistentState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7f99e3b8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadPersistentState { block, masterchain_block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(masterchain_block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadPersistentState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let masterchain_block =
                _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block, masterchain_block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadPersistentState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7f99e3b8)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x7f99e3b8) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadPersistentState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x7f99e3b8), self)
    }
}
impl crate::Function for DownloadPersistentState {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadPersistentStateSlice`\n\n```text\ntonNode.downloadPersistentStateSlice block:tonNode.blockIdExt masterchain_block:tonNode.blockIdExt offset:long max_size:long = tonNode.Data;\n```\n"]
pub struct DownloadPersistentStateSlice {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub masterchain_block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub offset: crate::ton::long,
    pub max_size: crate::ton::long,
}
impl Eq for DownloadPersistentStateSlice {}
impl crate::BareSerialize for DownloadPersistentStateSlice {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf5e9e6e3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadPersistentStateSlice { block, masterchain_block, offset, max_size } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(masterchain_block)?;
        _ser.write_bare::<crate::ton::long>(offset)?;
        _ser.write_bare::<crate::ton::long>(max_size)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadPersistentStateSlice {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let masterchain_block =
                _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let offset = _de.read_bare::<crate::ton::long>()?;
            let max_size = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { block, masterchain_block, offset, max_size })
        }
    }
}
impl crate::BoxedDeserialize for DownloadPersistentStateSlice {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf5e9e6e3)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf5e9e6e3) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadPersistentStateSlice {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf5e9e6e3), self)
    }
}
impl crate::Function for DownloadPersistentStateSlice {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadQueueUpdate`\n\n```text\ntonNode.downloadQueueUpdate block:tonNode.blockIdExt target_wc:int = tonNode.Data;\n```\n"]
pub struct DownloadQueueUpdate {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub target_wc: crate::ton::int,
}
impl Eq for DownloadQueueUpdate {}
impl crate::BareSerialize for DownloadQueueUpdate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb67c0e6c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadQueueUpdate { block, target_wc } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::int>(target_wc)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadQueueUpdate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let target_wc = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, target_wc })
        }
    }
}
impl crate::BoxedDeserialize for DownloadQueueUpdate {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb67c0e6c)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xb67c0e6c) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadQueueUpdate {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xb67c0e6c), self)
    }
}
impl crate::Function for DownloadQueueUpdate {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.downloadZeroState`\n\n```text\ntonNode.downloadZeroState block:tonNode.blockIdExt = tonNode.Data;\n```\n"]
pub struct DownloadZeroState {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for DownloadZeroState {}
impl crate::BareSerialize for DownloadZeroState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xadcc1e5a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DownloadZeroState { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DownloadZeroState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for DownloadZeroState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xadcc1e5a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xadcc1e5a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DownloadZeroState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xadcc1e5a), self)
    }
}
impl crate::Function for DownloadZeroState {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.getArchiveInfo`\n\n```text\ntonNode.getArchiveInfo masterchain_seqno:int = tonNode.ArchiveInfo;\n```\n"]
pub struct GetArchiveInfo {
    pub masterchain_seqno: crate::ton::int,
}
impl Eq for GetArchiveInfo {}
impl crate::BareSerialize for GetArchiveInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7b2dd941)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetArchiveInfo { masterchain_seqno } = self;
        _ser.write_bare::<crate::ton::int>(masterchain_seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetArchiveInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let masterchain_seqno = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { masterchain_seqno })
        }
    }
}
impl crate::BoxedDeserialize for GetArchiveInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7b2dd941)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x7b2dd941) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetArchiveInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x7b2dd941), self)
    }
}
impl crate::Function for GetArchiveInfo {
    type Reply = crate::ton::ton_node::ArchiveInfo;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.getArchiveSlice`\n\n```text\ntonNode.getArchiveSlice archive_id:long offset:long max_size:int = tonNode.Data;\n```\n"]
pub struct GetArchiveSlice {
    pub archive_id: crate::ton::long,
    pub offset: crate::ton::long,
    pub max_size: crate::ton::int,
}
impl Eq for GetArchiveSlice {}
impl crate::BareSerialize for GetArchiveSlice {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x203b5168)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetArchiveSlice { archive_id, offset, max_size } = self;
        _ser.write_bare::<crate::ton::long>(archive_id)?;
        _ser.write_bare::<crate::ton::long>(offset)?;
        _ser.write_bare::<crate::ton::int>(max_size)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetArchiveSlice {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let archive_id = _de.read_bare::<crate::ton::long>()?;
            let offset = _de.read_bare::<crate::ton::long>()?;
            let max_size = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { archive_id, offset, max_size })
        }
    }
}
impl crate::BoxedDeserialize for GetArchiveSlice {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x203b5168)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x203b5168) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetArchiveSlice {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x203b5168), self)
    }
}
impl crate::Function for GetArchiveSlice {
    type Reply = crate::ton::ton_node::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.getCapabilities`\n\n```text\ntonNode.getCapabilities = tonNode.Capabilities;\n```\n"]
pub struct GetCapabilities;
impl Eq for GetCapabilities {}
impl crate::BareSerialize for GetCapabilities {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdee618f8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetCapabilities {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetCapabilities {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdee618f8)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xdee618f8) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetCapabilities {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xdee618f8), self)
    }
}
impl crate::Function for GetCapabilities {
    type Reply = crate::ton::ton_node::Capabilities;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.getNextBlockDescription`\n\n```text\ntonNode.getNextBlockDescription prev_block:tonNode.blockIdExt = tonNode.BlockDescription;\n```\n"]
pub struct GetNextBlockDescription {
    pub prev_block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for GetNextBlockDescription {}
impl crate::BareSerialize for GetNextBlockDescription {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1455b0f3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetNextBlockDescription { prev_block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(prev_block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetNextBlockDescription {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let prev_block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { prev_block })
        }
    }
}
impl crate::BoxedDeserialize for GetNextBlockDescription {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1455b0f3)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x1455b0f3) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetNextBlockDescription {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x1455b0f3), self)
    }
}
impl crate::Function for GetNextBlockDescription {
    type Reply = crate::ton::ton_node::BlockDescription;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.getNextBlocksDescription`\n\n```text\ntonNode.getNextBlocksDescription prev_block:tonNode.blockIdExt limit:int = tonNode.BlocksDescription;\n```\n"]
pub struct GetNextBlocksDescription {
    pub prev_block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub limit: crate::ton::int,
}
impl Eq for GetNextBlocksDescription {}
impl crate::BareSerialize for GetNextBlocksDescription {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3f2812c4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetNextBlocksDescription { prev_block, limit } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(prev_block)?;
        _ser.write_bare::<crate::ton::int>(limit)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetNextBlocksDescription {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let prev_block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let limit = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { prev_block, limit })
        }
    }
}
impl crate::BoxedDeserialize for GetNextBlocksDescription {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3f2812c4)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x3f2812c4) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetNextBlocksDescription {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x3f2812c4), self)
    }
}
impl crate::Function for GetNextBlocksDescription {
    type Reply = crate::ton::ton_node::BlocksDescription;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.getNextKeyBlockIds`\n\n```text\ntonNode.getNextKeyBlockIds block:tonNode.blockIdExt max_size:int = tonNode.KeyBlocks;\n```\n"]
pub struct GetNextKeyBlockIds {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub max_size: crate::ton::int,
}
impl Eq for GetNextKeyBlockIds {}
impl crate::BareSerialize for GetNextKeyBlockIds {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf2e7cfbb)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetNextKeyBlockIds { block, max_size } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::int>(max_size)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetNextKeyBlockIds {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let max_size = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, max_size })
        }
    }
}
impl crate::BoxedDeserialize for GetNextKeyBlockIds {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf2e7cfbb)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf2e7cfbb) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetNextKeyBlockIds {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf2e7cfbb), self)
    }
}
impl crate::Function for GetNextKeyBlockIds {
    type Reply = crate::ton::ton_node::KeyBlocks;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.getPrevBlocksDescription`\n\n```text\ntonNode.getPrevBlocksDescription next_block:tonNode.blockIdExt limit:int cutoff_seqno:int = tonNode.BlocksDescription;\n```\n"]
pub struct GetPrevBlocksDescription {
    pub next_block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub limit: crate::ton::int,
    pub cutoff_seqno: crate::ton::int,
}
impl Eq for GetPrevBlocksDescription {}
impl crate::BareSerialize for GetPrevBlocksDescription {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5c6d6cc9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetPrevBlocksDescription { next_block, limit, cutoff_seqno } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(next_block)?;
        _ser.write_bare::<crate::ton::int>(limit)?;
        _ser.write_bare::<crate::ton::int>(cutoff_seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetPrevBlocksDescription {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let next_block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let limit = _de.read_bare::<crate::ton::int>()?;
            let cutoff_seqno = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { next_block, limit, cutoff_seqno })
        }
    }
}
impl crate::BoxedDeserialize for GetPrevBlocksDescription {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5c6d6cc9)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x5c6d6cc9) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetPrevBlocksDescription {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x5c6d6cc9), self)
    }
}
impl crate::Function for GetPrevBlocksDescription {
    type Reply = crate::ton::ton_node::BlocksDescription;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.prepareBlock`\n\n```text\ntonNode.prepareBlock block:tonNode.blockIdExt = tonNode.Prepared;\n```\n"]
pub struct PrepareBlock {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for PrepareBlock {}
impl crate::BareSerialize for PrepareBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x75a37f4e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PrepareBlock { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PrepareBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for PrepareBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x75a37f4e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x75a37f4e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PrepareBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x75a37f4e), self)
    }
}
impl crate::Function for PrepareBlock {
    type Reply = crate::ton::ton_node::Prepared;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.prepareBlockProof`\n\n```text\ntonNode.prepareBlockProof block:tonNode.blockIdExt allow_partial:Bool = tonNode.PreparedProof;\n```\n"]
pub struct PrepareBlockProof {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub allow_partial: crate::ton::Bool,
}
impl Eq for PrepareBlockProof {}
impl crate::BareSerialize for PrepareBlockProof {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x875c3308)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PrepareBlockProof { block, allow_partial } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_boxed::<crate::ton::Bool>(allow_partial)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PrepareBlockProof {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let allow_partial = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { block, allow_partial })
        }
    }
}
impl crate::BoxedDeserialize for PrepareBlockProof {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x875c3308)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x875c3308) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PrepareBlockProof {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x875c3308), self)
    }
}
impl crate::Function for PrepareBlockProof {
    type Reply = crate::ton::ton_node::PreparedProof;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.prepareBlockProofs`\n\n```text\ntonNode.prepareBlockProofs blocks:(vector tonNode.blockIdExt) allow_partial:Bool = tonNode.PreparedProof;\n```\n"]
pub struct PrepareBlockProofs {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
    pub allow_partial: crate::ton::Bool,
}
impl Eq for PrepareBlockProofs {}
impl crate::BareSerialize for PrepareBlockProofs {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xed79b2b8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PrepareBlockProofs { blocks, allow_partial } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        _ser.write_boxed::<crate::ton::Bool>(allow_partial)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PrepareBlockProofs {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            let allow_partial = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { blocks, allow_partial })
        }
    }
}
impl crate::BoxedDeserialize for PrepareBlockProofs {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xed79b2b8)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xed79b2b8) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PrepareBlockProofs {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xed79b2b8), self)
    }
}
impl crate::Function for PrepareBlockProofs {
    type Reply = crate::ton::ton_node::PreparedProof;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.prepareBlocks`\n\n```text\ntonNode.prepareBlocks blocks:(vector tonNode.blockIdExt) = tonNode.Prepared;\n```\n"]
pub struct PrepareBlocks {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for PrepareBlocks {}
impl crate::BareSerialize for PrepareBlocks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6affabfc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PrepareBlocks { blocks } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for PrepareBlocks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { blocks })
        }
    }
}
impl crate::BoxedDeserialize for PrepareBlocks {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6affabfc)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6affabfc) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PrepareBlocks {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6affabfc), self)
    }
}
impl crate::Function for PrepareBlocks {
    type Reply = crate::ton::ton_node::Prepared;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.prepareKeyBlockProof`\n\n```text\ntonNode.prepareKeyBlockProof block:tonNode.blockIdExt allow_partial:Bool = tonNode.PreparedProof;\n```\n"]
pub struct PrepareKeyBlockProof {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub allow_partial: crate::ton::Bool,
}
impl Eq for PrepareKeyBlockProof {}
impl crate::BareSerialize for PrepareKeyBlockProof {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x77364c38)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PrepareKeyBlockProof { block, allow_partial } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_boxed::<crate::ton::Bool>(allow_partial)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PrepareKeyBlockProof {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let allow_partial = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { block, allow_partial })
        }
    }
}
impl crate::BoxedDeserialize for PrepareKeyBlockProof {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x77364c38)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x77364c38) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PrepareKeyBlockProof {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x77364c38), self)
    }
}
impl crate::Function for PrepareKeyBlockProof {
    type Reply = crate::ton::ton_node::PreparedProof;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.prepareKeyBlockProofs`\n\n```text\ntonNode.prepareKeyBlockProofs blocks:(vector tonNode.blockIdExt) allow_partial:Bool = tonNode.PreparedProof;\n```\n"]
pub struct PrepareKeyBlockProofs {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
    pub allow_partial: crate::ton::Bool,
}
impl Eq for PrepareKeyBlockProofs {}
impl crate::BareSerialize for PrepareKeyBlockProofs {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8c6cfbe4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PrepareKeyBlockProofs { blocks, allow_partial } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (blocks) ? ;
        _ser.write_boxed::<crate::ton::Bool>(allow_partial)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PrepareKeyBlockProofs {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            let allow_partial = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { blocks, allow_partial })
        }
    }
}
impl crate::BoxedDeserialize for PrepareKeyBlockProofs {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8c6cfbe4)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x8c6cfbe4) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PrepareKeyBlockProofs {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x8c6cfbe4), self)
    }
}
impl crate::Function for PrepareKeyBlockProofs {
    type Reply = crate::ton::ton_node::PreparedProof;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.preparePersistentMsgQueue`\n\n```text\ntonNode.preparePersistentMsgQueue block:tonNode.blockIdExt masterchain_block:tonNode.blockIdExt target_wc:int = tonNode.PreparedState;\n```\n"]
pub struct PreparePersistentMsgQueue {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub masterchain_block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub target_wc: crate::ton::int,
}
impl Eq for PreparePersistentMsgQueue {}
impl crate::BareSerialize for PreparePersistentMsgQueue {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf4ea081f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PreparePersistentMsgQueue { block, masterchain_block, target_wc } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(masterchain_block)?;
        _ser.write_bare::<crate::ton::int>(target_wc)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PreparePersistentMsgQueue {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let masterchain_block =
                _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let target_wc = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, masterchain_block, target_wc })
        }
    }
}
impl crate::BoxedDeserialize for PreparePersistentMsgQueue {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf4ea081f)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf4ea081f) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PreparePersistentMsgQueue {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf4ea081f), self)
    }
}
impl crate::Function for PreparePersistentMsgQueue {
    type Reply = crate::ton::ton_node::PreparedState;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.preparePersistentState`\n\n```text\ntonNode.preparePersistentState block:tonNode.blockIdExt masterchain_block:tonNode.blockIdExt = tonNode.PreparedState;\n```\n"]
pub struct PreparePersistentState {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub masterchain_block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for PreparePersistentState {}
impl crate::BareSerialize for PreparePersistentState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xfeea269e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PreparePersistentState { block, masterchain_block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(masterchain_block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PreparePersistentState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let masterchain_block =
                _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block, masterchain_block })
        }
    }
}
impl crate::BoxedDeserialize for PreparePersistentState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfeea269e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xfeea269e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PreparePersistentState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xfeea269e), self)
    }
}
impl crate::Function for PreparePersistentState {
    type Reply = crate::ton::ton_node::PreparedState;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.prepareQueueUpdate`\n\n```text\ntonNode.prepareQueueUpdate block:tonNode.blockIdExt target_wc:int = tonNode.Prepared;\n```\n"]
pub struct PrepareQueueUpdate {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
    pub target_wc: crate::ton::int,
}
impl Eq for PrepareQueueUpdate {}
impl crate::BareSerialize for PrepareQueueUpdate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2032ce4e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PrepareQueueUpdate { block, target_wc } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        _ser.write_bare::<crate::ton::int>(target_wc)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PrepareQueueUpdate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            let target_wc = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, target_wc })
        }
    }
}
impl crate::BoxedDeserialize for PrepareQueueUpdate {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2032ce4e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x2032ce4e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PrepareQueueUpdate {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x2032ce4e), self)
    }
}
impl crate::Function for PrepareQueueUpdate {
    type Reply = crate::ton::ton_node::Prepared;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.prepareZeroState`\n\n```text\ntonNode.prepareZeroState block:tonNode.blockIdExt = tonNode.PreparedState;\n```\n"]
pub struct PrepareZeroState {
    pub block: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for PrepareZeroState {}
impl crate::BareSerialize for PrepareZeroState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x41ce0825)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PrepareZeroState { block } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PrepareZeroState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for PrepareZeroState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x41ce0825)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x41ce0825) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PrepareZeroState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x41ce0825), self)
    }
}
impl crate::Function for PrepareZeroState {
    type Reply = crate::ton::ton_node::PreparedState;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.query`\n\n```text\ntonNode.query = Object;\n```\n"]
pub struct Query;
impl Eq for Query {}
impl crate::BareSerialize for Query {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x69f324d3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for Query {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for Query {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x69f324d3)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x69f324d3) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Query {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x69f324d3), self)
    }
}
impl crate::Function for Query {
    type Reply = crate::ton::Object;
}
pub mod slave;
