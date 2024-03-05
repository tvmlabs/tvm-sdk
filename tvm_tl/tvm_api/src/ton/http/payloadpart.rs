use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.payloadPart`\n\n```text\nhttp.payloadPart data:bytes trailer:(vector http.header) last:Bool = http.PayloadPart;\n```\n"]
pub struct PayloadPart {
    pub data: crate::ton::bytes,
    pub trailer: crate::ton::vector<crate::ton::Bare, crate::ton::http::header::Header>,
    pub last: crate::ton::Bool,
}
impl Eq for PayloadPart {}
impl crate::BareSerialize for PayloadPart {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x295ad764)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PayloadPart { data, trailer, last } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::http::header::Header>>(
            trailer,
        )?;
        _ser.write_boxed::<crate::ton::Bool>(last)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PayloadPart {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let trailer = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: http :: header :: Header > > () ? ;
            let last = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { data, trailer, last })
        }
    }
}
impl crate::IntoBoxed for PayloadPart {
    type Boxed = crate::ton::http::PayloadPart;

    fn into_boxed(self) -> crate::ton::http::PayloadPart {
        crate::ton::http::PayloadPart::Http_PayloadPart(self)
    }
}
