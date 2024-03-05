use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.electionBid`\n\n```text\nengine.validator.electionBid election_date:int perm_key:int256 adnl_addr:int256 to_send_payload:bytes = engine.validator.ElectionBid;\n```\n"]
pub struct ElectionBid {
    pub election_date: crate::ton::int,
    pub perm_key: crate::ton::int256,
    pub adnl_addr: crate::ton::int256,
    pub to_send_payload: crate::ton::bytes,
}
impl Eq for ElectionBid {}
impl crate::BareSerialize for ElectionBid {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x23b27a3d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ElectionBid { election_date, perm_key, adnl_addr, to_send_payload } = self;
        _ser.write_bare::<crate::ton::int>(election_date)?;
        _ser.write_bare::<crate::ton::int256>(perm_key)?;
        _ser.write_bare::<crate::ton::int256>(adnl_addr)?;
        _ser.write_bare::<crate::ton::bytes>(to_send_payload)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ElectionBid {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let election_date = _de.read_bare::<crate::ton::int>()?;
            let perm_key = _de.read_bare::<crate::ton::int256>()?;
            let adnl_addr = _de.read_bare::<crate::ton::int256>()?;
            let to_send_payload = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { election_date, perm_key, adnl_addr, to_send_payload })
        }
    }
}
impl crate::IntoBoxed for ElectionBid {
    type Boxed = crate::ton::engine::validator::ElectionBid;

    fn into_boxed(self) -> crate::ton::engine::validator::ElectionBid {
        crate::ton::engine::validator::ElectionBid::Engine_Validator_ElectionBid(self)
    }
}
