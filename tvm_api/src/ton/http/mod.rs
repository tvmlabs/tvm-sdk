use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `http.Header`\n\n```text\nhttp.header name:string value:string = http.Header;\n```\n"]
pub enum Header {
    Http_Header(crate::ton::http::header::Header),
}
impl Header {
    pub fn name(&self) -> &crate::ton::string {
        match self {
            Header::Http_Header(ref x) => &x.name,
        }
    }

    pub fn value(&self) -> &crate::ton::string {
        match self {
            Header::Http_Header(ref x) => &x.value,
        }
    }

    pub fn only(self) -> crate::ton::http::header::Header {
        match self {
            Header::Http_Header(x) => x,
        }
    }
}
impl Eq for Header {}
impl Default for Header {
    fn default() -> Self {
        Header::Http_Header(crate::ton::http::header::Header::default())
    }
}
impl crate::BoxedSerialize for Header {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Header::Http_Header(x) => (crate::ConstructorNumber(0x8e9be511), x),
        }
    }
}
impl crate::BoxedDeserialize for Header {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8e9be511)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8e9be511) => {
                Ok(Header::Http_Header(_de.read_bare::<crate::ton::http::header::Header>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `http.PayloadPart`\n\n```text\nhttp.payloadPart data:bytes trailer:(vector http.header) last:Bool = http.PayloadPart;\n```\n"]
pub enum PayloadPart {
    Http_PayloadPart(crate::ton::http::payloadpart::PayloadPart),
}
impl PayloadPart {
    pub fn data(&self) -> &crate::ton::bytes {
        match self {
            PayloadPart::Http_PayloadPart(ref x) => &x.data,
        }
    }

    pub fn last(&self) -> &crate::ton::Bool {
        match self {
            PayloadPart::Http_PayloadPart(ref x) => &x.last,
        }
    }

    pub fn trailer(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::http::header::Header> {
        match self {
            PayloadPart::Http_PayloadPart(ref x) => &x.trailer,
        }
    }

    pub fn only(self) -> crate::ton::http::payloadpart::PayloadPart {
        match self {
            PayloadPart::Http_PayloadPart(x) => x,
        }
    }
}
impl Eq for PayloadPart {}
impl Default for PayloadPart {
    fn default() -> Self {
        PayloadPart::Http_PayloadPart(crate::ton::http::payloadpart::PayloadPart::default())
    }
}
impl crate::BoxedSerialize for PayloadPart {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            PayloadPart::Http_PayloadPart(x) => (crate::ConstructorNumber(0x295ad764), x),
        }
    }
}
impl crate::BoxedDeserialize for PayloadPart {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x295ad764)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x295ad764) => Ok(PayloadPart::Http_PayloadPart(
                _de.read_bare::<crate::ton::http::payloadpart::PayloadPart>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `http.Response`\n\n```text\nhttp.response http_version:string status_code:int reason:string headers:(vector http.header) = http.Response;\n```\n"]
pub enum Response {
    Http_Response(crate::ton::http::response::Response),
}
impl Response {
    pub fn headers(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::http::header::Header> {
        match self {
            Response::Http_Response(ref x) => &x.headers,
        }
    }

    pub fn http_version(&self) -> &crate::ton::string {
        match self {
            Response::Http_Response(ref x) => &x.http_version,
        }
    }

    pub fn reason(&self) -> &crate::ton::string {
        match self {
            Response::Http_Response(ref x) => &x.reason,
        }
    }

    pub fn status_code(&self) -> &crate::ton::int {
        match self {
            Response::Http_Response(ref x) => &x.status_code,
        }
    }

    pub fn only(self) -> crate::ton::http::response::Response {
        match self {
            Response::Http_Response(x) => x,
        }
    }
}
impl Eq for Response {}
impl Default for Response {
    fn default() -> Self {
        Response::Http_Response(crate::ton::http::response::Response::default())
    }
}
impl crate::BoxedSerialize for Response {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Response::Http_Response(x) => (crate::ConstructorNumber(0xefb5a773), x),
        }
    }
}
impl crate::BoxedDeserialize for Response {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xefb5a773)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xefb5a773) => Ok(Response::Http_Response(
                _de.read_bare::<crate::ton::http::response::Response>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod header;
pub mod payloadpart;
pub mod response;
pub mod server;
