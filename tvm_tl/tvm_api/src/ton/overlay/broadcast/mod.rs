use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcast`\n\n```text\noverlay.broadcast src:PublicKey certificate:overlay.Certificate flags:int data:bytes date:int signature:bytes = overlay.Broadcast;\n```\n"]
pub struct Broadcast {
    pub src: crate::ton::PublicKey,
    pub certificate: crate::ton::overlay::Certificate,
    pub flags: crate::ton::int,
    pub data: crate::ton::bytes,
    pub date: crate::ton::int,
    pub signature: crate::ton::bytes,
}
impl Eq for Broadcast {}
impl crate::Signing for Broadcast {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Broadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb15a2b6b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Broadcast { src, certificate, flags, data, date, signature } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(src)?;
        _ser.write_boxed::<crate::ton::overlay::Certificate>(certificate)?;
        _ser.write_bare::<crate::ton::int>(flags)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::int>(date)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Broadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_boxed::<crate::ton::PublicKey>()?;
            let certificate = _de.read_boxed::<crate::ton::overlay::Certificate>()?;
            let flags = _de.read_bare::<crate::ton::int>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let date = _de.read_bare::<crate::ton::int>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { src, certificate, flags, data, date, signature })
        }
    }
}
impl crate::IntoBoxed for Broadcast {
    type Boxed = crate::ton::overlay::Broadcast;

    fn into_boxed(self) -> crate::ton::overlay::Broadcast {
        crate::ton::overlay::Broadcast::Overlay_Broadcast(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcastFec`\n\n```text\noverlay.broadcastFec src:PublicKey certificate:overlay.Certificate data_hash:int256 data_size:int flags:int\n          data:bytes seqno:int fec:fec.Type date:int signature:bytes = overlay.Broadcast;\n```\n"]
pub struct BroadcastFec {
    pub src: crate::ton::PublicKey,
    pub certificate: crate::ton::overlay::Certificate,
    pub data_hash: crate::ton::int256,
    pub data_size: crate::ton::int,
    pub flags: crate::ton::int,
    pub data: crate::ton::bytes,
    pub seqno: crate::ton::int,
    pub fec: crate::ton::fec::Type,
    pub date: crate::ton::int,
    pub signature: crate::ton::bytes,
}
impl Eq for BroadcastFec {}
impl crate::Signing for BroadcastFec {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for BroadcastFec {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbad7c36a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BroadcastFec {
            src,
            certificate,
            data_hash,
            data_size,
            flags,
            data,
            seqno,
            fec,
            date,
            signature,
        } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(src)?;
        _ser.write_boxed::<crate::ton::overlay::Certificate>(certificate)?;
        _ser.write_bare::<crate::ton::int256>(data_hash)?;
        _ser.write_bare::<crate::ton::int>(data_size)?;
        _ser.write_bare::<crate::ton::int>(flags)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_boxed::<crate::ton::fec::Type>(fec)?;
        _ser.write_bare::<crate::ton::int>(date)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BroadcastFec {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_boxed::<crate::ton::PublicKey>()?;
            let certificate = _de.read_boxed::<crate::ton::overlay::Certificate>()?;
            let data_hash = _de.read_bare::<crate::ton::int256>()?;
            let data_size = _de.read_bare::<crate::ton::int>()?;
            let flags = _de.read_bare::<crate::ton::int>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let fec = _de.read_boxed::<crate::ton::fec::Type>()?;
            let date = _de.read_bare::<crate::ton::int>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self {
                src,
                certificate,
                data_hash,
                data_size,
                flags,
                data,
                seqno,
                fec,
                date,
                signature,
            })
        }
    }
}
impl crate::IntoBoxed for BroadcastFec {
    type Boxed = crate::ton::overlay::Broadcast;

    fn into_boxed(self) -> crate::ton::overlay::Broadcast {
        crate::ton::overlay::Broadcast::Overlay_BroadcastFec(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcastFecShort`\n\n```text\noverlay.broadcastFecShort src:PublicKey certificate:overlay.Certificate broadcast_hash:int256 part_data_hash:int256 seqno:int signature:bytes = overlay.Broadcast;\n```\n"]
pub struct BroadcastFecShort {
    pub src: crate::ton::PublicKey,
    pub certificate: crate::ton::overlay::Certificate,
    pub broadcast_hash: crate::ton::int256,
    pub part_data_hash: crate::ton::int256,
    pub seqno: crate::ton::int,
    pub signature: crate::ton::bytes,
}
impl Eq for BroadcastFecShort {}
impl crate::Signing for BroadcastFecShort {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for BroadcastFecShort {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf1881342)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BroadcastFecShort {
            src,
            certificate,
            broadcast_hash,
            part_data_hash,
            seqno,
            signature,
        } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(src)?;
        _ser.write_boxed::<crate::ton::overlay::Certificate>(certificate)?;
        _ser.write_bare::<crate::ton::int256>(broadcast_hash)?;
        _ser.write_bare::<crate::ton::int256>(part_data_hash)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BroadcastFecShort {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let src = _de.read_boxed::<crate::ton::PublicKey>()?;
            let certificate = _de.read_boxed::<crate::ton::overlay::Certificate>()?;
            let broadcast_hash = _de.read_bare::<crate::ton::int256>()?;
            let part_data_hash = _de.read_bare::<crate::ton::int256>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { src, certificate, broadcast_hash, part_data_hash, seqno, signature })
        }
    }
}
impl crate::IntoBoxed for BroadcastFecShort {
    type Boxed = crate::ton::overlay::Broadcast;

    fn into_boxed(self) -> crate::ton::overlay::Broadcast {
        crate::ton::overlay::Broadcast::Overlay_BroadcastFecShort(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcast.Id`\n\n```text\noverlay.broadcast.id src:int256 data_hash:int256 flags:int = overlay.broadcast.Id;\n```\n"]
pub enum Id {
    Overlay_Broadcast_Id(crate::ton::overlay::broadcast::id::Id),
}
impl Id {
    pub fn data_hash(&self) -> &crate::ton::int256 {
        match self {
            Id::Overlay_Broadcast_Id(ref x) => &x.data_hash,
        }
    }

    pub fn flags(&self) -> &crate::ton::int {
        match self {
            Id::Overlay_Broadcast_Id(ref x) => &x.flags,
        }
    }

    pub fn src(&self) -> &crate::ton::int256 {
        match self {
            Id::Overlay_Broadcast_Id(ref x) => &x.src,
        }
    }

    pub fn only(self) -> crate::ton::overlay::broadcast::id::Id {
        match self {
            Id::Overlay_Broadcast_Id(x) => x,
        }
    }
}
impl Eq for Id {}
impl Default for Id {
    fn default() -> Self {
        Id::Overlay_Broadcast_Id(crate::ton::overlay::broadcast::id::Id::default())
    }
}
impl crate::BoxedSerialize for Id {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Id::Overlay_Broadcast_Id(x) => (crate::ConstructorNumber(0x51fd789a), x),
        }
    }
}
impl crate::BoxedDeserialize for Id {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x51fd789a)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x51fd789a) => Ok(Id::Overlay_Broadcast_Id(
                _de.read_bare::<crate::ton::overlay::broadcast::id::Id>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcast.ToSign`\n\n```text\noverlay.broadcast.toSign hash:int256 date:int = overlay.broadcast.ToSign;\n```\n"]
pub enum ToSign {
    Overlay_Broadcast_ToSign(crate::ton::overlay::broadcast::tosign::ToSign),
}
impl ToSign {
    pub fn date(&self) -> &crate::ton::int {
        match self {
            ToSign::Overlay_Broadcast_ToSign(ref x) => &x.date,
        }
    }

    pub fn hash(&self) -> &crate::ton::int256 {
        match self {
            ToSign::Overlay_Broadcast_ToSign(ref x) => &x.hash,
        }
    }

    pub fn only(self) -> crate::ton::overlay::broadcast::tosign::ToSign {
        match self {
            ToSign::Overlay_Broadcast_ToSign(x) => x,
        }
    }
}
impl Eq for ToSign {}
impl Default for ToSign {
    fn default() -> Self {
        ToSign::Overlay_Broadcast_ToSign(crate::ton::overlay::broadcast::tosign::ToSign::default())
    }
}
impl crate::BoxedSerialize for ToSign {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ToSign::Overlay_Broadcast_ToSign(x) => (crate::ConstructorNumber(0xfa374e7c), x),
        }
    }
}
impl crate::BoxedDeserialize for ToSign {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfa374e7c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xfa374e7c) => Ok(ToSign::Overlay_Broadcast_ToSign(
                _de.read_bare::<crate::ton::overlay::broadcast::tosign::ToSign>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.unicast`\n\n```text\noverlay.unicast data:bytes = overlay.Broadcast;\n```\n"]
pub struct Unicast {
    pub data: crate::ton::bytes,
}
impl Eq for Unicast {}
impl crate::BareSerialize for Unicast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x33534e24)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Unicast { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Unicast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for Unicast {
    type Boxed = crate::ton::overlay::Broadcast;

    fn into_boxed(self) -> crate::ton::overlay::Broadcast {
        crate::ton::overlay::Broadcast::Overlay_Unicast(self)
    }
}
pub mod id;
pub mod tosign;
