use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.cell`\n\n```text\ntvm.cell bytes:bytes = tvm.Cell;\n```\n"]
pub struct Cell {
    pub bytes: crate::ton::bytes,
}
impl Eq for Cell {}
impl crate::BareSerialize for Cell {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe75ba3a1)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Cell { bytes: ref bytes_ } = self;
        _ser.write_bare::<crate::ton::bytes>(bytes_)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Cell {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let bytes = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { bytes })
        }
    }
}
impl crate::IntoBoxed for Cell {
    type Boxed = crate::ton::tvm::Cell;
    fn into_boxed(self) -> crate::ton::tvm::Cell {
        crate::ton::tvm::Cell::Tvm_Cell(self)
    }
}
