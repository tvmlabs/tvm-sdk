use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.AsyncSerializer`\n\n```text\ndb.state.asyncSerializer block:tonNode.blockIdExt last:tonNode.blockIdExt last_ts:int = db.state.AsyncSerializer;\n```\n"]
pub enum AsyncSerializer {
    Db_State_AsyncSerializer(crate::ton::db::state::asyncserializer::AsyncSerializer),
}
impl AsyncSerializer {
    pub fn block(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            AsyncSerializer::Db_State_AsyncSerializer(ref x) => &x.block,
        }
    }

    pub fn last(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            AsyncSerializer::Db_State_AsyncSerializer(ref x) => &x.last,
        }
    }

    pub fn last_ts(&self) -> &crate::ton::int {
        match self {
            AsyncSerializer::Db_State_AsyncSerializer(ref x) => &x.last_ts,
        }
    }

    pub fn only(self) -> crate::ton::db::state::asyncserializer::AsyncSerializer {
        match self {
            AsyncSerializer::Db_State_AsyncSerializer(x) => x,
        }
    }
}
impl Eq for AsyncSerializer {}
impl Default for AsyncSerializer {
    fn default() -> Self {
        AsyncSerializer::Db_State_AsyncSerializer(
            crate::ton::db::state::asyncserializer::AsyncSerializer::default(),
        )
    }
}
impl crate::BoxedSerialize for AsyncSerializer {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AsyncSerializer::Db_State_AsyncSerializer(x) => {
                (crate::ConstructorNumber(0xd32f29a1), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for AsyncSerializer {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd32f29a1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd32f29a1) => Ok(AsyncSerializer::Db_State_AsyncSerializer(
                _de.read_bare::<crate::ton::db::state::asyncserializer::AsyncSerializer>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.DbVersion`\n\n```text\ndb.state.dbVersion version:int = db.state.DbVersion;\n```\n"]
pub enum DbVersion {
    Db_State_DbVersion(crate::ton::db::state::dbversion::DbVersion),
}
impl DbVersion {
    pub fn version(&self) -> &crate::ton::int {
        match self {
            DbVersion::Db_State_DbVersion(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::db::state::dbversion::DbVersion {
        match self {
            DbVersion::Db_State_DbVersion(x) => x,
        }
    }
}
impl Eq for DbVersion {}
impl Default for DbVersion {
    fn default() -> Self {
        DbVersion::Db_State_DbVersion(crate::ton::db::state::dbversion::DbVersion::default())
    }
}
impl crate::BoxedSerialize for DbVersion {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DbVersion::Db_State_DbVersion(x) => (crate::ConstructorNumber(0xd93720f7), x),
        }
    }
}
impl crate::BoxedDeserialize for DbVersion {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd93720f7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd93720f7) => Ok(DbVersion::Db_State_DbVersion(
                _de.read_bare::<crate::ton::db::state::dbversion::DbVersion>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.DestroyedSessions`\n\n```text\ndb.state.destroyedSessions sessions:(vector int256) = db.state.DestroyedSessions;\n```\n"]
pub enum DestroyedSessions {
    Db_State_DestroyedSessions(crate::ton::db::state::destroyedsessions::DestroyedSessions),
}
impl DestroyedSessions {
    pub fn sessions(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int256> {
        match self {
            DestroyedSessions::Db_State_DestroyedSessions(ref x) => &x.sessions,
        }
    }

    pub fn only(self) -> crate::ton::db::state::destroyedsessions::DestroyedSessions {
        match self {
            DestroyedSessions::Db_State_DestroyedSessions(x) => x,
        }
    }
}
impl Eq for DestroyedSessions {}
impl Default for DestroyedSessions {
    fn default() -> Self {
        DestroyedSessions::Db_State_DestroyedSessions(
            crate::ton::db::state::destroyedsessions::DestroyedSessions::default(),
        )
    }
}
impl crate::BoxedSerialize for DestroyedSessions {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DestroyedSessions::Db_State_DestroyedSessions(x) => {
                (crate::ConstructorNumber(0xada8d984), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for DestroyedSessions {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xada8d984)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xada8d984) => {
                Ok(DestroyedSessions::Db_State_DestroyedSessions(
                    _de.read_bare::<crate::ton::db::state::destroyedsessions::DestroyedSessions>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.GcBlockId`\n\n```text\ndb.state.gcBlockId block:tonNode.blockIdExt = db.state.GcBlockId;\n```\n"]
pub enum GcBlockId {
    Db_State_GcBlockId(crate::ton::db::state::gcblockid::GcBlockId),
}
impl GcBlockId {
    pub fn block(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            GcBlockId::Db_State_GcBlockId(ref x) => &x.block,
        }
    }

    pub fn only(self) -> crate::ton::db::state::gcblockid::GcBlockId {
        match self {
            GcBlockId::Db_State_GcBlockId(x) => x,
        }
    }
}
impl Eq for GcBlockId {}
impl Default for GcBlockId {
    fn default() -> Self {
        GcBlockId::Db_State_GcBlockId(crate::ton::db::state::gcblockid::GcBlockId::default())
    }
}
impl crate::BoxedSerialize for GcBlockId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            GcBlockId::Db_State_GcBlockId(x) => (crate::ConstructorNumber(0xdf30bd4f), x),
        }
    }
}
impl crate::BoxedDeserialize for GcBlockId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdf30bd4f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xdf30bd4f) => Ok(GcBlockId::Db_State_GcBlockId(
                _de.read_bare::<crate::ton::db::state::gcblockid::GcBlockId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.Hardforks`\n\n```text\ndb.state.hardforks blocks:(vector tonNode.blockIdExt) = db.state.Hardforks;\n```\n"]
pub enum Hardforks {
    Db_State_Hardforks(crate::ton::db::state::hardforks::Hardforks),
}
impl Hardforks {
    pub fn blocks(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt> {
        match self {
            Hardforks::Db_State_Hardforks(ref x) => &x.blocks,
        }
    }

    pub fn only(self) -> crate::ton::db::state::hardforks::Hardforks {
        match self {
            Hardforks::Db_State_Hardforks(x) => x,
        }
    }
}
impl Eq for Hardforks {}
impl Default for Hardforks {
    fn default() -> Self {
        Hardforks::Db_State_Hardforks(crate::ton::db::state::hardforks::Hardforks::default())
    }
}
impl crate::BoxedSerialize for Hardforks {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Hardforks::Db_State_Hardforks(x) => (crate::ConstructorNumber(0x85f30d04), x),
        }
    }
}
impl crate::BoxedDeserialize for Hardforks {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x85f30d04)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x85f30d04) => Ok(Hardforks::Db_State_Hardforks(
                _de.read_bare::<crate::ton::db::state::hardforks::Hardforks>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.InitBlockId`\n\n```text\ndb.state.initBlockId block:tonNode.blockIdExt = db.state.InitBlockId;\n```\n"]
pub enum InitBlockId {
    Db_State_InitBlockId(crate::ton::db::state::initblockid::InitBlockId),
}
impl InitBlockId {
    pub fn block(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            InitBlockId::Db_State_InitBlockId(ref x) => &x.block,
        }
    }

    pub fn only(self) -> crate::ton::db::state::initblockid::InitBlockId {
        match self {
            InitBlockId::Db_State_InitBlockId(x) => x,
        }
    }
}
impl Eq for InitBlockId {}
impl Default for InitBlockId {
    fn default() -> Self {
        InitBlockId::Db_State_InitBlockId(crate::ton::db::state::initblockid::InitBlockId::default())
    }
}
impl crate::BoxedSerialize for InitBlockId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            InitBlockId::Db_State_InitBlockId(x) => (crate::ConstructorNumber(0x732c9cf5), x),
        }
    }
}
impl crate::BoxedDeserialize for InitBlockId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x732c9cf5)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x732c9cf5) => Ok(InitBlockId::Db_State_InitBlockId(
                _de.read_bare::<crate::ton::db::state::initblockid::InitBlockId>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `db.state.Key`\n\n```text\ndb.state.key.asyncSerializer = db.state.Key;\n\ndb.state.key.dbVersion = db.state.Key;\n\ndb.state.key.destroyedSessions = db.state.Key;\n\ndb.state.key.gcBlockId = db.state.Key;\n\ndb.state.key.hardforks = db.state.Key;\n\ndb.state.key.initBlockId = db.state.Key;\n\ndb.state.key.shardClient = db.state.Key;\n```\n"]
pub enum Key {
    #[default]
    Db_State_Key_AsyncSerializer,
    Db_State_Key_DbVersion,
    Db_State_Key_DestroyedSessions,
    Db_State_Key_GcBlockId,
    Db_State_Key_Hardforks,
    Db_State_Key_InitBlockId,
    Db_State_Key_ShardClient,
}
impl Eq for Key {}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Db_State_Key_AsyncSerializer => (crate::ConstructorNumber(0x29ae8a1f), &()),
            Key::Db_State_Key_DbVersion => (crate::ConstructorNumber(0x724f2154), &()),
            Key::Db_State_Key_DestroyedSessions => (crate::ConstructorNumber(0xe8f7f159), &()),
            Key::Db_State_Key_GcBlockId => (crate::ConstructorNumber(0xc379f3de), &()),
            Key::Db_State_Key_Hardforks => (crate::ConstructorNumber(0xe6f427ba), &()),
            Key::Db_State_Key_InitBlockId => (crate::ConstructorNumber(0x758278e3), &()),
            Key::Db_State_Key_ShardClient => (crate::ConstructorNumber(0xc99b3187), &()),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x29ae8a1f),
            crate::ConstructorNumber(0x724f2154),
            crate::ConstructorNumber(0xe8f7f159),
            crate::ConstructorNumber(0xc379f3de),
            crate::ConstructorNumber(0xe6f427ba),
            crate::ConstructorNumber(0x758278e3),
            crate::ConstructorNumber(0xc99b3187),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x29ae8a1f) => Ok(Key::Db_State_Key_AsyncSerializer),
            crate::ConstructorNumber(0x724f2154) => Ok(Key::Db_State_Key_DbVersion),
            crate::ConstructorNumber(0xe8f7f159) => Ok(Key::Db_State_Key_DestroyedSessions),
            crate::ConstructorNumber(0xc379f3de) => Ok(Key::Db_State_Key_GcBlockId),
            crate::ConstructorNumber(0xe6f427ba) => Ok(Key::Db_State_Key_Hardforks),
            crate::ConstructorNumber(0x758278e3) => Ok(Key::Db_State_Key_InitBlockId),
            crate::ConstructorNumber(0xc99b3187) => Ok(Key::Db_State_Key_ShardClient),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `db.state.ShardClient`\n\n```text\ndb.state.shardClient block:tonNode.blockIdExt = db.state.ShardClient;\n```\n"]
pub enum ShardClient {
    Db_State_ShardClient(crate::ton::db::state::shardclient::ShardClient),
}
impl ShardClient {
    pub fn block(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            ShardClient::Db_State_ShardClient(ref x) => &x.block,
        }
    }

    pub fn only(self) -> crate::ton::db::state::shardclient::ShardClient {
        match self {
            ShardClient::Db_State_ShardClient(x) => x,
        }
    }
}
impl Eq for ShardClient {}
impl Default for ShardClient {
    fn default() -> Self {
        ShardClient::Db_State_ShardClient(crate::ton::db::state::shardclient::ShardClient::default())
    }
}
impl crate::BoxedSerialize for ShardClient {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ShardClient::Db_State_ShardClient(x) => (crate::ConstructorNumber(0x0b16a69d), x),
        }
    }
}
impl crate::BoxedDeserialize for ShardClient {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0b16a69d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0b16a69d) => Ok(ShardClient::Db_State_ShardClient(
                _de.read_bare::<crate::ton::db::state::shardclient::ShardClient>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod asyncserializer;
pub mod dbversion;
pub mod destroyedsessions;
pub mod gcblockid;
pub mod hardforks;
pub mod initblockid;
pub mod key;
pub mod shardclient;
