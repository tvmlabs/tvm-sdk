use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.Key`\n\n```text\ndht.key id:int256 name:bytes idx:int = dht.Key;\n```\n"]
pub enum Key {
    Dht_Key(crate::ton::dht::key::Key),
}
impl Key {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            Key::Dht_Key(ref x) => &x.id,
        }
    }

    pub fn idx(&self) -> &crate::ton::int {
        match self {
            Key::Dht_Key(ref x) => &x.idx,
        }
    }

    pub fn name(&self) -> &crate::ton::bytes {
        match self {
            Key::Dht_Key(ref x) => &x.name,
        }
    }

    pub fn only(self) -> crate::ton::dht::key::Key {
        match self {
            Key::Dht_Key(x) => x,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Dht_Key(crate::ton::dht::key::Key::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Dht_Key(x) => (crate::ConstructorNumber(0xf667de8f), x),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf667de8f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf667de8f) => {
                Ok(Key::Dht_Key(_de.read_bare::<crate::ton::dht::key::Key>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.KeyDescription`\n\n```text\ndht.keyDescription key:dht.key id:PublicKey update_rule:dht.UpdateRule signature:bytes = dht.KeyDescription;\n```\n"]
pub enum KeyDescription {
    Dht_KeyDescription(crate::ton::dht::keydescription::KeyDescription),
}
impl KeyDescription {
    pub fn id(&self) -> &crate::ton::PublicKey {
        match self {
            KeyDescription::Dht_KeyDescription(ref x) => &x.id,
        }
    }

    pub fn key(&self) -> &crate::ton::dht::key::Key {
        match self {
            KeyDescription::Dht_KeyDescription(ref x) => &x.key,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            KeyDescription::Dht_KeyDescription(ref x) => &x.signature,
        }
    }

    pub fn update_rule(&self) -> &crate::ton::dht::UpdateRule {
        match self {
            KeyDescription::Dht_KeyDescription(ref x) => &x.update_rule,
        }
    }

    pub fn only(self) -> crate::ton::dht::keydescription::KeyDescription {
        match self {
            KeyDescription::Dht_KeyDescription(x) => x,
        }
    }
}
impl Eq for KeyDescription {}
impl Default for KeyDescription {
    fn default() -> Self {
        KeyDescription::Dht_KeyDescription(
            crate::ton::dht::keydescription::KeyDescription::default(),
        )
    }
}
impl crate::BoxedSerialize for KeyDescription {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            KeyDescription::Dht_KeyDescription(x) => (crate::ConstructorNumber(0x281d4e05), x),
        }
    }
}
impl crate::BoxedDeserialize for KeyDescription {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x281d4e05)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x281d4e05) => Ok(KeyDescription::Dht_KeyDescription(
                _de.read_bare::<crate::ton::dht::keydescription::KeyDescription>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.Message`\n\n```text\ndht.message node:dht.node = dht.Message;\n```\n"]
pub enum Message {
    Dht_Message(crate::ton::dht::message::Message),
}
impl Message {
    pub fn node(&self) -> &crate::ton::dht::node::Node {
        match self {
            Message::Dht_Message(ref x) => &x.node,
        }
    }

    pub fn only(self) -> crate::ton::dht::message::Message {
        match self {
            Message::Dht_Message(x) => x,
        }
    }
}
impl Eq for Message {}
impl Default for Message {
    fn default() -> Self {
        Message::Dht_Message(crate::ton::dht::message::Message::default())
    }
}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::Dht_Message(x) => (crate::ConstructorNumber(0xbc0cdb8e), x),
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbc0cdb8e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbc0cdb8e) => {
                Ok(Message::Dht_Message(_de.read_bare::<crate::ton::dht::message::Message>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.Node`\n\n```text\ndht.node id:PublicKey addr_list:adnl.addressList version:int signature:bytes = dht.Node;\n```\n"]
pub enum Node {
    Dht_Node(crate::ton::dht::node::Node),
}
impl Node {
    pub fn addr_list(&self) -> &crate::ton::adnl::addresslist::AddressList {
        match self {
            Node::Dht_Node(ref x) => &x.addr_list,
        }
    }

    pub fn id(&self) -> &crate::ton::PublicKey {
        match self {
            Node::Dht_Node(ref x) => &x.id,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            Node::Dht_Node(ref x) => &x.signature,
        }
    }

    pub fn version(&self) -> &crate::ton::int {
        match self {
            Node::Dht_Node(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::dht::node::Node {
        match self {
            Node::Dht_Node(x) => x,
        }
    }
}
impl Eq for Node {}
impl Default for Node {
    fn default() -> Self {
        Node::Dht_Node(crate::ton::dht::node::Node::default())
    }
}
impl crate::BoxedSerialize for Node {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Node::Dht_Node(x) => (crate::ConstructorNumber(0x84533248), x),
        }
    }
}
impl crate::BoxedDeserialize for Node {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x84533248)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x84533248) => {
                Ok(Node::Dht_Node(_de.read_bare::<crate::ton::dht::node::Node>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.Nodes`\n\n```text\ndht.nodes nodes:(vector dht.node) = dht.Nodes;\n```\n"]
pub enum Nodes {
    Dht_Nodes(crate::ton::dht::nodes::Nodes),
}
impl Nodes {
    pub fn nodes(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::dht::node::Node> {
        match self {
            Nodes::Dht_Nodes(ref x) => &x.nodes,
        }
    }

    pub fn only(self) -> crate::ton::dht::nodes::Nodes {
        match self {
            Nodes::Dht_Nodes(x) => x,
        }
    }
}
impl Eq for Nodes {}
impl Default for Nodes {
    fn default() -> Self {
        Nodes::Dht_Nodes(crate::ton::dht::nodes::Nodes::default())
    }
}
impl crate::BoxedSerialize for Nodes {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Nodes::Dht_Nodes(x) => (crate::ConstructorNumber(0x7974a0be), x),
        }
    }
}
impl crate::BoxedDeserialize for Nodes {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7974a0be)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7974a0be) => {
                Ok(Nodes::Dht_Nodes(_de.read_bare::<crate::ton::dht::nodes::Nodes>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.Pong`\n\n```text\ndht.pong random_id:long = dht.Pong;\n```\n"]
pub enum Pong {
    Dht_Pong(crate::ton::dht::pong::Pong),
}
impl Pong {
    pub fn random_id(&self) -> &crate::ton::long {
        match self {
            Pong::Dht_Pong(ref x) => &x.random_id,
        }
    }

    pub fn only(self) -> crate::ton::dht::pong::Pong {
        match self {
            Pong::Dht_Pong(x) => x,
        }
    }
}
impl Eq for Pong {}
impl Default for Pong {
    fn default() -> Self {
        Pong::Dht_Pong(crate::ton::dht::pong::Pong::default())
    }
}
impl crate::BoxedSerialize for Pong {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Pong::Dht_Pong(x) => (crate::ConstructorNumber(0x5a8aef81), x),
        }
    }
}
impl crate::BoxedDeserialize for Pong {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5a8aef81)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5a8aef81) => {
                Ok(Pong::Dht_Pong(_de.read_bare::<crate::ton::dht::pong::Pong>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `dht.Stored`\n\n```text\ndht.stored = dht.Stored;\n```\n"]
pub enum Stored {
    #[default]
    Dht_Stored,
}
impl Eq for Stored {}
impl crate::BoxedSerialize for Stored {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Stored::Dht_Stored => (crate::ConstructorNumber(0x7026fb08), &()),
        }
    }
}
impl crate::BoxedDeserialize for Stored {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7026fb08)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7026fb08) => Ok(Stored::Dht_Stored),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `dht.UpdateRule`\n\n```text\ndht.updateRule.anybody = dht.UpdateRule;\n\ndht.updateRule.overlayNodes = dht.UpdateRule;\n\ndht.updateRule.signature = dht.UpdateRule;\n```\n"]
pub enum UpdateRule {
    #[default]
    Dht_UpdateRule_Anybody,
    Dht_UpdateRule_OverlayNodes,
    Dht_UpdateRule_Signature,
}
impl Eq for UpdateRule {}
impl crate::BoxedSerialize for UpdateRule {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            UpdateRule::Dht_UpdateRule_Anybody => (crate::ConstructorNumber(0x61578e14), &()),
            UpdateRule::Dht_UpdateRule_OverlayNodes => (crate::ConstructorNumber(0x26779383), &()),
            UpdateRule::Dht_UpdateRule_Signature => (crate::ConstructorNumber(0xcc9f31f7), &()),
        }
    }
}
impl crate::BoxedDeserialize for UpdateRule {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x61578e14),
            crate::ConstructorNumber(0x26779383),
            crate::ConstructorNumber(0xcc9f31f7),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x61578e14) => Ok(UpdateRule::Dht_UpdateRule_Anybody),
            crate::ConstructorNumber(0x26779383) => Ok(UpdateRule::Dht_UpdateRule_OverlayNodes),
            crate::ConstructorNumber(0xcc9f31f7) => Ok(UpdateRule::Dht_UpdateRule_Signature),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.Value`\n\n```text\ndht.value key:dht.keyDescription value:bytes ttl:int signature:bytes = dht.Value;\n```\n"]
pub enum Value {
    Dht_Value(crate::ton::dht::value::Value),
}
impl Value {
    pub fn key(&self) -> &crate::ton::dht::keydescription::KeyDescription {
        match self {
            Value::Dht_Value(ref x) => &x.key,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            Value::Dht_Value(ref x) => &x.signature,
        }
    }

    pub fn ttl(&self) -> &crate::ton::int {
        match self {
            Value::Dht_Value(ref x) => &x.ttl,
        }
    }

    pub fn value(&self) -> &crate::ton::bytes {
        match self {
            Value::Dht_Value(ref x) => &x.value,
        }
    }

    pub fn only(self) -> crate::ton::dht::value::Value {
        match self {
            Value::Dht_Value(x) => x,
        }
    }
}
impl Eq for Value {}
impl Default for Value {
    fn default() -> Self {
        Value::Dht_Value(crate::ton::dht::value::Value::default())
    }
}
impl crate::BoxedSerialize for Value {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Value::Dht_Value(x) => (crate::ConstructorNumber(0x90ad27cb), x),
        }
    }
}
impl crate::BoxedDeserialize for Value {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x90ad27cb)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x90ad27cb) => {
                Ok(Value::Dht_Value(_de.read_bare::<crate::ton::dht::value::Value>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `dht.ValueResult`\n\n```text\ndht.valueFound value:dht.Value = dht.ValueResult;\n\ndht.valueNotFound nodes:dht.nodes = dht.ValueResult;\n```\n"]
pub enum ValueResult {
    Dht_ValueFound(crate::ton::dht::valueresult::ValueFound),
    Dht_ValueNotFound(crate::ton::dht::valueresult::ValueNotFound),
}
impl ValueResult {
    pub fn nodes(&self) -> Option<&crate::ton::dht::nodes::Nodes> {
        match self {
            ValueResult::Dht_ValueNotFound(ref x) => Some(&x.nodes),
            _ => None,
        }
    }

    pub fn value(&self) -> Option<&crate::ton::dht::Value> {
        match self {
            ValueResult::Dht_ValueFound(ref x) => Some(&x.value),
            _ => None,
        }
    }
}
impl Eq for ValueResult {}
impl Default for ValueResult {
    fn default() -> Self {
        ValueResult::Dht_ValueFound(crate::ton::dht::valueresult::ValueFound::default())
    }
}
impl crate::BoxedSerialize for ValueResult {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ValueResult::Dht_ValueFound(x) => (crate::ConstructorNumber(0xe40cf774), x),
            ValueResult::Dht_ValueNotFound(x) => (crate::ConstructorNumber(0xa2620568), x),
        }
    }
}
impl crate::BoxedDeserialize for ValueResult {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe40cf774), crate::ConstructorNumber(0xa2620568)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe40cf774) => Ok(ValueResult::Dht_ValueFound(
                _de.read_bare::<crate::ton::dht::valueresult::ValueFound>()?,
            )),
            crate::ConstructorNumber(0xa2620568) => Ok(ValueResult::Dht_ValueNotFound(
                _de.read_bare::<crate::ton::dht::valueresult::ValueNotFound>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod config;
pub mod db;
pub mod key;
pub mod keydescription;
pub mod message;
pub mod node;
pub mod nodes;
pub mod pong;
pub mod stored;
pub mod update_rule;
pub mod value;
pub mod valueresult;
