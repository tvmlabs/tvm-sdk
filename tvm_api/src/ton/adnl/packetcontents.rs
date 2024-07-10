use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.packetContents`\n\n```text\nadnl.packetContents \n  rand1:bytes \n  flags:# \n  from:flags.0?PublicKey \n  from_short:flags.1?adnl.id.short\n  message:flags.2?adnl.Message \n  messages:flags.3?(vector adnl.Message)\n  address:flags.4?adnl.addressList \n  priority_address:flags.5?adnl.addressList\n  seqno:flags.6?long \n  confirm_seqno:flags.7?long \n  recv_addr_list_version:flags.8?int\n  recv_priority_addr_list_version:flags.9?int\n  reinit_date:flags.10?int \n  dst_reinit_date:flags.10?int\n  signature:flags.11?bytes \n  rand2:bytes \n        = adnl.PacketContents;\n```\n"]
pub struct PacketContents {
    pub rand1: crate::ton::bytes,
    pub from: Option<crate::ton::PublicKey>,
    pub from_short: Option<crate::ton::adnl::id::short::Short>,
    pub message: Option<crate::ton::adnl::Message>,
    pub messages: Option<crate::ton::vector<crate::ton::Boxed, crate::ton::adnl::Message>>,
    pub address: Option<crate::ton::adnl::addresslist::AddressList>,
    pub priority_address: Option<crate::ton::adnl::addresslist::AddressList>,
    pub seqno: Option<crate::ton::long>,
    pub confirm_seqno: Option<crate::ton::long>,
    pub recv_addr_list_version: Option<crate::ton::int>,
    pub recv_priority_addr_list_version: Option<crate::ton::int>,
    pub reinit_date: Option<crate::ton::int>,
    pub dst_reinit_date: Option<crate::ton::int>,
    pub signature: Option<crate::ton::bytes>,
    pub rand2: crate::ton::bytes,
}
impl Eq for PacketContents {}
impl crate::BareSerialize for PacketContents {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd142cd89)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PacketContents {
            rand1,
            from,
            from_short,
            message,
            messages,
            address,
            priority_address,
            seqno,
            confirm_seqno,
            recv_addr_list_version,
            recv_priority_addr_list_version,
            reinit_date,
            dst_reinit_date,
            signature,
            rand2,
        } = self;
        let mut _flags = 0u32;
        if from.is_some() {
            _flags |= 1 << 0u32;
        }
        if from_short.is_some() {
            _flags |= 1 << 1u32;
        }
        if message.is_some() {
            _flags |= 1 << 2u32;
        }
        if messages.is_some() {
            _flags |= 1 << 3u32;
        }
        if address.is_some() {
            _flags |= 1 << 4u32;
        }
        if priority_address.is_some() {
            _flags |= 1 << 5u32;
        }
        if seqno.is_some() {
            _flags |= 1 << 6u32;
        }
        if confirm_seqno.is_some() {
            _flags |= 1 << 7u32;
        }
        if recv_addr_list_version.is_some() {
            _flags |= 1 << 8u32;
        }
        if recv_priority_addr_list_version.is_some() {
            _flags |= 1 << 9u32;
        }
        if reinit_date.is_some() {
            _flags |= 1 << 10u32;
        }
        if dst_reinit_date.is_some() {
            _flags |= 1 << 10u32;
        }
        if signature.is_some() {
            _flags |= 1 << 11u32;
        }
        _ser.write_bare::<crate::ton::bytes>(rand1)?;
        _ser.write_bare::<crate::ton::Flags>(&_flags)?;
        if let Some(inner) = from {
            _ser.write_boxed::<crate::ton::PublicKey>(inner)?;
        }
        if let Some(inner) = from_short {
            _ser.write_bare::<crate::ton::adnl::id::short::Short>(inner)?;
        }
        if let Some(inner) = message {
            _ser.write_boxed::<crate::ton::adnl::Message>(inner)?;
        }
        if let Some(inner) = messages {
            _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::adnl::Message>>(
                inner,
            )?;
        }
        if let Some(inner) = address {
            _ser.write_bare::<crate::ton::adnl::addresslist::AddressList>(inner)?;
        }
        if let Some(inner) = priority_address {
            _ser.write_bare::<crate::ton::adnl::addresslist::AddressList>(inner)?;
        }
        if let Some(inner) = seqno {
            _ser.write_bare::<crate::ton::long>(inner)?;
        }
        if let Some(inner) = confirm_seqno {
            _ser.write_bare::<crate::ton::long>(inner)?;
        }
        if let Some(inner) = recv_addr_list_version {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = recv_priority_addr_list_version {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = reinit_date {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = dst_reinit_date {
            _ser.write_bare::<crate::ton::int>(inner)?;
        }
        if let Some(inner) = signature {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        _ser.write_bare::<crate::ton::bytes>(rand2)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PacketContents {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let rand1 = _de.read_bare::<crate::ton::bytes>()?;
            let flags = _de.read_bare::<crate::ton::Flags>()?;
            let from = if flags & (1 << 0u32) != 0 {
                Some(_de.read_boxed::<crate::ton::PublicKey>()?)
            } else {
                None
            };
            let from_short = if flags & (1 << 1u32) != 0 {
                Some(_de.read_bare::<crate::ton::adnl::id::short::Short>()?)
            } else {
                None
            };
            let message = if flags & (1 << 2u32) != 0 {
                Some(_de.read_boxed::<crate::ton::adnl::Message>()?)
            } else {
                None
            };
            let messages = if flags & (1 << 3u32) != 0 {
                Some (_de . read_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: adnl :: Message > > () ?)
            } else {
                None
            };
            let address = if flags & (1 << 4u32) != 0 {
                Some(_de.read_bare::<crate::ton::adnl::addresslist::AddressList>()?)
            } else {
                None
            };
            let priority_address = if flags & (1 << 5u32) != 0 {
                Some(_de.read_bare::<crate::ton::adnl::addresslist::AddressList>()?)
            } else {
                None
            };
            let seqno = if flags & (1 << 6u32) != 0 {
                Some(_de.read_bare::<crate::ton::long>()?)
            } else {
                None
            };
            let confirm_seqno = if flags & (1 << 7u32) != 0 {
                Some(_de.read_bare::<crate::ton::long>()?)
            } else {
                None
            };
            let recv_addr_list_version = if flags & (1 << 8u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let recv_priority_addr_list_version = if flags & (1 << 9u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let reinit_date = if flags & (1 << 10u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let dst_reinit_date = if flags & (1 << 10u32) != 0 {
                Some(_de.read_bare::<crate::ton::int>()?)
            } else {
                None
            };
            let signature = if flags & (1 << 11u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let rand2 = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self {
                rand1,
                from,
                from_short,
                message,
                messages,
                address,
                priority_address,
                seqno,
                confirm_seqno,
                recv_addr_list_version,
                recv_priority_addr_list_version,
                reinit_date,
                dst_reinit_date,
                signature,
                rand2,
            })
        }
    }
}
impl crate::IntoBoxed for PacketContents {
    type Boxed = crate::ton::adnl::PacketContents;
    fn into_boxed(self) -> crate::ton::adnl::PacketContents {
        crate::ton::adnl::PacketContents::Adnl_PacketContents(self)
    }
}
