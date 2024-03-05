use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `query.Fees`\n\n```text\nquery.fees source_fees:fees destination_fees:vector<fees> = query.Fees;\n```\n"]
pub enum Fees {
    Query_Fees(crate::ton::query::fees::Fees),
}
impl Fees {
    pub fn destination_fees(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::fees::Fees> {
        match self {
            Fees::Query_Fees(ref x) => &x.destination_fees,
        }
    }

    pub fn source_fees(&self) -> &crate::ton::fees::Fees {
        match self {
            Fees::Query_Fees(ref x) => &x.source_fees,
        }
    }

    pub fn only(self) -> crate::ton::query::fees::Fees {
        match self {
            Fees::Query_Fees(x) => x,
        }
    }
}
impl Eq for Fees {}
impl Default for Fees {
    fn default() -> Self {
        Fees::Query_Fees(crate::ton::query::fees::Fees::default())
    }
}
impl crate::BoxedSerialize for Fees {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Fees::Query_Fees(x) => (crate::ConstructorNumber(0x603d17be), x),
        }
    }
}
impl crate::BoxedDeserialize for Fees {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x603d17be)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x603d17be) => {
                Ok(Fees::Query_Fees(_de.read_bare::<crate::ton::query::fees::Fees>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `query.Info`\n\n```text\nquery.info id:int53 valid_until:int53 body_hash:bytes body:bytes init_state:bytes = query.Info;\n```\n"]
pub enum Info {
    Query_Info(crate::ton::query::info::Info),
}
impl Info {
    pub fn body(&self) -> &crate::ton::bytes {
        match self {
            Info::Query_Info(ref x) => &x.body,
        }
    }

    pub fn body_hash(&self) -> &crate::ton::bytes {
        match self {
            Info::Query_Info(ref x) => &x.body_hash,
        }
    }

    pub fn id(&self) -> &crate::ton::int53 {
        match self {
            Info::Query_Info(ref x) => &x.id,
        }
    }

    pub fn init_state(&self) -> &crate::ton::bytes {
        match self {
            Info::Query_Info(ref x) => &x.init_state,
        }
    }

    pub fn valid_until(&self) -> &crate::ton::int53 {
        match self {
            Info::Query_Info(ref x) => &x.valid_until,
        }
    }

    pub fn only(self) -> crate::ton::query::info::Info {
        match self {
            Info::Query_Info(x) => x,
        }
    }
}
impl Eq for Info {}
impl Default for Info {
    fn default() -> Self {
        Info::Query_Info(crate::ton::query::info::Info::default())
    }
}
impl crate::BoxedSerialize for Info {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Info::Query_Info(x) => (crate::ConstructorNumber(0x5689dc70), x),
        }
    }
}
impl crate::BoxedDeserialize for Info {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5689dc70)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5689dc70) => {
                Ok(Info::Query_Info(_de.read_bare::<crate::ton::query::info::Info>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod fees;
pub mod info;
