use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.list`\n\n```text\ntvm.list elements:vector<tvm.StackEntry> = tvm.List;\n```\n"]
pub struct List {
    pub elements: crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>,
}
impl Eq for List {}
impl crate::BareSerialize for List {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4bb78d08)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let List { elements } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
            elements,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for List {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let elements = _de
                .read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
                )?;
            Ok(Self { elements })
        }
    }
}
impl crate::IntoBoxed for List {
    type Boxed = crate::ton::tvm::List;

    fn into_boxed(self) -> crate::ton::tvm::List {
        crate::ton::tvm::List::Tvm_List(self)
    }
}
