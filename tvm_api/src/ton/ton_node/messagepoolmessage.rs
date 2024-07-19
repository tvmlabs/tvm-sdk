use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolAck`\n\n```text\ntonNode.messagePoolAck transfer_tag:uint8 = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolAck {
    pub transfer_tag: crate::ton::uint8,
}
impl Eq for MessagePoolAck {}
impl crate::BareSerialize for MessagePoolAck {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x9cb4dad3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolAck { transfer_tag } = self;
        _ser.write_bare::<crate::ton::uint8>(transfer_tag)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolAck {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let transfer_tag = _de.read_bare::<crate::ton::uint8>()?;
            Ok(Self { transfer_tag })
        }
    }
}
impl crate::IntoBoxed for MessagePoolAck {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolAck(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolFetch`\n\n```text\ntonNode.messagePoolFetch digests:(vector int256) = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolFetch {
    pub digests: crate::ton::vector<crate::ton::int256>,
}
impl Eq for MessagePoolFetch {}
impl crate::BareSerialize for MessagePoolFetch {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x873d80c8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolFetch { digests } = self;
        (digests as &dyn crate::ton::VectoredBare<crate::ton::int256>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolFetch {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let digests = <Vec<crate::ton::int256> as crate::ton::VectoredBare<
                crate::ton::int256,
            >>::deserialize(_de)?;
            Ok(Self { digests })
        }
    }
}
impl crate::IntoBoxed for MessagePoolFetch {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolFetch(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolFetchPack`\n\n```text\ntonNode.messagePoolFetchPack digest:int256 = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolFetchPack {
    pub digest: crate::ton::int256,
}
impl Eq for MessagePoolFetchPack {}
impl crate::BareSerialize for MessagePoolFetchPack {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x3f4cc517)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolFetchPack { digest } = self;
        _ser.write_bare::<crate::ton::int256>(digest)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolFetchPack {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let digest = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { digest })
        }
    }
}
impl crate::IntoBoxed for MessagePoolFetchPack {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolFetchPack(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolFetchReply`\n\n```text\ntonNode.messagePoolFetchReply data:(vector bytes) = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolFetchReply {
    pub data: crate::ton::vector<crate::ton::bytes>,
}
impl Eq for MessagePoolFetchReply {}
impl crate::BareSerialize for MessagePoolFetchReply {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xface9e5f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolFetchReply { data } = self;
        (data as &dyn crate::ton::VectoredBare<crate::ton::bytes>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolFetchReply {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = < Vec < crate :: ton :: bytes > as crate :: ton :: VectoredBare < crate :: ton :: bytes >> :: deserialize (_de) ? ;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for MessagePoolFetchReply {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolFetchReply(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolPack`\n\n```text\ntonNode.messagePoolPack digest:int256 = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolPack {
    pub digest: crate::ton::int256,
}
impl Eq for MessagePoolPack {}
impl crate::BareSerialize for MessagePoolPack {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x26af7a02)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolPack { digest } = self;
        _ser.write_bare::<crate::ton::int256>(digest)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolPack {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let digest = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { digest })
        }
    }
}
impl crate::IntoBoxed for MessagePoolPack {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolPack(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolPackReply`\n\n```text\ntonNode.messagePoolPackReply digest:int256 = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolPackReply {
    pub digest: crate::ton::int256,
}
impl Eq for MessagePoolPackReply {}
impl crate::BareSerialize for MessagePoolPackReply {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbb622a24)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolPackReply { digest } = self;
        _ser.write_bare::<crate::ton::int256>(digest)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolPackReply {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let digest = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { digest })
        }
    }
}
impl crate::IntoBoxed for MessagePoolPackReply {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolPackReply(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolShare`\n\n```text\ntonNode.messagePoolShare digests:(vector int256) = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolShare {
    pub digests: crate::ton::vector<crate::ton::int256>,
}
impl Eq for MessagePoolShare {}
impl crate::BareSerialize for MessagePoolShare {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8148d92a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolShare { digests } = self;
        (digests as &dyn crate::ton::VectoredBare<crate::ton::int256>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolShare {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let digests = <Vec<crate::ton::int256> as crate::ton::VectoredBare<
                crate::ton::int256,
            >>::deserialize(_de)?;
            Ok(Self { digests })
        }
    }
}
impl crate::IntoBoxed for MessagePoolShare {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolShare(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolShareData`\n\n```text\ntonNode.messagePoolShareData flags:# info:flags.0?tonNode.packInfo data:(vector bytes) = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolShareData {
    pub info: Option<crate::ton::ton_node::packinfo::PackInfo>,
    pub data: crate::ton::vector<crate::ton::bytes>,
}
impl Eq for MessagePoolShareData {}
impl crate::BareSerialize for MessagePoolShareData {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0e87a560)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolShareData { info, data } = self;
        let mut _flags = 0u32;
        if info.is_some() {
            _flags |= 1 << 0u32;
        }
        _ser.write_bare::<crate::ton::Flags>(&_flags)?;
        if let Some(inner) = info {
            _ser.write_bare::<crate::ton::ton_node::packinfo::PackInfo>(inner)?;
        }
        (data as &dyn crate::ton::VectoredBare<crate::ton::bytes>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolShareData {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let flags = _de.read_bare::<crate::ton::Flags>()?;
            let info = if flags & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::ton_node::packinfo::PackInfo>()?)
            } else {
                None
            };
            let data = < Vec < crate :: ton :: bytes > as crate :: ton :: VectoredBare < crate :: ton :: bytes >> :: deserialize (_de) ? ;
            Ok(Self { info, data })
        }
    }
}
impl crate::IntoBoxed for MessagePoolShareData {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolShareData(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolSharePack`\n\n```text\ntonNode.messagePoolSharePack data:bytes = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolSharePack {
    pub data: crate::ton::bytes,
}
impl Eq for MessagePoolSharePack {}
impl crate::BareSerialize for MessagePoolSharePack {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x717fd1d9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolSharePack { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolSharePack {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::IntoBoxed for MessagePoolSharePack {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolSharePack(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.messagePoolVote`\n\n```text\ntonNode.messagePoolVote digests:(vector int256) = tonNode.MessagePoolMessage;\n```\n"]
pub struct MessagePoolVote {
    pub digests: crate::ton::vector<crate::ton::int256>,
}
impl Eq for MessagePoolVote {}
impl crate::BareSerialize for MessagePoolVote {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa76b40aa)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let MessagePoolVote { digests } = self;
        (digests as &dyn crate::ton::VectoredBare<crate::ton::int256>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for MessagePoolVote {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let digests = <Vec<crate::ton::int256> as crate::ton::VectoredBare<
                crate::ton::int256,
            >>::deserialize(_de)?;
            Ok(Self { digests })
        }
    }
}
impl crate::IntoBoxed for MessagePoolVote {
    type Boxed = crate::ton::ton_node::MessagePoolMessage;

    fn into_boxed(self) -> crate::ton::ton_node::MessagePoolMessage {
        crate::ton::ton_node::MessagePoolMessage::TonNode_MessagePoolVote(self)
    }
}
