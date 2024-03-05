use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.proxyPacketHeader`\n\n```text\nadnl.proxyPacketHeader\n  proxy_id:int256\n  flags:# \n  ip:flags.0?int\n  port:flags.0?int\n  adnl_start_time:flags.1?int\n  seqno:flags.2?long\n  date:flags.3?int\n  signature:int256 = adnl.ProxyPacketHeader;\n```\n"]
pub struct ProxyPacketHeader {
    pub proxy_id: crate::ton::int256,
    pub ip: Option<crate::ton::int>,
    pub port: Option<crate::ton::int>,
    pub adnl_start_time: Option<crate::ton::int>,
    pub seqno: Option<crate::ton::long>,
    pub date: Option<crate::ton::int>,
    pub signature: crate::ton::int256,
}
impl Eq for ProxyPacketHeader {}
impl crate::BareSerialize for ProxyPacketHeader {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x08693c78)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ProxyPacketHeader { proxy_id, ip, port, adnl_start_time, seqno, date, signature } =
            self;
        let mut _flags = 0u32;
        if ip.is_some() {
            _flags |= 1 << 0u32;
        }
        if port.is_some() {
            _flags |= 1 << 0u32;
        }
        if adnl_start_time.is_some() {
            _flags |= 1 << 1u32;
        }
        if seqno.is_some() {
            _flags |= 1 << 2u32;
        }
        if date.is_some() {
            _flags |= 1 << 3u32;
        }
        _ser.write_bare::<crate::ton::int256>(proxy_id)?;
        _ser.write_bare::<crate::ton::Flags>(&_flags)?;
        if let Some(inner) = ip {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = port {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = adnl_start_time {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = seqno {
            _ser.write_bare::<crate::ton::long>(inner)?;
        }
        if let Some(inner) = date {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        _ser.write_bare::<crate::ton::int256>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ProxyPacketHeader {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let proxy_id = _de.read_bare::<crate::ton::int256>()?;
            let flags = _de.read_bare::<crate::ton::Flags>()?;
            let ip = if flags & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let port = if flags & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let adnl_start_time = if flags & (1 << 1u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let seqno = if flags & (1 << 2u32) != 0 {
                Some(_de.read_bare::<crate::ton::long>()?)
            } else {
                None
            };
            let date = if flags & (1 << 3u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let signature = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { proxy_id, ip, port, adnl_start_time, seqno, date, signature })
        }
    }
}
impl crate::IntoBoxed for ProxyPacketHeader {
    type Boxed = crate::ton::adnl::ProxyPacketHeader;

    fn into_boxed(self) -> crate::ton::adnl::ProxyPacketHeader {
        crate::ton::adnl::ProxyPacketHeader::Adnl_ProxyPacketHeader(self)
    }
}
