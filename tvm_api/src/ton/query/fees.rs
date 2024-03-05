use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `query.fees`\n\n```text\nquery.fees source_fees:fees destination_fees:vector<fees> = query.Fees;\n```\n"]
pub struct Fees {
    pub source_fees: crate::ton::fees::Fees,
    pub destination_fees: crate::ton::vector<crate::ton::Bare, crate::ton::fees::Fees>,
}
impl Eq for Fees {}
impl crate::BareSerialize for Fees {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x603d17be)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Fees { source_fees, destination_fees } = self;
        _ser.write_bare::<crate::ton::fees::Fees>(source_fees)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::fees::Fees>>(
            destination_fees,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for Fees {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let source_fees = _de.read_bare::<crate::ton::fees::Fees>()?;
            let destination_fees =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::fees::Fees>>()?;
            Ok(Self { source_fees, destination_fees })
        }
    }
}
impl crate::IntoBoxed for Fees {
    type Boxed = crate::ton::query::Fees;

    fn into_boxed(self) -> crate::ton::query::Fees {
        crate::ton::query::Fees::Query_Fees(self)
    }
}
