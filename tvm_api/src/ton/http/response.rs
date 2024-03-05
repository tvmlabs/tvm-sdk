use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `http.response`\n\n```text\nhttp.response http_version:string status_code:int reason:string headers:(vector http.header) = http.Response;\n```\n"]
pub struct Response {
    pub http_version: crate::ton::string,
    pub status_code: crate::ton::int,
    pub reason: crate::ton::string,
    pub headers: crate::ton::vector<crate::ton::Bare, crate::ton::http::header::Header>,
}
impl Eq for Response {}
impl crate::BareSerialize for Response {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xefb5a773)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Response { http_version, status_code, reason, headers } = self;
        _ser.write_bare::<crate::ton::string>(http_version)?;
        _ser.write_bare::<crate::ton::int>(status_code)?;
        _ser.write_bare::<crate::ton::string>(reason)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::http::header::Header>>(
            headers,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Response {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let http_version = _de.read_bare::<crate::ton::string>()?;
            let status_code = _de.read_bare::<crate::ton::int>()?;
            let reason = _de.read_bare::<crate::ton::string>()?;
            let headers = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: http :: header :: Header > > () ? ;
            Ok(Self { http_version, status_code, reason, headers })
        }
    }
}
impl crate::IntoBoxed for Response {
    type Boxed = crate::ton::http::Response;

    fn into_boxed(self) -> crate::ton::http::Response {
        crate::ton::http::Response::Http_Response(self)
    }
}
