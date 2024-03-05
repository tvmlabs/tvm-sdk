use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.Address`\n\n```text\nadnl.address.tunnel to:int256 pubkey:PublicKey = adnl.Address;\n\nadnl.address.udp ip:int port:int = adnl.Address;\n\nadnl.address.udp6 ip:int128 port:int = adnl.Address;\n```\n"]
pub enum Address {
    Adnl_Address_Tunnel(crate::ton::adnl::address::address::Tunnel),
    Adnl_Address_Udp(crate::ton::adnl::address::address::Udp),
    Adnl_Address_Udp6(crate::ton::adnl::address::address::Udp6),
}
impl Address {
    pub fn port(&self) -> Option<&crate::ton::int> {
        match self {
            Address::Adnl_Address_Udp(ref x) => Some(&x.port),
            Address::Adnl_Address_Udp6(ref x) => Some(&x.port),
            _ => None,
        }
    }

    pub fn pubkey(&self) -> Option<&crate::ton::PublicKey> {
        match self {
            Address::Adnl_Address_Tunnel(ref x) => Some(&x.pubkey),
            _ => None,
        }
    }

    pub fn to(&self) -> Option<&crate::ton::int256> {
        match self {
            Address::Adnl_Address_Tunnel(ref x) => Some(&x.to),
            _ => None,
        }
    }
}
impl Eq for Address {}
impl Default for Address {
    fn default() -> Self {
        Address::Adnl_Address_Tunnel(crate::ton::adnl::address::address::Tunnel::default())
    }
}
impl crate::BoxedSerialize for Address {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Address::Adnl_Address_Tunnel(x) => (crate::ConstructorNumber(0x092b02eb), x),
            Address::Adnl_Address_Udp(x) => (crate::ConstructorNumber(0x670da6e7), x),
            Address::Adnl_Address_Udp6(x) => (crate::ConstructorNumber(0xe31d63fa), x),
        }
    }
}
impl crate::BoxedDeserialize for Address {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x092b02eb),
            crate::ConstructorNumber(0x670da6e7),
            crate::ConstructorNumber(0xe31d63fa),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x092b02eb) => Ok(Address::Adnl_Address_Tunnel(
                _de.read_bare::<crate::ton::adnl::address::address::Tunnel>()?,
            )),
            crate::ConstructorNumber(0x670da6e7) => Ok(Address::Adnl_Address_Udp(
                _de.read_bare::<crate::ton::adnl::address::address::Udp>()?,
            )),
            crate::ConstructorNumber(0xe31d63fa) => Ok(Address::Adnl_Address_Udp6(
                _de.read_bare::<crate::ton::adnl::address::address::Udp6>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.AddressList`\n\n```text\nadnl.addressList addrs:(vector adnl.Address) version:int reinit_date:int priority:int expire_at:int = adnl.AddressList;\n```\n"]
pub enum AddressList {
    Adnl_AddressList(crate::ton::adnl::addresslist::AddressList),
}
impl AddressList {
    pub fn addrs(&self) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::adnl::Address> {
        match self {
            AddressList::Adnl_AddressList(ref x) => &x.addrs,
        }
    }

    pub fn expire_at(&self) -> &crate::ton::int {
        match self {
            AddressList::Adnl_AddressList(ref x) => &x.expire_at,
        }
    }

    pub fn priority(&self) -> &crate::ton::int {
        match self {
            AddressList::Adnl_AddressList(ref x) => &x.priority,
        }
    }

    pub fn reinit_date(&self) -> &crate::ton::int {
        match self {
            AddressList::Adnl_AddressList(ref x) => &x.reinit_date,
        }
    }

    pub fn version(&self) -> &crate::ton::int {
        match self {
            AddressList::Adnl_AddressList(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::adnl::addresslist::AddressList {
        match self {
            AddressList::Adnl_AddressList(x) => x,
        }
    }
}
impl Eq for AddressList {}
impl Default for AddressList {
    fn default() -> Self {
        AddressList::Adnl_AddressList(crate::ton::adnl::addresslist::AddressList::default())
    }
}
impl crate::BoxedSerialize for AddressList {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AddressList::Adnl_AddressList(x) => (crate::ConstructorNumber(0x2227e658), x),
        }
    }
}
impl crate::BoxedDeserialize for AddressList {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2227e658)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x2227e658) => Ok(AddressList::Adnl_AddressList(
                _de.read_bare::<crate::ton::adnl::addresslist::AddressList>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.Message`\n\n```text\nadnl.message.answer query_id:int256 answer:bytes = adnl.Message;\n\nadnl.message.confirmChannel key:int256 peer_key:int256 date:int = adnl.Message;\n\nadnl.message.createChannel key:int256 date:int = adnl.Message;\n\nadnl.message.custom data:bytes = adnl.Message;\n\nadnl.message.nop = adnl.Message;\n\nadnl.message.part hash:int256 total_size:int offset:int data:bytes = adnl.Message;\n\nadnl.message.query query_id:int256 query:bytes = adnl.Message;\n\nadnl.message.reinit date:int = adnl.Message;\n```\n"]
pub enum Message {
    Adnl_Message_Answer(crate::ton::adnl::message::message::Answer),
    Adnl_Message_ConfirmChannel(crate::ton::adnl::message::message::ConfirmChannel),
    Adnl_Message_CreateChannel(crate::ton::adnl::message::message::CreateChannel),
    Adnl_Message_Custom(crate::ton::adnl::message::message::Custom),
    Adnl_Message_Nop,
    Adnl_Message_Part(crate::ton::adnl::message::message::Part),
    Adnl_Message_Query(crate::ton::adnl::message::message::Query),
    Adnl_Message_Reinit(crate::ton::adnl::message::message::Reinit),
}
impl Message {
    pub fn answer(&self) -> Option<&crate::ton::bytes> {
        match self {
            Message::Adnl_Message_Answer(ref x) => Some(&x.answer),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            Message::Adnl_Message_Custom(ref x) => Some(&x.data),
            Message::Adnl_Message_Part(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn date(&self) -> Option<&crate::ton::int> {
        match self {
            Message::Adnl_Message_ConfirmChannel(ref x) => Some(&x.date),
            Message::Adnl_Message_CreateChannel(ref x) => Some(&x.date),
            Message::Adnl_Message_Reinit(ref x) => Some(&x.date),
            _ => None,
        }
    }

    pub fn hash(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::Adnl_Message_Part(ref x) => Some(&x.hash),
            _ => None,
        }
    }

    pub fn key(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::Adnl_Message_ConfirmChannel(ref x) => Some(&x.key),
            Message::Adnl_Message_CreateChannel(ref x) => Some(&x.key),
            _ => None,
        }
    }

    pub fn offset(&self) -> Option<&crate::ton::int> {
        match self {
            Message::Adnl_Message_Part(ref x) => Some(&x.offset),
            _ => None,
        }
    }

    pub fn peer_key(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::Adnl_Message_ConfirmChannel(ref x) => Some(&x.peer_key),
            _ => None,
        }
    }

    pub fn query(&self) -> Option<&crate::ton::bytes> {
        match self {
            Message::Adnl_Message_Query(ref x) => Some(&x.query),
            _ => None,
        }
    }

    pub fn query_id(&self) -> Option<&crate::ton::int256> {
        match self {
            Message::Adnl_Message_Answer(ref x) => Some(&x.query_id),
            Message::Adnl_Message_Query(ref x) => Some(&x.query_id),
            _ => None,
        }
    }

    pub fn total_size(&self) -> Option<&crate::ton::int> {
        match self {
            Message::Adnl_Message_Part(ref x) => Some(&x.total_size),
            _ => None,
        }
    }
}
impl Eq for Message {}
impl Default for Message {
    fn default() -> Self {
        Message::Adnl_Message_Answer(crate::ton::adnl::message::message::Answer::default())
    }
}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::Adnl_Message_Answer(x) => (crate::ConstructorNumber(0x0fac8416), x),
            Message::Adnl_Message_ConfirmChannel(x) => (crate::ConstructorNumber(0x60dd1d69), x),
            Message::Adnl_Message_CreateChannel(x) => (crate::ConstructorNumber(0xe673c3bb), x),
            Message::Adnl_Message_Custom(x) => (crate::ConstructorNumber(0x204818f5), x),
            Message::Adnl_Message_Nop => (crate::ConstructorNumber(0x17f8dfda), &()),
            Message::Adnl_Message_Part(x) => (crate::ConstructorNumber(0xfd452d39), x),
            Message::Adnl_Message_Query(x) => (crate::ConstructorNumber(0xb48bf97a), x),
            Message::Adnl_Message_Reinit(x) => (crate::ConstructorNumber(0x10c20520), x),
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x0fac8416),
            crate::ConstructorNumber(0x60dd1d69),
            crate::ConstructorNumber(0xe673c3bb),
            crate::ConstructorNumber(0x204818f5),
            crate::ConstructorNumber(0x17f8dfda),
            crate::ConstructorNumber(0xfd452d39),
            crate::ConstructorNumber(0xb48bf97a),
            crate::ConstructorNumber(0x10c20520),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0fac8416) => Ok(Message::Adnl_Message_Answer(
                _de.read_bare::<crate::ton::adnl::message::message::Answer>()?,
            )),
            crate::ConstructorNumber(0x60dd1d69) => Ok(Message::Adnl_Message_ConfirmChannel(
                _de.read_bare::<crate::ton::adnl::message::message::ConfirmChannel>()?,
            )),
            crate::ConstructorNumber(0xe673c3bb) => Ok(Message::Adnl_Message_CreateChannel(
                _de.read_bare::<crate::ton::adnl::message::message::CreateChannel>()?,
            )),
            crate::ConstructorNumber(0x204818f5) => Ok(Message::Adnl_Message_Custom(
                _de.read_bare::<crate::ton::adnl::message::message::Custom>()?,
            )),
            crate::ConstructorNumber(0x17f8dfda) => Ok(Message::Adnl_Message_Nop),
            crate::ConstructorNumber(0xfd452d39) => Ok(Message::Adnl_Message_Part(
                _de.read_bare::<crate::ton::adnl::message::message::Part>()?,
            )),
            crate::ConstructorNumber(0xb48bf97a) => Ok(Message::Adnl_Message_Query(
                _de.read_bare::<crate::ton::adnl::message::message::Query>()?,
            )),
            crate::ConstructorNumber(0x10c20520) => Ok(Message::Adnl_Message_Reinit(
                _de.read_bare::<crate::ton::adnl::message::message::Reinit>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.Node`\n\n```text\nadnl.node id:PublicKey addr_list:adnl.addressList = adnl.Node;\n```\n"]
pub enum Node {
    Adnl_Node(crate::ton::adnl::node::Node),
}
impl Node {
    pub fn addr_list(&self) -> &crate::ton::adnl::addresslist::AddressList {
        match self {
            Node::Adnl_Node(ref x) => &x.addr_list,
        }
    }

    pub fn id(&self) -> &crate::ton::PublicKey {
        match self {
            Node::Adnl_Node(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::adnl::node::Node {
        match self {
            Node::Adnl_Node(x) => x,
        }
    }
}
impl Eq for Node {}
impl Default for Node {
    fn default() -> Self {
        Node::Adnl_Node(crate::ton::adnl::node::Node::default())
    }
}
impl crate::BoxedSerialize for Node {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Node::Adnl_Node(x) => (crate::ConstructorNumber(0x6b561285), x),
        }
    }
}
impl crate::BoxedDeserialize for Node {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6b561285)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x6b561285) => {
                Ok(Node::Adnl_Node(_de.read_bare::<crate::ton::adnl::node::Node>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.Nodes`\n\n```text\nadnl.nodes nodes:(vector adnl.node) = adnl.Nodes;\n```\n"]
pub enum Nodes {
    Adnl_Nodes(crate::ton::adnl::nodes::Nodes),
}
impl Nodes {
    pub fn nodes(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::adnl::node::Node> {
        match self {
            Nodes::Adnl_Nodes(ref x) => &x.nodes,
        }
    }

    pub fn only(self) -> crate::ton::adnl::nodes::Nodes {
        match self {
            Nodes::Adnl_Nodes(x) => x,
        }
    }
}
impl Eq for Nodes {}
impl Default for Nodes {
    fn default() -> Self {
        Nodes::Adnl_Nodes(crate::ton::adnl::nodes::Nodes::default())
    }
}
impl crate::BoxedSerialize for Nodes {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Nodes::Adnl_Nodes(x) => (crate::ConstructorNumber(0xa209db56), x),
        }
    }
}
impl crate::BoxedDeserialize for Nodes {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa209db56)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa209db56) => {
                Ok(Nodes::Adnl_Nodes(_de.read_bare::<crate::ton::adnl::nodes::Nodes>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.PacketContents`\n\n```text\nadnl.packetContents \n  rand1:bytes \n  flags:# \n  from:flags.0?PublicKey \n  from_short:flags.1?adnl.id.short\n  message:flags.2?adnl.Message \n  messages:flags.3?(vector adnl.Message)\n  address:flags.4?adnl.addressList \n  priority_address:flags.5?adnl.addressList\n  seqno:flags.6?long \n  confirm_seqno:flags.7?long \n  recv_addr_list_version:flags.8?int\n  recv_priority_addr_list_version:flags.9?int\n  reinit_date:flags.10?int \n  dst_reinit_date:flags.10?int\n  signature:flags.11?bytes \n  rand2:bytes \n        = adnl.PacketContents;\n```\n"]
pub enum PacketContents {
    Adnl_PacketContents(crate::ton::adnl::packetcontents::PacketContents),
}
impl PacketContents {
    pub fn address(&self) -> Option<&crate::ton::adnl::addresslist::AddressList> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.address.as_ref(),
        }
    }

    pub fn confirm_seqno(&self) -> Option<&crate::ton::long> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.confirm_seqno.as_ref(),
        }
    }

    pub fn dst_reinit_date(&self) -> Option<&crate::ton::int> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.dst_reinit_date.as_ref(),
        }
    }

    pub fn from(&self) -> Option<&crate::ton::PublicKey> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.from.as_ref(),
        }
    }

    pub fn from_short(&self) -> Option<&crate::ton::adnl::id::short::Short> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.from_short.as_ref(),
        }
    }

    pub fn message(&self) -> Option<&crate::ton::adnl::Message> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.message.as_ref(),
        }
    }

    pub fn messages(
        &self,
    ) -> Option<&crate::ton::vector<crate::ton::Boxed, crate::ton::adnl::Message>> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.messages.as_ref(),
        }
    }

    pub fn priority_address(&self) -> Option<&crate::ton::adnl::addresslist::AddressList> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.priority_address.as_ref(),
        }
    }

    pub fn rand1(&self) -> &crate::ton::bytes {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => &x.rand1,
        }
    }

    pub fn rand2(&self) -> &crate::ton::bytes {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => &x.rand2,
        }
    }

    pub fn recv_addr_list_version(&self) -> Option<&crate::ton::int> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.recv_addr_list_version.as_ref(),
        }
    }

    pub fn recv_priority_addr_list_version(&self) -> Option<&crate::ton::int> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => {
                x.recv_priority_addr_list_version.as_ref()
            }
        }
    }

    pub fn reinit_date(&self) -> Option<&crate::ton::int> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.reinit_date.as_ref(),
        }
    }

    pub fn seqno(&self) -> Option<&crate::ton::long> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.seqno.as_ref(),
        }
    }

    pub fn signature(&self) -> Option<&crate::ton::bytes> {
        match self {
            PacketContents::Adnl_PacketContents(ref x) => x.signature.as_ref(),
        }
    }

    pub fn only(self) -> crate::ton::adnl::packetcontents::PacketContents {
        match self {
            PacketContents::Adnl_PacketContents(x) => x,
        }
    }
}
impl Eq for PacketContents {}
impl Default for PacketContents {
    fn default() -> Self {
        PacketContents::Adnl_PacketContents(
            crate::ton::adnl::packetcontents::PacketContents::default(),
        )
    }
}
impl crate::BoxedSerialize for PacketContents {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            PacketContents::Adnl_PacketContents(x) => (crate::ConstructorNumber(0xd142cd89), x),
        }
    }
}
impl crate::BoxedDeserialize for PacketContents {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd142cd89)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd142cd89) => Ok(PacketContents::Adnl_PacketContents(
                _de.read_bare::<crate::ton::adnl::packetcontents::PacketContents>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.Pong`\n\n```text\nadnl.pong value:long = adnl.Pong;\n```\n"]
pub enum Pong {
    Adnl_Pong(crate::ton::adnl::pong::Pong),
}
impl Pong {
    pub fn value(&self) -> &crate::ton::long {
        match self {
            Pong::Adnl_Pong(ref x) => &x.value,
        }
    }

    pub fn only(self) -> crate::ton::adnl::pong::Pong {
        match self {
            Pong::Adnl_Pong(x) => x,
        }
    }
}
impl Eq for Pong {}
impl Default for Pong {
    fn default() -> Self {
        Pong::Adnl_Pong(crate::ton::adnl::pong::Pong::default())
    }
}
impl crate::BoxedSerialize for Pong {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Pong::Adnl_Pong(x) => (crate::ConstructorNumber(0x20747c0e), x),
        }
    }
}
impl crate::BoxedDeserialize for Pong {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x20747c0e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x20747c0e) => {
                Ok(Pong::Adnl_Pong(_de.read_bare::<crate::ton::adnl::pong::Pong>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.Proxy`\n\n```text\nadnl.proxy.fast id:int256 shared_secret:bytes = adnl.Proxy;\n\nadnl.proxy.none id:int256 = adnl.Proxy;\n```\n"]
pub enum Proxy {
    Adnl_Proxy_Fast(crate::ton::adnl::proxy::proxy::Fast),
    Adnl_Proxy_None(crate::ton::adnl::proxy::proxy::None),
}
impl Proxy {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            Proxy::Adnl_Proxy_Fast(ref x) => &x.id,
            Proxy::Adnl_Proxy_None(ref x) => &x.id,
        }
    }

    pub fn shared_secret(&self) -> Option<&crate::ton::bytes> {
        match self {
            Proxy::Adnl_Proxy_Fast(ref x) => Some(&x.shared_secret),
            _ => None,
        }
    }
}
impl Eq for Proxy {}
impl Default for Proxy {
    fn default() -> Self {
        Proxy::Adnl_Proxy_Fast(crate::ton::adnl::proxy::proxy::Fast::default())
    }
}
impl crate::BoxedSerialize for Proxy {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Proxy::Adnl_Proxy_Fast(x) => (crate::ConstructorNumber(0x3a8b45b5), x),
            Proxy::Adnl_Proxy_None(x) => (crate::ConstructorNumber(0x3532487b), x),
        }
    }
}
impl crate::BoxedDeserialize for Proxy {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3a8b45b5), crate::ConstructorNumber(0x3532487b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x3a8b45b5) => {
                Ok(Proxy::Adnl_Proxy_Fast(_de.read_bare::<crate::ton::adnl::proxy::proxy::Fast>()?))
            }
            crate::ConstructorNumber(0x3532487b) => {
                Ok(Proxy::Adnl_Proxy_None(_de.read_bare::<crate::ton::adnl::proxy::proxy::None>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.ProxyControlPacket`\n\n```text\nadnl.proxyControlPacketPing id:int256 = adnl.ProxyControlPacket;\n\nadnl.proxyControlPacketPong id:int256 = adnl.ProxyControlPacket;\n\nadnl.proxyControlPacketRegister ip:int port:int = adnl.ProxyControlPacket;\n```\n"]
pub enum ProxyControlPacket {
    Adnl_ProxyControlPacketPing(crate::ton::adnl::proxycontrolpacket::ProxyControlPacketPing),
    Adnl_ProxyControlPacketPong(crate::ton::adnl::proxycontrolpacket::ProxyControlPacketPong),
    Adnl_ProxyControlPacketRegister(
        crate::ton::adnl::proxycontrolpacket::ProxyControlPacketRegister,
    ),
}
impl ProxyControlPacket {
    pub fn id(&self) -> Option<&crate::ton::int256> {
        match self {
            ProxyControlPacket::Adnl_ProxyControlPacketPing(ref x) => Some(&x.id),
            ProxyControlPacket::Adnl_ProxyControlPacketPong(ref x) => Some(&x.id),
            _ => None,
        }
    }

    pub fn ip(&self) -> Option<&crate::ton::int> {
        match self {
            ProxyControlPacket::Adnl_ProxyControlPacketRegister(ref x) => Some(&x.ip),
            _ => None,
        }
    }

    pub fn port(&self) -> Option<&crate::ton::int> {
        match self {
            ProxyControlPacket::Adnl_ProxyControlPacketRegister(ref x) => Some(&x.port),
            _ => None,
        }
    }
}
impl Eq for ProxyControlPacket {}
impl Default for ProxyControlPacket {
    fn default() -> Self {
        ProxyControlPacket::Adnl_ProxyControlPacketPing(
            crate::ton::adnl::proxycontrolpacket::ProxyControlPacketPing::default(),
        )
    }
}
impl crate::BoxedSerialize for ProxyControlPacket {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ProxyControlPacket::Adnl_ProxyControlPacketPing(x) => {
                (crate::ConstructorNumber(0x3796e44b), x)
            }
            ProxyControlPacket::Adnl_ProxyControlPacketPong(x) => {
                (crate::ConstructorNumber(0x4bd1dbfc), x)
            }
            ProxyControlPacket::Adnl_ProxyControlPacketRegister(x) => {
                (crate::ConstructorNumber(0xc309b23f), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ProxyControlPacket {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x3796e44b),
            crate::ConstructorNumber(0x4bd1dbfc),
            crate::ConstructorNumber(0xc309b23f),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x3796e44b) => Ok (ProxyControlPacket :: Adnl_ProxyControlPacketPing (_de . read_bare :: < crate :: ton :: adnl :: proxycontrolpacket :: ProxyControlPacketPing > () ?)) , crate :: ConstructorNumber (0x4bd1dbfc) => Ok (ProxyControlPacket :: Adnl_ProxyControlPacketPong (_de . read_bare :: < crate :: ton :: adnl :: proxycontrolpacket :: ProxyControlPacketPong > () ?)) , crate :: ConstructorNumber (0xc309b23f) => Ok (ProxyControlPacket :: Adnl_ProxyControlPacketRegister (_de . read_bare :: < crate :: ton :: adnl :: proxycontrolpacket :: ProxyControlPacketRegister > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.ProxyPacketHeader`\n\n```text\nadnl.proxyPacketHeader\n  proxy_id:int256\n  flags:# \n  ip:flags.0?int\n  port:flags.0?int\n  adnl_start_time:flags.1?int\n  seqno:flags.2?long\n  date:flags.3?int\n  signature:int256 = adnl.ProxyPacketHeader;\n```\n"]
pub enum ProxyPacketHeader {
    Adnl_ProxyPacketHeader(crate::ton::adnl::proxypacketheader::ProxyPacketHeader),
}
impl ProxyPacketHeader {
    pub fn adnl_start_time(&self) -> Option<&crate::ton::int> {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(ref x) => x.adnl_start_time.as_ref(),
        }
    }

    pub fn date(&self) -> Option<&crate::ton::int> {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(ref x) => x.date.as_ref(),
        }
    }

    pub fn ip(&self) -> Option<&crate::ton::int> {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(ref x) => x.ip.as_ref(),
        }
    }

    pub fn port(&self) -> Option<&crate::ton::int> {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(ref x) => x.port.as_ref(),
        }
    }

    pub fn proxy_id(&self) -> &crate::ton::int256 {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(ref x) => &x.proxy_id,
        }
    }

    pub fn seqno(&self) -> Option<&crate::ton::long> {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(ref x) => x.seqno.as_ref(),
        }
    }

    pub fn signature(&self) -> &crate::ton::int256 {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(ref x) => &x.signature,
        }
    }

    pub fn only(self) -> crate::ton::adnl::proxypacketheader::ProxyPacketHeader {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(x) => x,
        }
    }
}
impl Eq for ProxyPacketHeader {}
impl Default for ProxyPacketHeader {
    fn default() -> Self {
        ProxyPacketHeader::Adnl_ProxyPacketHeader(
            crate::ton::adnl::proxypacketheader::ProxyPacketHeader::default(),
        )
    }
}
impl crate::BoxedSerialize for ProxyPacketHeader {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ProxyPacketHeader::Adnl_ProxyPacketHeader(x) => {
                (crate::ConstructorNumber(0x08693c78), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ProxyPacketHeader {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x08693c78)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x08693c78) => Ok(ProxyPacketHeader::Adnl_ProxyPacketHeader(
                _de.read_bare::<crate::ton::adnl::proxypacketheader::ProxyPacketHeader>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.ProxyTo`\n\n```text\nadnl.proxyToFastHash ip:int port:int date:int data_hash:int256 shared_secret:int256 = adnl.ProxyTo;\n```\n"]
pub enum ProxyTo {
    Adnl_ProxyToFastHash(crate::ton::adnl::proxytofasthash::ProxyToFastHash),
}
impl ProxyTo {
    pub fn data_hash(&self) -> &crate::ton::int256 {
        match self {
            ProxyTo::Adnl_ProxyToFastHash(ref x) => &x.data_hash,
        }
    }

    pub fn date(&self) -> &crate::ton::int {
        match self {
            ProxyTo::Adnl_ProxyToFastHash(ref x) => &x.date,
        }
    }

    pub fn ip(&self) -> &crate::ton::int {
        match self {
            ProxyTo::Adnl_ProxyToFastHash(ref x) => &x.ip,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            ProxyTo::Adnl_ProxyToFastHash(ref x) => &x.port,
        }
    }

    pub fn shared_secret(&self) -> &crate::ton::int256 {
        match self {
            ProxyTo::Adnl_ProxyToFastHash(ref x) => &x.shared_secret,
        }
    }

    pub fn only(self) -> crate::ton::adnl::proxytofasthash::ProxyToFastHash {
        match self {
            ProxyTo::Adnl_ProxyToFastHash(x) => x,
        }
    }
}
impl Eq for ProxyTo {}
impl Default for ProxyTo {
    fn default() -> Self {
        ProxyTo::Adnl_ProxyToFastHash(crate::ton::adnl::proxytofasthash::ProxyToFastHash::default())
    }
}
impl crate::BoxedSerialize for ProxyTo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ProxyTo::Adnl_ProxyToFastHash(x) => (crate::ConstructorNumber(0xddbdf85e), x),
        }
    }
}
impl crate::BoxedDeserialize for ProxyTo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xddbdf85e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xddbdf85e) => Ok(ProxyTo::Adnl_ProxyToFastHash(
                _de.read_bare::<crate::ton::adnl::proxytofasthash::ProxyToFastHash>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.ProxyToSign`\n\n```text\nadnl.proxyToFast ip:int port:int date:int signature:int256 = adnl.ProxyToSign;\n```\n"]
pub enum ProxyToSign {
    Adnl_ProxyToFast(crate::ton::adnl::proxytofast::ProxyToFast),
}
impl ProxyToSign {
    pub fn date(&self) -> &crate::ton::int {
        match self {
            ProxyToSign::Adnl_ProxyToFast(ref x) => &x.date,
        }
    }

    pub fn ip(&self) -> &crate::ton::int {
        match self {
            ProxyToSign::Adnl_ProxyToFast(ref x) => &x.ip,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            ProxyToSign::Adnl_ProxyToFast(ref x) => &x.port,
        }
    }

    pub fn signature(&self) -> &crate::ton::int256 {
        match self {
            ProxyToSign::Adnl_ProxyToFast(ref x) => &x.signature,
        }
    }

    pub fn only(self) -> crate::ton::adnl::proxytofast::ProxyToFast {
        match self {
            ProxyToSign::Adnl_ProxyToFast(x) => x,
        }
    }
}
impl Eq for ProxyToSign {}
impl Default for ProxyToSign {
    fn default() -> Self {
        ProxyToSign::Adnl_ProxyToFast(crate::ton::adnl::proxytofast::ProxyToFast::default())
    }
}
impl crate::BoxedSerialize for ProxyToSign {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ProxyToSign::Adnl_ProxyToFast(x) => (crate::ConstructorNumber(0xb4ee21d6), x),
        }
    }
}
impl crate::BoxedDeserialize for ProxyToSign {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb4ee21d6)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb4ee21d6) => Ok(ProxyToSign::Adnl_ProxyToFast(
                _de.read_bare::<crate::ton::adnl::proxytofast::ProxyToFast>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `adnl.TunnelPacketContents`\n\n```text\nadnl.tunnelPacketContents \n  rand1:bytes \n  flags:# \n  from_ip:flags.0?int\n  from_port:flags.0?int\n  message:flags.1?bytes \n  statistics:flags.2?bytes\n  payment:flags.3?bytes\n  rand2:bytes \n        = adnl.TunnelPacketContents;\n```\n"]
pub enum TunnelPacketContents {
    Adnl_TunnelPacketContents(crate::ton::adnl::tunnelpacketcontents::TunnelPacketContents),
}
impl TunnelPacketContents {
    pub fn from_ip(&self) -> Option<&crate::ton::int> {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(ref x) => x.from_ip.as_ref(),
        }
    }

    pub fn from_port(&self) -> Option<&crate::ton::int> {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(ref x) => x.from_port.as_ref(),
        }
    }

    pub fn message(&self) -> Option<&crate::ton::bytes> {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(ref x) => x.message.as_ref(),
        }
    }

    pub fn payment(&self) -> Option<&crate::ton::bytes> {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(ref x) => x.payment.as_ref(),
        }
    }

    pub fn rand1(&self) -> &crate::ton::bytes {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(ref x) => &x.rand1,
        }
    }

    pub fn rand2(&self) -> &crate::ton::bytes {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(ref x) => &x.rand2,
        }
    }

    pub fn statistics(&self) -> Option<&crate::ton::bytes> {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(ref x) => x.statistics.as_ref(),
        }
    }

    pub fn only(self) -> crate::ton::adnl::tunnelpacketcontents::TunnelPacketContents {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(x) => x,
        }
    }
}
impl Eq for TunnelPacketContents {}
impl Default for TunnelPacketContents {
    fn default() -> Self {
        TunnelPacketContents::Adnl_TunnelPacketContents(
            crate::ton::adnl::tunnelpacketcontents::TunnelPacketContents::default(),
        )
    }
}
impl crate::BoxedSerialize for TunnelPacketContents {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            TunnelPacketContents::Adnl_TunnelPacketContents(x) => {
                (crate::ConstructorNumber(0xc59138b4), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for TunnelPacketContents {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc59138b4)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc59138b4) => {
                Ok(TunnelPacketContents::Adnl_TunnelPacketContents(
                    _de.read_bare::<crate::ton::adnl::tunnelpacketcontents::TunnelPacketContents>(
                    )?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod address;
pub mod addresslist;
pub mod config;
pub mod db;
pub mod id;
pub mod message;
pub mod node;
pub mod nodes;
pub mod packetcontents;
pub mod pong;
pub mod proxy;
pub mod proxycontrolpacket;
pub mod proxypacketheader;
pub mod proxytofast;
pub mod proxytofasthash;
pub mod tunnelpacketcontents;
