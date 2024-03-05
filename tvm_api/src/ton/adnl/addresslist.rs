use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.addressList`\n\n```text\nadnl.addressList addrs:(vector adnl.Address) version:int reinit_date:int priority:int expire_at:int = adnl.AddressList;\n```\n"]
pub struct AddressList {
    pub addrs: crate::ton::vector<crate::ton::Boxed, crate::ton::adnl::Address>,
    pub version: crate::ton::int,
    pub reinit_date: crate::ton::int,
    pub priority: crate::ton::int,
    pub expire_at: crate::ton::int,
}
impl Eq for AddressList {}
impl crate::BareSerialize for AddressList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2227e658)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddressList { addrs, version, reinit_date, priority, expire_at } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::adnl::Address>>(addrs)?;
        _ser.write_bare::<crate::ton::int>(version)?;
        _ser.write_bare::<crate::ton::int>(reinit_date)?;
        _ser.write_bare::<crate::ton::int>(priority)?;
        _ser.write_bare::<crate::ton::int>(expire_at)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddressList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let addrs = _de
                .read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::adnl::Address>>()?;
            let version = _de.read_bare::<crate::ton::int>()?;
            let reinit_date = _de.read_bare::<crate::ton::int>()?;
            let priority = _de.read_bare::<crate::ton::int>()?;
            let expire_at = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { addrs, version, reinit_date, priority, expire_at })
        }
    }
}
impl crate::IntoBoxed for AddressList {
    type Boxed = crate::ton::adnl::AddressList;

    fn into_boxed(self) -> crate::ton::adnl::AddressList {
        crate::ton::adnl::AddressList::Adnl_AddressList(self)
    }
}
