use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.getNextPayloadPart`\n\n```text\nhttp.getNextPayloadPart id:int256 seqno:int max_chunk_size:int = http.PayloadPart;\n```\n"]
pub struct GetNextPayloadPart {
    pub id: crate::ton::int256,
    pub seqno: crate::ton::int,
    pub max_chunk_size: crate::ton::int,
}
impl Eq for GetNextPayloadPart {}
impl crate::BareSerialize for GetNextPayloadPart {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x90745d0c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetNextPayloadPart { id, seqno, max_chunk_size } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_bare::<crate::ton::int>(max_chunk_size)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetNextPayloadPart {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let max_chunk_size = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, seqno, max_chunk_size })
        }
    }
}
impl crate::BoxedDeserialize for GetNextPayloadPart {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x90745d0c)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x90745d0c) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetNextPayloadPart {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x90745d0c), self)
    }
}
impl crate::Function for GetNextPayloadPart {
    type Reply = crate::ton::http::PayloadPart;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.request`\n\n```text\nhttp.request id:int256 method:string url:string http_version:string headers:(vector http.header) = http.Response;\n```\n"]
pub struct Request {
    pub id: crate::ton::int256,
    pub method: crate::ton::string,
    pub url: crate::ton::string,
    pub http_version: crate::ton::string,
    pub headers: crate::ton::vector<crate::ton::Bare, crate::ton::http::header::Header>,
}
impl Eq for Request {}
impl crate::BareSerialize for Request {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x61b191e1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Request { id, method, url, http_version, headers } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::string>(method)?;
        _ser.write_bare::<crate::ton::string>(url)?;
        _ser.write_bare::<crate::ton::string>(http_version)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::http::header::Header>>(
            headers,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Request {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let method = _de.read_bare::<crate::ton::string>()?;
            let url = _de.read_bare::<crate::ton::string>()?;
            let http_version = _de.read_bare::<crate::ton::string>()?;
            let headers = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: http :: header :: Header > > () ? ;
            Ok(Self { id, method, url, http_version, headers })
        }
    }
}
impl crate::BoxedDeserialize for Request {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x61b191e1)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x61b191e1) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Request {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x61b191e1), self)
    }
}
impl crate::Function for Request {
    type Reply = crate::ton::http::Response;
}
