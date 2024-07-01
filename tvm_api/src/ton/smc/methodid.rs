use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.methodIdName`\n\n```text\nsmc.methodIdName name:string = smc.MethodId;\n```\n"]
pub struct MethodIdName {
    pub name: crate::ton::string,
}
impl Eq for MethodIdName {}
impl crate::BareSerialize for MethodIdName {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf127ff94)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MethodIdName { name } = self;
        _ser.write_bare::<crate::ton::string>(name)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MethodIdName {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let name = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { name })
        }
    }
}
impl crate::IntoBoxed for MethodIdName {
    type Boxed = crate::ton::smc::MethodId;
    fn into_boxed(self) -> crate::ton::smc::MethodId {
        crate::ton::smc::MethodId::Smc_MethodIdName(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.methodIdNumber`\n\n```text\nsmc.methodIdNumber number:int32 = smc.MethodId;\n```\n"]
pub struct MethodIdNumber {
    pub number: crate::ton::int32,
}
impl Eq for MethodIdNumber {}
impl crate::BareSerialize for MethodIdNumber {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa423b9fc)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MethodIdNumber { number } = self;
        _ser.write_bare::<crate::ton::int32>(number)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MethodIdNumber {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let number = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { number })
        }
    }
}
impl crate::IntoBoxed for MethodIdNumber {
    type Boxed = crate::ton::smc::MethodId;
    fn into_boxed(self) -> crate::ton::smc::MethodId {
        crate::ton::smc::MethodId::Smc_MethodIdNumber(self)
    }
}
