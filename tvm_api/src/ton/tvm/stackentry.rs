use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.stackEntryBuilder`\n\n```text\ntvm.stackEntryBuilder builder:tvm.builder = tvm.StackEntry;\n```\n"]
pub struct StackEntryBuilder {
    pub builder: crate::ton::tvm::builder::Builder,
}
impl Eq for StackEntryBuilder {}
impl crate::BareSerialize for StackEntryBuilder {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8e5615d8)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StackEntryBuilder { builder } = self;
        _ser.write_bare::<crate::ton::tvm::builder::Builder>(builder)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StackEntryBuilder {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let builder = _de.read_bare::<crate::ton::tvm::builder::Builder>()?;
            Ok(Self { builder })
        }
    }
}
impl crate::IntoBoxed for StackEntryBuilder {
    type Boxed = crate::ton::tvm::StackEntry;
    fn into_boxed(self) -> crate::ton::tvm::StackEntry {
        crate::ton::tvm::StackEntry::Tvm_StackEntryBuilder(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.stackEntryCell`\n\n```text\ntvm.stackEntryCell cell:tvm.cell = tvm.StackEntry;\n```\n"]
pub struct StackEntryCell {
    pub cell: crate::ton::tvm::cell::Cell,
}
impl Eq for StackEntryCell {}
impl crate::BareSerialize for StackEntryCell {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4db16f20)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StackEntryCell { cell } = self;
        _ser.write_bare::<crate::ton::tvm::cell::Cell>(cell)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StackEntryCell {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let cell = _de.read_bare::<crate::ton::tvm::cell::Cell>()?;
            Ok(Self { cell })
        }
    }
}
impl crate::IntoBoxed for StackEntryCell {
    type Boxed = crate::ton::tvm::StackEntry;
    fn into_boxed(self) -> crate::ton::tvm::StackEntry {
        crate::ton::tvm::StackEntry::Tvm_StackEntryCell(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.stackEntryList`\n\n```text\ntvm.stackEntryList list:tvm.List = tvm.StackEntry;\n```\n"]
pub struct StackEntryList {
    pub list: crate::ton::tvm::List,
}
impl Eq for StackEntryList {}
impl crate::BareSerialize for StackEntryList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb9442d8b)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StackEntryList { list } = self;
        _ser.write_boxed::<crate::ton::tvm::List>(list)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StackEntryList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let list = _de.read_boxed::<crate::ton::tvm::List>()?;
            Ok(Self { list })
        }
    }
}
impl crate::IntoBoxed for StackEntryList {
    type Boxed = crate::ton::tvm::StackEntry;
    fn into_boxed(self) -> crate::ton::tvm::StackEntry {
        crate::ton::tvm::StackEntry::Tvm_StackEntryList(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.stackEntryNumber`\n\n```text\ntvm.stackEntryNumber number:tvm.Number = tvm.StackEntry;\n```\n"]
pub struct StackEntryNumber {
    pub number: crate::ton::tvm::Number,
}
impl Eq for StackEntryNumber {}
impl crate::BareSerialize for StackEntryNumber {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x50fb3dbe)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StackEntryNumber { number } = self;
        _ser.write_boxed::<crate::ton::tvm::Number>(number)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StackEntryNumber {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let number = _de.read_boxed::<crate::ton::tvm::Number>()?;
            Ok(Self { number })
        }
    }
}
impl crate::IntoBoxed for StackEntryNumber {
    type Boxed = crate::ton::tvm::StackEntry;
    fn into_boxed(self) -> crate::ton::tvm::StackEntry {
        crate::ton::tvm::StackEntry::Tvm_StackEntryNumber(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.stackEntrySlice`\n\n```text\ntvm.stackEntrySlice slice:tvm.slice = tvm.StackEntry;\n```\n"]
pub struct StackEntrySlice {
    pub slice: crate::ton::tvm::slice::Slice,
}
impl Eq for StackEntrySlice {}
impl crate::BareSerialize for StackEntrySlice {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x532d6b25)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StackEntrySlice { slice } = self;
        _ser.write_bare::<crate::ton::tvm::slice::Slice>(slice)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StackEntrySlice {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let slice = _de.read_bare::<crate::ton::tvm::slice::Slice>()?;
            Ok(Self { slice })
        }
    }
}
impl crate::IntoBoxed for StackEntrySlice {
    type Boxed = crate::ton::tvm::StackEntry;
    fn into_boxed(self) -> crate::ton::tvm::StackEntry {
        crate::ton::tvm::StackEntry::Tvm_StackEntrySlice(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.stackEntryTuple`\n\n```text\ntvm.stackEntryTuple tuple:tvm.Tuple = tvm.StackEntry;\n```\n"]
pub struct StackEntryTuple {
    pub tuple: crate::ton::tvm::Tuple,
}
impl Eq for StackEntryTuple {}
impl crate::BareSerialize for StackEntryTuple {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf69e63dc)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StackEntryTuple { tuple } = self;
        _ser.write_boxed::<crate::ton::tvm::Tuple>(tuple)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StackEntryTuple {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let tuple = _de.read_boxed::<crate::ton::tvm::Tuple>()?;
            Ok(Self { tuple })
        }
    }
}
impl crate::IntoBoxed for StackEntryTuple {
    type Boxed = crate::ton::tvm::StackEntry;
    fn into_boxed(self) -> crate::ton::tvm::StackEntry {
        crate::ton::tvm::StackEntry::Tvm_StackEntryTuple(self)
    }
}
