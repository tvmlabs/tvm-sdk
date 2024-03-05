use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tcp.Message`\n\n```text\ntcp.authentificate nonce:bytes = tcp.Message;\n\ntcp.authentificationComplete key:PublicKey signature:bytes = tcp.Message;\n\ntcp.authentificationNonce nonce:bytes = tcp.Message;\n```\n"]
pub enum Message {
    Tcp_Authentificate(crate::ton::tcp::message::Authentificate),
    Tcp_AuthentificationComplete(crate::ton::tcp::message::AuthentificationComplete),
    Tcp_AuthentificationNonce(crate::ton::tcp::message::AuthentificationNonce),
}
impl Message {
    pub fn key(&self) -> Option<&crate::ton::PublicKey> {
        match self {
            Message::Tcp_AuthentificationComplete(ref x) => Some(&x.key),
            _ => None,
        }
    }

    pub fn nonce(&self) -> Option<&crate::ton::bytes> {
        match self {
            Message::Tcp_Authentificate(ref x) => Some(&x.nonce),
            Message::Tcp_AuthentificationNonce(ref x) => Some(&x.nonce),
            _ => None,
        }
    }

    pub fn signature(&self) -> Option<&crate::ton::bytes> {
        match self {
            Message::Tcp_AuthentificationComplete(ref x) => Some(&x.signature),
            _ => None,
        }
    }
}
impl Eq for Message {}
impl Default for Message {
    fn default() -> Self {
        Message::Tcp_Authentificate(crate::ton::tcp::message::Authentificate::default())
    }
}
impl crate::BoxedSerialize for Message {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Message::Tcp_Authentificate(x) => (crate::ConstructorNumber(0x445bab12), x),
            Message::Tcp_AuthentificationComplete(x) => (crate::ConstructorNumber(0xf7ad9ea6), x),
            Message::Tcp_AuthentificationNonce(x) => (crate::ConstructorNumber(0xe35d4ab6), x),
        }
    }
}
impl crate::BoxedDeserialize for Message {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![
            crate::ConstructorNumber(0x445bab12),
            crate::ConstructorNumber(0xf7ad9ea6),
            crate::ConstructorNumber(0xe35d4ab6),
        ]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x445bab12) => Ok(Message::Tcp_Authentificate(
                _de.read_bare::<crate::ton::tcp::message::Authentificate>()?,
            )),
            crate::ConstructorNumber(0xf7ad9ea6) => Ok(Message::Tcp_AuthentificationComplete(
                _de.read_bare::<crate::ton::tcp::message::AuthentificationComplete>()?,
            )),
            crate::ConstructorNumber(0xe35d4ab6) => Ok(Message::Tcp_AuthentificationNonce(
                _de.read_bare::<crate::ton::tcp::message::AuthentificationNonce>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `tcp.Pong`\n\n```text\ntcp.pong random_id:long = tcp.Pong;\n```\n"]
pub enum Pong {
    Tcp_Pong(crate::ton::tcp::pong::Pong),
}
impl Pong {
    pub fn random_id(&self) -> &crate::ton::long {
        match self {
            Pong::Tcp_Pong(ref x) => &x.random_id,
        }
    }

    pub fn only(self) -> crate::ton::tcp::pong::Pong {
        match self {
            Pong::Tcp_Pong(x) => x,
        }
    }
}
impl Eq for Pong {}
impl Default for Pong {
    fn default() -> Self {
        Pong::Tcp_Pong(crate::ton::tcp::pong::Pong::default())
    }
}
impl crate::BoxedSerialize for Pong {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Pong::Tcp_Pong(x) => (crate::ConstructorNumber(0xdc69fb03), x),
        }
    }
}
impl crate::BoxedDeserialize for Pong {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xdc69fb03)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xdc69fb03) => {
                Ok(Pong::Tcp_Pong(_de.read_bare::<crate::ton::tcp::pong::Pong>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
pub mod message;
pub mod pong;
