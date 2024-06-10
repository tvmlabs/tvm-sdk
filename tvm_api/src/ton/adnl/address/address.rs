use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.address.tunnel`\n\n```text\nadnl.address.tunnel to:int256 pubkey:PublicKey = adnl.Address;\n```\n"]
pub struct Tunnel {
    pub to: crate::ton::int256,
    pub pubkey: crate::ton::PublicKey,
}
impl Eq for Tunnel {}
impl crate::BareSerialize for Tunnel {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x092b02eb)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Tunnel { to, pubkey } = self;
        _ser.write_bare::<crate::ton::int256>(to)?;
        _ser.write_boxed::<crate::ton::PublicKey>(pubkey)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Tunnel {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let to = _de.read_bare::<crate::ton::int256>()?;
            let pubkey = _de.read_boxed::<crate::ton::PublicKey>()?;
            Ok(Self { to, pubkey })
        }
    }
}
impl crate::IntoBoxed for Tunnel {
    type Boxed = crate::ton::adnl::Address;
    fn into_boxed(self) -> crate::ton::adnl::Address {
        crate::ton::adnl::Address::Adnl_Address_Tunnel(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.address.udp`\n\n```text\nadnl.address.udp ip:int port:int = adnl.Address;\n```\n"]
pub struct Udp {
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
}
impl Eq for Udp {}
impl crate::BareSerialize for Udp {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x670da6e7)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Udp { ip, port } = self;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Udp {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { ip, port })
        }
    }
}
impl crate::IntoBoxed for Udp {
    type Boxed = crate::ton::adnl::Address;
    fn into_boxed(self) -> crate::ton::adnl::Address {
        crate::ton::adnl::Address::Adnl_Address_Udp(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.address.udp6`\n\n```text\nadnl.address.udp6 ip:int128 port:int = adnl.Address;\n```\n"]
pub struct Udp6 {
    pub ip: crate::ton::int128,
    pub port: crate::ton::int,
}
impl Eq for Udp6 {}
impl crate::BareSerialize for Udp6 {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe31d63fa)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Udp6 { ip, port } = self;
        _ser.write_bare::<crate::ton::int128>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Udp6 {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int128>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { ip, port })
        }
    }
}
impl crate::IntoBoxed for Udp6 {
    type Boxed = crate::ton::adnl::Address;
    fn into_boxed(self) -> crate::ton::adnl::Address {
        crate::ton::adnl::Address::Adnl_Address_Udp6(self)
    }
}
