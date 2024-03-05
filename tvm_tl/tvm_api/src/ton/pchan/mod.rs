use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.Action`\n\n```text\npchan.actionClose extra_A:int64 extra_B:int64 promise:pchan.promise = pchan.Action;\n\npchan.actionInit inc_A:int64 inc_B:int64 min_A:int64 min_B:int64 = pchan.Action;\n\npchan.actionTimeout = pchan.Action;\n```\n"]
pub enum Action {
    Pchan_ActionClose(crate::ton::pchan::action::ActionClose),
    Pchan_ActionInit(crate::ton::pchan::action::ActionInit),
    Pchan_ActionTimeout,
}
impl Action {
    pub fn extra_A(&self) -> Option<&crate::ton::int64> {
        match self {
            Action::Pchan_ActionClose(ref x) => Some(&x.extra_A),
            _ => None,
        }
    }

    pub fn extra_B(&self) -> Option<&crate::ton::int64> {
        match self {
            Action::Pchan_ActionClose(ref x) => Some(&x.extra_B),
            _ => None,
        }
    }

    pub fn inc_A(&self) -> Option<&crate::ton::int64> {
        match self {
            Action::Pchan_ActionInit(ref x) => Some(&x.inc_A),
            _ => None,
        }
    }

    pub fn inc_B(&self) -> Option<&crate::ton::int64> {
        match self {
            Action::Pchan_ActionInit(ref x) => Some(&x.inc_B),
            _ => None,
        }
    }

    pub fn min_A(&self) -> Option<&crate::ton::int64> {
        match self {
            Action::Pchan_ActionInit(ref x) => Some(&x.min_A),
            _ => None,
        }
    }

    pub fn min_B(&self) -> Option<&crate::ton::int64> {
        match self {
            Action::Pchan_ActionInit(ref x) => Some(&x.min_B),
            _ => None,
        }
    }

    pub fn promise(&self) -> Option<&crate::ton::pchan::promise::Promise> {
        match self {
            Action::Pchan_ActionClose(ref x) => Some(&x.promise),
            _ => None,
        }
    }
}
impl Eq for Action {}
impl Default for Action {
    fn default() -> Self {
        Action::Pchan_ActionClose(crate::ton::pchan::action::ActionClose::default())
    }
}
impl crate::BoxedSerialize for Action {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Action::Pchan_ActionClose(x) => (crate::ConstructorNumber(0x639c4b16), x),
            Action::Pchan_ActionInit(x) => (crate::ConstructorNumber(0x1a2bf68a), x),
            Action::Pchan_ActionTimeout => (crate::ConstructorNumber(0x771e80f3), &()),
        }
    }
}
impl crate::BoxedDeserialize for Action {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x639c4b16),
            crate::ConstructorNumber(0x1a2bf68a),
            crate::ConstructorNumber(0x771e80f3),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x639c4b16) => Ok(Action::Pchan_ActionClose(
                _de.read_bare::<crate::ton::pchan::action::ActionClose>()?,
            )),
            crate::ConstructorNumber(0x1a2bf68a) => Ok(Action::Pchan_ActionInit(
                _de.read_bare::<crate::ton::pchan::action::ActionInit>()?,
            )),
            crate::ConstructorNumber(0x771e80f3) => Ok(Action::Pchan_ActionTimeout),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.Config`\n\n```text\npchan.config alice_public_key:string alice_address:accountAddress bob_public_key:string bob_address:accountAddress init_timeout:int32 close_timeout:int32 channel_id:int64 = pchan.Config;\n```\n"]
pub enum Config {
    Pchan_Config(crate::ton::pchan::config::Config),
}
impl Config {
    pub fn alice_address(&self) -> &crate::ton::accountaddress::AccountAddress {
        match self {
            Config::Pchan_Config(ref x) => &x.alice_address,
        }
    }

    pub fn alice_public_key(&self) -> &crate::ton::string {
        match self {
            Config::Pchan_Config(ref x) => &x.alice_public_key,
        }
    }

    pub fn bob_address(&self) -> &crate::ton::accountaddress::AccountAddress {
        match self {
            Config::Pchan_Config(ref x) => &x.bob_address,
        }
    }

    pub fn bob_public_key(&self) -> &crate::ton::string {
        match self {
            Config::Pchan_Config(ref x) => &x.bob_public_key,
        }
    }

    pub fn channel_id(&self) -> &crate::ton::int64 {
        match self {
            Config::Pchan_Config(ref x) => &x.channel_id,
        }
    }

    pub fn close_timeout(&self) -> &crate::ton::int32 {
        match self {
            Config::Pchan_Config(ref x) => &x.close_timeout,
        }
    }

    pub fn init_timeout(&self) -> &crate::ton::int32 {
        match self {
            Config::Pchan_Config(ref x) => &x.init_timeout,
        }
    }

    pub fn only(self) -> crate::ton::pchan::config::Config {
        match self {
            Config::Pchan_Config(x) => x,
        }
    }
}
impl Eq for Config {}
impl Default for Config {
    fn default() -> Self {
        Config::Pchan_Config(crate::ton::pchan::config::Config::default())
    }
}
impl crate::BoxedSerialize for Config {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Config::Pchan_Config(x) => (crate::ConstructorNumber(0x8486f436), x),
        }
    }
}
impl crate::BoxedDeserialize for Config {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x8486f436)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x8486f436) => {
                Ok(Config::Pchan_Config(_de.read_bare::<crate::ton::pchan::config::Config>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.Promise`\n\n```text\npchan.promise signature:bytes promise_A:int64 promise_B:int64 channel_id:int64 = pchan.Promise;\n```\n"]
pub enum Promise {
    Pchan_Promise(crate::ton::pchan::promise::Promise),
}
impl Promise {
    pub fn channel_id(&self) -> &crate::ton::int64 {
        match self {
            Promise::Pchan_Promise(ref x) => &x.channel_id,
        }
    }

    pub fn promise_A(&self) -> &crate::ton::int64 {
        match self {
            Promise::Pchan_Promise(ref x) => &x.promise_A,
        }
    }

    pub fn promise_B(&self) -> &crate::ton::int64 {
        match self {
            Promise::Pchan_Promise(ref x) => &x.promise_B,
        }
    }

    pub fn signature(&self) -> &crate::ton::bytes {
        match self {
            Promise::Pchan_Promise(ref x) => &x.signature,
        }
    }

    pub fn only(self) -> crate::ton::pchan::promise::Promise {
        match self {
            Promise::Pchan_Promise(x) => x,
        }
    }
}
impl Eq for Promise {}
impl Default for Promise {
    fn default() -> Self {
        Promise::Pchan_Promise(crate::ton::pchan::promise::Promise::default())
    }
}
impl crate::BoxedSerialize for Promise {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Promise::Pchan_Promise(x) => (crate::ConstructorNumber(0xa20e945d), x),
        }
    }
}
impl crate::BoxedDeserialize for Promise {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xa20e945d)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xa20e945d) => {
                Ok(Promise::Pchan_Promise(_de.read_bare::<crate::ton::pchan::promise::Promise>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.State`\n\n```text\npchan.stateClose signed_A:Bool signed_B:Bool min_A:int64 min_B:int64 expire_at:int53 A:int64 B:int64 = pchan.State;\n\npchan.stateInit signed_A:Bool signed_B:Bool min_A:int64 min_B:int64 expire_at:int53 A:int64 B:int64 = pchan.State;\n\npchan.statePayout A:int64 B:int64 = pchan.State;\n```\n"]
pub enum State {
    Pchan_StateClose(crate::ton::pchan::state::StateClose),
    Pchan_StateInit(crate::ton::pchan::state::StateInit),
    Pchan_StatePayout(crate::ton::pchan::state::StatePayout),
}
impl State {
    pub fn A(&self) -> &crate::ton::int64 {
        match self {
            State::Pchan_StateClose(ref x) => &x.A,
            State::Pchan_StateInit(ref x) => &x.A,
            State::Pchan_StatePayout(ref x) => &x.A,
        }
    }

    pub fn B(&self) -> &crate::ton::int64 {
        match self {
            State::Pchan_StateClose(ref x) => &x.B,
            State::Pchan_StateInit(ref x) => &x.B,
            State::Pchan_StatePayout(ref x) => &x.B,
        }
    }

    pub fn expire_at(&self) -> Option<&crate::ton::int53> {
        match self {
            State::Pchan_StateClose(ref x) => Some(&x.expire_at),
            State::Pchan_StateInit(ref x) => Some(&x.expire_at),
            _ => None,
        }
    }

    pub fn min_A(&self) -> Option<&crate::ton::int64> {
        match self {
            State::Pchan_StateClose(ref x) => Some(&x.min_A),
            State::Pchan_StateInit(ref x) => Some(&x.min_A),
            _ => None,
        }
    }

    pub fn min_B(&self) -> Option<&crate::ton::int64> {
        match self {
            State::Pchan_StateClose(ref x) => Some(&x.min_B),
            State::Pchan_StateInit(ref x) => Some(&x.min_B),
            _ => None,
        }
    }

    pub fn signed_A(&self) -> Option<&crate::ton::Bool> {
        match self {
            State::Pchan_StateClose(ref x) => Some(&x.signed_A),
            State::Pchan_StateInit(ref x) => Some(&x.signed_A),
            _ => None,
        }
    }

    pub fn signed_B(&self) -> Option<&crate::ton::Bool> {
        match self {
            State::Pchan_StateClose(ref x) => Some(&x.signed_B),
            State::Pchan_StateInit(ref x) => Some(&x.signed_B),
            _ => None,
        }
    }
}
impl Eq for State {}
impl Default for State {
    fn default() -> Self {
        State::Pchan_StateClose(crate::ton::pchan::state::StateClose::default())
    }
}
impl crate::BoxedSerialize for State {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            State::Pchan_StateClose(x) => (crate::ConstructorNumber(0x34e201f3), x),
            State::Pchan_StateInit(x) => (crate::ConstructorNumber(0xb92a0cf8), x),
            State::Pchan_StatePayout(x) => (crate::ConstructorNumber(0x279e1447), x),
        }
    }
}
impl crate::BoxedDeserialize for State {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x34e201f3),
            crate::ConstructorNumber(0xb92a0cf8),
            crate::ConstructorNumber(0x279e1447),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x34e201f3) => Ok(State::Pchan_StateClose(
                _de.read_bare::<crate::ton::pchan::state::StateClose>()?,
            )),
            crate::ConstructorNumber(0xb92a0cf8) => {
                Ok(State::Pchan_StateInit(_de.read_bare::<crate::ton::pchan::state::StateInit>()?))
            }
            crate::ConstructorNumber(0x279e1447) => Ok(State::Pchan_StatePayout(
                _de.read_bare::<crate::ton::pchan::state::StatePayout>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod accountstate;
pub mod action;
pub mod config;
pub mod initialaccountstate;
pub mod promise;
pub mod state;
