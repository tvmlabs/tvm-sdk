#![allow(bare_trait_objects, unused_variables, unused_imports, non_snake_case)]
#![allow(clippy::module_inception)]
pub use crate::ton_prelude::*;
pub const LAYER: i32 = 0i32;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `AccountAddress`\n\n```text\naccountAddress account_address:string = AccountAddress;\n```\n"]
pub enum AccountAddress {
    AccountAddress(crate::ton::accountaddress::AccountAddress),
}
impl AccountAddress {
    pub fn account_address(&self) -> &crate::ton::string {
        match self {
            AccountAddress::AccountAddress(ref x) => &x.account_address,
        }
    }

    pub fn only(self) -> crate::ton::accountaddress::AccountAddress {
        match self {
            AccountAddress::AccountAddress(x) => x,
        }
    }
}
impl Eq for AccountAddress {}
impl Default for AccountAddress {
    fn default() -> Self {
        AccountAddress::AccountAddress(crate::ton::accountaddress::AccountAddress::default())
    }
}
impl crate::BoxedSerialize for AccountAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AccountAddress::AccountAddress(x) => (crate::ConstructorNumber(0x2d09bdab), x),
        }
    }
}
impl crate::BoxedDeserialize for AccountAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2d09bdab)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x2d09bdab) => Ok(AccountAddress::AccountAddress(
                _de.read_bare::<crate::ton::accountaddress::AccountAddress>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `AccountList`\n\n```text\naccountList accounts:vector<fullAccountState> = AccountList;\n```\n"]
pub enum AccountList {
    AccountList(crate::ton::accountlist::AccountList),
}
impl AccountList {
    pub fn accounts(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::fullaccountstate::FullAccountState> {
        match self {
            AccountList::AccountList(ref x) => &x.accounts,
        }
    }

    pub fn only(self) -> crate::ton::accountlist::AccountList {
        match self {
            AccountList::AccountList(x) => x,
        }
    }
}
impl Eq for AccountList {}
impl Default for AccountList {
    fn default() -> Self {
        AccountList::AccountList(crate::ton::accountlist::AccountList::default())
    }
}
impl crate::BoxedSerialize for AccountList {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AccountList::AccountList(x) => (crate::ConstructorNumber(0x783eb255), x),
        }
    }
}
impl crate::BoxedDeserialize for AccountList {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x783eb255)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x783eb255) => Ok(AccountList::AccountList(
                _de.read_bare::<crate::ton::accountlist::AccountList>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `AccountRevisionList`\n\n```text\naccountRevisionList revisions:vector<fullAccountState> = AccountRevisionList;\n```\n"]
pub enum AccountRevisionList {
    AccountRevisionList(crate::ton::accountrevisionlist::AccountRevisionList),
}
impl AccountRevisionList {
    pub fn revisions(
        &self,
    ) -> &crate::ton::vector<crate::ton::Bare, crate::ton::fullaccountstate::FullAccountState> {
        match self {
            AccountRevisionList::AccountRevisionList(ref x) => &x.revisions,
        }
    }

    pub fn only(self) -> crate::ton::accountrevisionlist::AccountRevisionList {
        match self {
            AccountRevisionList::AccountRevisionList(x) => x,
        }
    }
}
impl Eq for AccountRevisionList {}
impl Default for AccountRevisionList {
    fn default() -> Self {
        AccountRevisionList::AccountRevisionList(
            crate::ton::accountrevisionlist::AccountRevisionList::default(),
        )
    }
}
impl crate::BoxedSerialize for AccountRevisionList {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AccountRevisionList::AccountRevisionList(x) => {
                (crate::ConstructorNumber(0x1f6c64ca), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for AccountRevisionList {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x1f6c64ca)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x1f6c64ca) => Ok(AccountRevisionList::AccountRevisionList(
                _de.read_bare::<crate::ton::accountrevisionlist::AccountRevisionList>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `AccountState`\n\n```text\ndns.accountState wallet_id:int64 = AccountState;\n\npchan.accountState config:pchan.config state:pchan.State description:string = AccountState;\n\nraw.accountState code:bytes data:bytes frozen_hash:bytes = AccountState;\n\nrwallet.accountState wallet_id:int64 seqno:int32 unlocked_balance:int64 config:rwallet.config = AccountState;\n\ntestGiver.accountState seqno:int32 = AccountState;\n\ntestWallet.accountState seqno:int32 = AccountState;\n\nuninited.accountState frozen_hash:bytes = AccountState;\n\nwallet.accountState seqno:int32 = AccountState;\n\nwallet.highload.v1.accountState wallet_id:int64 seqno:int32 = AccountState;\n\nwallet.highload.v2.accountState wallet_id:int64 = AccountState;\n\nwallet.v3.accountState wallet_id:int64 seqno:int32 = AccountState;\n```\n"]
pub enum AccountState {
    Dns_AccountState(crate::ton::dns::accountstate::AccountState),
    Pchan_AccountState(crate::ton::pchan::accountstate::AccountState),
    Raw_AccountState(crate::ton::raw::accountstate::AccountState),
    Rwallet_AccountState(crate::ton::rwallet::accountstate::AccountState),
    TestGiver_AccountState(crate::ton::test_giver::accountstate::AccountState),
    TestWallet_AccountState(crate::ton::test_wallet::accountstate::AccountState),
    Uninited_AccountState(crate::ton::uninited::accountstate::AccountState),
    Wallet_AccountState(crate::ton::wallet::accountstate::AccountState),
    Wallet_Highload_V1_AccountState(crate::ton::wallet::highload::v1::accountstate::AccountState),
    Wallet_Highload_V2_AccountState(crate::ton::wallet::highload::v2::accountstate::AccountState),
    Wallet_V3_AccountState(crate::ton::wallet::v3::accountstate::AccountState),
}
impl AccountState {
    pub fn code(&self) -> Option<&crate::ton::bytes> {
        match self {
            AccountState::Raw_AccountState(ref x) => Some(&x.code),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            AccountState::Raw_AccountState(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn description(&self) -> Option<&crate::ton::string> {
        match self {
            AccountState::Pchan_AccountState(ref x) => Some(&x.description),
            _ => None,
        }
    }

    pub fn frozen_hash(&self) -> Option<&crate::ton::bytes> {
        match self {
            AccountState::Raw_AccountState(ref x) => Some(&x.frozen_hash),
            AccountState::Uninited_AccountState(ref x) => Some(&x.frozen_hash),
            _ => None,
        }
    }

    pub fn seqno(&self) -> Option<&crate::ton::int32> {
        match self {
            AccountState::Rwallet_AccountState(ref x) => Some(&x.seqno),
            AccountState::TestGiver_AccountState(ref x) => Some(&x.seqno),
            AccountState::TestWallet_AccountState(ref x) => Some(&x.seqno),
            AccountState::Wallet_AccountState(ref x) => Some(&x.seqno),
            AccountState::Wallet_Highload_V1_AccountState(ref x) => Some(&x.seqno),
            AccountState::Wallet_V3_AccountState(ref x) => Some(&x.seqno),
            _ => None,
        }
    }

    pub fn state(&self) -> Option<&crate::ton::pchan::State> {
        match self {
            AccountState::Pchan_AccountState(ref x) => Some(&x.state),
            _ => None,
        }
    }

    pub fn unlocked_balance(&self) -> Option<&crate::ton::int64> {
        match self {
            AccountState::Rwallet_AccountState(ref x) => Some(&x.unlocked_balance),
            _ => None,
        }
    }

    pub fn wallet_id(&self) -> Option<&crate::ton::int64> {
        match self {
            AccountState::Dns_AccountState(ref x) => Some(&x.wallet_id),
            AccountState::Rwallet_AccountState(ref x) => Some(&x.wallet_id),
            AccountState::Wallet_Highload_V1_AccountState(ref x) => Some(&x.wallet_id),
            AccountState::Wallet_Highload_V2_AccountState(ref x) => Some(&x.wallet_id),
            AccountState::Wallet_V3_AccountState(ref x) => Some(&x.wallet_id),
            _ => None,
        }
    }
}
impl Eq for AccountState {}
impl Default for AccountState {
    fn default() -> Self {
        AccountState::Dns_AccountState(crate::ton::dns::accountstate::AccountState::default())
    }
}
impl crate::BoxedSerialize for AccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AccountState::Dns_AccountState(x) => (crate::ConstructorNumber(0x66fad86a), x),
            AccountState::Pchan_AccountState(x) => (crate::ConstructorNumber(0x60226f78), x),
            AccountState::Raw_AccountState(x) => (crate::ConstructorNumber(0xe04b963a), x),
            AccountState::Rwallet_AccountState(x) => (crate::ConstructorNumber(0xd3eb83d8), x),
            AccountState::TestGiver_AccountState(x) => (crate::ConstructorNumber(0xd67779aa), x),
            AccountState::TestWallet_AccountState(x) => (crate::ConstructorNumber(0x8593d255), x),
            AccountState::Uninited_AccountState(x) => (crate::ConstructorNumber(0x5abd9708), x),
            AccountState::Wallet_AccountState(x) => (crate::ConstructorNumber(0xe8c0cf58), x),
            AccountState::Wallet_Highload_V1_AccountState(x) => {
                (crate::ConstructorNumber(0x6057e4dc), x)
            }
            AccountState::Wallet_Highload_V2_AccountState(x) => {
                (crate::ConstructorNumber(0x947d5d4f), x)
            }
            AccountState::Wallet_V3_AccountState(x) => (crate::ConstructorNumber(0x9f7aa84a), x),
        }
    }
}
impl crate::BoxedDeserialize for AccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x66fad86a),
            crate::ConstructorNumber(0x60226f78),
            crate::ConstructorNumber(0xe04b963a),
            crate::ConstructorNumber(0xd3eb83d8),
            crate::ConstructorNumber(0xd67779aa),
            crate::ConstructorNumber(0x8593d255),
            crate::ConstructorNumber(0x5abd9708),
            crate::ConstructorNumber(0xe8c0cf58),
            crate::ConstructorNumber(0x6057e4dc),
            crate::ConstructorNumber(0x947d5d4f),
            crate::ConstructorNumber(0x9f7aa84a),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x66fad86a) => Ok(AccountState::Dns_AccountState(
                _de.read_bare::<crate::ton::dns::accountstate::AccountState>()?,
            )),
            crate::ConstructorNumber(0x60226f78) => Ok(AccountState::Pchan_AccountState(
                _de.read_bare::<crate::ton::pchan::accountstate::AccountState>()?,
            )),
            crate::ConstructorNumber(0xe04b963a) => Ok(AccountState::Raw_AccountState(
                _de.read_bare::<crate::ton::raw::accountstate::AccountState>()?,
            )),
            crate::ConstructorNumber(0xd3eb83d8) => Ok(AccountState::Rwallet_AccountState(
                _de.read_bare::<crate::ton::rwallet::accountstate::AccountState>()?,
            )),
            crate::ConstructorNumber(0xd67779aa) => Ok(AccountState::TestGiver_AccountState(
                _de.read_bare::<crate::ton::test_giver::accountstate::AccountState>()?,
            )),
            crate::ConstructorNumber(0x8593d255) => Ok(AccountState::TestWallet_AccountState(
                _de.read_bare::<crate::ton::test_wallet::accountstate::AccountState>()?,
            )),
            crate::ConstructorNumber(0x5abd9708) => Ok(AccountState::Uninited_AccountState(
                _de.read_bare::<crate::ton::uninited::accountstate::AccountState>()?,
            )),
            crate::ConstructorNumber(0xe8c0cf58) => Ok(AccountState::Wallet_AccountState(
                _de.read_bare::<crate::ton::wallet::accountstate::AccountState>()?,
            )),
            crate::ConstructorNumber(0x6057e4dc) => {
                Ok(AccountState::Wallet_Highload_V1_AccountState(
                    _de.read_bare::<crate::ton::wallet::highload::v1::accountstate::AccountState>(
                    )?,
                ))
            }
            crate::ConstructorNumber(0x947d5d4f) => {
                Ok(AccountState::Wallet_Highload_V2_AccountState(
                    _de.read_bare::<crate::ton::wallet::highload::v2::accountstate::AccountState>(
                    )?,
                ))
            }
            crate::ConstructorNumber(0x9f7aa84a) => Ok(AccountState::Wallet_V3_AccountState(
                _de.read_bare::<crate::ton::wallet::v3::accountstate::AccountState>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Action`\n\n```text\nactionDns actions:vector<dns.Action> = Action;\n\nactionMsg messages:vector<msg.message> allow_send_to_uninited:Bool = Action;\n\nactionNoop  = Action;\n\nactionPchan action:pchan.Action = Action;\n\nactionRwallet action:rwallet.actionInit = Action;\n```\n"]
pub enum Action {
    ActionDns(crate::ton::action::ActionDns),
    ActionMsg(crate::ton::action::ActionMsg),
    ActionNoop,
    ActionPchan(crate::ton::action::ActionPchan),
    ActionRwallet(crate::ton::action::ActionRwallet),
}
impl Action {
    pub fn actions(
        &self,
    ) -> Option<&crate::ton::vector<crate::ton::Boxed, crate::ton::dns::Action>> {
        match self {
            Action::ActionDns(ref x) => Some(&x.actions),
            _ => None,
        }
    }

    pub fn allow_send_to_uninited(&self) -> Option<&crate::ton::Bool> {
        match self {
            Action::ActionMsg(ref x) => Some(&x.allow_send_to_uninited),
            _ => None,
        }
    }

    pub fn messages(
        &self,
    ) -> Option<&crate::ton::vector<crate::ton::Bare, crate::ton::msg::message::Message>> {
        match self {
            Action::ActionMsg(ref x) => Some(&x.messages),
            _ => None,
        }
    }
}
impl Eq for Action {}
impl Default for Action {
    fn default() -> Self {
        Action::ActionDns(crate::ton::action::ActionDns::default())
    }
}
impl crate::BoxedSerialize for Action {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Action::ActionDns(x) => (crate::ConstructorNumber(0x47273021), x),
            Action::ActionMsg(x) => (crate::ConstructorNumber(0x0eb67750), x),
            Action::ActionNoop => (crate::ConstructorNumber(0x43b3ac9b), &()),
            Action::ActionPchan(x) => (crate::ConstructorNumber(0xa72dc5e1), x),
            Action::ActionRwallet(x) => (crate::ConstructorNumber(0xf90237c5), x),
        }
    }
}
impl crate::BoxedDeserialize for Action {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x47273021),
            crate::ConstructorNumber(0x0eb67750),
            crate::ConstructorNumber(0x43b3ac9b),
            crate::ConstructorNumber(0xa72dc5e1),
            crate::ConstructorNumber(0xf90237c5),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x47273021) => {
                Ok(Action::ActionDns(_de.read_bare::<crate::ton::action::ActionDns>()?))
            }
            crate::ConstructorNumber(0x0eb67750) => {
                Ok(Action::ActionMsg(_de.read_bare::<crate::ton::action::ActionMsg>()?))
            }
            crate::ConstructorNumber(0x43b3ac9b) => Ok(Action::ActionNoop),
            crate::ConstructorNumber(0xa72dc5e1) => {
                Ok(Action::ActionPchan(_de.read_bare::<crate::ton::action::ActionPchan>()?))
            }
            crate::ConstructorNumber(0xf90237c5) => {
                Ok(Action::ActionRwallet(_de.read_bare::<crate::ton::action::ActionRwallet>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `AdnlAddress`\n\n```text\nadnlAddress adnl_address:string = AdnlAddress;\n```\n"]
pub enum AdnlAddress {
    AdnlAddress(crate::ton::adnladdress::AdnlAddress),
}
impl AdnlAddress {
    pub fn adnl_address(&self) -> &crate::ton::string {
        match self {
            AdnlAddress::AdnlAddress(ref x) => &x.adnl_address,
        }
    }

    pub fn only(self) -> crate::ton::adnladdress::AdnlAddress {
        match self {
            AdnlAddress::AdnlAddress(x) => x,
        }
    }
}
impl Eq for AdnlAddress {}
impl Default for AdnlAddress {
    fn default() -> Self {
        AdnlAddress::AdnlAddress(crate::ton::adnladdress::AdnlAddress::default())
    }
}
impl crate::BoxedSerialize for AdnlAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            AdnlAddress::AdnlAddress(x) => (crate::ConstructorNumber(0x0431950c), x),
        }
    }
}
impl crate::BoxedDeserialize for AdnlAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x0431950c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0431950c) => Ok(AdnlAddress::AdnlAddress(
                _de.read_bare::<crate::ton::adnladdress::AdnlAddress>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Bip39Hints`\n\n```text\nbip39Hints words:vector<string> = Bip39Hints;\n```\n"]
pub enum Bip39Hints {
    Bip39Hints(crate::ton::bip39hints::Bip39Hints),
}
impl Bip39Hints {
    pub fn words(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::string> {
        match self {
            Bip39Hints::Bip39Hints(ref x) => &x.words,
        }
    }

    pub fn only(self) -> crate::ton::bip39hints::Bip39Hints {
        match self {
            Bip39Hints::Bip39Hints(x) => x,
        }
    }
}
impl Eq for Bip39Hints {}
impl Default for Bip39Hints {
    fn default() -> Self {
        Bip39Hints::Bip39Hints(crate::ton::bip39hints::Bip39Hints::default())
    }
}
impl crate::BoxedSerialize for Bip39Hints {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Bip39Hints::Bip39Hints(x) => (crate::ConstructorNumber(0x3c559c00), x),
        }
    }
}
impl crate::BoxedDeserialize for Bip39Hints {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x3c559c00)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x3c559c00) => {
                Ok(Bip39Hints::Bip39Hints(_de.read_bare::<crate::ton::bip39hints::Bip39Hints>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `Bool`\n\n```text\nboolFalse = Bool;\n\nboolTrue = Bool;\n```\n"]
pub enum Bool {
    #[default]
    BoolFalse,
    BoolTrue,
}
impl Eq for Bool {}
impl crate::BoxedSerialize for Bool {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Bool::BoolFalse => (crate::ConstructorNumber(0xbc799737), &()),
            Bool::BoolTrue => (crate::ConstructorNumber(0x997275b5), &()),
        }
    }
}
impl crate::BoxedDeserialize for Bool {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbc799737), crate::ConstructorNumber(0x997275b5)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbc799737) => Ok(Bool::BoolFalse),
            crate::ConstructorNumber(0x997275b5) => Ok(Bool::BoolTrue),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Config`\n\n```text\nconfig config:string blockchain_name:string use_callbacks_for_network:Bool ignore_cache:Bool = Config;\n```\n"]
pub enum Config {
    Config(crate::ton::config::Config),
}
impl Config {
    pub fn blockchain_name(&self) -> &crate::ton::string {
        match self {
            Config::Config(ref x) => &x.blockchain_name,
        }
    }

    pub fn config(&self) -> &crate::ton::string {
        match self {
            Config::Config(ref x) => &x.config,
        }
    }

    pub fn ignore_cache(&self) -> &crate::ton::Bool {
        match self {
            Config::Config(ref x) => &x.ignore_cache,
        }
    }

    pub fn use_callbacks_for_network(&self) -> &crate::ton::Bool {
        match self {
            Config::Config(ref x) => &x.use_callbacks_for_network,
        }
    }

    pub fn only(self) -> crate::ton::config::Config {
        match self {
            Config::Config(x) => x,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::Config(crate::ton::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::Config(x) => (crate::ConstructorNumber(0xa44e0238), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa44e0238)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa44e0238) => {
                Ok(Config::Config(_de.read_bare::<crate::ton::config::Config>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Data`\n\n```text\ndata bytes:secureBytes = Data;\n```\n"]
pub enum Data {
    Data(crate::ton::data::Data),
}
impl Data {
    pub fn bytes(&self) -> &crate::ton::secureBytes {
        match self {
            Data::Data(ref x) => &x.bytes,
        }
    }

    pub fn only(self) -> crate::ton::data::Data {
        match self {
            Data::Data(x) => x,
        }
    }
}
impl Eq for Data {}
impl Default for Data {
    fn default() -> Self {
        Data::Data(crate::ton::data::Data::default())
    }
}
impl crate::BoxedSerialize for Data {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Data::Data(x) => (crate::ConstructorNumber(0xe747a971), x),
        }
    }
}
impl crate::BoxedDeserialize for Data {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe747a971)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe747a971) => {
                Ok(Data::Data(_de.read_bare::<crate::ton::data::Data>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Error`\n\n```text\nerror code:int32 message:string = Error;\n```\n"]
pub enum Error {
    Error(crate::ton::error::Error),
}
impl Error {
    pub fn code(&self) -> &crate::ton::int32 {
        match self {
            Error::Error(ref x) => &x.code,
        }
    }

    pub fn message(&self) -> &crate::ton::string {
        match self {
            Error::Error(ref x) => &x.message,
        }
    }

    pub fn only(self) -> crate::ton::error::Error {
        match self {
            Error::Error(x) => x,
        }
    }
}
impl Eq for Error {}
impl Default for Error {
    fn default() -> Self {
        Error::Error(crate::ton::error::Error::default())
    }
}
impl crate::BoxedSerialize for Error {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Error::Error(x) => (crate::ConstructorNumber(0x9bdd8f1a), x),
        }
    }
}
impl crate::BoxedDeserialize for Error {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x9bdd8f1a)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x9bdd8f1a) => {
                Ok(Error::Error(_de.read_bare::<crate::ton::error::Error>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `ExportedEncryptedKey`\n\n```text\nexportedEncryptedKey data:secureBytes = ExportedEncryptedKey;\n```\n"]
pub enum ExportedEncryptedKey {
    ExportedEncryptedKey(crate::ton::exportedencryptedkey::ExportedEncryptedKey),
}
impl ExportedEncryptedKey {
    pub fn data(&self) -> &crate::ton::secureBytes {
        match self {
            ExportedEncryptedKey::ExportedEncryptedKey(ref x) => &x.data,
        }
    }

    pub fn only(self) -> crate::ton::exportedencryptedkey::ExportedEncryptedKey {
        match self {
            ExportedEncryptedKey::ExportedEncryptedKey(x) => x,
        }
    }
}
impl Eq for ExportedEncryptedKey {}
impl Default for ExportedEncryptedKey {
    fn default() -> Self {
        ExportedEncryptedKey::ExportedEncryptedKey(
            crate::ton::exportedencryptedkey::ExportedEncryptedKey::default(),
        )
    }
}
impl crate::BoxedSerialize for ExportedEncryptedKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ExportedEncryptedKey::ExportedEncryptedKey(x) => {
                (crate::ConstructorNumber(0x78a9fe54), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ExportedEncryptedKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x78a9fe54)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x78a9fe54) => Ok(ExportedEncryptedKey::ExportedEncryptedKey(
                _de.read_bare::<crate::ton::exportedencryptedkey::ExportedEncryptedKey>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `ExportedKey`\n\n```text\nexportedKey word_list:vector<secureString> = ExportedKey;\n```\n"]
pub enum ExportedKey {
    ExportedKey(crate::ton::exportedkey::ExportedKey),
}
impl ExportedKey {
    pub fn word_list(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::secureString> {
        match self {
            ExportedKey::ExportedKey(ref x) => &x.word_list,
        }
    }

    pub fn only(self) -> crate::ton::exportedkey::ExportedKey {
        match self {
            ExportedKey::ExportedKey(x) => x,
        }
    }
}
impl Eq for ExportedKey {}
impl Default for ExportedKey {
    fn default() -> Self {
        ExportedKey::ExportedKey(crate::ton::exportedkey::ExportedKey::default())
    }
}
impl crate::BoxedSerialize for ExportedKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ExportedKey::ExportedKey(x) => (crate::ConstructorNumber(0xa99e39d7), x),
        }
    }
}
impl crate::BoxedDeserialize for ExportedKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa99e39d7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa99e39d7) => Ok(ExportedKey::ExportedKey(
                _de.read_bare::<crate::ton::exportedkey::ExportedKey>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `ExportedPemKey`\n\n```text\nexportedPemKey pem:secureString = ExportedPemKey;\n```\n"]
pub enum ExportedPemKey {
    ExportedPemKey(crate::ton::exportedpemkey::ExportedPemKey),
}
impl ExportedPemKey {
    pub fn pem(&self) -> &crate::ton::secureString {
        match self {
            ExportedPemKey::ExportedPemKey(ref x) => &x.pem,
        }
    }

    pub fn only(self) -> crate::ton::exportedpemkey::ExportedPemKey {
        match self {
            ExportedPemKey::ExportedPemKey(x) => x,
        }
    }
}
impl Eq for ExportedPemKey {}
impl Default for ExportedPemKey {
    fn default() -> Self {
        ExportedPemKey::ExportedPemKey(crate::ton::exportedpemkey::ExportedPemKey::default())
    }
}
impl crate::BoxedSerialize for ExportedPemKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ExportedPemKey::ExportedPemKey(x) => (crate::ConstructorNumber(0x54f700bd), x),
        }
    }
}
impl crate::BoxedDeserialize for ExportedPemKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x54f700bd)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x54f700bd) => Ok(ExportedPemKey::ExportedPemKey(
                _de.read_bare::<crate::ton::exportedpemkey::ExportedPemKey>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `ExportedUnencryptedKey`\n\n```text\nexportedUnencryptedKey data:secureBytes = ExportedUnencryptedKey;\n```\n"]
pub enum ExportedUnencryptedKey {
    ExportedUnencryptedKey(crate::ton::exportedunencryptedkey::ExportedUnencryptedKey),
}
impl ExportedUnencryptedKey {
    pub fn data(&self) -> &crate::ton::secureBytes {
        match self {
            ExportedUnencryptedKey::ExportedUnencryptedKey(ref x) => &x.data,
        }
    }

    pub fn only(self) -> crate::ton::exportedunencryptedkey::ExportedUnencryptedKey {
        match self {
            ExportedUnencryptedKey::ExportedUnencryptedKey(x) => x,
        }
    }
}
impl Eq for ExportedUnencryptedKey {}
impl Default for ExportedUnencryptedKey {
    fn default() -> Self {
        ExportedUnencryptedKey::ExportedUnencryptedKey(
            crate::ton::exportedunencryptedkey::ExportedUnencryptedKey::default(),
        )
    }
}
impl crate::BoxedSerialize for ExportedUnencryptedKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ExportedUnencryptedKey::ExportedUnencryptedKey(x) => {
                (crate::ConstructorNumber(0x2b839ae8), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for ExportedUnencryptedKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2b839ae8)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x2b839ae8) => {
                Ok(ExportedUnencryptedKey::ExportedUnencryptedKey(
                    _de.read_bare::<crate::ton::exportedunencryptedkey::ExportedUnencryptedKey>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Fees`\n\n```text\nfees in_fwd_fee:int53 storage_fee:int53 gas_fee:int53 fwd_fee:int53 = Fees;\n```\n"]
pub enum Fees {
    Fees(crate::ton::fees::Fees),
}
impl Fees {
    pub fn fwd_fee(&self) -> &crate::ton::int53 {
        match self {
            Fees::Fees(ref x) => &x.fwd_fee,
        }
    }

    pub fn gas_fee(&self) -> &crate::ton::int53 {
        match self {
            Fees::Fees(ref x) => &x.gas_fee,
        }
    }

    pub fn in_fwd_fee(&self) -> &crate::ton::int53 {
        match self {
            Fees::Fees(ref x) => &x.in_fwd_fee,
        }
    }

    pub fn storage_fee(&self) -> &crate::ton::int53 {
        match self {
            Fees::Fees(ref x) => &x.storage_fee,
        }
    }

    pub fn only(self) -> crate::ton::fees::Fees {
        match self {
            Fees::Fees(x) => x,
        }
    }
}
impl Eq for Fees {}
impl Default for Fees {
    fn default() -> Self {
        Fees::Fees(crate::ton::fees::Fees::default())
    }
}
impl crate::BoxedSerialize for Fees {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Fees::Fees(x) => (crate::ConstructorNumber(0x63e9e6bc), x),
        }
    }
}
impl crate::BoxedDeserialize for Fees {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x63e9e6bc)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x63e9e6bc) => {
                Ok(Fees::Fees(_de.read_bare::<crate::ton::fees::Fees>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `FullAccountState`\n\n```text\nfullAccountState address:accountAddress balance:int64 last_transaction_id:internal.transactionId block_id:tonNode.blockIdExt sync_utime:int53 account_state:AccountState revision:int32 = FullAccountState;\n```\n"]
pub enum FullAccountState {
    FullAccountState(crate::ton::fullaccountstate::FullAccountState),
}
impl FullAccountState {
    pub fn account_state(&self) -> &crate::ton::AccountState {
        match self {
            FullAccountState::FullAccountState(ref x) => &x.account_state,
        }
    }

    pub fn address(&self) -> &crate::ton::accountaddress::AccountAddress {
        match self {
            FullAccountState::FullAccountState(ref x) => &x.address,
        }
    }

    pub fn balance(&self) -> &crate::ton::int64 {
        match self {
            FullAccountState::FullAccountState(ref x) => &x.balance,
        }
    }

    pub fn block_id(&self) -> &crate::ton::ton_node::blockidext::BlockIdExt {
        match self {
            FullAccountState::FullAccountState(ref x) => &x.block_id,
        }
    }

    pub fn last_transaction_id(&self) -> &crate::ton::internal::transactionid::TransactionId {
        match self {
            FullAccountState::FullAccountState(ref x) => &x.last_transaction_id,
        }
    }

    pub fn revision(&self) -> &crate::ton::int32 {
        match self {
            FullAccountState::FullAccountState(ref x) => &x.revision,
        }
    }

    pub fn sync_utime(&self) -> &crate::ton::int53 {
        match self {
            FullAccountState::FullAccountState(ref x) => &x.sync_utime,
        }
    }

    pub fn only(self) -> crate::ton::fullaccountstate::FullAccountState {
        match self {
            FullAccountState::FullAccountState(x) => x,
        }
    }
}
impl Eq for FullAccountState {}
impl Default for FullAccountState {
    fn default() -> Self {
        FullAccountState::FullAccountState(crate::ton::fullaccountstate::FullAccountState::default())
    }
}
impl crate::BoxedSerialize for FullAccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            FullAccountState::FullAccountState(x) => (crate::ConstructorNumber(0x8419b860), x),
        }
    }
}
impl crate::BoxedDeserialize for FullAccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8419b860)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8419b860) => Ok(FullAccountState::FullAccountState(
                _de.read_bare::<crate::ton::fullaccountstate::FullAccountState>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Hashable`\n\n```text\nhashable.blockCandidate block:int approved:int = Hashable;\n\nhashable.blockCandidateAttempt block:int votes:int = Hashable;\n\nhashable.blockSignature signature:int = Hashable;\n\nhashable.blockVoteCandidate block:int approved:int = Hashable;\n\nhashable.bool value:Bool = Hashable;\n\nhashable.bytes value:bytes = Hashable;\n\nhashable.cntSortedVector data:int = Hashable;\n\nhashable.cntVector data:int = Hashable;\n\nhashable.int256 value:int256 = Hashable;\n\nhashable.int32 value:int = Hashable;\n\nhashable.int64 value:long = Hashable;\n\nhashable.pair left:int right:int = Hashable;\n\nhashable.sentBlock src:int root_hash:int file_hash:int collated_data_file_hash:int = Hashable;\n\nhashable.sentBlockEmpty = Hashable;\n\nhashable.validatorSession ts:int old_rounds:int cur_round:int = Hashable;\n\nhashable.validatorSessionOldRound seqno:int block:int signatures:int approve_signatures:int = Hashable;\n\nhashable.validatorSessionRound locked_round:int locked_block:int seqno:int precommitted:Bool\n          first_attempt:int approved_blocks:int signatures:int attempts:int = Hashable;\n\nhashable.validatorSessionRoundAttempt seqno:int votes:int precommitted:int vote_for_inited:int vote_for:int = Hashable;\n\nhashable.vector value:(vector int) = Hashable;\n\nhashable.vote block:int node:int = Hashable;\n```\n"]
pub enum Hashable {
    Hashable_BlockCandidate(crate::ton::hashable::hashable::BlockCandidate),
    Hashable_BlockCandidateAttempt(crate::ton::hashable::hashable::BlockCandidateAttempt),
    Hashable_BlockSignature(crate::ton::hashable::hashable::BlockSignature),
    Hashable_BlockVoteCandidate(crate::ton::hashable::hashable::BlockVoteCandidate),
    Hashable_Bool(crate::ton::hashable::hashable::Bool),
    Hashable_Bytes(crate::ton::hashable::hashable::Bytes),
    Hashable_CntSortedVector(crate::ton::hashable::hashable::CntSortedVector),
    Hashable_CntVector(crate::ton::hashable::hashable::CntVector),
    Hashable_Int256(crate::ton::hashable::hashable::Int256),
    Hashable_Int32(crate::ton::hashable::hashable::Int32),
    Hashable_Int64(crate::ton::hashable::hashable::Int64),
    Hashable_Pair(crate::ton::hashable::hashable::Pair),
    Hashable_SentBlock(crate::ton::hashable::hashable::SentBlock),
    Hashable_SentBlockEmpty,
    Hashable_ValidatorSession(crate::ton::hashable::hashable::ValidatorSession),
    Hashable_ValidatorSessionOldRound(crate::ton::hashable::hashable::ValidatorSessionOldRound),
    Hashable_ValidatorSessionRound(crate::ton::hashable::hashable::ValidatorSessionRound),
    Hashable_ValidatorSessionRoundAttempt(
        crate::ton::hashable::hashable::ValidatorSessionRoundAttempt,
    ),
    Hashable_Vector(crate::ton::hashable::hashable::Vector),
    Hashable_Vote(crate::ton::hashable::hashable::Vote),
}
impl Hashable {
    pub fn approve_signatures(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionOldRound(ref x) => Some(&x.approve_signatures),
            _ => None,
        }
    }

    pub fn approved(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_BlockCandidate(ref x) => Some(&x.approved),
            Hashable::Hashable_BlockVoteCandidate(ref x) => Some(&x.approved),
            _ => None,
        }
    }

    pub fn approved_blocks(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionRound(ref x) => Some(&x.approved_blocks),
            _ => None,
        }
    }

    pub fn attempts(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionRound(ref x) => Some(&x.attempts),
            _ => None,
        }
    }

    pub fn block(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_BlockCandidate(ref x) => Some(&x.block),
            Hashable::Hashable_BlockCandidateAttempt(ref x) => Some(&x.block),
            Hashable::Hashable_BlockVoteCandidate(ref x) => Some(&x.block),
            Hashable::Hashable_ValidatorSessionOldRound(ref x) => Some(&x.block),
            Hashable::Hashable_Vote(ref x) => Some(&x.block),
            _ => None,
        }
    }

    pub fn collated_data_file_hash(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_SentBlock(ref x) => Some(&x.collated_data_file_hash),
            _ => None,
        }
    }

    pub fn cur_round(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSession(ref x) => Some(&x.cur_round),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_CntSortedVector(ref x) => Some(&x.data),
            Hashable::Hashable_CntVector(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn file_hash(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_SentBlock(ref x) => Some(&x.file_hash),
            _ => None,
        }
    }

    pub fn first_attempt(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionRound(ref x) => Some(&x.first_attempt),
            _ => None,
        }
    }

    pub fn left(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_Pair(ref x) => Some(&x.left),
            _ => None,
        }
    }

    pub fn locked_block(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionRound(ref x) => Some(&x.locked_block),
            _ => None,
        }
    }

    pub fn locked_round(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionRound(ref x) => Some(&x.locked_round),
            _ => None,
        }
    }

    pub fn node(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_Vote(ref x) => Some(&x.node),
            _ => None,
        }
    }

    pub fn old_rounds(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSession(ref x) => Some(&x.old_rounds),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_Pair(ref x) => Some(&x.right),
            _ => None,
        }
    }

    pub fn root_hash(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_SentBlock(ref x) => Some(&x.root_hash),
            _ => None,
        }
    }

    pub fn seqno(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionOldRound(ref x) => Some(&x.seqno),
            Hashable::Hashable_ValidatorSessionRound(ref x) => Some(&x.seqno),
            Hashable::Hashable_ValidatorSessionRoundAttempt(ref x) => Some(&x.seqno),
            _ => None,
        }
    }

    pub fn signature(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_BlockSignature(ref x) => Some(&x.signature),
            _ => None,
        }
    }

    pub fn signatures(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionOldRound(ref x) => Some(&x.signatures),
            Hashable::Hashable_ValidatorSessionRound(ref x) => Some(&x.signatures),
            _ => None,
        }
    }

    pub fn src(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_SentBlock(ref x) => Some(&x.src),
            _ => None,
        }
    }

    pub fn ts(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSession(ref x) => Some(&x.ts),
            _ => None,
        }
    }

    pub fn vote_for(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionRoundAttempt(ref x) => Some(&x.vote_for),
            _ => None,
        }
    }

    pub fn vote_for_inited(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_ValidatorSessionRoundAttempt(ref x) => Some(&x.vote_for_inited),
            _ => None,
        }
    }

    pub fn votes(&self) -> Option<&crate::ton::int> {
        match self {
            Hashable::Hashable_BlockCandidateAttempt(ref x) => Some(&x.votes),
            Hashable::Hashable_ValidatorSessionRoundAttempt(ref x) => Some(&x.votes),
            _ => None,
        }
    }
}
impl Eq for Hashable {}
impl Default for Hashable {
    fn default() -> Self {
        Hashable::Hashable_BlockCandidate(crate::ton::hashable::hashable::BlockCandidate::default())
    }
}
impl crate::BoxedSerialize for Hashable {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Hashable::Hashable_BlockCandidate(x) => (crate::ConstructorNumber(0x0ba9b10d), x),
            Hashable::Hashable_BlockCandidateAttempt(x) => {
                (crate::ConstructorNumber(0x3f5c7d0b), x)
            }
            Hashable::Hashable_BlockSignature(x) => (crate::ConstructorNumber(0x37e192a2), x),
            Hashable::Hashable_BlockVoteCandidate(x) => (crate::ConstructorNumber(0xcf0d6fe5), x),
            Hashable::Hashable_Bool(x) => (crate::ConstructorNumber(0xcf61441c), x),
            Hashable::Hashable_Bytes(x) => (crate::ConstructorNumber(0x0713de12), x),
            Hashable::Hashable_CntSortedVector(x) => (crate::ConstructorNumber(0x7b964659), x),
            Hashable::Hashable_CntVector(x) => (crate::ConstructorNumber(0x0b286f38), x),
            Hashable::Hashable_Int256(x) => (crate::ConstructorNumber(0x3a2313cf), x),
            Hashable::Hashable_Int32(x) => (crate::ConstructorNumber(0xd3b59356), x),
            Hashable::Hashable_Int64(x) => (crate::ConstructorNumber(0xe7da8e42), x),
            Hashable::Hashable_Pair(x) => (crate::ConstructorNumber(0xc7e56895), x),
            Hashable::Hashable_SentBlock(x) => (crate::ConstructorNumber(0xbdb9952b), x),
            Hashable::Hashable_SentBlockEmpty => (crate::ConstructorNumber(0x9ef246af), &()),
            Hashable::Hashable_ValidatorSession(x) => (crate::ConstructorNumber(0x681263d5), x),
            Hashable::Hashable_ValidatorSessionOldRound(x) => {
                (crate::ConstructorNumber(0x478b67a9), x)
            }
            Hashable::Hashable_ValidatorSessionRound(x) => {
                (crate::ConstructorNumber(0x35774fe3), x)
            }
            Hashable::Hashable_ValidatorSessionRoundAttempt(x) => {
                (crate::ConstructorNumber(0x4c11ffad), x)
            }
            Hashable::Hashable_Vector(x) => (crate::ConstructorNumber(0xdf34c36d), x),
            Hashable::Hashable_Vote(x) => (crate::ConstructorNumber(0xaebf2bc5), x),
        }
    }
}
impl crate::BoxedDeserialize for Hashable {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x0ba9b10d),
            crate::ConstructorNumber(0x3f5c7d0b),
            crate::ConstructorNumber(0x37e192a2),
            crate::ConstructorNumber(0xcf0d6fe5),
            crate::ConstructorNumber(0xcf61441c),
            crate::ConstructorNumber(0x0713de12),
            crate::ConstructorNumber(0x7b964659),
            crate::ConstructorNumber(0x0b286f38),
            crate::ConstructorNumber(0x3a2313cf),
            crate::ConstructorNumber(0xd3b59356),
            crate::ConstructorNumber(0xe7da8e42),
            crate::ConstructorNumber(0xc7e56895),
            crate::ConstructorNumber(0xbdb9952b),
            crate::ConstructorNumber(0x9ef246af),
            crate::ConstructorNumber(0x681263d5),
            crate::ConstructorNumber(0x478b67a9),
            crate::ConstructorNumber(0x35774fe3),
            crate::ConstructorNumber(0x4c11ffad),
            crate::ConstructorNumber(0xdf34c36d),
            crate::ConstructorNumber(0xaebf2bc5),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x0ba9b10d) => Ok(Hashable::Hashable_BlockCandidate(
                _de.read_bare::<crate::ton::hashable::hashable::BlockCandidate>()?,
            )),
            crate::ConstructorNumber(0x3f5c7d0b) => Ok(Hashable::Hashable_BlockCandidateAttempt(
                _de.read_bare::<crate::ton::hashable::hashable::BlockCandidateAttempt>()?,
            )),
            crate::ConstructorNumber(0x37e192a2) => Ok(Hashable::Hashable_BlockSignature(
                _de.read_bare::<crate::ton::hashable::hashable::BlockSignature>()?,
            )),
            crate::ConstructorNumber(0xcf0d6fe5) => Ok(Hashable::Hashable_BlockVoteCandidate(
                _de.read_bare::<crate::ton::hashable::hashable::BlockVoteCandidate>()?,
            )),
            crate::ConstructorNumber(0xcf61441c) => Ok(Hashable::Hashable_Bool(
                _de.read_bare::<crate::ton::hashable::hashable::Bool>()?,
            )),
            crate::ConstructorNumber(0x0713de12) => Ok(Hashable::Hashable_Bytes(
                _de.read_bare::<crate::ton::hashable::hashable::Bytes>()?,
            )),
            crate::ConstructorNumber(0x7b964659) => Ok(Hashable::Hashable_CntSortedVector(
                _de.read_bare::<crate::ton::hashable::hashable::CntSortedVector>()?,
            )),
            crate::ConstructorNumber(0x0b286f38) => Ok(Hashable::Hashable_CntVector(
                _de.read_bare::<crate::ton::hashable::hashable::CntVector>()?,
            )),
            crate::ConstructorNumber(0x3a2313cf) => Ok(Hashable::Hashable_Int256(
                _de.read_bare::<crate::ton::hashable::hashable::Int256>()?,
            )),
            crate::ConstructorNumber(0xd3b59356) => Ok(Hashable::Hashable_Int32(
                _de.read_bare::<crate::ton::hashable::hashable::Int32>()?,
            )),
            crate::ConstructorNumber(0xe7da8e42) => Ok(Hashable::Hashable_Int64(
                _de.read_bare::<crate::ton::hashable::hashable::Int64>()?,
            )),
            crate::ConstructorNumber(0xc7e56895) => Ok(Hashable::Hashable_Pair(
                _de.read_bare::<crate::ton::hashable::hashable::Pair>()?,
            )),
            crate::ConstructorNumber(0xbdb9952b) => Ok(Hashable::Hashable_SentBlock(
                _de.read_bare::<crate::ton::hashable::hashable::SentBlock>()?,
            )),
            crate::ConstructorNumber(0x9ef246af) => Ok(Hashable::Hashable_SentBlockEmpty),
            crate::ConstructorNumber(0x681263d5) => Ok(Hashable::Hashable_ValidatorSession(
                _de.read_bare::<crate::ton::hashable::hashable::ValidatorSession>()?,
            )),
            crate::ConstructorNumber(0x478b67a9) => {
                Ok(Hashable::Hashable_ValidatorSessionOldRound(
                    _de.read_bare::<crate::ton::hashable::hashable::ValidatorSessionOldRound>()?,
                ))
            }
            crate::ConstructorNumber(0x35774fe3) => Ok(Hashable::Hashable_ValidatorSessionRound(
                _de.read_bare::<crate::ton::hashable::hashable::ValidatorSessionRound>()?,
            )),
            crate::ConstructorNumber(0x4c11ffad) => {
                Ok(Hashable::Hashable_ValidatorSessionRoundAttempt(
                    _de.read_bare::<crate::ton::hashable::hashable::ValidatorSessionRoundAttempt>(
                    )?,
                ))
            }
            crate::ConstructorNumber(0xdf34c36d) => Ok(Hashable::Hashable_Vector(
                _de.read_bare::<crate::ton::hashable::hashable::Vector>()?,
            )),
            crate::ConstructorNumber(0xaebf2bc5) => Ok(Hashable::Hashable_Vote(
                _de.read_bare::<crate::ton::hashable::hashable::Vote>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `InitialAccountState`\n\n```text\ndns.initialAccountState public_key:string wallet_id:int64 = InitialAccountState;\n\npchan.initialAccountState config:pchan.config = InitialAccountState;\n\nraw.initialAccountState code:bytes data:bytes = InitialAccountState;\n\nrwallet.initialAccountState init_public_key:string public_key:string wallet_id:int64 = InitialAccountState;\n\ntestGiver.initialAccountState = InitialAccountState;\n\ntestWallet.initialAccountState public_key:string = InitialAccountState;\n\nwallet.highload.v1.initialAccountState public_key:string wallet_id:int64 = InitialAccountState;\n\nwallet.highload.v2.initialAccountState public_key:string wallet_id:int64 = InitialAccountState;\n\nwallet.initialAccountState public_key:string = InitialAccountState;\n\nwallet.v3.initialAccountState public_key:string wallet_id:int64 = InitialAccountState;\n```\n"]
pub enum InitialAccountState {
    Dns_InitialAccountState(crate::ton::dns::initialaccountstate::InitialAccountState),
    Pchan_InitialAccountState(crate::ton::pchan::initialaccountstate::InitialAccountState),
    Raw_InitialAccountState(crate::ton::raw::initialaccountstate::InitialAccountState),
    Rwallet_InitialAccountState(crate::ton::rwallet::initialaccountstate::InitialAccountState),
    TestGiver_InitialAccountState,
    TestWallet_InitialAccountState(
        crate::ton::test_wallet::initialaccountstate::InitialAccountState,
    ),
    Wallet_Highload_V1_InitialAccountState(
        crate::ton::wallet::highload::v1::initialaccountstate::InitialAccountState,
    ),
    Wallet_Highload_V2_InitialAccountState(
        crate::ton::wallet::highload::v2::initialaccountstate::InitialAccountState,
    ),
    Wallet_InitialAccountState(crate::ton::wallet::initialaccountstate::InitialAccountState),
    Wallet_V3_InitialAccountState(crate::ton::wallet::v3::initialaccountstate::InitialAccountState),
}
impl InitialAccountState {
    pub fn code(&self) -> Option<&crate::ton::bytes> {
        match self {
            InitialAccountState::Raw_InitialAccountState(ref x) => Some(&x.code),
            _ => None,
        }
    }

    pub fn config(&self) -> Option<&crate::ton::pchan::config::Config> {
        match self {
            InitialAccountState::Pchan_InitialAccountState(ref x) => Some(&x.config),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            InitialAccountState::Raw_InitialAccountState(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn init_public_key(&self) -> Option<&crate::ton::string> {
        match self {
            InitialAccountState::Rwallet_InitialAccountState(ref x) => Some(&x.init_public_key),
            _ => None,
        }
    }

    pub fn public_key(&self) -> Option<&crate::ton::string> {
        match self {
            InitialAccountState::Dns_InitialAccountState(ref x) => Some(&x.public_key),
            InitialAccountState::Rwallet_InitialAccountState(ref x) => Some(&x.public_key),
            InitialAccountState::TestWallet_InitialAccountState(ref x) => Some(&x.public_key),
            InitialAccountState::Wallet_Highload_V1_InitialAccountState(ref x) => {
                Some(&x.public_key)
            }
            InitialAccountState::Wallet_Highload_V2_InitialAccountState(ref x) => {
                Some(&x.public_key)
            }
            InitialAccountState::Wallet_InitialAccountState(ref x) => Some(&x.public_key),
            InitialAccountState::Wallet_V3_InitialAccountState(ref x) => Some(&x.public_key),
            _ => None,
        }
    }

    pub fn wallet_id(&self) -> Option<&crate::ton::int64> {
        match self {
            InitialAccountState::Dns_InitialAccountState(ref x) => Some(&x.wallet_id),
            InitialAccountState::Rwallet_InitialAccountState(ref x) => Some(&x.wallet_id),
            InitialAccountState::Wallet_Highload_V1_InitialAccountState(ref x) => {
                Some(&x.wallet_id)
            }
            InitialAccountState::Wallet_Highload_V2_InitialAccountState(ref x) => {
                Some(&x.wallet_id)
            }
            InitialAccountState::Wallet_V3_InitialAccountState(ref x) => Some(&x.wallet_id),
            _ => None,
        }
    }
}
impl Eq for InitialAccountState {}
impl Default for InitialAccountState {
    fn default() -> Self {
        InitialAccountState::Dns_InitialAccountState(
            crate::ton::dns::initialaccountstate::InitialAccountState::default(),
        )
    }
}
impl crate::BoxedSerialize for InitialAccountState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            InitialAccountState::Dns_InitialAccountState(x) => {
                (crate::ConstructorNumber(0x6dcba4bf), x)
            }
            InitialAccountState::Pchan_InitialAccountState(x) => {
                (crate::ConstructorNumber(0xb23e1d44), x)
            }
            InitialAccountState::Raw_InitialAccountState(x) => {
                (crate::ConstructorNumber(0xebdb5c47), x)
            }
            InitialAccountState::Rwallet_InitialAccountState(x) => {
                (crate::ConstructorNumber(0x45b90c14), x)
            }
            InitialAccountState::TestGiver_InitialAccountState => {
                (crate::ConstructorNumber(0xa9aafbf0), &())
            }
            InitialAccountState::TestWallet_InitialAccountState(x) => {
                (crate::ConstructorNumber(0x30d6bf64), x)
            }
            InitialAccountState::Wallet_Highload_V1_InitialAccountState(x) => {
                (crate::ConstructorNumber(0xec749e46), x)
            }
            InitialAccountState::Wallet_Highload_V2_InitialAccountState(x) => {
                (crate::ConstructorNumber(0x75347929), x)
            }
            InitialAccountState::Wallet_InitialAccountState(x) => {
                (crate::ConstructorNumber(0xbd1d17fa), x)
            }
            InitialAccountState::Wallet_V3_InitialAccountState(x) => {
                (crate::ConstructorNumber(0xf8f65540), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for InitialAccountState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x6dcba4bf),
            crate::ConstructorNumber(0xb23e1d44),
            crate::ConstructorNumber(0xebdb5c47),
            crate::ConstructorNumber(0x45b90c14),
            crate::ConstructorNumber(0xa9aafbf0),
            crate::ConstructorNumber(0x30d6bf64),
            crate::ConstructorNumber(0xec749e46),
            crate::ConstructorNumber(0x75347929),
            crate::ConstructorNumber(0xbd1d17fa),
            crate::ConstructorNumber(0xf8f65540),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id { crate :: ConstructorNumber (0x6dcba4bf) => Ok (InitialAccountState :: Dns_InitialAccountState (_de . read_bare :: < crate :: ton :: dns :: initialaccountstate :: InitialAccountState > () ?)) , crate :: ConstructorNumber (0xb23e1d44) => Ok (InitialAccountState :: Pchan_InitialAccountState (_de . read_bare :: < crate :: ton :: pchan :: initialaccountstate :: InitialAccountState > () ?)) , crate :: ConstructorNumber (0xebdb5c47) => Ok (InitialAccountState :: Raw_InitialAccountState (_de . read_bare :: < crate :: ton :: raw :: initialaccountstate :: InitialAccountState > () ?)) , crate :: ConstructorNumber (0x45b90c14) => Ok (InitialAccountState :: Rwallet_InitialAccountState (_de . read_bare :: < crate :: ton :: rwallet :: initialaccountstate :: InitialAccountState > () ?)) , crate :: ConstructorNumber (0xa9aafbf0) => Ok (InitialAccountState :: TestGiver_InitialAccountState) , crate :: ConstructorNumber (0x30d6bf64) => Ok (InitialAccountState :: TestWallet_InitialAccountState (_de . read_bare :: < crate :: ton :: test_wallet :: initialaccountstate :: InitialAccountState > () ?)) , crate :: ConstructorNumber (0xec749e46) => Ok (InitialAccountState :: Wallet_Highload_V1_InitialAccountState (_de . read_bare :: < crate :: ton :: wallet :: highload :: v1 :: initialaccountstate :: InitialAccountState > () ?)) , crate :: ConstructorNumber (0x75347929) => Ok (InitialAccountState :: Wallet_Highload_V2_InitialAccountState (_de . read_bare :: < crate :: ton :: wallet :: highload :: v2 :: initialaccountstate :: InitialAccountState > () ?)) , crate :: ConstructorNumber (0xbd1d17fa) => Ok (InitialAccountState :: Wallet_InitialAccountState (_de . read_bare :: < crate :: ton :: wallet :: initialaccountstate :: InitialAccountState > () ?)) , crate :: ConstructorNumber (0xf8f65540) => Ok (InitialAccountState :: Wallet_V3_InitialAccountState (_de . read_bare :: < crate :: ton :: wallet :: v3 :: initialaccountstate :: InitialAccountState > () ?)) , id => _invalid_id ! (id) , }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `InputKey`\n\n```text\ninputKeyFake = InputKey;\n\ninputKeyRegular key:key local_password:secureBytes = InputKey;\n```\n"]
pub enum InputKey {
    #[default]
    InputKeyFake,
    InputKeyRegular(crate::ton::inputkey::InputKeyRegular),
}
impl InputKey {
    pub fn key(&self) -> Option<&crate::ton::key::Key> {
        match self {
            InputKey::InputKeyRegular(ref x) => Some(&x.key),
            _ => None,
        }
    }

    pub fn local_password(&self) -> Option<&crate::ton::secureBytes> {
        match self {
            InputKey::InputKeyRegular(ref x) => Some(&x.local_password),
            _ => None,
        }
    }
}
impl Eq for InputKey {}
impl crate::BoxedSerialize for InputKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            InputKey::InputKeyFake => (crate::ConstructorNumber(0xbffb39be), &()),
            InputKey::InputKeyRegular(x) => (crate::ConstructorNumber(0xdee5469e), x),
        }
    }
}
impl crate::BoxedDeserialize for InputKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbffb39be), crate::ConstructorNumber(0xdee5469e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbffb39be) => Ok(InputKey::InputKeyFake),
            crate::ConstructorNumber(0xdee5469e) => Ok(InputKey::InputKeyRegular(
                _de.read_bare::<crate::ton::inputkey::InputKeyRegular>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::inputkey::InputKeyRegular> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0xbffb39be), &()),
            Some(ref x) => (crate::ConstructorNumber(0xdee5469e), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::inputkey::InputKeyRegular> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xbffb39be), crate::ConstructorNumber(0xdee5469e)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xbffb39be) => Ok(None),
            crate::ConstructorNumber(0xdee5469e) => {
                Ok(Some(_de.read_bare::<crate::ton::inputkey::InputKeyRegular>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `Int8`\n\n```text\nint8 = Int8;\n```\n"]
pub enum Int8 {
    #[default]
    Int8,
}
impl Eq for Int8 {}
impl crate::BoxedSerialize for Int8 {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Int8::Int8 => (crate::ConstructorNumber(0x05cb0099), &()),
        }
    }
}
impl crate::BoxedDeserialize for Int8 {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x05cb0099)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x05cb0099) => Ok(Int8::Int8),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Key`\n\n```text\nkey public_key:string secret:secureBytes = Key;\n```\n"]
pub enum Key {
    Key(crate::ton::key::Key),
}
impl Key {
    pub fn public_key(&self) -> &crate::ton::string {
        match self {
            Key::Key(ref x) => &x.public_key,
        }
    }

    pub fn secret(&self) -> &crate::ton::secureBytes {
        match self {
            Key::Key(ref x) => &x.secret,
        }
    }

    pub fn only(self) -> crate::ton::key::Key {
        match self {
            Key::Key(x) => x,
        }
    }
}
impl Eq for Key {}
impl Default for Key {
    fn default() -> Self {
        Key::Key(crate::ton::key::Key::default())
    }
}
impl crate::BoxedSerialize for Key {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Key::Key(x) => (crate::ConstructorNumber(0x8a1493d5), x),
        }
    }
}
impl crate::BoxedDeserialize for Key {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8a1493d5)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8a1493d5) => {
                Ok(Key::Key(_de.read_bare::<crate::ton::key::Key>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `KeyStoreType`\n\n```text\nkeyStoreTypeDirectory directory:string = KeyStoreType;\n\nkeyStoreTypeInMemory = KeyStoreType;\n```\n"]
pub enum KeyStoreType {
    KeyStoreTypeDirectory(crate::ton::keystoretype::KeyStoreTypeDirectory),
    KeyStoreTypeInMemory,
}
impl KeyStoreType {
    pub fn directory(&self) -> Option<&crate::ton::string> {
        match self {
            KeyStoreType::KeyStoreTypeDirectory(ref x) => Some(&x.directory),
            _ => None,
        }
    }
}
impl Eq for KeyStoreType {}
impl Default for KeyStoreType {
    fn default() -> Self {
        KeyStoreType::KeyStoreTypeDirectory(
            crate::ton::keystoretype::KeyStoreTypeDirectory::default(),
        )
    }
}
impl crate::BoxedSerialize for KeyStoreType {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            KeyStoreType::KeyStoreTypeDirectory(x) => (crate::ConstructorNumber(0xe969122a), x),
            KeyStoreType::KeyStoreTypeInMemory => (crate::ConstructorNumber(0x826c09c7), &()),
        }
    }
}
impl crate::BoxedDeserialize for KeyStoreType {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xe969122a), crate::ConstructorNumber(0x826c09c7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xe969122a) => Ok(KeyStoreType::KeyStoreTypeDirectory(
                _de.read_bare::<crate::ton::keystoretype::KeyStoreTypeDirectory>()?,
            )),
            crate::ConstructorNumber(0x826c09c7) => Ok(KeyStoreType::KeyStoreTypeInMemory),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::keystoretype::KeyStoreTypeDirectory> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x826c09c7), &()),
            Some(ref x) => (crate::ConstructorNumber(0xe969122a), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::keystoretype::KeyStoreTypeDirectory> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x826c09c7), crate::ConstructorNumber(0xe969122a)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x826c09c7) => Ok(None),
            crate::ConstructorNumber(0xe969122a) => {
                Ok(Some(_de.read_bare::<crate::ton::keystoretype::KeyStoreTypeDirectory>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `LogStream`\n\n```text\nlogStreamDefault = LogStream;\n\nlogStreamEmpty = LogStream;\n\nlogStreamFile path:string max_file_size:int53 = LogStream;\n```\n"]
pub enum LogStream {
    #[default]
    LogStreamDefault,
    LogStreamEmpty,
    LogStreamFile(crate::ton::logstream::LogStreamFile),
}
impl LogStream {
    pub fn max_file_size(&self) -> Option<&crate::ton::int53> {
        match self {
            LogStream::LogStreamFile(ref x) => Some(&x.max_file_size),
            _ => None,
        }
    }

    pub fn path(&self) -> Option<&crate::ton::string> {
        match self {
            LogStream::LogStreamFile(ref x) => Some(&x.path),
            _ => None,
        }
    }
}
impl Eq for LogStream {}
impl crate::BoxedSerialize for LogStream {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            LogStream::LogStreamDefault => (crate::ConstructorNumber(0x52e296bc), &()),
            LogStream::LogStreamEmpty => (crate::ConstructorNumber(0xe233f1cc), &()),
            LogStream::LogStreamFile(x) => (crate::ConstructorNumber(0x8ff02a56), x),
        }
    }
}
impl crate::BoxedDeserialize for LogStream {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x52e296bc),
            crate::ConstructorNumber(0xe233f1cc),
            crate::ConstructorNumber(0x8ff02a56),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x52e296bc) => Ok(LogStream::LogStreamDefault),
            crate::ConstructorNumber(0xe233f1cc) => Ok(LogStream::LogStreamEmpty),
            crate::ConstructorNumber(0x8ff02a56) => Ok(LogStream::LogStreamFile(
                _de.read_bare::<crate::ton::logstream::LogStreamFile>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `LogTags`\n\n```text\nlogTags tags:vector<string> = LogTags;\n```\n"]
pub enum LogTags {
    LogTags(crate::ton::logtags::LogTags),
}
impl LogTags {
    pub fn tags(&self) -> &crate::ton::vector<crate::ton::Bare, crate::ton::string> {
        match self {
            LogTags::LogTags(ref x) => &x.tags,
        }
    }

    pub fn only(self) -> crate::ton::logtags::LogTags {
        match self {
            LogTags::LogTags(x) => x,
        }
    }
}
impl Eq for LogTags {}
impl Default for LogTags {
    fn default() -> Self {
        LogTags::LogTags(crate::ton::logtags::LogTags::default())
    }
}
impl crate::BoxedSerialize for LogTags {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            LogTags::LogTags(x) => (crate::ConstructorNumber(0xa056b3d7), x),
        }
    }
}
impl crate::BoxedDeserialize for LogTags {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa056b3d7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa056b3d7) => {
                Ok(LogTags::LogTags(_de.read_bare::<crate::ton::logtags::LogTags>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `LogVerbosityLevel`\n\n```text\nlogVerbosityLevel verbosity_level:int32 = LogVerbosityLevel;\n```\n"]
pub enum LogVerbosityLevel {
    LogVerbosityLevel(crate::ton::logverbositylevel::LogVerbosityLevel),
}
impl LogVerbosityLevel {
    pub fn verbosity_level(&self) -> &crate::ton::int32 {
        match self {
            LogVerbosityLevel::LogVerbosityLevel(ref x) => &x.verbosity_level,
        }
    }

    pub fn only(self) -> crate::ton::logverbositylevel::LogVerbosityLevel {
        match self {
            LogVerbosityLevel::LogVerbosityLevel(x) => x,
        }
    }
}
impl Eq for LogVerbosityLevel {}
impl Default for LogVerbosityLevel {
    fn default() -> Self {
        LogVerbosityLevel::LogVerbosityLevel(
            crate::ton::logverbositylevel::LogVerbosityLevel::default(),
        )
    }
}
impl crate::BoxedSerialize for LogVerbosityLevel {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            LogVerbosityLevel::LogVerbosityLevel(x) => (crate::ConstructorNumber(0x676443ea), x),
        }
    }
}
impl crate::BoxedDeserialize for LogVerbosityLevel {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x676443ea)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x676443ea) => Ok(LogVerbosityLevel::LogVerbosityLevel(
                _de.read_bare::<crate::ton::logverbositylevel::LogVerbosityLevel>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `Ok`\n\n```text\nok = Ok;\n```\n"]
pub enum Ok {
    #[default]
    Ok,
}
impl Eq for Ok {}
impl crate::BoxedSerialize for Ok {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Ok::Ok => (crate::ConstructorNumber(0xd4edbe69), &()),
        }
    }
}
impl crate::BoxedDeserialize for Ok {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd4edbe69)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xd4edbe69) => Ok(Ok::Ok),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Options`\n\n```text\noptions config:config keystore_type:KeyStoreType = Options;\n```\n"]
pub enum Options {
    Options(crate::ton::options::Options),
}
impl Options {
    pub fn config(&self) -> &crate::ton::config::Config {
        match self {
            Options::Options(ref x) => &x.config,
        }
    }

    pub fn keystore_type(&self) -> &crate::ton::KeyStoreType {
        match self {
            Options::Options(ref x) => &x.keystore_type,
        }
    }

    pub fn only(self) -> crate::ton::options::Options {
        match self {
            Options::Options(x) => x,
        }
    }
}
impl Eq for Options {}
impl Default for Options {
    fn default() -> Self {
        Options::Options(crate::ton::options::Options::default())
    }
}
impl crate::BoxedSerialize for Options {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Options::Options(x) => (crate::ConstructorNumber(0x8d4c29f9), x),
        }
    }
}
impl crate::BoxedDeserialize for Options {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8d4c29f9)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8d4c29f9) => {
                Ok(Options::Options(_de.read_bare::<crate::ton::options::Options>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `PrivateKey`\n\n```text\npk.aes key:int256 = PrivateKey;\n\npk.ed25519 key:int256 = PrivateKey;\n\npk.overlay name:bytes = PrivateKey;\n\npk.unenc data:bytes = PrivateKey;\n```\n"]
pub enum PrivateKey {
    Pk_Aes(crate::ton::pk::privatekey::Aes),
    Pk_Ed25519(crate::ton::pk::privatekey::Ed25519),
    Pk_Overlay(crate::ton::pk::privatekey::Overlay),
    Pk_Unenc(crate::ton::pk::privatekey::Unenc),
}
impl PrivateKey {
    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            PrivateKey::Pk_Unenc(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn key(&self) -> Option<&crate::ton::int256> {
        match self {
            PrivateKey::Pk_Aes(ref x) => Some(&x.key),
            PrivateKey::Pk_Ed25519(ref x) => Some(&x.key),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&crate::ton::bytes> {
        match self {
            PrivateKey::Pk_Overlay(ref x) => Some(&x.name),
            _ => None,
        }
    }
}
impl Eq for PrivateKey {}
impl Default for PrivateKey {
    fn default() -> Self {
        PrivateKey::Pk_Aes(crate::ton::pk::privatekey::Aes::default())
    }
}
impl crate::BoxedSerialize for PrivateKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            PrivateKey::Pk_Aes(x) => (crate::ConstructorNumber(0xa5e85137), x),
            PrivateKey::Pk_Ed25519(x) => (crate::ConstructorNumber(0x49682317), x),
            PrivateKey::Pk_Overlay(x) => (crate::ConstructorNumber(0x37a5f65b), x),
            PrivateKey::Pk_Unenc(x) => (crate::ConstructorNumber(0xb1db9b30), x),
        }
    }
}
impl crate::BoxedDeserialize for PrivateKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0xa5e85137),
            crate::ConstructorNumber(0x49682317),
            crate::ConstructorNumber(0x37a5f65b),
            crate::ConstructorNumber(0xb1db9b30),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa5e85137) => {
                Ok(PrivateKey::Pk_Aes(_de.read_bare::<crate::ton::pk::privatekey::Aes>()?))
            }
            crate::ConstructorNumber(0x49682317) => {
                Ok(PrivateKey::Pk_Ed25519(_de.read_bare::<crate::ton::pk::privatekey::Ed25519>()?))
            }
            crate::ConstructorNumber(0x37a5f65b) => {
                Ok(PrivateKey::Pk_Overlay(_de.read_bare::<crate::ton::pk::privatekey::Overlay>()?))
            }
            crate::ConstructorNumber(0xb1db9b30) => {
                Ok(PrivateKey::Pk_Unenc(_de.read_bare::<crate::ton::pk::privatekey::Unenc>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `PublicKey`\n\n```text\npub.aes key:int256 = PublicKey;\n\npub.bls bls_key:bytes = PublicKey;\n\npub.ed25519 key:int256 = PublicKey;\n\npub.overlay name:bytes = PublicKey;\n\npub.unenc data:bytes = PublicKey;\n```\n"]
pub enum PublicKey {
    Pub_Aes(crate::ton::pub_::publickey::Aes),
    Pub_Bls(crate::ton::pub_::publickey::Bls),
    Pub_Ed25519(crate::ton::pub_::publickey::Ed25519),
    Pub_Overlay(crate::ton::pub_::publickey::Overlay),
    Pub_Unenc(crate::ton::pub_::publickey::Unenc),
}
impl PublicKey {
    pub fn bls_key(&self) -> Option<&crate::ton::bytes> {
        match self {
            PublicKey::Pub_Bls(ref x) => Some(&x.bls_key),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            PublicKey::Pub_Unenc(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn key(&self) -> Option<&crate::ton::int256> {
        match self {
            PublicKey::Pub_Aes(ref x) => Some(&x.key),
            PublicKey::Pub_Ed25519(ref x) => Some(&x.key),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&crate::ton::bytes> {
        match self {
            PublicKey::Pub_Overlay(ref x) => Some(&x.name),
            _ => None,
        }
    }
}
impl Eq for PublicKey {}
impl Default for PublicKey {
    fn default() -> Self {
        PublicKey::Pub_Aes(crate::ton::pub_::publickey::Aes::default())
    }
}
impl crate::BoxedSerialize for PublicKey {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            PublicKey::Pub_Aes(x) => (crate::ConstructorNumber(0x2dbcadd4), x),
            PublicKey::Pub_Bls(x) => (crate::ConstructorNumber(0x99d1d8e9), x),
            PublicKey::Pub_Ed25519(x) => (crate::ConstructorNumber(0x4813b4c6), x),
            PublicKey::Pub_Overlay(x) => (crate::ConstructorNumber(0x34ba45cb), x),
            PublicKey::Pub_Unenc(x) => (crate::ConstructorNumber(0xb61f450a), x),
        }
    }
}
impl crate::BoxedDeserialize for PublicKey {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x2dbcadd4),
            crate::ConstructorNumber(0x99d1d8e9),
            crate::ConstructorNumber(0x4813b4c6),
            crate::ConstructorNumber(0x34ba45cb),
            crate::ConstructorNumber(0xb61f450a),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x2dbcadd4) => {
                Ok(PublicKey::Pub_Aes(_de.read_bare::<crate::ton::pub_::publickey::Aes>()?))
            }
            crate::ConstructorNumber(0x99d1d8e9) => {
                Ok(PublicKey::Pub_Bls(_de.read_bare::<crate::ton::pub_::publickey::Bls>()?))
            }
            crate::ConstructorNumber(0x4813b4c6) => {
                Ok(PublicKey::Pub_Ed25519(_de.read_bare::<crate::ton::pub_::publickey::Ed25519>()?))
            }
            crate::ConstructorNumber(0x34ba45cb) => {
                Ok(PublicKey::Pub_Overlay(_de.read_bare::<crate::ton::pub_::publickey::Overlay>()?))
            }
            crate::ConstructorNumber(0xb61f450a) => {
                Ok(PublicKey::Pub_Unenc(_de.read_bare::<crate::ton::pub_::publickey::Unenc>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[doc = "TL-derived from `SyncState`\n\n```text\nsyncStateDone = SyncState;\n\nsyncStateInProgress from_seqno:int32 to_seqno:int32 current_seqno:int32 = SyncState;\n```\n"]
pub enum SyncState {
    #[default]
    SyncStateDone,
    SyncStateInProgress(crate::ton::syncstate::SyncStateInProgress),
}
impl SyncState {
    pub fn current_seqno(&self) -> Option<&crate::ton::int32> {
        match self {
            SyncState::SyncStateInProgress(ref x) => Some(&x.current_seqno),
            _ => None,
        }
    }

    pub fn from_seqno(&self) -> Option<&crate::ton::int32> {
        match self {
            SyncState::SyncStateInProgress(ref x) => Some(&x.from_seqno),
            _ => None,
        }
    }

    pub fn to_seqno(&self) -> Option<&crate::ton::int32> {
        match self {
            SyncState::SyncStateInProgress(ref x) => Some(&x.to_seqno),
            _ => None,
        }
    }
}
impl Eq for SyncState {}
impl crate::BoxedSerialize for SyncState {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            SyncState::SyncStateDone => (crate::ConstructorNumber(0x53f33909), &()),
            SyncState::SyncStateInProgress(x) => (crate::ConstructorNumber(0x066bc4c7), x),
        }
    }
}
impl crate::BoxedDeserialize for SyncState {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x53f33909), crate::ConstructorNumber(0x066bc4c7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x53f33909) => Ok(SyncState::SyncStateDone),
            crate::ConstructorNumber(0x066bc4c7) => Ok(SyncState::SyncStateInProgress(
                _de.read_bare::<crate::ton::syncstate::SyncStateInProgress>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
impl crate::BoxedSerialize for Option<crate::ton::syncstate::SyncStateInProgress> {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match *self {
            None => (crate::ConstructorNumber(0x53f33909), &()),
            Some(ref x) => (crate::ConstructorNumber(0x066bc4c7), x),
        }
    }
}
impl crate::BoxedDeserialize for Option<crate::ton::syncstate::SyncStateInProgress> {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x53f33909), crate::ConstructorNumber(0x066bc4c7)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x53f33909) => Ok(None),
            crate::ConstructorNumber(0x066bc4c7) => {
                Ok(Some(_de.read_bare::<crate::ton::syncstate::SyncStateInProgress>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `UnpackedAccountAddress`\n\n```text\nunpackedAccountAddress workchain_id:int32 bounceable:Bool testnet:Bool addr:bytes = UnpackedAccountAddress;\n```\n"]
pub enum UnpackedAccountAddress {
    UnpackedAccountAddress(crate::ton::unpackedaccountaddress::UnpackedAccountAddress),
}
impl UnpackedAccountAddress {
    pub fn addr(&self) -> &crate::ton::bytes {
        match self {
            UnpackedAccountAddress::UnpackedAccountAddress(ref x) => &x.addr,
        }
    }

    pub fn bounceable(&self) -> &crate::ton::Bool {
        match self {
            UnpackedAccountAddress::UnpackedAccountAddress(ref x) => &x.bounceable,
        }
    }

    pub fn testnet(&self) -> &crate::ton::Bool {
        match self {
            UnpackedAccountAddress::UnpackedAccountAddress(ref x) => &x.testnet,
        }
    }

    pub fn workchain_id(&self) -> &crate::ton::int32 {
        match self {
            UnpackedAccountAddress::UnpackedAccountAddress(ref x) => &x.workchain_id,
        }
    }

    pub fn only(self) -> crate::ton::unpackedaccountaddress::UnpackedAccountAddress {
        match self {
            UnpackedAccountAddress::UnpackedAccountAddress(x) => x,
        }
    }
}
impl Eq for UnpackedAccountAddress {}
impl Default for UnpackedAccountAddress {
    fn default() -> Self {
        UnpackedAccountAddress::UnpackedAccountAddress(
            crate::ton::unpackedaccountaddress::UnpackedAccountAddress::default(),
        )
    }
}
impl crate::BoxedSerialize for UnpackedAccountAddress {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            UnpackedAccountAddress::UnpackedAccountAddress(x) => {
                (crate::ConstructorNumber(0x70d41436), x)
            }
        }
    }
}
impl crate::BoxedDeserialize for UnpackedAccountAddress {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x70d41436)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x70d41436) => {
                Ok(UnpackedAccountAddress::UnpackedAccountAddress(
                    _de.read_bare::<crate::ton::unpackedaccountaddress::UnpackedAccountAddress>()?,
                ))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `Update`\n\n```text\nupdateSendLiteServerQuery id:int64 data:bytes = Update;\n\nupdateSyncState sync_state:SyncState = Update;\n```\n"]
pub enum Update {
    UpdateSendLiteServerQuery(crate::ton::update::UpdateSendLiteServerQuery),
    UpdateSyncState(crate::ton::update::UpdateSyncState),
}
impl Update {
    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            Update::UpdateSendLiteServerQuery(ref x) => Some(&x.data),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&crate::ton::int64> {
        match self {
            Update::UpdateSendLiteServerQuery(ref x) => Some(&x.id),
            _ => None,
        }
    }

    pub fn sync_state(&self) -> Option<&crate::ton::SyncState> {
        match self {
            Update::UpdateSyncState(ref x) => Some(&x.sync_state),
            _ => None,
        }
    }
}
impl Eq for Update {}
impl Default for Update {
    fn default() -> Self {
        Update::UpdateSendLiteServerQuery(crate::ton::update::UpdateSendLiteServerQuery::default())
    }
}
impl crate::BoxedSerialize for Update {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Update::UpdateSendLiteServerQuery(x) => (crate::ConstructorNumber(0xa34e95dc), x),
            Update::UpdateSyncState(x) => (crate::ConstructorNumber(0x47c823de), x),
        }
    }
}
impl crate::BoxedDeserialize for Update {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa34e95dc), crate::ConstructorNumber(0x47c823de)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa34e95dc) => Ok(Update::UpdateSendLiteServerQuery(
                _de.read_bare::<crate::ton::update::UpdateSendLiteServerQuery>()?,
            )),
            crate::ConstructorNumber(0x47c823de) => {
                Ok(Update::UpdateSyncState(_de.read_bare::<crate::ton::update::UpdateSyncState>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod accountaddress;
pub mod accountlist;
pub mod accountrevisionlist;
pub mod action;
pub mod adnl;
pub mod adnladdress;
pub mod bip39hints;
pub mod bool;
pub mod catchain;
pub mod config;
pub mod control;
pub mod data;
pub mod db;
pub mod dht;
pub mod dns;
pub mod dummyworkchain0;
pub mod dynamic;
pub mod engine;
pub mod error;
pub mod exportedencryptedkey;
pub mod exportedkey;
pub mod exportedpemkey;
pub mod exportedunencryptedkey;
pub mod fec;
pub mod fees;
pub mod fullaccountstate;
pub mod hashable;
pub mod http;
pub mod id;
pub mod inputkey;
pub mod int8;
pub mod internal;
pub mod key;
pub mod keystoretype;
pub mod lite_server;
pub mod liteclient;
pub mod liteserver;
pub mod logstream;
pub mod logtags;
pub mod logverbositylevel;
pub mod mbpp;
pub mod msg;
pub mod ok;
pub mod options;
pub mod overlay;
pub mod pchan;
pub mod pk;
pub mod pub_;
pub mod query;
pub mod raw;
pub mod rldp;
pub mod rpc;
pub mod rwallet;
pub mod smc;
pub mod syncstate;
pub mod tcp;
pub mod test_giver;
pub mod test_wallet;
pub mod ton;
pub mod ton_engine;
pub mod ton_node;
pub mod tvm;
pub mod uninited;
pub mod unpackedaccountaddress;
pub mod update;
pub mod validator;
pub mod validator_session;
pub mod wallet;
