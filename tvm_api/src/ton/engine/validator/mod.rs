use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.Config`\n\n```text\nengine.validator.config out_port:int addrs:(vector engine.Addr) adnl:(vector engine.adnl) \n        dht:(vector engine.dht)\n        validators:(vector engine.validator) fullnode:int256 fullnodeslaves:(vector engine.validator.fullNodeSlave)\n        fullnodemasters:(vector engine.validator.fullNodeMaster)\n        liteservers:(vector engine.liteServer) control:(vector engine.controlInterface)\n        gc:engine.gc = engine.validator.Config;\n```\n"]
pub enum Config {
    Engine_Validator_Config(crate::ton::engine::validator::config::Config),
}
impl Config {
    pub fn addrs(&self) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::engine::Addr> {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.addrs,
        }
    }

    pub fn adnl(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::adnl::Adnl> {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.adnl,
        }
    }

    pub fn control(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::controlinterface::ControlInterface>
    {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.control,
        }
    }

    pub fn dht(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::dht::Dht> {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.dht,
        }
    }

    pub fn fullnode(&self) -> &crate::ton::int256 {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.fullnode,
        }
    }

    pub fn fullnodemasters(
        &self,
    ) -> &crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::fullnodemaster::FullNodeMaster,
    > {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.fullnodemasters,
        }
    }

    pub fn fullnodeslaves(
        &self,
    ) -> &crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::fullnodeslave::FullNodeSlave,
    > {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.fullnodeslaves,
        }
    }

    pub fn gc(&self) -> &crate::ton::engine::gc::Gc {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.gc,
        }
    }

    pub fn liteservers(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::liteserver::LiteServer> {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.liteservers,
        }
    }

    pub fn out_port(&self) -> &crate::ton::int {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.out_port,
        }
    }

    pub fn validators(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::validator::Validator> {
        match self {
            Config::Engine_Validator_Config(ref x) => &x.validators,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::config::Config {
        match self {
            Config::Engine_Validator_Config(x) => x,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::Engine_Validator_Config(crate::ton::engine::validator::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::Engine_Validator_Config(x) => (crate::ConstructorNumber(0xcec219a4), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xcec219a4)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xcec219a4) => Ok(Config::Engine_Validator_Config(
                _de.read_bare::<crate::ton::engine::validator::config::Config>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.ControlQueryError`\n\n```text\nengine.validator.controlQueryError code:int message:string = engine.validator.ControlQueryError;\n```\n"]
pub enum ControlQueryError {
    Engine_Validator_ControlQueryError(
        crate::ton::engine::validator::controlqueryerror::ControlQueryError,
    ),
}
impl ControlQueryError {
    pub fn code(&self) -> &crate::ton::int {
        match self {
            ControlQueryError::Engine_Validator_ControlQueryError(ref x) => &x.code,
        }
    }

    pub fn message(&self) -> &crate::ton::string {
        match self {
            ControlQueryError::Engine_Validator_ControlQueryError(ref x) => &x.message,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::controlqueryerror::ControlQueryError {
        match self {
            ControlQueryError::Engine_Validator_ControlQueryError(x) => x,
        }
    }
}
impl Eq for ControlQueryError {}
impl Default for ControlQueryError {
    fn default() -> Self {
        ControlQueryError::Engine_Validator_ControlQueryError(
            crate::ton::engine::validator::controlqueryerror::ControlQueryError::default(),
        )
    }
}
impl crate::BoxedSerialize for ControlQueryError {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ControlQueryError::Engine_Validator_ControlQueryError(x) => {
                (crate::ConstructorNumber(0x77269a1f), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ControlQueryError {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x77269a1f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x77269a1f) => Ok (ControlQueryError :: Engine_Validator_ControlQueryError (_de . read_bare :: < crate :: ton :: engine :: validator :: controlqueryerror :: ControlQueryError > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.DhtServerStatus`\n\n```text\nengine.validator.dhtServerStatus id:int256 status:int = engine.validator.DhtServerStatus;\n```\n"]
pub enum DhtServerStatus {
    Engine_Validator_DhtServerStatus(
        crate::ton::engine::validator::dhtserverstatus::DhtServerStatus,
    ),
}
impl DhtServerStatus {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            DhtServerStatus::Engine_Validator_DhtServerStatus(ref x) => &x.id,
        }
    }

    pub fn status(&self) -> &crate::ton::int {
        match self {
            DhtServerStatus::Engine_Validator_DhtServerStatus(ref x) => &x.status,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::dhtserverstatus::DhtServerStatus {
        match self {
            DhtServerStatus::Engine_Validator_DhtServerStatus(x) => x,
        }
    }
}
impl Eq for DhtServerStatus {}
impl Default for DhtServerStatus {
    fn default() -> Self {
        DhtServerStatus::Engine_Validator_DhtServerStatus(
            crate::ton::engine::validator::dhtserverstatus::DhtServerStatus::default(),
        )
    }
}
impl crate::BoxedSerialize for DhtServerStatus {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DhtServerStatus::Engine_Validator_DhtServerStatus(x) => {
                (crate::ConstructorNumber(0xb11de75e), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for DhtServerStatus {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb11de75e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0xb11de75e) => Ok (DhtServerStatus :: Engine_Validator_DhtServerStatus (_de . read_bare :: < crate :: ton :: engine :: validator :: dhtserverstatus :: DhtServerStatus > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.DhtServersStatus`\n\n```text\nengine.validator.dhtServersStatus servers:(vector engine.validator.dhtServerStatus) = engine.validator.DhtServersStatus;\n```\n"]
pub enum DhtServersStatus {
    Engine_Validator_DhtServersStatus(
        crate::ton::engine::validator::dhtserversstatus::DhtServersStatus,
    ),
}
impl DhtServersStatus {
    pub fn servers(
        &self,
    ) -> &crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::dhtserverstatus::DhtServerStatus,
    > {
        match self {
            DhtServersStatus::Engine_Validator_DhtServersStatus(ref x) => &x.servers,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::dhtserversstatus::DhtServersStatus {
        match self {
            DhtServersStatus::Engine_Validator_DhtServersStatus(x) => x,
        }
    }
}
impl Eq for DhtServersStatus {}
impl Default for DhtServersStatus {
    fn default() -> Self {
        DhtServersStatus::Engine_Validator_DhtServersStatus(
            crate::ton::engine::validator::dhtserversstatus::DhtServersStatus::default(),
        )
    }
}
impl crate::BoxedSerialize for DhtServersStatus {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            DhtServersStatus::Engine_Validator_DhtServersStatus(x) => {
                (crate::ConstructorNumber(0x2b38fd28), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for DhtServersStatus {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2b38fd28)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x2b38fd28) => Ok (DhtServersStatus :: Engine_Validator_DhtServersStatus (_de . read_bare :: < crate :: ton :: engine :: validator :: dhtserversstatus :: DhtServersStatus > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.ElectionBid`\n\n```text\nengine.validator.electionBid election_date:int perm_key:int256 adnl_addr:int256 to_send_payload:bytes = engine.validator.ElectionBid;\n```\n"]
pub enum ElectionBid {
    Engine_Validator_ElectionBid(crate::ton::engine::validator::electionbid::ElectionBid),
}
impl ElectionBid {
    pub fn adnl_addr(&self) -> &crate::ton::int256 {
        match self {
            ElectionBid::Engine_Validator_ElectionBid(ref x) => &x.adnl_addr,
        }
    }

    pub fn election_date(&self) -> &crate::ton::int {
        match self {
            ElectionBid::Engine_Validator_ElectionBid(ref x) => &x.election_date,
        }
    }

    pub fn perm_key(&self) -> &crate::ton::int256 {
        match self {
            ElectionBid::Engine_Validator_ElectionBid(ref x) => &x.perm_key,
        }
    }

    pub fn to_send_payload(&self) -> &crate::ton::bytes {
        match self {
            ElectionBid::Engine_Validator_ElectionBid(ref x) => &x.to_send_payload,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::electionbid::ElectionBid {
        match self {
            ElectionBid::Engine_Validator_ElectionBid(x) => x,
        }
    }
}
impl Eq for ElectionBid {}
impl Default for ElectionBid {
    fn default() -> Self {
        ElectionBid::Engine_Validator_ElectionBid(
            crate::ton::engine::validator::electionbid::ElectionBid::default(),
        )
    }
}
impl crate::BoxedSerialize for ElectionBid {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ElectionBid::Engine_Validator_ElectionBid(x) => {
                (crate::ConstructorNumber(0x23b27a3d), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ElectionBid {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x23b27a3d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x23b27a3d) => Ok(ElectionBid::Engine_Validator_ElectionBid(
                _de.read_bare::<crate::ton::engine::validator::electionbid::ElectionBid>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.FullNodeMaster`\n\n```text\nengine.validator.fullNodeMaster port:int adnl:int256 = engine.validator.FullNodeMaster;\n```\n"]
pub enum FullNodeMaster {
    Engine_Validator_FullNodeMaster(crate::ton::engine::validator::fullnodemaster::FullNodeMaster),
}
impl FullNodeMaster {
    pub fn adnl(&self) -> &crate::ton::int256 {
        match self {
            FullNodeMaster::Engine_Validator_FullNodeMaster(ref x) => &x.adnl,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            FullNodeMaster::Engine_Validator_FullNodeMaster(ref x) => &x.port,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::fullnodemaster::FullNodeMaster {
        match self {
            FullNodeMaster::Engine_Validator_FullNodeMaster(x) => x,
        }
    }
}
impl Eq for FullNodeMaster {}
impl Default for FullNodeMaster {
    fn default() -> Self {
        FullNodeMaster::Engine_Validator_FullNodeMaster(
            crate::ton::engine::validator::fullnodemaster::FullNodeMaster::default(),
        )
    }
}
impl crate::BoxedSerialize for FullNodeMaster {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            FullNodeMaster::Engine_Validator_FullNodeMaster(x) => {
                (crate::ConstructorNumber(0x8485f668), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for FullNodeMaster {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8485f668)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8485f668) => {
                Ok(FullNodeMaster::Engine_Validator_FullNodeMaster(
                    _de.read_bare::<crate::ton::engine::validator::fullnodemaster::FullNodeMaster>(
                    )?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.FullNodeSlave`\n\n```text\nengine.validator.fullNodeSlave ip:int port:int adnl:PublicKey = engine.validator.FullNodeSlave;\n```\n"]
pub enum FullNodeSlave {
    Engine_Validator_FullNodeSlave(crate::ton::engine::validator::fullnodeslave::FullNodeSlave),
}
impl FullNodeSlave {
    pub fn adnl(&self) -> &crate::ton::PublicKey {
        match self {
            FullNodeSlave::Engine_Validator_FullNodeSlave(ref x) => &x.adnl,
        }
    }

    pub fn ip(&self) -> &crate::ton::int {
        match self {
            FullNodeSlave::Engine_Validator_FullNodeSlave(ref x) => &x.ip,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            FullNodeSlave::Engine_Validator_FullNodeSlave(ref x) => &x.port,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::fullnodeslave::FullNodeSlave {
        match self {
            FullNodeSlave::Engine_Validator_FullNodeSlave(x) => x,
        }
    }
}
impl Eq for FullNodeSlave {}
impl Default for FullNodeSlave {
    fn default() -> Self {
        FullNodeSlave::Engine_Validator_FullNodeSlave(
            crate::ton::engine::validator::fullnodeslave::FullNodeSlave::default(),
        )
    }
}
impl crate::BoxedSerialize for FullNodeSlave {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            FullNodeSlave::Engine_Validator_FullNodeSlave(x) => {
                (crate::ConstructorNumber(0x88256b79), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for FullNodeSlave {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x88256b79)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x88256b79) => {
                Ok(FullNodeSlave::Engine_Validator_FullNodeSlave(
                    _de.read_bare::<crate::ton::engine::validator::fullnodeslave::FullNodeSlave>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.GroupMember`\n\n```text\nvalidator.groupMember public_key_hash:int256 adnl:int256 weight:long = engine.validator.GroupMember;\n```\n"]
pub enum GroupMember {
    Validator_GroupMember(crate::ton::engine::validator::validator::groupmember::GroupMember),
}
impl GroupMember {
    pub fn adnl(&self) -> &crate::ton::int256 {
        match self {
            GroupMember::Validator_GroupMember(ref x) => &x.adnl,
        }
    }

    pub fn public_key_hash(&self) -> &crate::ton::int256 {
        match self {
            GroupMember::Validator_GroupMember(ref x) => &x.public_key_hash,
        }
    }

    pub fn weight(&self) -> &crate::ton::long {
        match self {
            GroupMember::Validator_GroupMember(ref x) => &x.weight,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::validator::groupmember::GroupMember {
        match self {
            GroupMember::Validator_GroupMember(x) => x,
        }
    }
}
impl Eq for GroupMember {}
impl Default for GroupMember {
    fn default() -> Self {
        GroupMember::Validator_GroupMember(
            crate::ton::engine::validator::validator::groupmember::GroupMember::default(),
        )
    }
}
impl crate::BoxedSerialize for GroupMember {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            GroupMember::Validator_GroupMember(x) => (crate::ConstructorNumber(0x8b9465e4), x),
        }
    }
}
impl crate::BoxedDeserialize for GroupMember {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8b9465e4)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x8b9465e4) => Ok (GroupMember :: Validator_GroupMember (_de . read_bare :: < crate :: ton :: engine :: validator :: validator :: groupmember :: GroupMember > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.JsonConfig`\n\n```text\nengine.validator.jsonConfig data:string = engine.validator.JsonConfig;\n```\n"]
pub enum JsonConfig {
    Engine_Validator_JsonConfig(crate::ton::engine::validator::jsonconfig::JsonConfig),
}
impl JsonConfig {
    pub fn data(&self) -> &crate::ton::string {
        match self {
            JsonConfig::Engine_Validator_JsonConfig(ref x) => &x.data,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::jsonconfig::JsonConfig {
        match self {
            JsonConfig::Engine_Validator_JsonConfig(x) => x,
        }
    }
}
impl Eq for JsonConfig {}
impl Default for JsonConfig {
    fn default() -> Self {
        JsonConfig::Engine_Validator_JsonConfig(
            crate::ton::engine::validator::jsonconfig::JsonConfig::default(),
        )
    }
}
impl crate::BoxedSerialize for JsonConfig {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            JsonConfig::Engine_Validator_JsonConfig(x) => (crate::ConstructorNumber(0x132d920b), x),
        }
    }
}
impl crate::BoxedDeserialize for JsonConfig {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x132d920b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x132d920b) => Ok(JsonConfig::Engine_Validator_JsonConfig(
                _de.read_bare::<crate::ton::engine::validator::jsonconfig::JsonConfig>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.KeyHash`\n\n```text\nengine.validator.keyHash key_hash:int256 = engine.validator.KeyHash;\n```\n"]
pub enum KeyHash {
    Engine_Validator_KeyHash(crate::ton::engine::validator::keyhash::KeyHash),
}
impl KeyHash {
    pub fn key_hash(&self) -> &crate::ton::int256 {
        match self {
            KeyHash::Engine_Validator_KeyHash(ref x) => &x.key_hash,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::keyhash::KeyHash {
        match self {
            KeyHash::Engine_Validator_KeyHash(x) => x,
        }
    }
}
impl Eq for KeyHash {}
impl Default for KeyHash {
    fn default() -> Self {
        KeyHash::Engine_Validator_KeyHash(crate::ton::engine::validator::keyhash::KeyHash::default())
    }
}
impl crate::BoxedSerialize for KeyHash {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            KeyHash::Engine_Validator_KeyHash(x) => (crate::ConstructorNumber(0xc2c6a54e), x),
        }
    }
}
impl crate::BoxedDeserialize for KeyHash {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc2c6a54e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xc2c6a54e) => Ok(KeyHash::Engine_Validator_KeyHash(
                _de.read_bare::<crate::ton::engine::validator::keyhash::KeyHash>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.OneStat`\n\n```text\nengine.validator.oneStat key:string value:string = engine.validator.OneStat;\n```\n"]
pub enum OneStat {
    Engine_Validator_OneStat(crate::ton::engine::validator::onestat::OneStat),
}
impl OneStat {
    pub fn key(&self) -> &crate::ton::string {
        match self {
            OneStat::Engine_Validator_OneStat(ref x) => &x.key,
        }
    }

    pub fn value(&self) -> &crate::ton::string {
        match self {
            OneStat::Engine_Validator_OneStat(ref x) => &x.value,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::onestat::OneStat {
        match self {
            OneStat::Engine_Validator_OneStat(x) => x,
        }
    }
}
impl Eq for OneStat {}
impl Default for OneStat {
    fn default() -> Self {
        OneStat::Engine_Validator_OneStat(crate::ton::engine::validator::onestat::OneStat::default())
    }
}
impl crate::BoxedSerialize for OneStat {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            OneStat::Engine_Validator_OneStat(x) => (crate::ConstructorNumber(0xa4983aed), x),
        }
    }
}
impl crate::BoxedDeserialize for OneStat {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa4983aed)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa4983aed) => Ok(OneStat::Engine_Validator_OneStat(
                _de.read_bare::<crate::ton::engine::validator::onestat::OneStat>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.ProposalVote`\n\n```text\nengine.validator.proposalVote perm_key:int256 to_send:bytes = engine.validator.ProposalVote;\n```\n"]
pub enum ProposalVote {
    Engine_Validator_ProposalVote(crate::ton::engine::validator::proposalvote::ProposalVote),
}
impl ProposalVote {
    pub fn perm_key(&self) -> &crate::ton::int256 {
        match self {
            ProposalVote::Engine_Validator_ProposalVote(ref x) => &x.perm_key,
        }
    }

    pub fn to_send(&self) -> &crate::ton::bytes {
        match self {
            ProposalVote::Engine_Validator_ProposalVote(ref x) => &x.to_send,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::proposalvote::ProposalVote {
        match self {
            ProposalVote::Engine_Validator_ProposalVote(x) => x,
        }
    }
}
impl Eq for ProposalVote {}
impl Default for ProposalVote {
    fn default() -> Self {
        ProposalVote::Engine_Validator_ProposalVote(
            crate::ton::engine::validator::proposalvote::ProposalVote::default(),
        )
    }
}
impl crate::BoxedSerialize for ProposalVote {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ProposalVote::Engine_Validator_ProposalVote(x) => {
                (crate::ConstructorNumber(0x7f6626ed), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ProposalVote {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x7f6626ed)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x7f6626ed) => {
                Ok(ProposalVote::Engine_Validator_ProposalVote(
                    _de.read_bare::<crate::ton::engine::validator::proposalvote::ProposalVote>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.SessionStats`\n\n```text\nengine.validator.sessionStats stats:(vector engine.validator.oneSessionStat) = engine.validator.SessionStats;\n```\n"]
pub enum SessionStats {
    Engine_Validator_SessionStats(crate::ton::engine::validator::sessionstats::SessionStats),
}
impl SessionStats {
    pub fn stats(
        &self,
    ) -> &crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validator::onesessionstat::OneSessionStat,
    > {
        match self {
            SessionStats::Engine_Validator_SessionStats(ref x) => &x.stats,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::sessionstats::SessionStats {
        match self {
            SessionStats::Engine_Validator_SessionStats(x) => x,
        }
    }
}
impl Eq for SessionStats {}
impl Default for SessionStats {
    fn default() -> Self {
        SessionStats::Engine_Validator_SessionStats(
            crate::ton::engine::validator::sessionstats::SessionStats::default(),
        )
    }
}
impl crate::BoxedSerialize for SessionStats {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            SessionStats::Engine_Validator_SessionStats(x) => {
                (crate::ConstructorNumber(0x8a0adbde), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for SessionStats {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8a0adbde)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8a0adbde) => {
                Ok(SessionStats::Engine_Validator_SessionStats(
                    _de.read_bare::<crate::ton::engine::validator::sessionstats::SessionStats>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.Signature`\n\n```text\nengine.validator.signature signature:bytes = engine.validator.Signature;\n```\n"]
pub enum Signature {
    Engine_Validator_Signature(crate::ton::engine::validator::signature::Signature),
}
impl Signature {
    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            Signature::Engine_Validator_Signature(ref x) => &x.signature,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::signature::Signature {
        match self {
            Signature::Engine_Validator_Signature(x) => x,
        }
    }
}
impl Eq for Signature {}
impl Default for Signature {
    fn default() -> Self {
        Signature::Engine_Validator_Signature(
            crate::ton::engine::validator::signature::Signature::default(),
        )
    }
}
impl crate::BoxedSerialize for Signature {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Signature::Engine_Validator_Signature(x) => (crate::ConstructorNumber(0xfb6c4328), x),
        }
    }
}
impl crate::BoxedDeserialize for Signature {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xfb6c4328)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xfb6c4328) => Ok(Signature::Engine_Validator_Signature(
                _de.read_bare::<crate::ton::engine::validator::signature::Signature>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.Stats`\n\n```text\nengine.validator.stats stats:(vector engine.validator.oneStat) = engine.validator.Stats;\n```\n"]
pub enum Stats {
    Engine_Validator_Stats(crate::ton::engine::validator::stats::Stats),
}
impl Stats {
    pub fn stats(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::validator::onestat::OneStat>
    {
        match self {
            Stats::Engine_Validator_Stats(ref x) => &x.stats,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::stats::Stats {
        match self {
            Stats::Engine_Validator_Stats(x) => x,
        }
    }
}
impl Eq for Stats {}
impl Default for Stats {
    fn default() -> Self {
        Stats::Engine_Validator_Stats(crate::ton::engine::validator::stats::Stats::default())
    }
}
impl crate::BoxedSerialize for Stats {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Stats::Engine_Validator_Stats(x) => (crate::ConstructorNumber(0x5d49d36f), x),
        }
    }
}
impl crate::BoxedDeserialize for Stats {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5d49d36f)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5d49d36f) => Ok(Stats::Engine_Validator_Stats(
                _de.read_bare::<crate::ton::engine::validator::stats::Stats>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `engine.validator.Success`\n\n```text\nengine.validator.success = engine.validator.Success;\n```\n"]
pub enum Success {
    #[default]
    Engine_Validator_Success,
}
impl Eq for Success {}
impl crate::BoxedSerialize for Success {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Success::Engine_Validator_Success => (crate::ConstructorNumber(0xb3e4a68b), &()),
        }
    }
}
impl crate::BoxedDeserialize for Success {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb3e4a68b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xb3e4a68b) => Ok(Success::Engine_Validator_Success),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.Time`\n\n```text\nengine.validator.time time:int = engine.validator.Time;\n```\n"]
pub enum Time {
    Engine_Validator_Time(crate::ton::engine::validator::time::Time),
}
impl Time {
    pub fn time(&self) -> &crate::ton::int {
        match self {
            Time::Engine_Validator_Time(ref x) => &x.time,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::time::Time {
        match self {
            Time::Engine_Validator_Time(x) => x,
        }
    }
}
impl Eq for Time {}
impl Default for Time {
    fn default() -> Self {
        Time::Engine_Validator_Time(crate::ton::engine::validator::time::Time::default())
    }
}
impl crate::BoxedSerialize for Time {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Time::Engine_Validator_Time(x) => (crate::ConstructorNumber(0xdf5fa1fe), x),
        }
    }
}
impl crate::BoxedDeserialize for Time {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdf5fa1fe)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xdf5fa1fe) => Ok(Time::Engine_Validator_Time(
                _de.read_bare::<crate::ton::engine::validator::time::Time>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator`\n\n```text\nengine.validator id:int256 temp_keys:(vector engine.validatorTempKey) adnl_addrs:(vector engine.validatorAdnlAddress) election_date:int expire_at:int = engine.Validator;\n```\n"]
pub struct Validator {
    pub id: crate::ton::int256,
    pub temp_keys: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validatortempkey::ValidatorTempKey,
    >,
    pub adnl_addrs: crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validatoradnladdress::ValidatorAdnlAddress,
    >,
    pub election_date: crate::ton::int,
    pub expire_at: crate::ton::int,
}
impl Eq for Validator {}
impl crate::BareSerialize for Validator {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x885fea29)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Validator { id, temp_keys, adnl_addrs, election_date, expire_at } = self;
        _ser.write_bare::<crate::ton::int256>(id)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::validatortempkey::ValidatorTempKey,
        >>(temp_keys)?;
        _ser.write_bare::<crate::ton::vector<
            crate::ton::Bare,
            crate::ton::engine::validatoradnladdress::ValidatorAdnlAddress,
        >>(adnl_addrs)?;
        _ser.write_bare::<crate::ton::int>(election_date)?;
        _ser.write_bare::<crate::ton::int>(expire_at)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Validator {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int256>()?;
            let temp_keys = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validatortempkey::ValidatorTempKey,
            >>()?;
            let adnl_addrs = _de.read_bare::<crate::ton::vector<
                crate::ton::Bare,
                crate::ton::engine::validatoradnladdress::ValidatorAdnlAddress,
            >>()?;
            let election_date = _de.read_bare::<crate::ton::int>()?;
            let expire_at = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, temp_keys, adnl_addrs, election_date, expire_at })
        }
    }
}
impl crate::IntoBoxed for Validator {
    type Boxed = crate::ton::engine::Validator;

    fn into_boxed(self) -> crate::ton::engine::Validator {
        crate::ton::engine::Validator::Engine_Validator(self)
    }
}
pub mod config;
pub mod controlqueryerror;
pub mod dhtserversstatus;
pub mod dhtserverstatus;
pub mod electionbid;
pub mod fullnodemaster;
pub mod fullnodeslave;
pub mod jsonconfig;
pub mod keyhash;
pub mod onesessionstat;
pub mod onestat;
pub mod proposalvote;
pub mod sessionstats;
pub mod signature;
pub mod stats;
pub mod success;
pub mod time;
pub mod validator;
