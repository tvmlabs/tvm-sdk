use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `data`\n\n```text\ndata bytes:secureBytes = Data;\n```\n"]
pub struct Data {
    pub bytes: crate::ton::secureBytes,
}
impl Eq for Data {}
impl crate::BareSerialize for Data {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe747a971)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Data { bytes: ref bytes_ } = self;
        _ser.write_bare::<crate::ton::secureBytes>(bytes_)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Data {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let bytes = _de.read_bare::<crate::ton::secureBytes>()?;
            Ok(Self { bytes })
        }
    }
}
impl crate::IntoBoxed for Data {
    type Boxed = crate::ton::Data;

    fn into_boxed(self) -> crate::ton::Data {
        crate::ton::Data::Data(self)
    }
}
