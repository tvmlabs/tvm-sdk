use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.Addr`\n\n```text\nengine.addr ip:int port:int categories:(vector int) priority_categories:(vector int) = engine.Addr;\n\nengine.addrProxy in_ip:int in_port:int out_ip:int out_port:int \n          proxy_type:adnl.Proxy categories:(vector int) priority_categories:(vector int) = engine.Addr;\n```\n"]
pub enum Addr {
    Engine_Addr(crate::ton::engine::addr::Addr),
    Engine_AddrProxy(crate::ton::engine::addr::AddrProxy),
}
impl Addr {
    pub fn categories(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int> {
        match self {
            Addr::Engine_Addr(ref x) => &x.categories,
            Addr::Engine_AddrProxy(ref x) => &x.categories,
        }
    }

    pub fn in_ip(&self) -> Option<&crate::ton::int> {
        match self {
            Addr::Engine_AddrProxy(ref x) => Some(&x.in_ip),
            _ => None,
        }
    }

    pub fn in_port(&self) -> Option<&crate::ton::int> {
        match self {
            Addr::Engine_AddrProxy(ref x) => Some(&x.in_port),
            _ => None,
        }
    }

    pub fn ip(&self) -> Option<&crate::ton::int> {
        match self {
            Addr::Engine_Addr(ref x) => Some(&x.ip),
            _ => None,
        }
    }

    pub fn out_ip(&self) -> Option<&crate::ton::int> {
        match self {
            Addr::Engine_AddrProxy(ref x) => Some(&x.out_ip),
            _ => None,
        }
    }

    pub fn out_port(&self) -> Option<&crate::ton::int> {
        match self {
            Addr::Engine_AddrProxy(ref x) => Some(&x.out_port),
            _ => None,
        }
    }

    pub fn port(&self) -> Option<&crate::ton::int> {
        match self {
            Addr::Engine_Addr(ref x) => Some(&x.port),
            _ => None,
        }
    }

    pub fn priority_categories(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int> {
        match self {
            Addr::Engine_Addr(ref x) => &x.priority_categories,
            Addr::Engine_AddrProxy(ref x) => &x.priority_categories,
        }
    }

    pub fn proxy_type(&self) -> Option<&crate::ton::adnl::Proxy> {
        match self {
            Addr::Engine_AddrProxy(ref x) => Some(&x.proxy_type),
            _ => None,
        }
    }
}
impl Eq for Addr {}
impl Default for Addr {
    fn default() -> Self {
        Addr::Engine_Addr(crate::ton::engine::addr::Addr::default())
    }
}
impl crate::BoxedSerialize for Addr {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Addr::Engine_Addr(x) => (crate::ConstructorNumber(0xef311fec), x),
            Addr::Engine_AddrProxy(x) => (crate::ConstructorNumber(0x8adf6549), x),
        }
    }
}
impl crate::BoxedDeserialize for Addr {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xef311fec), crate::ConstructorNumber(0x8adf6549)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xef311fec) => {
                Ok(Addr::Engine_Addr(_de.read_bare::<crate::ton::engine::addr::Addr>()?))
            }
            crate::ConstructorNumber(0x8adf6549) => {
                Ok(Addr::Engine_AddrProxy(_de.read_bare::<crate::ton::engine::addr::AddrProxy>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.Adnl`\n\n```text\nengine.adnl id:int256 category:int = engine.Adnl;\n```\n"]
pub enum Adnl {
    Engine_Adnl(crate::ton::engine::adnl::Adnl),
}
impl Adnl {
    pub fn category(&self) -> &crate::ton::int {
        match self {
            Adnl::Engine_Adnl(ref x) => &x.category,
        }
    }

    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            Adnl::Engine_Adnl(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::engine::adnl::Adnl {
        match self {
            Adnl::Engine_Adnl(x) => x,
        }
    }
}
impl Eq for Adnl {}
impl Default for Adnl {
    fn default() -> Self {
        Adnl::Engine_Adnl(crate::ton::engine::adnl::Adnl::default())
    }
}
impl crate::BoxedSerialize for Adnl {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Adnl::Engine_Adnl(x) => (crate::ConstructorNumber(0x62d76550), x),
        }
    }
}
impl crate::BoxedDeserialize for Adnl {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x62d76550)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x62d76550) => {
                Ok(Adnl::Engine_Adnl(_de.read_bare::<crate::ton::engine::adnl::Adnl>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.ControlInterface`\n\n```text\nengine.controlInterface id:int256 port:int allowed:(vector engine.controlProcess) = engine.ControlInterface;\n```\n"]
pub enum ControlInterface {
    Engine_ControlInterface(crate::ton::engine::controlinterface::ControlInterface),
}
impl ControlInterface {
    pub fn allowed(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::controlprocess::ControlProcess>
    {
        match self {
            ControlInterface::Engine_ControlInterface(ref x) => &x.allowed,
        }
    }

    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            ControlInterface::Engine_ControlInterface(ref x) => &x.id,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            ControlInterface::Engine_ControlInterface(ref x) => &x.port,
        }
    }

    pub fn only(self) -> crate::ton::engine::controlinterface::ControlInterface {
        match self {
            ControlInterface::Engine_ControlInterface(x) => x,
        }
    }
}
impl Eq for ControlInterface {}
impl Default for ControlInterface {
    fn default() -> Self {
        ControlInterface::Engine_ControlInterface(
            crate::ton::engine::controlinterface::ControlInterface::default(),
        )
    }
}
impl crate::BoxedSerialize for ControlInterface {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ControlInterface::Engine_ControlInterface(x) => {
                (crate::ConstructorNumber(0x31816fab), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ControlInterface {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x31816fab)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x31816fab) => Ok(ControlInterface::Engine_ControlInterface(
                _de.read_bare::<crate::ton::engine::controlinterface::ControlInterface>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.ControlProcess`\n\n```text\nengine.controlProcess id:int256 permissions:int = engine.ControlProcess;\n```\n"]
pub enum ControlProcess {
    Engine_ControlProcess(crate::ton::engine::controlprocess::ControlProcess),
}
impl ControlProcess {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            ControlProcess::Engine_ControlProcess(ref x) => &x.id,
        }
    }

    pub fn permissions(&self) -> &crate::ton::int {
        match self {
            ControlProcess::Engine_ControlProcess(ref x) => &x.permissions,
        }
    }

    pub fn only(self) -> crate::ton::engine::controlprocess::ControlProcess {
        match self {
            ControlProcess::Engine_ControlProcess(x) => x,
        }
    }
}
impl Eq for ControlProcess {}
impl Default for ControlProcess {
    fn default() -> Self {
        ControlProcess::Engine_ControlProcess(
            crate::ton::engine::controlprocess::ControlProcess::default(),
        )
    }
}
impl crate::BoxedSerialize for ControlProcess {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ControlProcess::Engine_ControlProcess(x) => (crate::ConstructorNumber(0x6ac04817), x),
        }
    }
}
impl crate::BoxedDeserialize for ControlProcess {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x6ac04817)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x6ac04817) => Ok(ControlProcess::Engine_ControlProcess(
                _de.read_bare::<crate::ton::engine::controlprocess::ControlProcess>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.Dht`\n\n```text\nengine.dht id:int256 = engine.Dht;\n```\n"]
pub enum Dht {
    Engine_Dht(crate::ton::engine::dht::Dht),
}
impl Dht {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            Dht::Engine_Dht(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::engine::dht::Dht {
        match self {
            Dht::Engine_Dht(x) => x,
        }
    }
}
impl Eq for Dht {}
impl Default for Dht {
    fn default() -> Self {
        Dht::Engine_Dht(crate::ton::engine::dht::Dht::default())
    }
}
impl crate::BoxedSerialize for Dht {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Dht::Engine_Dht(x) => (crate::ConstructorNumber(0x5de9f2fa), x),
        }
    }
}
impl crate::BoxedDeserialize for Dht {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5de9f2fa)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5de9f2fa) => {
                Ok(Dht::Engine_Dht(_de.read_bare::<crate::ton::engine::dht::Dht>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.Gc`\n\n```text\nengine.gc ids:(vector int256) = engine.Gc;\n```\n"]
pub enum Gc {
    Engine_Gc(crate::ton::engine::gc::Gc),
}
impl Gc {
    pub fn ids(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::int256> {
        match self {
            Gc::Engine_Gc(ref x) => &x.ids,
        }
    }

    pub fn only(self) -> crate::ton::engine::gc::Gc {
        match self {
            Gc::Engine_Gc(x) => x,
        }
    }
}
impl Eq for Gc {}
impl Default for Gc {
    fn default() -> Self {
        Gc::Engine_Gc(crate::ton::engine::gc::Gc::default())
    }
}
impl crate::BoxedSerialize for Gc {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Gc::Engine_Gc(x) => (crate::ConstructorNumber(0xbfbd987b), x),
        }
    }
}
impl crate::BoxedDeserialize for Gc {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbfbd987b)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbfbd987b) => {
                Ok(Gc::Engine_Gc(_de.read_bare::<crate::ton::engine::gc::Gc>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.LiteServer`\n\n```text\nengine.liteServer id:int256 port:int = engine.LiteServer;\n```\n"]
pub enum LiteServer {
    Engine_LiteServer(crate::ton::engine::liteserver::LiteServer),
}
impl LiteServer {
    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            LiteServer::Engine_LiteServer(ref x) => &x.id,
        }
    }

    pub fn port(&self) -> &crate::ton::int {
        match self {
            LiteServer::Engine_LiteServer(ref x) => &x.port,
        }
    }

    pub fn only(self) -> crate::ton::engine::liteserver::LiteServer {
        match self {
            LiteServer::Engine_LiteServer(x) => x,
        }
    }
}
impl Eq for LiteServer {}
impl Default for LiteServer {
    fn default() -> Self {
        LiteServer::Engine_LiteServer(crate::ton::engine::liteserver::LiteServer::default())
    }
}
impl crate::BoxedSerialize for LiteServer {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            LiteServer::Engine_LiteServer(x) => (crate::ConstructorNumber(0xbb708efe), x),
        }
    }
}
impl crate::BoxedDeserialize for LiteServer {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbb708efe)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbb708efe) => Ok(LiteServer::Engine_LiteServer(
                _de.read_bare::<crate::ton::engine::liteserver::LiteServer>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.OneSessionStat`\n\n```text\nengine.validator.oneSessionStat session_id:string stats:(vector engine.validator.oneStat) = engine.OneSessionStat;\n```\n"]
pub enum OneSessionStat {
    Engine_Validator_OneSessionStat(crate::ton::engine::validator::onesessionstat::OneSessionStat),
}
impl OneSessionStat {
    pub fn session_id(&self) -> &crate::ton::string {
        match self {
            OneSessionStat::Engine_Validator_OneSessionStat(ref x) => &x.session_id,
        }
    }

    pub fn stats(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::validator::onestat::OneStat>
    {
        match self {
            OneSessionStat::Engine_Validator_OneSessionStat(ref x) => &x.stats,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::onesessionstat::OneSessionStat {
        match self {
            OneSessionStat::Engine_Validator_OneSessionStat(x) => x,
        }
    }
}
impl Eq for OneSessionStat {}
impl Default for OneSessionStat {
    fn default() -> Self {
        OneSessionStat::Engine_Validator_OneSessionStat(
            crate::ton::engine::validator::onesessionstat::OneSessionStat::default(),
        )
    }
}
impl crate::BoxedSerialize for OneSessionStat {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            OneSessionStat::Engine_Validator_OneSessionStat(x) => {
                (crate::ConstructorNumber(0xadf42035), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for OneSessionStat {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xadf42035)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xadf42035) => {
                Ok(OneSessionStat::Engine_Validator_OneSessionStat(
                    _de.read_bare::<crate::ton::engine::validator::onesessionstat::OneSessionStat>(
                    )?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.Validator`\n\n```text\nengine.validator id:int256 temp_keys:(vector engine.validatorTempKey) adnl_addrs:(vector engine.validatorAdnlAddress) election_date:int expire_at:int = engine.Validator;\n```\n"]
pub enum Validator {
    Engine_Validator(crate::ton::engine::validator::Validator),
}
impl Validator {
    pub fn adnl_addrs(
        &self,
    ) -> &crate::ton::vector<
        crate::ton::Bare,
        crate::ton::engine::validatoradnladdress::ValidatorAdnlAddress,
    > {
        match self {
            Validator::Engine_Validator(ref x) => &x.adnl_addrs,
        }
    }

    pub fn election_date(&self) -> &crate::ton::int {
        match self {
            Validator::Engine_Validator(ref x) => &x.election_date,
        }
    }

    pub fn expire_at(&self) -> &crate::ton::int {
        match self {
            Validator::Engine_Validator(ref x) => &x.expire_at,
        }
    }

    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            Validator::Engine_Validator(ref x) => &x.id,
        }
    }

    pub fn temp_keys(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::engine::validatortempkey::ValidatorTempKey>
    {
        match self {
            Validator::Engine_Validator(ref x) => &x.temp_keys,
        }
    }

    pub fn only(self) -> crate::ton::engine::validator::Validator {
        match self {
            Validator::Engine_Validator(x) => x,
        }
    }
}
impl Eq for Validator {}
impl Default for Validator {
    fn default() -> Self {
        Validator::Engine_Validator(crate::ton::engine::validator::Validator::default())
    }
}
impl crate::BoxedSerialize for Validator {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Validator::Engine_Validator(x) => (crate::ConstructorNumber(0x885fea29), x),
        }
    }
}
impl crate::BoxedDeserialize for Validator {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x885fea29)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x885fea29) => Ok(Validator::Engine_Validator(
                _de.read_bare::<crate::ton::engine::validator::Validator>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.ValidatorAdnlAddress`\n\n```text\nengine.validatorAdnlAddress id:int256 expire_at:int = engine.ValidatorAdnlAddress;\n```\n"]
pub enum ValidatorAdnlAddress {
    Engine_ValidatorAdnlAddress(crate::ton::engine::validatoradnladdress::ValidatorAdnlAddress),
}
impl ValidatorAdnlAddress {
    pub fn expire_at(&self) -> &crate::ton::int {
        match self {
            ValidatorAdnlAddress::Engine_ValidatorAdnlAddress(ref x) => &x.expire_at,
        }
    }

    pub fn id(&self) -> &crate::ton::int256 {
        match self {
            ValidatorAdnlAddress::Engine_ValidatorAdnlAddress(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::engine::validatoradnladdress::ValidatorAdnlAddress {
        match self {
            ValidatorAdnlAddress::Engine_ValidatorAdnlAddress(x) => x,
        }
    }
}
impl Eq for ValidatorAdnlAddress {}
impl Default for ValidatorAdnlAddress {
    fn default() -> Self {
        ValidatorAdnlAddress::Engine_ValidatorAdnlAddress(
            crate::ton::engine::validatoradnladdress::ValidatorAdnlAddress::default(),
        )
    }
}
impl crate::BoxedSerialize for ValidatorAdnlAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ValidatorAdnlAddress::Engine_ValidatorAdnlAddress(x) => {
                (crate::ConstructorNumber(0xd34545be), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ValidatorAdnlAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd34545be)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0xd34545be) => Ok (ValidatorAdnlAddress :: Engine_ValidatorAdnlAddress (_de . read_bare :: < crate :: ton :: engine :: validatoradnladdress :: ValidatorAdnlAddress > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `engine.ValidatorTempKey`\n\n```text\nengine.validatorTempKey key:int256 expire_at:int = engine.ValidatorTempKey;\n```\n"]
pub enum ValidatorTempKey {
    Engine_ValidatorTempKey(crate::ton::engine::validatortempkey::ValidatorTempKey),
}
impl ValidatorTempKey {
    pub fn expire_at(&self) -> &crate::ton::int {
        match self {
            ValidatorTempKey::Engine_ValidatorTempKey(ref x) => &x.expire_at,
        }
    }

    pub fn key(&self) -> &crate::ton::int256 {
        match self {
            ValidatorTempKey::Engine_ValidatorTempKey(ref x) => &x.key,
        }
    }

    pub fn only(self) -> crate::ton::engine::validatortempkey::ValidatorTempKey {
        match self {
            ValidatorTempKey::Engine_ValidatorTempKey(x) => x,
        }
    }
}
impl Eq for ValidatorTempKey {}
impl Default for ValidatorTempKey {
    fn default() -> Self {
        ValidatorTempKey::Engine_ValidatorTempKey(
            crate::ton::engine::validatortempkey::ValidatorTempKey::default(),
        )
    }
}
impl crate::BoxedSerialize for ValidatorTempKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ValidatorTempKey::Engine_ValidatorTempKey(x) => {
                (crate::ConstructorNumber(0x5e4ad6de), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ValidatorTempKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5e4ad6de)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5e4ad6de) => Ok(ValidatorTempKey::Engine_ValidatorTempKey(
                _de.read_bare::<crate::ton::engine::validatortempkey::ValidatorTempKey>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod addr;
pub mod adnl;
pub mod adnl_proxy;
pub mod controlinterface;
pub mod controlprocess;
pub mod dht;
pub mod gc;
pub mod liteserver;
pub mod validator;
pub mod validatoradnladdress;
pub mod validatortempkey;
