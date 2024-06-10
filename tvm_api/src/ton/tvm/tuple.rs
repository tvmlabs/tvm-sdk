use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.tuple`\n\n```text\ntvm.tuple elements:vector<tvm.StackEntry> = tvm.Tuple;\n```\n"]
pub struct Tuple {
    pub elements: crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>,
}
impl Eq for Tuple {}
impl crate::BareSerialize for Tuple {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xaeb3ba63)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Tuple { elements } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
            elements,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Tuple {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let elements = _de
                .read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
                )?;
            Ok(Self { elements })
        }
    }
}
impl crate::IntoBoxed for Tuple {
    type Boxed = crate::ton::tvm::Tuple;
    fn into_boxed(self) -> crate::ton::tvm::Tuple {
        crate::ton::tvm::Tuple::Tvm_Tuple(self)
    }
}
