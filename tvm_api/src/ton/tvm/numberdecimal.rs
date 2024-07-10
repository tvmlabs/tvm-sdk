use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tvm.numberDecimal`\n\n```text\ntvm.numberDecimal number:string = tvm.Number;\n```\n"]
pub struct NumberDecimal {
    pub number: crate::ton::string,
}
impl Eq for NumberDecimal {}
impl crate::BareSerialize for NumberDecimal {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x45e296b3)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let NumberDecimal { number } = self;
        _ser.write_bare::<crate::ton::string>(number)?;
        Ok(())
    }
}
impl crate::BareDeserialize for NumberDecimal {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let number = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { number })
        }
    }
}
impl crate::IntoBoxed for NumberDecimal {
    type Boxed = crate::ton::tvm::Number;
    fn into_boxed(self) -> crate::ton::tvm::Number {
        crate::ton::tvm::Number::Tvm_NumberDecimal(self)
    }
}
