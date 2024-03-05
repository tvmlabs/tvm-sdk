use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.header`\n\n```text\nhttp.header name:string value:string = http.Header;\n```\n"]
pub struct Header {
    pub name: crate::ton::string,
    pub value: crate::ton::string,
}
impl Eq for Header {}
impl crate::BareSerialize for Header {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8e9be511)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Header { name, value } = self;
        _ser.write_bare::<crate::ton::string>(name)?;
        _ser.write_bare::<crate::ton::string>(value)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Header {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let name = _de.read_bare::<crate::ton::string>()?;
            let value = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { name, value })
        }
    }
}
impl crate::IntoBoxed for Header {
    type Boxed = crate::ton::http::Header;

    fn into_boxed(self) -> crate::ton::http::Header {
        crate::ton::http::Header::Http_Header(self)
    }
}
