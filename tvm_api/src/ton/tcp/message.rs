use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tcp.authentificate`\n\n```text\ntcp.authentificate nonce:bytes = tcp.Message;\n```\n"]
pub struct Authentificate {
    pub nonce: crate::ton::bytes,
}
impl Eq for Authentificate {}
impl crate::BareSerialize for Authentificate {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x445bab12)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Authentificate { nonce } = self;
        _ser.write_bare::<crate::ton::bytes>(nonce)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Authentificate {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nonce = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { nonce })
        }
    }
}
impl crate::IntoBoxed for Authentificate {
    type Boxed = crate::ton::tcp::Message;
    fn into_boxed(self) -> crate::ton::tcp::Message {
        crate::ton::tcp::Message::Tcp_Authentificate(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tcp.authentificationComplete`\n\n```text\ntcp.authentificationComplete key:PublicKey signature:bytes = tcp.Message;\n```\n"]
pub struct AuthentificationComplete {
    pub key: crate::ton::PublicKey,
    pub signature: crate::ton::bytes,
}
impl Eq for AuthentificationComplete {}
impl crate::Signing for AuthentificationComplete {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for AuthentificationComplete {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf7ad9ea6)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AuthentificationComplete { key, signature } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(key)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AuthentificationComplete {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let key = _de.read_boxed::<crate::ton::PublicKey>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { key, signature })
        }
    }
}
impl crate::IntoBoxed for AuthentificationComplete {
    type Boxed = crate::ton::tcp::Message;
    fn into_boxed(self) -> crate::ton::tcp::Message {
        crate::ton::tcp::Message::Tcp_AuthentificationComplete(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tcp.authentificationNonce`\n\n```text\ntcp.authentificationNonce nonce:bytes = tcp.Message;\n```\n"]
pub struct AuthentificationNonce {
    pub nonce: crate::ton::bytes,
}
impl Eq for AuthentificationNonce {}
impl crate::BareSerialize for AuthentificationNonce {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xe35d4ab6)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AuthentificationNonce { nonce } = self;
        _ser.write_bare::<crate::ton::bytes>(nonce)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AuthentificationNonce {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nonce = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { nonce })
        }
    }
}
impl crate::IntoBoxed for AuthentificationNonce {
    type Boxed = crate::ton::tcp::Message;
    fn into_boxed(self) -> crate::ton::tcp::Message {
        crate::ton::tcp::Message::Tcp_AuthentificationNonce(self)
    }
}
