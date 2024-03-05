use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addAdnlId`\n\n```text\nengine.validator.addAdnlId key_hash:int256 category:int = engine.validator.Success;\n```\n"]
pub struct AddAdnlId {
    pub key_hash: crate::ton::int256,
    pub category: crate::ton::int,
}
impl Eq for AddAdnlId {}
impl crate::BareSerialize for AddAdnlId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xed8554ab)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddAdnlId { key_hash, category } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::int>(category)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddAdnlId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let category = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key_hash, category })
        }
    }
}
impl crate::BoxedDeserialize for AddAdnlId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xed8554ab)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xed8554ab) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddAdnlId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xed8554ab), self)
    }
}
impl crate::Function for AddAdnlId {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addControlInterface`\n\n```text\nengine.validator.addControlInterface key_hash:int256 port:int = engine.validator.Success;\n```\n"]
pub struct AddControlInterface {
    pub key_hash: crate::ton::int256,
    pub port: crate::ton::int,
}
impl Eq for AddControlInterface {}
impl crate::BareSerialize for AddControlInterface {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x348bf3fc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddControlInterface { key_hash, port } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddControlInterface {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key_hash, port })
        }
    }
}
impl crate::BoxedDeserialize for AddControlInterface {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x348bf3fc)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x348bf3fc) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddControlInterface {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x348bf3fc), self)
    }
}
impl crate::Function for AddControlInterface {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addControlProcess`\n\n```text\nengine.validator.addControlProcess key_hash:int256 port:int peer_key:int256 permissions:int = engine.validator.Success;\n```\n"]
pub struct AddControlProcess {
    pub key_hash: crate::ton::int256,
    pub port: crate::ton::int,
    pub peer_key: crate::ton::int256,
    pub permissions: crate::ton::int,
}
impl Eq for AddControlProcess {}
impl crate::BareSerialize for AddControlProcess {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5ae0f750)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddControlProcess { key_hash, port, peer_key, permissions } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_bare::<crate::ton::int256>(peer_key)?;
        _ser.write_bare::<crate::ton::int>(permissions)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddControlProcess {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let peer_key = _de.read_bare::<crate::ton::int256>()?;
            let permissions = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key_hash, port, peer_key, permissions })
        }
    }
}
impl crate::BoxedDeserialize for AddControlProcess {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5ae0f750)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x5ae0f750) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddControlProcess {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x5ae0f750), self)
    }
}
impl crate::Function for AddControlProcess {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addDhtId`\n\n```text\nengine.validator.addDhtId key_hash:int256 = engine.validator.Success;\n```\n"]
pub struct AddDhtId {
    pub key_hash: crate::ton::int256,
}
impl Eq for AddDhtId {}
impl crate::BareSerialize for AddDhtId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf50c1e8c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddDhtId { key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddDhtId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key_hash })
        }
    }
}
impl crate::BoxedDeserialize for AddDhtId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf50c1e8c)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf50c1e8c) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddDhtId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf50c1e8c), self)
    }
}
impl crate::Function for AddDhtId {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addListeningPort`\n\n```text\nengine.validator.addListeningPort ip:int port:int categories:(vector int) priority_categories:(vector int) = engine.validator.Success;\n```\n"]
pub struct AddListeningPort {
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
    pub categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
    pub priority_categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for AddListeningPort {}
impl crate::BareSerialize for AddListeningPort {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xea6b89b5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddListeningPort { ip, port, categories, priority_categories } = self;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(categories)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(
            priority_categories,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddListeningPort {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            let priority_categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { ip, port, categories, priority_categories })
        }
    }
}
impl crate::BoxedDeserialize for AddListeningPort {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xea6b89b5)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xea6b89b5) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddListeningPort {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xea6b89b5), self)
    }
}
impl crate::Function for AddListeningPort {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addLiteserver`\n\n```text\nengine.validator.addLiteserver key_hash:int256 port:int = engine.validator.Success;\n```\n"]
pub struct AddLiteserver {
    pub key_hash: crate::ton::int256,
    pub port: crate::ton::int,
}
impl Eq for AddLiteserver {}
impl crate::BareSerialize for AddLiteserver {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf08a0f47)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddLiteserver { key_hash, port } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddLiteserver {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key_hash, port })
        }
    }
}
impl crate::BoxedDeserialize for AddLiteserver {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf08a0f47)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf08a0f47) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddLiteserver {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf08a0f47), self)
    }
}
impl crate::Function for AddLiteserver {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addProxy`\n\n```text\nengine.validator.addProxy in_ip:int in_port:int out_ip:int out_port:int proxy:adnl.Proxy categories:(vector int) priority_categories:(vector int) = engine.validator.Success;\n```\n"]
pub struct AddProxy {
    pub in_ip: crate::ton::int,
    pub in_port: crate::ton::int,
    pub out_ip: crate::ton::int,
    pub out_port: crate::ton::int,
    pub proxy: crate::ton::adnl::Proxy,
    pub categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
    pub priority_categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for AddProxy {}
impl crate::BareSerialize for AddProxy {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf6fd33f5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddProxy { in_ip, in_port, out_ip, out_port, proxy, categories, priority_categories } =
            self;
        _ser.write_bare::<crate::ton::int>(in_ip)?;
        _ser.write_bare::<crate::ton::int>(in_port)?;
        _ser.write_bare::<crate::ton::int>(out_ip)?;
        _ser.write_bare::<crate::ton::int>(out_port)?;
        _ser.write_boxed::<crate::ton::adnl::Proxy>(proxy)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(categories)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(
            priority_categories,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddProxy {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let in_ip = _de.read_bare::<crate::ton::int>()?;
            let in_port = _de.read_bare::<crate::ton::int>()?;
            let out_ip = _de.read_bare::<crate::ton::int>()?;
            let out_port = _de.read_bare::<crate::ton::int>()?;
            let proxy = _de.read_boxed::<crate::ton::adnl::Proxy>()?;
            let categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            let priority_categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { in_ip, in_port, out_ip, out_port, proxy, categories, priority_categories })
        }
    }
}
impl crate::BoxedDeserialize for AddProxy {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf6fd33f5)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf6fd33f5) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddProxy {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf6fd33f5), self)
    }
}
impl crate::Function for AddProxy {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addValidatorAdnlAddress`\n\n```text\nengine.validator.addValidatorAdnlAddress permanent_key_hash:int256 key_hash:int256 ttl:int = engine.validator.Success;\n```\n"]
pub struct AddValidatorAdnlAddress {
    pub permanent_key_hash: crate::ton::int256,
    pub key_hash: crate::ton::int256,
    pub ttl: crate::ton::int,
}
impl Eq for AddValidatorAdnlAddress {}
impl crate::BareSerialize for AddValidatorAdnlAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xdacba682)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddValidatorAdnlAddress { permanent_key_hash, key_hash, ttl } = self;
        _ser.write_bare::<crate::ton::int256>(permanent_key_hash)?;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::int>(ttl)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddValidatorAdnlAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let permanent_key_hash = _de.read_bare::<crate::ton::int256>()?;
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let ttl = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { permanent_key_hash, key_hash, ttl })
        }
    }
}
impl crate::BoxedDeserialize for AddValidatorAdnlAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdacba682)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xdacba682) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddValidatorAdnlAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xdacba682), self)
    }
}
impl crate::Function for AddValidatorAdnlAddress {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addValidatorBlsKey`\n\n```text\nengine.validator.addValidatorBlsKey permanent_key_hash:int256 key_hash:int256 ttl:int = engine.validator.Success;\n```\n"]
pub struct AddValidatorBlsKey {
    pub permanent_key_hash: crate::ton::int256,
    pub key_hash: crate::ton::int256,
    pub ttl: crate::ton::int,
}
impl Eq for AddValidatorBlsKey {}
impl crate::BareSerialize for AddValidatorBlsKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4a8aa32c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddValidatorBlsKey { permanent_key_hash, key_hash, ttl } = self;
        _ser.write_bare::<crate::ton::int256>(permanent_key_hash)?;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::int>(ttl)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddValidatorBlsKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let permanent_key_hash = _de.read_bare::<crate::ton::int256>()?;
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let ttl = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { permanent_key_hash, key_hash, ttl })
        }
    }
}
impl crate::BoxedDeserialize for AddValidatorBlsKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x4a8aa32c)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x4a8aa32c) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddValidatorBlsKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x4a8aa32c), self)
    }
}
impl crate::Function for AddValidatorBlsKey {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addValidatorPermanentKey`\n\n```text\nengine.validator.addValidatorPermanentKey key_hash:int256 election_date:int ttl:int = engine.validator.Success;\n```\n"]
pub struct AddValidatorPermanentKey {
    pub key_hash: crate::ton::int256,
    pub election_date: crate::ton::int,
    pub ttl: crate::ton::int,
}
impl Eq for AddValidatorPermanentKey {}
impl crate::BareSerialize for AddValidatorPermanentKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x92150578)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddValidatorPermanentKey { key_hash, election_date, ttl } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::int>(election_date)?;
        _ser.write_bare::<crate::ton::int>(ttl)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddValidatorPermanentKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let election_date = _de.read_bare::<crate::ton::int>()?;
            let ttl = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { key_hash, election_date, ttl })
        }
    }
}
impl crate::BoxedDeserialize for AddValidatorPermanentKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x92150578)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x92150578) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddValidatorPermanentKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x92150578), self)
    }
}
impl crate::Function for AddValidatorPermanentKey {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.addValidatorTempKey`\n\n```text\nengine.validator.addValidatorTempKey permanent_key_hash:int256 key_hash:int256 ttl:int = engine.validator.Success;\n```\n"]
pub struct AddValidatorTempKey {
    pub permanent_key_hash: crate::ton::int256,
    pub key_hash: crate::ton::int256,
    pub ttl: crate::ton::int,
}
impl Eq for AddValidatorTempKey {}
impl crate::BareSerialize for AddValidatorTempKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8d336f32)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AddValidatorTempKey { permanent_key_hash, key_hash, ttl } = self;
        _ser.write_bare::<crate::ton::int256>(permanent_key_hash)?;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::int>(ttl)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AddValidatorTempKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let permanent_key_hash = _de.read_bare::<crate::ton::int256>()?;
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let ttl = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { permanent_key_hash, key_hash, ttl })
        }
    }
}
impl crate::BoxedDeserialize for AddValidatorTempKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8d336f32)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x8d336f32) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for AddValidatorTempKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x8d336f32), self)
    }
}
impl crate::Function for AddValidatorTempKey {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.changeFullNodeAdnlAddress`\n\n```text\nengine.validator.changeFullNodeAdnlAddress adnl_id:int256 = engine.validator.Success;\n```\n"]
pub struct ChangeFullNodeAdnlAddress {
    pub adnl_id: crate::ton::int256,
}
impl Eq for ChangeFullNodeAdnlAddress {}
impl crate::BareSerialize for ChangeFullNodeAdnlAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbec6c985)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ChangeFullNodeAdnlAddress { adnl_id } = self;
        _ser.write_bare::<crate::ton::int256>(adnl_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ChangeFullNodeAdnlAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let adnl_id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { adnl_id })
        }
    }
}
impl crate::BoxedDeserialize for ChangeFullNodeAdnlAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbec6c985)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xbec6c985) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ChangeFullNodeAdnlAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xbec6c985), self)
    }
}
impl crate::Function for ChangeFullNodeAdnlAddress {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.checkDhtServers`\n\n```text\nengine.validator.checkDhtServers id:int256 = engine.validator.DhtServersStatus;\n```\n"]
pub struct CheckDhtServers {
    pub id: crate::ton::int256,
}
impl Eq for CheckDhtServers {}
impl crate::BareSerialize for CheckDhtServers {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd1e420ca)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CheckDhtServers { id } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CheckDhtServers {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for CheckDhtServers {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd1e420ca)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xd1e420ca) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for CheckDhtServers {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xd1e420ca), self)
    }
}
impl crate::Function for CheckDhtServers {
    type Reply = crate::ton::engine::validator::DhtServersStatus;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.controlQuery`\n\n```text\nengine.validator.controlQuery data:bytes = Object;\n```\n"]
pub struct ControlQuery {
    pub data: crate::ton::bytes,
}
impl Eq for ControlQuery {}
impl crate::BareSerialize for ControlQuery {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa476bdc0)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ControlQuery { data } = self;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ControlQuery {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { data })
        }
    }
}
impl crate::BoxedDeserialize for ControlQuery {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa476bdc0)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xa476bdc0) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ControlQuery {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xa476bdc0), self)
    }
}
impl crate::Function for ControlQuery {
    type Reply = crate::ton::Object;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.createComplaintVote`\n\n```text\nengine.validator.createComplaintVote election_id:int vote:bytes = engine.validator.ProposalVote;\n```\n"]
pub struct CreateComplaintVote {
    pub election_id: crate::ton::int,
    pub vote: crate::ton::bytes,
}
impl Eq for CreateComplaintVote {}
impl crate::BareSerialize for CreateComplaintVote {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb083ff2a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CreateComplaintVote { election_id, vote } = self;
        _ser.write_bare::<crate::ton::int>(election_id)?;
        _ser.write_bare::<crate::ton::bytes>(vote)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CreateComplaintVote {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let election_id = _de.read_bare::<crate::ton::int>()?;
            let vote = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { election_id, vote })
        }
    }
}
impl crate::BoxedDeserialize for CreateComplaintVote {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb083ff2a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xb083ff2a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for CreateComplaintVote {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xb083ff2a), self)
    }
}
impl crate::Function for CreateComplaintVote {
    type Reply = crate::ton::engine::validator::ProposalVote;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.createElectionBid`\n\n```text\nengine.validator.createElectionBid election_date:int election_addr:string wallet:string = engine.validator.ElectionBid;\n```\n"]
pub struct CreateElectionBid {
    pub election_date: crate::ton::int,
    pub election_addr: crate::ton::string,
    pub wallet: crate::ton::string,
}
impl Eq for CreateElectionBid {}
impl crate::BareSerialize for CreateElectionBid {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe51db145)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CreateElectionBid { election_date, election_addr, wallet } = self;
        _ser.write_bare::<crate::ton::int>(election_date)?;
        _ser.write_bare::<crate::ton::string>(election_addr)?;
        _ser.write_bare::<crate::ton::string>(wallet)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CreateElectionBid {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let election_date = _de.read_bare::<crate::ton::int>()?;
            let election_addr = _de.read_bare::<crate::ton::string>()?;
            let wallet = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { election_date, election_addr, wallet })
        }
    }
}
impl crate::BoxedDeserialize for CreateElectionBid {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe51db145)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xe51db145) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for CreateElectionBid {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xe51db145), self)
    }
}
impl crate::Function for CreateElectionBid {
    type Reply = crate::ton::engine::validator::ElectionBid;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.createProposalVote`\n\n```text\nengine.validator.createProposalVote vote:bytes = engine.validator.ProposalVote;\n```\n"]
pub struct CreateProposalVote {
    pub vote: crate::ton::bytes,
}
impl Eq for CreateProposalVote {}
impl crate::BareSerialize for CreateProposalVote {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1db3216d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let CreateProposalVote { vote } = self;
        _ser.write_bare::<crate::ton::bytes>(vote)?;
        Ok(())
    }
}
impl crate::BareDeserialize for CreateProposalVote {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let vote = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { vote })
        }
    }
}
impl crate::BoxedDeserialize for CreateProposalVote {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1db3216d)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x1db3216d) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for CreateProposalVote {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x1db3216d), self)
    }
}
impl crate::Function for CreateProposalVote {
    type Reply = crate::ton::engine::validator::ProposalVote;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.delAdnlId`\n\n```text\nengine.validator.delAdnlId key_hash:int256 = engine.validator.Success;\n```\n"]
pub struct DelAdnlId {
    pub key_hash: crate::ton::int256,
}
impl Eq for DelAdnlId {}
impl crate::BareSerialize for DelAdnlId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x293a74f2)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DelAdnlId { key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DelAdnlId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key_hash })
        }
    }
}
impl crate::BoxedDeserialize for DelAdnlId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x293a74f2)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x293a74f2) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DelAdnlId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x293a74f2), self)
    }
}
impl crate::Function for DelAdnlId {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.delDhtId`\n\n```text\nengine.validator.delDhtId key_hash:int256 = engine.validator.Success;\n```\n"]
pub struct DelDhtId {
    pub key_hash: crate::ton::int256,
}
impl Eq for DelDhtId {}
impl crate::BareSerialize for DelDhtId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x84fd5b3e)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DelDhtId { key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DelDhtId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key_hash })
        }
    }
}
impl crate::BoxedDeserialize for DelDhtId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x84fd5b3e)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x84fd5b3e) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DelDhtId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x84fd5b3e), self)
    }
}
impl crate::Function for DelDhtId {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.delListeningPort`\n\n```text\nengine.validator.delListeningPort ip:int port:int categories:(vector int) priority_categories:(vector int) = engine.validator.Success;\n```\n"]
pub struct DelListeningPort {
    pub ip: crate::ton::int,
    pub port: crate::ton::int,
    pub categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
    pub priority_categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for DelListeningPort {}
impl crate::BareSerialize for DelListeningPort {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x315bb84f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DelListeningPort { ip, port, categories, priority_categories } = self;
        _ser.write_bare::<crate::ton::int>(ip)?;
        _ser.write_bare::<crate::ton::int>(port)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(categories)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(
            priority_categories,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for DelListeningPort {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let ip = _de.read_bare::<crate::ton::int>()?;
            let port = _de.read_bare::<crate::ton::int>()?;
            let categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            let priority_categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { ip, port, categories, priority_categories })
        }
    }
}
impl crate::BoxedDeserialize for DelListeningPort {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x315bb84f)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x315bb84f) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DelListeningPort {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x315bb84f), self)
    }
}
impl crate::Function for DelListeningPort {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.delProxy`\n\n```text\nengine.validator.delProxy out_ip:int out_port:int categories:(vector int) priority_categories:(vector int) = engine.validator.Success;\n```\n"]
pub struct DelProxy {
    pub out_ip: crate::ton::int,
    pub out_port: crate::ton::int,
    pub categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
    pub priority_categories: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for DelProxy {}
impl crate::BareSerialize for DelProxy {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x7578cc7d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DelProxy { out_ip, out_port, categories, priority_categories } = self;
        _ser.write_bare::<crate::ton::int>(out_ip)?;
        _ser.write_bare::<crate::ton::int>(out_port)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(categories)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(
            priority_categories,
        )?;
        Ok(())
    }
}
impl crate::BareDeserialize for DelProxy {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let out_ip = _de.read_bare::<crate::ton::int>()?;
            let out_port = _de.read_bare::<crate::ton::int>()?;
            let categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            let priority_categories =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { out_ip, out_port, categories, priority_categories })
        }
    }
}
impl crate::BoxedDeserialize for DelProxy {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7578cc7d)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x7578cc7d) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DelProxy {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x7578cc7d), self)
    }
}
impl crate::Function for DelProxy {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.delValidatorAdnlAddress`\n\n```text\nengine.validator.delValidatorAdnlAddress permanent_key_hash:int256 key_hash:int256 = engine.validator.Success;\n```\n"]
pub struct DelValidatorAdnlAddress {
    pub permanent_key_hash: crate::ton::int256,
    pub key_hash: crate::ton::int256,
}
impl Eq for DelValidatorAdnlAddress {}
impl crate::BareSerialize for DelValidatorAdnlAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf708435a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DelValidatorAdnlAddress { permanent_key_hash, key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(permanent_key_hash)?;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DelValidatorAdnlAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let permanent_key_hash = _de.read_bare::<crate::ton::int256>()?;
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { permanent_key_hash, key_hash })
        }
    }
}
impl crate::BoxedDeserialize for DelValidatorAdnlAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf708435a)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xf708435a) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DelValidatorAdnlAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xf708435a), self)
    }
}
impl crate::Function for DelValidatorAdnlAddress {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.delValidatorPermanentKey`\n\n```text\nengine.validator.delValidatorPermanentKey key_hash:int256 = engine.validator.Success;\n```\n"]
pub struct DelValidatorPermanentKey {
    pub key_hash: crate::ton::int256,
}
impl Eq for DelValidatorPermanentKey {}
impl crate::BareSerialize for DelValidatorPermanentKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x174ac8fa)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DelValidatorPermanentKey { key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DelValidatorPermanentKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key_hash })
        }
    }
}
impl crate::BoxedDeserialize for DelValidatorPermanentKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x174ac8fa)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x174ac8fa) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DelValidatorPermanentKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x174ac8fa), self)
    }
}
impl crate::Function for DelValidatorPermanentKey {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.delValidatorTempKey`\n\n```text\nengine.validator.delValidatorTempKey permanent_key_hash:int256 key_hash:int256 = engine.validator.Success;\n```\n"]
pub struct DelValidatorTempKey {
    pub permanent_key_hash: crate::ton::int256,
    pub key_hash: crate::ton::int256,
}
impl Eq for DelValidatorTempKey {}
impl crate::BareSerialize for DelValidatorTempKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa0e6e0d1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DelValidatorTempKey { permanent_key_hash, key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(permanent_key_hash)?;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DelValidatorTempKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let permanent_key_hash = _de.read_bare::<crate::ton::int256>()?;
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { permanent_key_hash, key_hash })
        }
    }
}
impl crate::BoxedDeserialize for DelValidatorTempKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa0e6e0d1)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xa0e6e0d1) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for DelValidatorTempKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xa0e6e0d1), self)
    }
}
impl crate::Function for DelValidatorTempKey {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.exportPrivateKey`\n\n```text\nengine.validator.exportPrivateKey key_hash:int256 = PrivateKey;\n```\n"]
pub struct ExportPrivateKey {
    pub key_hash: crate::ton::int256,
}
impl Eq for ExportPrivateKey {}
impl crate::BareSerialize for ExportPrivateKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xcc728048)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportPrivateKey { key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportPrivateKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key_hash })
        }
    }
}
impl crate::BoxedDeserialize for ExportPrivateKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xcc728048)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xcc728048) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ExportPrivateKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xcc728048), self)
    }
}
impl crate::Function for ExportPrivateKey {
    type Reply = crate::ton::PrivateKey;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.exportPublicKey`\n\n```text\nengine.validator.exportPublicKey key_hash:int256 = PublicKey;\n```\n"]
pub struct ExportPublicKey {
    pub key_hash: crate::ton::int256,
}
impl Eq for ExportPublicKey {}
impl crate::BareSerialize for ExportPublicKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x6234a8b9)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ExportPublicKey { key_hash } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ExportPublicKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { key_hash })
        }
    }
}
impl crate::BoxedDeserialize for ExportPublicKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6234a8b9)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x6234a8b9) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ExportPublicKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x6234a8b9), self)
    }
}
impl crate::Function for ExportPublicKey {
    type Reply = crate::ton::PublicKey;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.generateBlsKeyPair`\n\n```text\nengine.validator.generateBlsKeyPair = engine.validator.KeyHash;\n```\n"]
pub struct GenerateBlsKeyPair;
impl Eq for GenerateBlsKeyPair {}
impl crate::BareSerialize for GenerateBlsKeyPair {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5ba28a06)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GenerateBlsKeyPair {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GenerateBlsKeyPair {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5ba28a06)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x5ba28a06) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GenerateBlsKeyPair {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x5ba28a06), self)
    }
}
impl crate::Function for GenerateBlsKeyPair {
    type Reply = crate::ton::engine::validator::KeyHash;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.generateKeyPair`\n\n```text\nengine.validator.generateKeyPair = engine.validator.KeyHash;\n```\n"]
pub struct GenerateKeyPair;
impl Eq for GenerateKeyPair {}
impl crate::BareSerialize for GenerateKeyPair {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xeb25607b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GenerateKeyPair {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GenerateKeyPair {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xeb25607b)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xeb25607b) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GenerateKeyPair {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xeb25607b), self)
    }
}
impl crate::Function for GenerateKeyPair {
    type Reply = crate::ton::engine::validator::KeyHash;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.getBundle`\n\n```text\nengine.validator.getBundle block_id:tonNode.blockIdExt = engine.validator.Success;\n```\n"]
pub struct GetBundle {
    pub block_id: crate::ton::ton_node::blockidext::BlockIdExt,
}
impl Eq for GetBundle {}
impl crate::BareSerialize for GetBundle {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x93343076)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetBundle { block_id } = self;
        _ser.write_bare::<crate::ton::ton_node::blockidext::BlockIdExt>(block_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetBundle {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_id = _de.read_bare::<crate::ton::ton_node::blockidext::BlockIdExt>()?;
            Ok(Self { block_id })
        }
    }
}
impl crate::BoxedDeserialize for GetBundle {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x93343076)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x93343076) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetBundle {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x93343076), self)
    }
}
impl crate::Function for GetBundle {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.getConfig`\n\n```text\nengine.validator.getConfig = engine.validator.JsonConfig;\n```\n"]
pub struct GetConfig;
impl Eq for GetConfig {}
impl crate::BareSerialize for GetConfig {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x59ad2225)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetConfig {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetConfig {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x59ad2225)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x59ad2225) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetConfig {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x59ad2225), self)
    }
}
impl crate::Function for GetConfig {
    type Reply = crate::ton::engine::validator::JsonConfig;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.getFutureBundle`\n\n```text\nengine.validator.getFutureBundle prev_block_ids:(vector tonNode.blockIdExt) = engine.validator.Success;\n```\n"]
pub struct GetFutureBundle {
    pub prev_block_ids:
        crate::ton::vector<crate::ton::Bare, crate::ton::ton_node::blockidext::BlockIdExt>,
}
impl Eq for GetFutureBundle {}
impl crate::BareSerialize for GetFutureBundle {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2834ba00)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetFutureBundle { prev_block_ids } = self;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: ton_node :: blockidext :: BlockIdExt > > (prev_block_ids) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for GetFutureBundle {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let prev_block_ids = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::ton_node::blockidext::BlockIdExt,
            >>()?;
            Ok(Self { prev_block_ids })
        }
    }
}
impl crate::BoxedDeserialize for GetFutureBundle {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2834ba00)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x2834ba00) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetFutureBundle {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x2834ba00), self)
    }
}
impl crate::Function for GetFutureBundle {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.getSelectedStats`\n\n```text\nengine.validator.getSelectedStats filter:string = engine.validator.Stats;\n```\n"]
pub struct GetSelectedStats {
    pub filter: crate::ton::string,
}
impl Eq for GetSelectedStats {}
impl crate::BareSerialize for GetSelectedStats {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xbbda1565)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetSelectedStats { filter } = self;
        _ser.write_bare::<crate::ton::string>(filter)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetSelectedStats {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let filter = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { filter })
        }
    }
}
impl crate::BoxedDeserialize for GetSelectedStats {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbbda1565)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xbbda1565) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetSelectedStats {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xbbda1565), self)
    }
}
impl crate::Function for GetSelectedStats {
    type Reply = crate::ton::engine::validator::Stats;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.getSessionStats`\n\n```text\nengine.validator.getSessionStats = engine.validator.SessionStats;\n```\n"]
pub struct GetSessionStats;
impl Eq for GetSessionStats {}
impl crate::BareSerialize for GetSessionStats {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xcf28e512)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetSessionStats {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetSessionStats {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xcf28e512)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xcf28e512) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetSessionStats {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xcf28e512), self)
    }
}
impl crate::Function for GetSessionStats {
    type Reply = crate::ton::engine::validator::SessionStats;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.getStats`\n\n```text\nengine.validator.getStats = engine.validator.Stats;\n```\n"]
pub struct GetStats;
impl Eq for GetStats {}
impl crate::BareSerialize for GetStats {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x52d5c311)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetStats {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetStats {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x52d5c311)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x52d5c311) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetStats {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x52d5c311), self)
    }
}
impl crate::Function for GetStats {
    type Reply = crate::ton::engine::validator::Stats;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.getTime`\n\n```text\nengine.validator.getTime = engine.validator.Time;\n```\n"]
pub struct GetTime;
impl Eq for GetTime {}
impl crate::BareSerialize for GetTime {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe140bed1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        Ok(())
    }
}
impl crate::BareDeserialize for GetTime {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        Ok(Self {})
    }
}
impl crate::BoxedDeserialize for GetTime {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe140bed1)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xe140bed1) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for GetTime {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xe140bed1), self)
    }
}
impl crate::Function for GetTime {
    type Reply = crate::ton::engine::validator::Time;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.importPrivateKey`\n\n```text\nengine.validator.importPrivateKey key:PrivateKey = engine.validator.KeyHash;\n```\n"]
pub struct ImportPrivateKey {
    pub key: crate::ton::PrivateKey,
}
impl Eq for ImportPrivateKey {}
impl crate::BareSerialize for ImportPrivateKey {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x15807ac7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ImportPrivateKey { key } = self;
        _ser.write_boxed::<crate::ton::PrivateKey>(key)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ImportPrivateKey {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_boxed::<crate::ton::PrivateKey>()?;
            Ok(Self { key })
        }
    }
}
impl crate::BoxedDeserialize for ImportPrivateKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x15807ac7)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x15807ac7) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for ImportPrivateKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x15807ac7), self)
    }
}
impl crate::Function for ImportPrivateKey {
    type Reply = crate::ton::engine::validator::KeyHash;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.setStatesGcInterval`\n\n```text\nengine.validator.setStatesGcInterval interval_ms:int = engine.validator.Success;\n```\n"]
pub struct SetStatesGcInterval {
    pub interval_ms: crate::ton::int,
}
impl Eq for SetStatesGcInterval {}
impl crate::BareSerialize for SetStatesGcInterval {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe7cef50b)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SetStatesGcInterval { interval_ms } = self;
        _ser.write_bare::<crate::ton::int>(interval_ms)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SetStatesGcInterval {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let interval_ms = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { interval_ms })
        }
    }
}
impl crate::BoxedDeserialize for SetStatesGcInterval {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe7cef50b)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xe7cef50b) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SetStatesGcInterval {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xe7cef50b), self)
    }
}
impl crate::Function for SetStatesGcInterval {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.setVerbosity`\n\n```text\nengine.validator.setVerbosity verbosity:int = engine.validator.Success;\n```\n"]
pub struct SetVerbosity {
    pub verbosity: crate::ton::int,
}
impl Eq for SetVerbosity {}
impl crate::BareSerialize for SetVerbosity {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb1825e82)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SetVerbosity { verbosity } = self;
        _ser.write_bare::<crate::ton::int>(verbosity)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SetVerbosity {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let verbosity = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { verbosity })
        }
    }
}
impl crate::BoxedDeserialize for SetVerbosity {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb1825e82)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xb1825e82) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for SetVerbosity {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xb1825e82), self)
    }
}
impl crate::Function for SetVerbosity {
    type Reply = crate::ton::engine::validator::Success;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.sign`\n\n```text\nengine.validator.sign key_hash:int256 data:bytes = engine.validator.Signature;\n```\n"]
pub struct Sign {
    pub key_hash: crate::ton::int256,
    pub data: crate::ton::bytes,
}
impl Eq for Sign {}
impl crate::BareSerialize for Sign {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1aea1a28)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Sign { key_hash, data } = self;
        _ser.write_bare::<crate::ton::int256>(key_hash)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Sign {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key_hash = _de.read_bare::<crate::ton::int256>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { key_hash, data })
        }
    }
}
impl crate::BoxedDeserialize for Sign {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1aea1a28)]
    }

    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x1aea1a28) { de.read_bare() } else { _invalid_id!(id) }
    }
}
impl crate::BoxedSerialize for Sign {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x1aea1a28), self)
    }
}
impl crate::Function for Sign {
    type Reply = crate::ton::engine::validator::Signature;
}
