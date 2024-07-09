use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.data.badBlock`\n\n```text\ncatchain.block.data.badBlock block:catchain.block = catchain.block.inner.Data;\n```\n"]
pub struct BadBlock {
    pub block: crate::ton::catchain::block::Block,
}
impl Eq for BadBlock {}
impl crate::BareSerialize for BadBlock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb6025a56)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BadBlock { block } = self;
        _ser.write_bare::<crate::ton::catchain::block::Block>(block)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BadBlock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block = _de.read_bare::<crate::ton::catchain::block::Block>()?;
            Ok(Self { block })
        }
    }
}
impl crate::IntoBoxed for BadBlock {
    type Boxed = crate::ton::catchain::block::inner::Data;
    fn into_boxed(self) -> crate::ton::catchain::block::inner::Data {
        crate::ton::catchain::block::inner::Data::Catchain_Block_Data_BadBlock(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.data.fork`\n\n```text\ncatchain.block.data.fork left:catchain.block.Dep right:catchain.block.Dep = catchain.block.inner.Data;\n```\n"]
pub struct Fork {
    pub left: crate::ton::catchain::block::Dep,
    pub right: crate::ton::catchain::block::Dep,
}
impl Eq for Fork {}
impl crate::BareSerialize for Fork {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x647a3a52)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Fork { left, right } = self;
        _ser.write_boxed::<crate::ton::catchain::block::Dep>(left)?;
        _ser.write_boxed::<crate::ton::catchain::block::Dep>(right)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Fork {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let left = _de.read_boxed::<crate::ton::catchain::block::Dep>()?;
            let right = _de.read_boxed::<crate::ton::catchain::block::Dep>()?;
            Ok(Self { left, right })
        }
    }
}
impl crate::IntoBoxed for Fork {
    type Boxed = crate::ton::catchain::block::inner::Data;
    fn into_boxed(self) -> crate::ton::catchain::block::inner::Data {
        crate::ton::catchain::block::inner::Data::Catchain_Block_Data_Fork(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.data.vector`\n\n```text\ncatchain.block.data.vector msgs:(vector bytes) = catchain.block.inner.Data;\n```\n"]
pub struct Vector {
    pub msgs: crate::ton::vector<crate::ton::Bare, crate::ton::bytes>,
}
impl Eq for Vector {}
impl crate::BareSerialize for Vector {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x64a92f2a)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Vector { msgs } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::bytes>>(msgs)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Vector {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let msgs =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::bytes>>()?;
            Ok(Self { msgs })
        }
    }
}
impl crate::IntoBoxed for Vector {
    type Boxed = crate::ton::catchain::block::inner::Data;
    fn into_boxed(self) -> crate::ton::catchain::block::inner::Data {
        crate::ton::catchain::block::inner::Data::Catchain_Block_Data_Vector(self)
    }
}
