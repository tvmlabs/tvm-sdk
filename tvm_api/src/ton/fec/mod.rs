use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `fec.Type`\n\n```text\nfec.online data_size:int symbol_size:int symbols_count:int = fec.Type;\n\nfec.raptorQ data_size:int symbol_size:int symbols_count:int = fec.Type;\n\nfec.roundRobin data_size:int symbol_size:int symbols_count:int = fec.Type;\n```\n"]
pub enum Type {
    Fec_Online(crate::ton::fec::type_::Online),
    Fec_RaptorQ(crate::ton::fec::type_::RaptorQ),
    Fec_RoundRobin(crate::ton::fec::type_::RoundRobin),
}
impl Type {
    pub fn data_size(&self) -> &crate::ton::int {
        match self {
            Type::Fec_Online(ref x) => &x.data_size,
            Type::Fec_RaptorQ(ref x) => &x.data_size,
            Type::Fec_RoundRobin(ref x) => &x.data_size,
        }
    }

    pub fn symbol_size(&self) -> &crate::ton::int {
        match self {
            Type::Fec_Online(ref x) => &x.symbol_size,
            Type::Fec_RaptorQ(ref x) => &x.symbol_size,
            Type::Fec_RoundRobin(ref x) => &x.symbol_size,
        }
    }

    pub fn symbols_count(&self) -> &crate::ton::int {
        match self {
            Type::Fec_Online(ref x) => &x.symbols_count,
            Type::Fec_RaptorQ(ref x) => &x.symbols_count,
            Type::Fec_RoundRobin(ref x) => &x.symbols_count,
        }
    }
}
impl Eq for Type {}
impl Default for Type {
    fn default() -> Self {
        Type::Fec_Online(crate::ton::fec::type_::Online::default())
    }
}
impl crate::BoxedSerialize for Type {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Type::Fec_Online(x) => (crate::ConstructorNumber(0x0127660c), x),
            Type::Fec_RaptorQ(x) => (crate::ConstructorNumber(0x8b93a7e0), x),
            Type::Fec_RoundRobin(x) => (crate::ConstructorNumber(0x32f528e4), x),
        }
    }
}
impl crate::BoxedDeserialize for Type {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x0127660c),
            crate::ConstructorNumber(0x8b93a7e0),
            crate::ConstructorNumber(0x32f528e4),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0127660c) => {
                Ok(Type::Fec_Online(_de.read_bare::<crate::ton::fec::type_::Online>()?))
            }
            crate::ConstructorNumber(0x8b93a7e0) => {
                Ok(Type::Fec_RaptorQ(_de.read_bare::<crate::ton::fec::type_::RaptorQ>()?))
            }
            crate::ConstructorNumber(0x32f528e4) => {
                Ok(Type::Fec_RoundRobin(_de.read_bare::<crate::ton::fec::type_::RoundRobin>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod type_;
