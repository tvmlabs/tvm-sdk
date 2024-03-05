use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.proxyControlPacketPing`\n\n```text\nadnl.proxyControlPacketPing id:int256 = adnl.ProxyControlPacket;\n```\n"]
pub struct ProxyControlPacketPing {
    pub id: crate::ton::int256,
}
impl Eq for ProxyControlPacketPing {}
impl crate::BareSerialize for ProxyControlPacketPing {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3796e44b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ProxyControlPacketPing { id } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ProxyControlPacketPing {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for ProxyControlPacketPing {
    type Boxed = crate::ton::adnl::ProxyControlPacket;

    fn into_boxed(self) -> crate::ton::adnl::ProxyControlPacket {
        crate::ton::adnl::ProxyControlPacket::Adnl_ProxyControlPacketPing(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.proxyControlPacketPong`\n\n```text\nadnl.proxyControlPacketPong id:int256 = adnl.ProxyControlPacket;\n```\n"]
pub struct ProxyControlPacketPong {
    pub id: crate::ton::int256,
}
impl Eq for ProxyControlPacketPong {}
impl crate::BareSerialize for ProxyControlPacketPong {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4bd1dbfc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ProxyControlPacketPong { id } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ProxyControlPacketPong {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for ProxyControlPacketPong {
    type Boxed = crate::ton::adnl::ProxyControlPacket;

    fn into_boxed(self) -> crate::ton::adnl::ProxyControlPacket {
        crate::ton::adnl::ProxyControlPacket::Adnl_ProxyControlPacketPong(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.proxyControlPacketRegister`\n\n```text\nadnl.proxyControlPacketRegister ip:int port:int = adnl.ProxyControlPacket;\n```\n"]
pub struct ProxyControlPacketRegister {
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
}
impl Eq for ProxyControlPacketRegister {}
impl crate::BareSerialize for ProxyControlPacketRegister {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc309b23f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ProxyControlPacketRegister { ip, port } = self;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ProxyControlPacketRegister {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { ip, port })
        }
    }
}
impl crate::IntoBoxed for ProxyControlPacketRegister {
    type Boxed = crate::ton::adnl::ProxyControlPacket;

    fn into_boxed(self) -> crate::ton::adnl::ProxyControlPacket {
        crate::ton::adnl::ProxyControlPacket::Adnl_ProxyControlPacketRegister(self)
    }
}
