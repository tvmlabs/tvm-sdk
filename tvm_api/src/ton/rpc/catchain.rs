use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.getBlock`\n\n```text\ncatchain.getBlock block:int256 = catchain.BlockResult;\n```\n"]
pub struct GetBlock {
    pub block: crate::ton::int256,
}
impl Eq for GetBlock {}
impl crate::BareSerialize for GetBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x093ddd78)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBlock { block } = self;
        _ser.write_bare::<crate::ton::int256>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { block })
        }
    }
}
impl crate::BoxedDeserialize for GetBlock {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x093ddd78)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x093ddd78) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBlock {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x093ddd78), self)
    }
}
impl crate::Function for GetBlock {
    type Reply = crate::ton::catchain::BlockResult;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.getBlockHistory`\n\n```text\ncatchain.getBlockHistory block:int256 height:long stop_if:(vector int256) = catchain.Sent;\n```\n"]
pub struct GetBlockHistory {
    pub block: crate::ton::int256,
    pub height: crate::ton::long,
    pub stop_if: crate::ton::vector<crate::ton::Bare, crate::ton::int256>,
}
impl Eq for GetBlockHistory {}
impl crate::BareSerialize for GetBlockHistory {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa8566df6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBlockHistory { block, height, stop_if } = self;
        _ser.write_bare::<crate::ton::int256>(block)?;
        _ser.write_bare::<crate::ton::long>(height)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>(stop_if)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBlockHistory {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::int256>()?;
            let height = _de.read_bare::<crate::ton::long>()?;
            let stop_if =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>()?;
            Ok(Self { block, height, stop_if })
        }
    }
}
impl crate::BoxedDeserialize for GetBlockHistory {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa8566df6)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xa8566df6) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBlockHistory {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xa8566df6), self)
    }
}
impl crate::Function for GetBlockHistory {
    type Reply = crate::ton::catchain::Sent;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.getBlocks`\n\n```text\ncatchain.getBlocks blocks:(vector int256) = catchain.Sent;\n```\n"]
pub struct GetBlocks {
    pub blocks: crate::ton::vector<crate::ton::Bare, crate::ton::int256>,
}
impl Eq for GetBlocks {}
impl crate::BareSerialize for GetBlocks {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0329abc2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBlocks { blocks } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>(blocks)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBlocks {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let blocks =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>()?;
            Ok(Self { blocks })
        }
    }
}
impl crate::BoxedDeserialize for GetBlocks {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0329abc2)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x0329abc2) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBlocks {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x0329abc2), self)
    }
}
impl crate::Function for GetBlocks {
    type Reply = crate::ton::catchain::Sent;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.getDifference`\n\n```text\ncatchain.getDifference rt:(vector int) = catchain.Difference;\n```\n"]
pub struct GetDifference {
    pub rt: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for GetDifference {}
impl crate::BareSerialize for GetDifference {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd06cced8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetDifference { rt } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(rt)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetDifference {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let rt = _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { rt })
        }
    }
}
impl crate::BoxedDeserialize for GetDifference {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd06cced8)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xd06cced8) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetDifference {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xd06cced8), self)
    }
}
impl crate::Function for GetDifference {
    type Reply = crate::ton::catchain::Difference;
}
