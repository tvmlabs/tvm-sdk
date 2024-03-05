use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.slice`\n\n```text\ntvm.slice bytes:bytes = tvm.Slice;\n```\n"]
pub struct Slice {
    pub bytes: crate::ton::bytes,
}
impl Eq for Slice {}
impl crate::BareSerialize for Slice {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x20068ae7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Slice { bytes: ref bytes_ } = self;
        _ser.write_bare::<crate::ton::bytes>(bytes_)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Slice {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let bytes = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { bytes })
        }
    }
}
impl crate::IntoBoxed for Slice {
    type Boxed = crate::ton::tvm::Slice;

    fn into_boxed(self) -> crate::ton::tvm::Slice {
        crate::ton::tvm::Slice::Tvm_Slice(self)
    }
}
