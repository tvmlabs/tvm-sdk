use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `fees`\n\n```text\nfees in_fwd_fee:int53 storage_fee:int53 gas_fee:int53 fwd_fee:int53 = Fees;\n```\n"]
pub struct Fees {
    pub in_fwd_fee: crate::ton::int53,
    pub storage_fee: crate::ton::int53,
    pub gas_fee: crate::ton::int53,
    pub fwd_fee: crate::ton::int53,
}
impl Eq for Fees {}
impl crate::BareSerialize for Fees {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x63e9e6bc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Fees { in_fwd_fee, storage_fee, gas_fee, fwd_fee } = self;
        _ser.write_bare::<crate::ton::int53>(in_fwd_fee)?;
        _ser.write_bare::<crate::ton::int53>(storage_fee)?;
        _ser.write_bare::<crate::ton::int53>(gas_fee)?;
        _ser.write_bare::<crate::ton::int53>(fwd_fee)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Fees {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let in_fwd_fee = _de.read_bare::<crate::ton::int53>()?;
            let storage_fee = _de.read_bare::<crate::ton::int53>()?;
            let gas_fee = _de.read_bare::<crate::ton::int53>()?;
            let fwd_fee = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { in_fwd_fee, storage_fee, gas_fee, fwd_fee })
        }
    }
}
impl crate::IntoBoxed for Fees {
    type Boxed = crate::ton::Fees;

    fn into_boxed(self) -> crate::ton::Fees {
        crate::ton::Fees::Fees(self)
    }
}
