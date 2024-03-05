use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.packPromise`\n\n```text\npchan.packPromise promise:pchan.promise = Data;\n```\n"]
pub struct PackPromise {
    pub promise: crate::ton::pchan::promise::Promise,
}
impl Eq for PackPromise {}
impl crate::BareSerialize for PackPromise {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xcd3c0ac1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PackPromise { promise } = self;
        _ser.write_bare::<crate::ton::pchan::promise::Promise>(promise)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PackPromise {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let promise = _de.read_bare::<crate::ton::pchan::promise::Promise>()?;
            Ok(Self { promise })
        }
    }
}
impl crate::BoxedDeserialize for PackPromise {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xcd3c0ac1)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xcd3c0ac1) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for PackPromise {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xcd3c0ac1), self)
    }
}
impl crate::Function for PackPromise {
    type Reply = crate::ton::Data;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.signPromise`\n\n```text\npchan.signPromise input_key:InputKey promise:pchan.promise = pchan.Promise;\n```\n"]
pub struct SignPromise {
    pub input_key: crate::ton::InputKey,
    pub promise: crate::ton::pchan::promise::Promise,
}
impl Eq for SignPromise {}
impl crate::BareSerialize for SignPromise {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6c245f1e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SignPromise { input_key, promise } = self;
        _ser.write_boxed::<crate::ton::InputKey>(input_key)?;
        _ser.write_bare::<crate::ton::pchan::promise::Promise>(promise)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SignPromise {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let input_key = _de.read_boxed::<crate::ton::InputKey>()?;
            let promise = _de.read_bare::<crate::ton::pchan::promise::Promise>()?;
            Ok(Self { input_key, promise })
        }
    }
}
impl crate::BoxedDeserialize for SignPromise {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6c245f1e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6c245f1e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SignPromise {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6c245f1e), self)
    }
}
impl crate::Function for SignPromise {
    type Reply = crate::ton::pchan::Promise;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.unpackPromise`\n\n```text\npchan.unpackPromise data:secureBytes = pchan.Promise;\n```\n"]
pub struct UnpackPromise {
    pub data: crate::ton::secureBytes,
}
impl Eq for UnpackPromise {}
impl crate::BareSerialize for UnpackPromise {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb57ce4d3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let UnpackPromise { data } = self;
        _ser.write_bare::<crate::ton::secureBytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for UnpackPromise {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::BoxedDeserialize for UnpackPromise {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb57ce4d3)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xb57ce4d3) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for UnpackPromise {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xb57ce4d3), self)
    }
}
impl crate::Function for UnpackPromise {
    type Reply = crate::ton::pchan::Promise;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.validatePromise`\n\n```text\npchan.validatePromise public_key:bytes promise:pchan.promise = Ok;\n```\n"]
pub struct ValidatePromise {
    pub public_key: crate::ton::bytes,
    pub promise: crate::ton::pchan::promise::Promise,
}
impl Eq for ValidatePromise {}
impl crate::BareSerialize for ValidatePromise {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0f64c4e2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ValidatePromise { public_key, promise } = self;
        _ser.write_bare::<crate::ton::bytes>(public_key)?;
        _ser.write_bare::<crate::ton::pchan::promise::Promise>(promise)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ValidatePromise {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let public_key = _de.read_bare::<crate::ton::bytes>()?;
            let promise = _de.read_bare::<crate::ton::pchan::promise::Promise>()?;
            Ok(Self { public_key, promise })
        }
    }
}
impl crate::BoxedDeserialize for ValidatePromise {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0f64c4e2)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x0f64c4e2) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ValidatePromise {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x0f64c4e2), self)
    }
}
impl crate::Function for ValidatePromise {
    type Reply = crate::ton::Ok;
}
