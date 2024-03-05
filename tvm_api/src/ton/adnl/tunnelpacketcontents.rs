use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.tunnelPacketContents`\n\n```text\nadnl.tunnelPacketContents \n  rand1:bytes \n  flags:# \n  from_ip:flags.0?int\n  from_port:flags.0?int\n  message:flags.1?bytes \n  statistics:flags.2?bytes\n  payment:flags.3?bytes\n  rand2:bytes \n        = adnl.TunnelPacketContents;\n```\n"]
pub struct TunnelPacketContents {
    pub rand1: crate::ton::bytes,
    pub from_ip: Option<crate::ton::int>,
    pub from_port: Option<crate::ton::int>,
    pub message: Option<crate::ton::bytes>,
    pub statistics: Option<crate::ton::bytes>,
    pub payment: Option<crate::ton::bytes>,
    pub rand2: crate::ton::bytes,
}
impl Eq for TunnelPacketContents {}
impl crate::BareSerialize for TunnelPacketContents {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc59138b4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let TunnelPacketContents { rand1, from_ip, from_port, message, statistics, payment, rand2 } =
            self;
        let mut _flags = 0u32;
        if from_ip.is_some() {
            _flags |= 1 << 0u32;
        }
        if from_port.is_some() {
            _flags |= 1 << 0u32;
        }
        if message.is_some() {
            _flags |= 1 << 1u32;
        }
        if statistics.is_some() {
            _flags |= 1 << 2u32;
        }
        if payment.is_some() {
            _flags |= 1 << 3u32;
        }
        _ser.write_bare::<crate::ton::bytes>(rand1)?;
        _ser.write_bare::<crate::ton::Flags>(&_flags)?;
        if let Some(inner) = from_ip {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = from_port {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = message {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        if let Some(inner) = statistics {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        if let Some(inner) = payment {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        _ser.write_bare::<crate::ton::bytes>(rand2)?;
        Ok(())
    }
}
impl crate::BareDeserialize for TunnelPacketContents {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let rand1 = _de.read_bare::<crate::ton::bytes>()?;
            let flags = _de.read_bare::<crate::ton::Flags>()?;
            let from_ip = if flags & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let from_port = if flags & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let message = if flags & (1 << 1u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let statistics = if flags & (1 << 2u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let payment = if flags & (1 << 3u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let rand2 = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { rand1, from_ip, from_port, message, statistics, payment, rand2 })
        }
    }
}
impl crate::IntoBoxed for TunnelPacketContents {
    type Boxed = crate::ton::adnl::TunnelPacketContents;

    fn into_boxed(self) -> crate::ton::adnl::TunnelPacketContents {
        crate::ton::adnl::TunnelPacketContents::Adnl_TunnelPacketContents(self)
    }
}
