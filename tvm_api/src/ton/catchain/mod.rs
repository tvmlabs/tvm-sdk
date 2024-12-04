use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.Block`\n\n```text\ncatchain.block incarnation:int256 src:int height:int data:catchain.block.data signature:bytes = catchain.Block;\n```\n"]
pub enum Block {
    Catchain_Block(crate::ton::catchain::block::Block),
}
impl Block {
    pub fn data(&self) -> &crate::ton::catchain::block::data::Data {
        match self {
            Block::Catchain_Block(ref x) => &x.data,
        }
    }

    pub fn height(&self) -> &crate::ton::int {
        match self {
            Block::Catchain_Block(ref x) => &x.height,
        }
    }

    pub fn incarnation(&self) -> &crate::ton::int256 {
        match self {
            Block::Catchain_Block(ref x) => &x.incarnation,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            Block::Catchain_Block(ref x) => &x.signature,
        }
    }

    pub fn src(&self) -> &crate::ton::int {
        match self {
            Block::Catchain_Block(ref x) => &x.src,
        }
    }

    pub fn only(self) -> crate::ton::catchain::block::Block {
        match self {
            Block::Catchain_Block(x) => x,
        }
    }
}
impl Eq for Block {}
impl Default for Block {
    fn default() -> Self {
        Block::Catchain_Block(crate::ton::catchain::block::Block::default())
    }
}
impl crate::BoxedSerialize for Block {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Block::Catchain_Block(x) => (crate::ConstructorNumber(0xd6554174), x),
        }
    }
}
impl crate::BoxedDeserialize for Block {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd6554174)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd6554174) => {
                Ok(Block::Catchain_Block(_de.read_bare::<crate::ton::catchain::block::Block>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `catchain.BlockResult`\n\n```text\ncatchain.blockNotFound = catchain.BlockResult;\n\ncatchain.blockResult block:catchain.block = catchain.BlockResult;\n```\n"]
pub enum BlockResult {
    #[default]
    Catchain_BlockNotFound,
    Catchain_BlockResult(crate::ton::catchain::blockresult::BlockResult),
}
impl BlockResult {
    pub fn block(&self) -> Option<&crate::ton::catchain::block::Block> {
        match self {
            BlockResult::Catchain_BlockResult(ref x) => Some(&x.block),
            _ => None,
        }
    }
}
impl Eq for BlockResult {}
impl crate::BoxedSerialize for BlockResult {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BlockResult::Catchain_BlockNotFound => (crate::ConstructorNumber(0xb6110884), &()),
            BlockResult::Catchain_BlockResult(x) => (crate::ConstructorNumber(0x9d2a3047), x),
        }
    }
}
impl crate::BoxedDeserialize for BlockResult {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb6110884), crate::ConstructorNumber(0x9d2a3047)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb6110884) => Ok(BlockResult::Catchain_BlockNotFound),
            crate::ConstructorNumber(0x9d2a3047) => Ok(BlockResult::Catchain_BlockResult(
                _de.read_bare::<crate::ton::catchain::blockresult::BlockResult>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::catchain::blockresult::BlockResult> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0xb6110884), &()),
            Some(ref x) => (crate::ConstructorNumber(0x9d2a3047), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::catchain::blockresult::BlockResult> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb6110884), crate::ConstructorNumber(0x9d2a3047)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb6110884) => Ok(None),
            crate::ConstructorNumber(0x9d2a3047) => {
                Ok(Some(_de.read_bare::<crate::ton::catchain::blockresult::BlockResult>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.Blocks`\n\n```text\ncatchain.blocks blocks:(vector catchain.block) = catchain.Blocks;\n```\n"]
pub enum Blocks {
    Catchain_Blocks(crate::ton::catchain::blocks::Blocks),
}
impl Blocks {
    pub fn blocks(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::catchain::block::Block> {
        match self {
            Blocks::Catchain_Blocks(ref x) => &x.blocks,
        }
    }

    pub fn only(self) -> crate::ton::catchain::blocks::Blocks {
        match self {
            Blocks::Catchain_Blocks(x) => x,
        }
    }
}
impl Eq for Blocks {}
impl Default for Blocks {
    fn default() -> Self {
        Blocks::Catchain_Blocks(crate::ton::catchain::blocks::Blocks::default())
    }
}
impl crate::BoxedSerialize for Blocks {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Blocks::Catchain_Blocks(x) => (crate::ConstructorNumber(0x50ecd1c1), x),
        }
    }
}
impl crate::BoxedDeserialize for Blocks {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x50ecd1c1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x50ecd1c1) => Ok(Blocks::Catchain_Blocks(
                _de.read_bare::<crate::ton::catchain::blocks::Blocks>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.Difference`\n\n```text\ncatchain.difference sent_upto:(vector int) = catchain.Difference;\n\ncatchain.differenceFork left:catchain.block.dep right:catchain.block.dep = catchain.Difference;\n```\n"]
pub enum Difference {
    Catchain_Difference(crate::ton::catchain::difference::Difference),
    Catchain_DifferenceFork(crate::ton::catchain::difference::DifferenceFork),
}
impl Difference {
    pub fn left(&self) -> Option<&crate::ton::catchain::block::dep::Dep> {
        match self {
            Difference::Catchain_DifferenceFork(ref x) => Some(&x.left),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&crate::ton::catchain::block::dep::Dep> {
        match self {
            Difference::Catchain_DifferenceFork(ref x) => Some(&x.right),
            _ => None,
        }
    }

    pub fn sent_upto(&self) -> Option<&crate::ton::vector<crate::ton::Bare, crate::ton::int>> {
        match self {
            Difference::Catchain_Difference(ref x) => Some(&x.sent_upto),
            _ => None,
        }
    }
}
impl Eq for Difference {}
impl Default for Difference {
    fn default() -> Self {
        Difference::Catchain_Difference(crate::ton::catchain::difference::Difference::default())
    }
}
impl crate::BoxedSerialize for Difference {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Difference::Catchain_Difference(x) => (crate::ConstructorNumber(0x1415d1ca), x),
            Difference::Catchain_DifferenceFork(x) => (crate::ConstructorNumber(0x4927c06f), x),
        }
    }
}
impl crate::BoxedDeserialize for Difference {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1415d1ca), crate::ConstructorNumber(0x4927c06f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x1415d1ca) => Ok(Difference::Catchain_Difference(
                _de.read_bare::<crate::ton::catchain::difference::Difference>()?,
            )),
            crate::ConstructorNumber(0x4927c06f) => Ok(Difference::Catchain_DifferenceFork(
                _de.read_bare::<crate::ton::catchain::difference::DifferenceFork>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.FirstBlock`\n\n```text\ncatchain.firstblock unique_hash:int256 nodes:(vector int256) = catchain.FirstBlock;\n```\n"]
pub enum FirstBlock {
    Catchain_Firstblock(crate::ton::catchain::firstblock::Firstblock),
}
impl FirstBlock {
    pub fn nodes(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int256> {
        match self {
            FirstBlock::Catchain_Firstblock(ref x) => &x.nodes,
        }
    }

    pub fn unique_hash(&self) -> &crate::ton::int256 {
        match self {
            FirstBlock::Catchain_Firstblock(ref x) => &x.unique_hash,
        }
    }

    pub fn only(self) -> crate::ton::catchain::firstblock::Firstblock {
        match self {
            FirstBlock::Catchain_Firstblock(x) => x,
        }
    }
}
impl Eq for FirstBlock {}
impl Default for FirstBlock {
    fn default() -> Self {
        FirstBlock::Catchain_Firstblock(crate::ton::catchain::firstblock::Firstblock::default())
    }
}
impl crate::BoxedSerialize for FirstBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            FirstBlock::Catchain_Firstblock(x) => (crate::ConstructorNumber(0x10c904fb), x),
        }
    }
}
impl crate::BoxedDeserialize for FirstBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x10c904fb)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x10c904fb) => Ok(FirstBlock::Catchain_Firstblock(
                _de.read_bare::<crate::ton::catchain::firstblock::Firstblock>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.Sent`\n\n```text\ncatchain.sent cnt:int = catchain.Sent;\n```\n"]
pub enum Sent {
    Catchain_Sent(crate::ton::catchain::sent::Sent),
}
impl Sent {
    pub fn cnt(&self) -> &crate::ton::int {
        match self {
            Sent::Catchain_Sent(ref x) => &x.cnt,
        }
    }

    pub fn only(self) -> crate::ton::catchain::sent::Sent {
        match self {
            Sent::Catchain_Sent(x) => x,
        }
    }
}
impl Eq for Sent {}
impl Default for Sent {
    fn default() -> Self {
        Sent::Catchain_Sent(crate::ton::catchain::sent::Sent::default())
    }
}
impl crate::BoxedSerialize for Sent {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Sent::Catchain_Sent(x) => (crate::ConstructorNumber(0xfaf751af), x),
        }
    }
}
impl crate::BoxedDeserialize for Sent {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfaf751af)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xfaf751af) => {
                Ok(Sent::Catchain_Sent(_de.read_bare::<crate::ton::catchain::sent::Sent>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.Update`\n\n```text\ncatchain.blockUpdate block:catchain.block = catchain.Update;\n```\n"]
pub enum Update {
    Catchain_BlockUpdate(crate::ton::catchain::blockupdate::BlockUpdate),
}
impl Update {
    pub fn block(&self) -> &crate::ton::catchain::block::Block {
        match self {
            Update::Catchain_BlockUpdate(ref x) => &x.block,
        }
    }

    pub fn only(self) -> crate::ton::catchain::blockupdate::BlockUpdate {
        match self {
            Update::Catchain_BlockUpdate(x) => x,
        }
    }
}
impl Eq for Update {}
impl Default for Update {
    fn default() -> Self {
        Update::Catchain_BlockUpdate(crate::ton::catchain::blockupdate::BlockUpdate::default())
    }
}
impl crate::BoxedSerialize for Update {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Update::Catchain_BlockUpdate(x) => (crate::ConstructorNumber(0x236758c4), x),
        }
    }
}
impl crate::BoxedDeserialize for Update {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x236758c4)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x236758c4) => Ok(Update::Catchain_BlockUpdate(
                _de.read_bare::<crate::ton::catchain::blockupdate::BlockUpdate>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod block;
pub mod blockresult;
pub mod blocks;
pub mod blockupdate;
pub mod config;
pub mod difference;
pub mod firstblock;
pub mod sent;
