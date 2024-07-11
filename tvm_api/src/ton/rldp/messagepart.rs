use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rldp.complete`\n\n```text\nrldp.complete transfer_id:int256 part:int = rldp.MessagePart;\n```\n"]
pub struct Complete {
    pub transfer_id: crate::ton::int256,
    pub part: crate::ton::int,
}
impl Eq for Complete {}
impl crate::BareSerialize for Complete {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbc0cb2bf)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Complete { transfer_id, part } = self;
        _ser.write_bare::<crate::ton::int256>(transfer_id)?;
        _ser.write_bare::<crate::ton::int>(part)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Complete {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let transfer_id = _de.read_bare::<crate::ton::int256>()?;
            let part = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { transfer_id, part })
        }
    }
}
impl crate::IntoBoxed for Complete {
    type Boxed = crate::ton::rldp::MessagePart;

    fn into_boxed(self) -> crate::ton::rldp::MessagePart {
        crate::ton::rldp::MessagePart::Rldp_Complete(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rldp.confirm`\n\n```text\nrldp.confirm transfer_id:int256 part:int seqno:int = rldp.MessagePart;\n```\n"]
pub struct Confirm {
    pub transfer_id: crate::ton::int256,
    pub part: crate::ton::int,
    pub seqno: crate::ton::int,
}
impl Eq for Confirm {}
impl crate::BareSerialize for Confirm {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf582dc58)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Confirm { transfer_id, part, seqno } = self;
        _ser.write_bare::<crate::ton::int256>(transfer_id)?;
        _ser.write_bare::<crate::ton::int>(part)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Confirm {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let transfer_id = _de.read_bare::<crate::ton::int256>()?;
            let part = _de.read_bare::<crate::ton::int>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { transfer_id, part, seqno })
        }
    }
}
impl crate::IntoBoxed for Confirm {
    type Boxed = crate::ton::rldp::MessagePart;

    fn into_boxed(self) -> crate::ton::rldp::MessagePart {
        crate::ton::rldp::MessagePart::Rldp_Confirm(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `rldp.messagePart`\n\n```text\nrldp.messagePart transfer_id:int256 fec_type:fec.Type part:int total_size:long seqno:int data:bytes = rldp.MessagePart;\n```\n"]
pub struct MessagePart {
    pub transfer_id: crate::ton::int256,
    pub fec_type: crate::ton::fec::Type,
    pub part: crate::ton::int,
    pub total_size: crate::ton::long,
    pub seqno: crate::ton::int,
    pub data: crate::ton::bytes,
}
impl Eq for MessagePart {}
impl crate::BareSerialize for MessagePart {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x185c22cc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePart { transfer_id, fec_type, part, total_size, seqno, data } = self;
        _ser.write_bare::<crate::ton::int256>(transfer_id)?;
        _ser.write_boxed::<crate::ton::fec::Type>(fec_type)?;
        _ser.write_bare::<crate::ton::int>(part)?;
        _ser.write_bare::<crate::ton::long>(total_size)?;
        _ser.write_bare::<crate::ton::int>(seqno)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePart {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let transfer_id = _de.read_bare::<crate::ton::int256>()?;
            let fec_type = _de.read_boxed::<crate::ton::fec::Type>()?;
            let part = _de.read_bare::<crate::ton::int>()?;
            let total_size = _de.read_bare::<crate::ton::long>()?;
            let seqno = _de.read_bare::<crate::ton::int>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { transfer_id, fec_type, part, total_size, seqno, data })
        }
    }
}
impl crate::IntoBoxed for MessagePart {
    type Boxed = crate::ton::rldp::MessagePart;

    fn into_boxed(self) -> crate::ton::rldp::MessagePart {
        crate::ton::rldp::MessagePart::Rldp_MessagePart(self)
    }
}
