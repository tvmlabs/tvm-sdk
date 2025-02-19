use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `mbpp.submitNewShardBlock`\n\n```text\nmbpp.submitNewShardBlock block:mbpp.newShardBlock last_known_mc_block:int = mbpp.NewMcBlock;\n```\n"]
pub struct SubmitNewShardBlock {
    pub block: crate::ton::mbpp::newshardblock::NewShardBlock,
    pub last_known_mc_block: crate::ton::int,
}
impl Eq for SubmitNewShardBlock {}
impl crate::BareSerialize for SubmitNewShardBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0e70042c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SubmitNewShardBlock { block, last_known_mc_block } = self;
        _ser.write_bare::<crate::ton::mbpp::newshardblock::NewShardBlock>(block)?;
        _ser.write_bare::<crate::ton::int>(last_known_mc_block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SubmitNewShardBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::mbpp::newshardblock::NewShardBlock>()?;
            let last_known_mc_block = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { block, last_known_mc_block })
        }
    }
}
impl crate::BoxedDeserialize for SubmitNewShardBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0e70042c)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x0e70042c) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SubmitNewShardBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x0e70042c), self)
    }
}
impl crate::Function for SubmitNewShardBlock {
    type Reply = crate::ton::mbpp::NewMcBlock;
}
