use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.inner.Data`\n\n```text\ncatchain.block.data.badBlock block:catchain.block = catchain.block.inner.Data;\n\ncatchain.block.data.fork left:catchain.block.Dep right:catchain.block.Dep = catchain.block.inner.Data;\n\ncatchain.block.data.nop = catchain.block.inner.Data;\n\ncatchain.block.data.vector msgs:(vector bytes) = catchain.block.inner.Data;\n```\n"]
pub enum Data {
    Catchain_Block_Data_BadBlock(
        crate::ton::catchain::block::inner::catchain::block::data::data::BadBlock,
    ),
    Catchain_Block_Data_Fork(crate::ton::catchain::block::inner::catchain::block::data::data::Fork),
    Catchain_Block_Data_Nop,
    Catchain_Block_Data_Vector(
        crate::ton::catchain::block::inner::catchain::block::data::data::Vector,
    ),
}
impl Data {
    pub fn block(&self) -> Option<&crate::ton::catchain::block::Block> {
        match self {
            Data::Catchain_Block_Data_BadBlock(ref x) => Some(&x.block),
            _ => None,
        }
    }

    pub fn left(&self) -> Option<&crate::ton::catchain::block::Dep> {
        match self {
            Data::Catchain_Block_Data_Fork(ref x) => Some(&x.left),
            _ => None,
        }
    }

    pub fn msgs(&self) -> Option<&crate::ton::vector<crate::ton::Bare, crate::ton::bytes>> {
        match self {
            Data::Catchain_Block_Data_Vector(ref x) => Some(&x.msgs),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&crate::ton::catchain::block::Dep> {
        match self {
            Data::Catchain_Block_Data_Fork(ref x) => Some(&x.right),
            _ => None,
        }
    }
}
impl Eq for Data {}
impl Default for Data {
    fn default() -> Self {
        Data::Catchain_Block_Data_BadBlock(
            crate::ton::catchain::block::inner::catchain::block::data::data::BadBlock::default(),
        )
    }
}
impl crate::BoxedSerialize for Data {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Data::Catchain_Block_Data_BadBlock(x) => (crate::ConstructorNumber(0xb6025a56), x),
            Data::Catchain_Block_Data_Fork(x) => (crate::ConstructorNumber(0x647a3a52), x),
            Data::Catchain_Block_Data_Nop => (crate::ConstructorNumber(0x5482b4d0), &()),
            Data::Catchain_Block_Data_Vector(x) => (crate::ConstructorNumber(0x64a92f2a), x),
        }
    }
}
impl crate::BoxedDeserialize for Data {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xb6025a56),
            crate::ConstructorNumber(0x647a3a52),
            crate::ConstructorNumber(0x5482b4d0),
            crate::ConstructorNumber(0x64a92f2a),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0xb6025a56) => Ok (Data :: Catchain_Block_Data_BadBlock (_de . read_bare :: < crate :: ton :: catchain :: block :: inner :: catchain :: block :: data :: data :: BadBlock > () ?)) , crate :: ConstructorNumber (0x647a3a52) => Ok (Data :: Catchain_Block_Data_Fork (_de . read_bare :: < crate :: ton :: catchain :: block :: inner :: catchain :: block :: data :: data :: Fork > () ?)) , crate :: ConstructorNumber (0x5482b4d0) => Ok (Data :: Catchain_Block_Data_Nop) , crate :: ConstructorNumber (0x64a92f2a) => Ok (Data :: Catchain_Block_Data_Vector (_de . read_bare :: < crate :: ton :: catchain :: block :: inner :: catchain :: block :: data :: data :: Vector > () ?)) , id => _invalid_id ! (id) , }
    }
}
pub mod catchain;
