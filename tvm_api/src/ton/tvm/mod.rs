use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.Builder`\n\n```text\ntvm.builder bytes:bytes = tvm.Builder;\n```\n"]
pub enum Builder {
    Tvm_Builder(crate::ton::tvm::builder::Builder),
}
impl Builder {
    pub fn bytes(&self) -> &crate::ton::bytes {
        match self {
            Builder::Tvm_Builder(ref x) => &x.bytes,
        }
    }

    pub fn only(self) -> crate::ton::tvm::builder::Builder {
        match self {
            Builder::Tvm_Builder(x) => x,
        }
    }
}
impl Eq for Builder {}
impl Default for Builder {
    fn default() -> Self {
        Builder::Tvm_Builder(crate::ton::tvm::builder::Builder::default())
    }
}
impl crate::BoxedSerialize for Builder {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Builder::Tvm_Builder(x) => (crate::ConstructorNumber(0xccf52e6d), x),
        }
    }
}
impl crate::BoxedDeserialize for Builder {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xccf52e6d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xccf52e6d) => {
                Ok(Builder::Tvm_Builder(_de.read_bare::<crate::ton::tvm::builder::Builder>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.Cell`\n\n```text\ntvm.cell bytes:bytes = tvm.Cell;\n```\n"]
pub enum Cell {
    Tvm_Cell(crate::ton::tvm::cell::Cell),
}
impl Cell {
    pub fn bytes(&self) -> &crate::ton::bytes {
        match self {
            Cell::Tvm_Cell(ref x) => &x.bytes,
        }
    }

    pub fn only(self) -> crate::ton::tvm::cell::Cell {
        match self {
            Cell::Tvm_Cell(x) => x,
        }
    }
}
impl Eq for Cell {}
impl Default for Cell {
    fn default() -> Self {
        Cell::Tvm_Cell(crate::ton::tvm::cell::Cell::default())
    }
}
impl crate::BoxedSerialize for Cell {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Cell::Tvm_Cell(x) => (crate::ConstructorNumber(0xe75ba3a1), x),
        }
    }
}
impl crate::BoxedDeserialize for Cell {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe75ba3a1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe75ba3a1) => {
                Ok(Cell::Tvm_Cell(_de.read_bare::<crate::ton::tvm::cell::Cell>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.List`\n\n```text\ntvm.list elements:vector<tvm.StackEntry> = tvm.List;\n```\n"]
pub enum List {
    Tvm_List(crate::ton::tvm::list::List),
}
impl List {
    pub fn elements(&self) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry> {
        match self {
            List::Tvm_List(ref x) => &x.elements,
        }
    }

    pub fn only(self) -> crate::ton::tvm::list::List {
        match self {
            List::Tvm_List(x) => x,
        }
    }
}
impl Eq for List {}
impl Default for List {
    fn default() -> Self {
        List::Tvm_List(crate::ton::tvm::list::List::default())
    }
}
impl crate::BoxedSerialize for List {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            List::Tvm_List(x) => (crate::ConstructorNumber(0x4bb78d08), x),
        }
    }
}
impl crate::BoxedDeserialize for List {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x4bb78d08)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x4bb78d08) => {
                Ok(List::Tvm_List(_de.read_bare::<crate::ton::tvm::list::List>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.Number`\n\n```text\ntvm.numberDecimal number:string = tvm.Number;\n```\n"]
pub enum Number {
    Tvm_NumberDecimal(crate::ton::tvm::numberdecimal::NumberDecimal),
}
impl Number {
    pub fn number(&self) -> &crate::ton::string {
        match self {
            Number::Tvm_NumberDecimal(ref x) => &x.number,
        }
    }

    pub fn only(self) -> crate::ton::tvm::numberdecimal::NumberDecimal {
        match self {
            Number::Tvm_NumberDecimal(x) => x,
        }
    }
}
impl Eq for Number {}
impl Default for Number {
    fn default() -> Self {
        Number::Tvm_NumberDecimal(crate::ton::tvm::numberdecimal::NumberDecimal::default())
    }
}
impl crate::BoxedSerialize for Number {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Number::Tvm_NumberDecimal(x) => (crate::ConstructorNumber(0x45e296b3), x),
        }
    }
}
impl crate::BoxedDeserialize for Number {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x45e296b3)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x45e296b3) => Ok(Number::Tvm_NumberDecimal(
                _de.read_bare::<crate::ton::tvm::numberdecimal::NumberDecimal>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.Slice`\n\n```text\ntvm.slice bytes:bytes = tvm.Slice;\n```\n"]
pub enum Slice {
    Tvm_Slice(crate::ton::tvm::slice::Slice),
}
impl Slice {
    pub fn bytes(&self) -> &crate::ton::bytes {
        match self {
            Slice::Tvm_Slice(ref x) => &x.bytes,
        }
    }

    pub fn only(self) -> crate::ton::tvm::slice::Slice {
        match self {
            Slice::Tvm_Slice(x) => x,
        }
    }
}
impl Eq for Slice {}
impl Default for Slice {
    fn default() -> Self {
        Slice::Tvm_Slice(crate::ton::tvm::slice::Slice::default())
    }
}
impl crate::BoxedSerialize for Slice {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Slice::Tvm_Slice(x) => (crate::ConstructorNumber(0x20068ae7), x),
        }
    }
}
impl crate::BoxedDeserialize for Slice {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x20068ae7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x20068ae7) => {
                Ok(Slice::Tvm_Slice(_de.read_bare::<crate::ton::tvm::slice::Slice>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.StackEntry`\n\n```text\ntvm.stackEntryBuilder builder:tvm.builder = tvm.StackEntry;\n\ntvm.stackEntryCell cell:tvm.cell = tvm.StackEntry;\n\ntvm.stackEntryList list:tvm.List = tvm.StackEntry;\n\ntvm.stackEntryNull = tvm.StackEntry;\n\ntvm.stackEntryNumber number:tvm.Number = tvm.StackEntry;\n\ntvm.stackEntrySlice slice:tvm.slice = tvm.StackEntry;\n\ntvm.stackEntryTuple tuple:tvm.Tuple = tvm.StackEntry;\n\ntvm.stackEntryUnsupported = tvm.StackEntry;\n```\n"]
pub enum StackEntry {
    Tvm_StackEntryBuilder(crate::ton::tvm::stackentry::StackEntryBuilder),
    Tvm_StackEntryCell(crate::ton::tvm::stackentry::StackEntryCell),
    Tvm_StackEntryList(crate::ton::tvm::stackentry::StackEntryList),
    Tvm_StackEntryNull,
    Tvm_StackEntryNumber(crate::ton::tvm::stackentry::StackEntryNumber),
    Tvm_StackEntrySlice(crate::ton::tvm::stackentry::StackEntrySlice),
    Tvm_StackEntryTuple(crate::ton::tvm::stackentry::StackEntryTuple),
    Tvm_StackEntryUnsupported,
}
impl StackEntry {
    pub fn builder(&self) -> Option<&crate::ton::tvm::builder::Builder> {
        match self {
            StackEntry::Tvm_StackEntryBuilder(ref x) => Some(&x.builder),
            _ => None,
        }
    }

    pub fn cell(&self) -> Option<&crate::ton::tvm::cell::Cell> {
        match self {
            StackEntry::Tvm_StackEntryCell(ref x) => Some(&x.cell),
            _ => None,
        }
    }

    pub fn list(&self) -> Option<&crate::ton::tvm::List> {
        match self {
            StackEntry::Tvm_StackEntryList(ref x) => Some(&x.list),
            _ => None,
        }
    }

    pub fn number(&self) -> Option<&crate::ton::tvm::Number> {
        match self {
            StackEntry::Tvm_StackEntryNumber(ref x) => Some(&x.number),
            _ => None,
        }
    }

    pub fn slice(&self) -> Option<&crate::ton::tvm::slice::Slice> {
        match self {
            StackEntry::Tvm_StackEntrySlice(ref x) => Some(&x.slice),
            _ => None,
        }
    }

    pub fn tuple(&self) -> Option<&crate::ton::tvm::Tuple> {
        match self {
            StackEntry::Tvm_StackEntryTuple(ref x) => Some(&x.tuple),
            _ => None,
        }
    }
}
impl Eq for StackEntry {}
impl Default for StackEntry {
    fn default() -> Self {
        StackEntry::Tvm_StackEntryBuilder(crate::ton::tvm::stackentry::StackEntryBuilder::default())
    }
}
impl crate::BoxedSerialize for StackEntry {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            StackEntry::Tvm_StackEntryBuilder(x) => (crate::ConstructorNumber(0x8e5615d8), x),
            StackEntry::Tvm_StackEntryCell(x) => (crate::ConstructorNumber(0x4db16f20), x),
            StackEntry::Tvm_StackEntryList(x) => (crate::ConstructorNumber(0xb9442d8b), x),
            StackEntry::Tvm_StackEntryNull => (crate::ConstructorNumber(0x12454423), &()),
            StackEntry::Tvm_StackEntryNumber(x) => (crate::ConstructorNumber(0x50fb3dbe), x),
            StackEntry::Tvm_StackEntrySlice(x) => (crate::ConstructorNumber(0x532d6b25), x),
            StackEntry::Tvm_StackEntryTuple(x) => (crate::ConstructorNumber(0xf69e63dc), x),
            StackEntry::Tvm_StackEntryUnsupported => (crate::ConstructorNumber(0x169541f2), &()),
        }
    }
}
impl crate::BoxedDeserialize for StackEntry {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x8e5615d8),
            crate::ConstructorNumber(0x4db16f20),
            crate::ConstructorNumber(0xb9442d8b),
            crate::ConstructorNumber(0x12454423),
            crate::ConstructorNumber(0x50fb3dbe),
            crate::ConstructorNumber(0x532d6b25),
            crate::ConstructorNumber(0xf69e63dc),
            crate::ConstructorNumber(0x169541f2),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8e5615d8) => Ok(StackEntry::Tvm_StackEntryBuilder(
                _de.read_bare::<crate::ton::tvm::stackentry::StackEntryBuilder>()?,
            )),
            crate::ConstructorNumber(0x4db16f20) => Ok(StackEntry::Tvm_StackEntryCell(
                _de.read_bare::<crate::ton::tvm::stackentry::StackEntryCell>()?,
            )),
            crate::ConstructorNumber(0xb9442d8b) => Ok(StackEntry::Tvm_StackEntryList(
                _de.read_bare::<crate::ton::tvm::stackentry::StackEntryList>()?,
            )),
            crate::ConstructorNumber(0x12454423) => Ok(StackEntry::Tvm_StackEntryNull),
            crate::ConstructorNumber(0x50fb3dbe) => Ok(StackEntry::Tvm_StackEntryNumber(
                _de.read_bare::<crate::ton::tvm::stackentry::StackEntryNumber>()?,
            )),
            crate::ConstructorNumber(0x532d6b25) => Ok(StackEntry::Tvm_StackEntrySlice(
                _de.read_bare::<crate::ton::tvm::stackentry::StackEntrySlice>()?,
            )),
            crate::ConstructorNumber(0xf69e63dc) => Ok(StackEntry::Tvm_StackEntryTuple(
                _de.read_bare::<crate::ton::tvm::stackentry::StackEntryTuple>()?,
            )),
            crate::ConstructorNumber(0x169541f2) => Ok(StackEntry::Tvm_StackEntryUnsupported),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.Tuple`\n\n```text\ntvm.tuple elements:vector<tvm.StackEntry> = tvm.Tuple;\n```\n"]
pub enum Tuple {
    Tvm_Tuple(crate::ton::tvm::tuple::Tuple),
}
impl Tuple {
    pub fn elements(&self) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry> {
        match self {
            Tuple::Tvm_Tuple(ref x) => &x.elements,
        }
    }

    pub fn only(self) -> crate::ton::tvm::tuple::Tuple {
        match self {
            Tuple::Tvm_Tuple(x) => x,
        }
    }
}
impl Eq for Tuple {}
impl Default for Tuple {
    fn default() -> Self {
        Tuple::Tvm_Tuple(crate::ton::tvm::tuple::Tuple::default())
    }
}
impl crate::BoxedSerialize for Tuple {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Tuple::Tvm_Tuple(x) => (crate::ConstructorNumber(0xaeb3ba63), x),
        }
    }
}
impl crate::BoxedDeserialize for Tuple {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xaeb3ba63)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xaeb3ba63) => {
                Ok(Tuple::Tvm_Tuple(_de.read_bare::<crate::ton::tvm::tuple::Tuple>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod builder;
pub mod cell;
pub mod list;
pub mod numberdecimal;
pub mod slice;
pub mod stackentry;
pub mod tuple;
