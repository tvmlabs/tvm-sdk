use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.Broadcast`\n\n```text\noverlay.broadcast src:PublicKey certificate:overlay.Certificate flags:int data:bytes date:int signature:bytes = overlay.Broadcast;\n\noverlay.broadcastFec src:PublicKey certificate:overlay.Certificate data_hash:int256 data_size:int flags:int\n          data:bytes seqno:int fec:fec.Type date:int signature:bytes = overlay.Broadcast;\n\noverlay.broadcastFecShort src:PublicKey certificate:overlay.Certificate broadcast_hash:int256 part_data_hash:int256 seqno:int signature:bytes = overlay.Broadcast;\n\noverlay.broadcastNotFound = overlay.Broadcast;\n\noverlay.fec.completed hash:int256 = overlay.Broadcast;\n\noverlay.fec.received hash:int256 = overlay.Broadcast;\n\noverlay.unicast data:bytes = overlay.Broadcast;\n```\n"]
pub enum Broadcast {
    Overlay_Broadcast(crate::ton::overlay::broadcast::Broadcast),
    Overlay_BroadcastFec(crate::ton::overlay::broadcast::BroadcastFec),
    Overlay_BroadcastFecShort(crate::ton::overlay::broadcast::BroadcastFecShort),
    Overlay_BroadcastNotFound,
    Overlay_Fec_Completed(crate::ton::overlay::fec::broadcast::Completed),
    Overlay_Fec_Received(crate::ton::overlay::fec::broadcast::Received),
    Overlay_Unicast(crate::ton::overlay::broadcast::Unicast),
}
impl Broadcast {
    pub fn broadcast_hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Broadcast::Overlay_BroadcastFecShort(ref x) => Some(&x.broadcast_hash),
            _ => None,
        }
    }

    pub fn certificate(&self) -> Option<&crate::ton::overlay::Certificate> {
        match self {
            Broadcast::Overlay_Broadcast(ref x) => Some(&x.certificate),
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.certificate),
            Broadcast::Overlay_BroadcastFecShort(ref x) => Some(&x.certificate),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            Broadcast::Overlay_Broadcast(ref x) => Some(&x.data),
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.data),
            Broadcast::Overlay_Unicast(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn data_hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.data_hash),
            _ => None,
        }
    }

    pub fn data_size(&self) -> Option<&crate::ton::int> {
        match self {
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.data_size),
            _ => None,
        }
    }

    pub fn date(&self) -> Option<&crate::ton::int> {
        match self {
            Broadcast::Overlay_Broadcast(ref x) => Some(&x.date),
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.date),
            _ => None,
        }
    }

    pub fn fec(&self) -> Option<&crate::ton::fec::Type> {
        match self {
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.fec),
            _ => None,
        }
    }

    pub fn flags(&self) -> Option<&crate::ton::int> {
        match self {
            Broadcast::Overlay_Broadcast(ref x) => Some(&x.flags),
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.flags),
            _ => None,
        }
    }

    pub fn hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Broadcast::Overlay_Fec_Completed(ref x) => Some(&x.hash),
            Broadcast::Overlay_Fec_Received(ref x) => Some(&x.hash),
            _ => None,
        }
    }

    pub fn part_data_hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Broadcast::Overlay_BroadcastFecShort(ref x) => Some(&x.part_data_hash),
            _ => None,
        }
    }

    pub fn seqno(&self) -> Option<&crate::ton::int> {
        match self {
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.seqno),
            Broadcast::Overlay_BroadcastFecShort(ref x) => Some(&x.seqno),
            _ => None,
        }
    }

    pub fn signature(&self) -> Option<&crate::ton::bytes> {
        match self {
            Broadcast::Overlay_Broadcast(ref x) => Some(&x.signature),
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.signature),
            Broadcast::Overlay_BroadcastFecShort(ref x) => Some(&x.signature),
            _ => None,
        }
    }

    pub fn src(&self) -> Option<&crate::ton::PublicKey> {
        match self {
            Broadcast::Overlay_Broadcast(ref x) => Some(&x.src),
            Broadcast::Overlay_BroadcastFec(ref x) => Some(&x.src),
            Broadcast::Overlay_BroadcastFecShort(ref x) => Some(&x.src),
            _ => None,
        }
    }
}
impl Eq for Broadcast {}
impl Default for Broadcast {
    fn default() -> Self {
        Broadcast::Overlay_Broadcast(crate::ton::overlay::broadcast::Broadcast::default())
    }
}
impl crate::BoxedSerialize for Broadcast {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Broadcast::Overlay_Broadcast(x) => (crate::ConstructorNumber(0xb15a2b6b), x),
            Broadcast::Overlay_BroadcastFec(x) => (crate::ConstructorNumber(0xbad7c36a), x),
            Broadcast::Overlay_BroadcastFecShort(x) => (crate::ConstructorNumber(0xf1881342), x),
            Broadcast::Overlay_BroadcastNotFound => (crate::ConstructorNumber(0x95863624), &()),
            Broadcast::Overlay_Fec_Completed(x) => (crate::ConstructorNumber(0x09d76914), x),
            Broadcast::Overlay_Fec_Received(x) => (crate::ConstructorNumber(0xd55c14ec), x),
            Broadcast::Overlay_Unicast(x) => (crate::ConstructorNumber(0x33534e24), x),
        }
    }
}
impl crate::BoxedDeserialize for Broadcast {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xb15a2b6b),
            crate::ConstructorNumber(0xbad7c36a),
            crate::ConstructorNumber(0xf1881342),
            crate::ConstructorNumber(0x95863624),
            crate::ConstructorNumber(0x09d76914),
            crate::ConstructorNumber(0xd55c14ec),
            crate::ConstructorNumber(0x33534e24),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb15a2b6b) => Ok(Broadcast::Overlay_Broadcast(
                _de.read_bare::<crate::ton::overlay::broadcast::Broadcast>()?,
            )),
            crate::ConstructorNumber(0xbad7c36a) => Ok(Broadcast::Overlay_BroadcastFec(
                _de.read_bare::<crate::ton::overlay::broadcast::BroadcastFec>()?,
            )),
            crate::ConstructorNumber(0xf1881342) => Ok(Broadcast::Overlay_BroadcastFecShort(
                _de.read_bare::<crate::ton::overlay::broadcast::BroadcastFecShort>()?,
            )),
            crate::ConstructorNumber(0x95863624) => Ok(Broadcast::Overlay_BroadcastNotFound),
            crate::ConstructorNumber(0x09d76914) => Ok(Broadcast::Overlay_Fec_Completed(
                _de.read_bare::<crate::ton::overlay::fec::broadcast::Completed>()?,
            )),
            crate::ConstructorNumber(0xd55c14ec) => Ok(Broadcast::Overlay_Fec_Received(
                _de.read_bare::<crate::ton::overlay::fec::broadcast::Received>()?,
            )),
            crate::ConstructorNumber(0x33534e24) => Ok(Broadcast::Overlay_Unicast(
                _de.read_bare::<crate::ton::overlay::broadcast::Unicast>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.BroadcastList`\n\n```text\noverlay.broadcastList hashes:(vector int256) = overlay.BroadcastList;\n```\n"]
pub enum BroadcastList {
    Overlay_BroadcastList(crate::ton::overlay::broadcastlist::BroadcastList),
}
impl BroadcastList {
    pub fn hashes(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int256> {
        match self {
            BroadcastList::Overlay_BroadcastList(ref x) => &x.hashes,
        }
    }

    pub fn only(self) -> crate::ton::overlay::broadcastlist::BroadcastList {
        match self {
            BroadcastList::Overlay_BroadcastList(x) => x,
        }
    }
}
impl Eq for BroadcastList {}
impl Default for BroadcastList {
    fn default() -> Self {
        BroadcastList::Overlay_BroadcastList(
            crate::ton::overlay::broadcastlist::BroadcastList::default(),
        )
    }
}
impl crate::BoxedSerialize for BroadcastList {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            BroadcastList::Overlay_BroadcastList(x) => (crate::ConstructorNumber(0x18d1dedf), x),
        }
    }
}
impl crate::BoxedDeserialize for BroadcastList {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x18d1dedf)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x18d1dedf) => Ok(BroadcastList::Overlay_BroadcastList(
                _de.read_bare::<crate::ton::overlay::broadcastlist::BroadcastList>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.Certificate`\n\n```text\noverlay.certificate issued_by:PublicKey expire_at:int max_size:int signature:bytes = overlay.Certificate;\n\noverlay.emptyCertificate = overlay.Certificate;\n```\n"]
pub enum Certificate {
    Overlay_Certificate(crate::ton::overlay::certificate::Certificate),
    Overlay_EmptyCertificate,
}
impl Certificate {
    pub fn expire_at(&self) -> Option<&crate::ton::int> {
        match self {
            Certificate::Overlay_Certificate(ref x) => Some(&x.expire_at),
            _ => None,
        }
    }

    pub fn issued_by(&self) -> Option<&crate::ton::PublicKey> {
        match self {
            Certificate::Overlay_Certificate(ref x) => Some(&x.issued_by),
            _ => None,
        }
    }

    pub fn max_size(&self) -> Option<&crate::ton::int> {
        match self {
            Certificate::Overlay_Certificate(ref x) => Some(&x.max_size),
            _ => None,
        }
    }

    pub fn signature(&self) -> Option<&crate::ton::bytes> {
        match self {
            Certificate::Overlay_Certificate(ref x) => Some(&x.signature),
            _ => None,
        }
    }
}
impl Eq for Certificate {}
impl Default for Certificate {
    fn default() -> Self {
        Certificate::Overlay_Certificate(crate::ton::overlay::certificate::Certificate::default())
    }
}
impl crate::BoxedSerialize for Certificate {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Certificate::Overlay_Certificate(x) => (crate::ConstructorNumber(0xe09ed731), x),
            Certificate::Overlay_EmptyCertificate => (crate::ConstructorNumber(0x32dabccf), &()),
        }
    }
}
impl crate::BoxedDeserialize for Certificate {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe09ed731), crate::ConstructorNumber(0x32dabccf)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe09ed731) => Ok(Certificate::Overlay_Certificate(
                _de.read_bare::<crate::ton::overlay::certificate::Certificate>()?,
            )),
            crate::ConstructorNumber(0x32dabccf) => Ok(Certificate::Overlay_EmptyCertificate),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::overlay::certificate::Certificate> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x32dabccf), &()),
            Some(ref x) => (crate::ConstructorNumber(0xe09ed731), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::overlay::certificate::Certificate> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x32dabccf), crate::ConstructorNumber(0xe09ed731)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x32dabccf) => Ok(None),
            crate::ConstructorNumber(0xe09ed731) => {
                Ok(Some(_de.read_bare::<crate::ton::overlay::certificate::Certificate>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.CertificateId`\n\n```text\noverlay.certificateId overlay_id:int256 node:int256 expire_at:int max_size:int = overlay.CertificateId;\n```\n"]
pub enum CertificateId {
    Overlay_CertificateId(crate::ton::overlay::certificateid::CertificateId),
}
impl CertificateId {
    pub fn expire_at(&self) -> &crate::ton::int {
        match self {
            CertificateId::Overlay_CertificateId(ref x) => &x.expire_at,
        }
    }

    pub fn max_size(&self) -> &crate::ton::int {
        match self {
            CertificateId::Overlay_CertificateId(ref x) => &x.max_size,
        }
    }

    pub fn node(&self) -> &crate::ton::int256 {
        match self {
            CertificateId::Overlay_CertificateId(ref x) => &x.node,
        }
    }

    pub fn overlay_id(&self) -> &crate::ton::int256 {
        match self {
            CertificateId::Overlay_CertificateId(ref x) => &x.overlay_id,
        }
    }

    pub fn only(self) -> crate::ton::overlay::certificateid::CertificateId {
        match self {
            CertificateId::Overlay_CertificateId(x) => x,
        }
    }
}
impl Eq for CertificateId {}
impl Default for CertificateId {
    fn default() -> Self {
        CertificateId::Overlay_CertificateId(
            crate::ton::overlay::certificateid::CertificateId::default(),
        )
    }
}
impl crate::BoxedSerialize for CertificateId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            CertificateId::Overlay_CertificateId(x) => (crate::ConstructorNumber(0x8fae60b9), x),
        }
    }
}
impl crate::BoxedDeserialize for CertificateId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8fae60b9)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8fae60b9) => Ok(CertificateId::Overlay_CertificateId(
                _de.read_bare::<crate::ton::overlay::certificateid::CertificateId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.Message`\n\n```text\noverlay.message overlay:int256 = overlay.Message;\n```\n"]
pub enum Message {
    Overlay_Message(crate::ton::overlay::message::Message),
}
impl Message {
    pub fn overlay(&self) -> &crate::ton::int256 {
        match self {
            Message::Overlay_Message(ref x) => &x.overlay,
        }
    }

    pub fn only(self) -> crate::ton::overlay::message::Message {
        match self {
            Message::Overlay_Message(x) => x,
        }
    }
}
impl Eq for Message {}
impl Default for Message {
    fn default() -> Self {
        Message::Overlay_Message(crate::ton::overlay::message::Message::default())
    }
}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::Overlay_Message(x) => (crate::ConstructorNumber(0x75252420), x),
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x75252420)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x75252420) => Ok(Message::Overlay_Message(
                _de.read_bare::<crate::ton::overlay::message::Message>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.Node`\n\n```text\noverlay.node id:PublicKey overlay:int256 version:int signature:bytes = overlay.Node;\n```\n"]
pub enum Node {
    Overlay_Node(crate::ton::overlay::node::Node),
}
impl Node {
    pub fn id(&self) -> &crate::ton::PublicKey {
        match self {
            Node::Overlay_Node(ref x) => &x.id,
        }
    }

    pub fn overlay(&self) -> &crate::ton::int256 {
        match self {
            Node::Overlay_Node(ref x) => &x.overlay,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            Node::Overlay_Node(ref x) => &x.signature,
        }
    }

    pub fn version(&self) -> &crate::ton::int {
        match self {
            Node::Overlay_Node(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::overlay::node::Node {
        match self {
            Node::Overlay_Node(x) => x,
        }
    }
}
impl Eq for Node {}
impl Default for Node {
    fn default() -> Self {
        Node::Overlay_Node(crate::ton::overlay::node::Node::default())
    }
}
impl crate::BoxedSerialize for Node {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Node::Overlay_Node(x) => (crate::ConstructorNumber(0xb86b8a83), x),
        }
    }
}
impl crate::BoxedDeserialize for Node {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb86b8a83)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb86b8a83) => {
                Ok(Node::Overlay_Node(_de.read_bare::<crate::ton::overlay::node::Node>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.Nodes`\n\n```text\noverlay.nodes nodes:(vector overlay.node) = overlay.Nodes;\n```\n"]
pub enum Nodes {
    Overlay_Nodes(crate::ton::overlay::nodes::Nodes),
}
impl Nodes {
    pub fn nodes(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::overlay::node::Node> {
        match self {
            Nodes::Overlay_Nodes(ref x) => &x.nodes,
        }
    }

    pub fn only(self) -> crate::ton::overlay::nodes::Nodes {
        match self {
            Nodes::Overlay_Nodes(x) => x,
        }
    }
}
impl Eq for Nodes {}
impl Default for Nodes {
    fn default() -> Self {
        Nodes::Overlay_Nodes(crate::ton::overlay::nodes::Nodes::default())
    }
}
impl crate::BoxedSerialize for Nodes {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Nodes::Overlay_Nodes(x) => (crate::ConstructorNumber(0xe487290e), x),
        }
    }
}
impl crate::BoxedDeserialize for Nodes {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe487290e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe487290e) => {
                Ok(Nodes::Overlay_Nodes(_de.read_bare::<crate::ton::overlay::nodes::Nodes>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod broadcast;
pub mod broadcast_fec;
pub mod broadcastlist;
pub mod certificate;
pub mod certificateid;
pub mod db;
pub mod fec;
pub mod message;
pub mod node;
pub mod nodes;
