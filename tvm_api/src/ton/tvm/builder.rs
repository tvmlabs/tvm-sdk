use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.builder`\n\n```text\ntvm.builder bytes:bytes = tvm.Builder;\n```\n"]
pub struct Builder {
    pub bytes: crate::ton::bytes,
}
impl Eq for Builder {}
impl crate::BareSerialize for Builder {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xccf52e6d)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Builder { bytes: ref bytes_ } = self;
        _ser.write_bare::<crate::ton::bytes>(bytes_)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Builder {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let bytes = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { bytes })
        }
    }
}
impl crate::IntoBoxed for Builder {
    type Boxed = crate::ton::tvm::Builder;
    fn into_boxed(self) -> crate::ton::tvm::Builder {
        crate::ton::tvm::Builder::Tvm_Builder(self)
    }
}
