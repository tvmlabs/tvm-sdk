use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.getBroadcast`\n\n```text\noverlay.getBroadcast hash:int256 = overlay.Broadcast;\n```\n"]
pub struct GetBroadcast {
    pub hash: crate::ton::int256,
}
impl Eq for GetBroadcast {}
impl crate::BareSerialize for GetBroadcast {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2d35f2a0)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBroadcast { hash } = self;
        _ser.write_bare::<crate::ton::int256>(hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBroadcast {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { hash })
        }
    }
}
impl crate::BoxedDeserialize for GetBroadcast {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2d35f2a0)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x2d35f2a0) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBroadcast {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x2d35f2a0), self)
    }
}
impl crate::Function for GetBroadcast {
    type Reply = crate::ton::overlay::Broadcast;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.getBroadcastList`\n\n```text\noverlay.getBroadcastList list:overlay.broadcastList = overlay.BroadcastList;\n```\n"]
pub struct GetBroadcastList {
    pub list: crate::ton::overlay::broadcastlist::BroadcastList,
}
impl Eq for GetBroadcastList {}
impl crate::BareSerialize for GetBroadcastList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x421c283a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBroadcastList { list } = self;
        _ser.write_bare::<crate::ton::overlay::broadcastlist::BroadcastList>(list)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBroadcastList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let list = _de.read_bare::<crate::ton::overlay::broadcastlist::BroadcastList>()?;
            Ok(Self { list })
        }
    }
}
impl crate::BoxedDeserialize for GetBroadcastList {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x421c283a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x421c283a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBroadcastList {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x421c283a), self)
    }
}
impl crate::Function for GetBroadcastList {
    type Reply = crate::ton::overlay::BroadcastList;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.getRandomPeers`\n\n```text\noverlay.getRandomPeers peers:overlay.nodes = overlay.Nodes;\n```\n"]
pub struct GetRandomPeers {
    pub peers: crate::ton::overlay::nodes::Nodes,
}
impl Eq for GetRandomPeers {}
impl crate::BareSerialize for GetRandomPeers {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x48ee64ab)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetRandomPeers { peers } = self;
        _ser.write_bare::<crate::ton::overlay::nodes::Nodes>(peers)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetRandomPeers {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let peers = _de.read_bare::<crate::ton::overlay::nodes::Nodes>()?;
            Ok(Self { peers })
        }
    }
}
impl crate::BoxedDeserialize for GetRandomPeers {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x48ee64ab)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x48ee64ab) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetRandomPeers {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x48ee64ab), self)
    }
}
impl crate::Function for GetRandomPeers {
    type Reply = crate::ton::overlay::Nodes;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.query`\n\n```text\noverlay.query overlay:int256 = True;\n```\n"]
pub struct Query {
    pub overlay: crate::ton::int256,
}
impl Eq for Query {}
impl crate::BareSerialize for Query {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xccfd8443)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Query { overlay } = self;
        _ser.write_bare::<crate::ton::int256>(overlay)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Query {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let overlay = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { overlay })
        }
    }
}
impl crate::BoxedDeserialize for Query {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xccfd8443)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xccfd8443) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Query {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xccfd8443), self)
    }
}
impl crate::Function for Query {
    type Reply = crate::ton::True;
}
