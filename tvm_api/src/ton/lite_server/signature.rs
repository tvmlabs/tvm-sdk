use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.signature`\n\n```text\nliteServer.signature node_id_short:int256 signature:bytes = liteServer.Signature;\n```\n"]
pub struct Signature {
    pub node_id_short: crate::ton::int256,
    pub signature: crate::ton::bytes,
}
impl Eq for Signature {}
impl crate::Signing for Signature {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Signature {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa3def855)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Signature { node_id_short, signature } = self;
        _ser.write_bare::<crate::ton::int256>(node_id_short)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Signature {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let node_id_short = _de.read_bare::<crate::ton::int256>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { node_id_short, signature })
        }
    }
}
impl crate::IntoBoxed for Signature {
    type Boxed = crate::ton::lite_server::Signature;

    fn into_boxed(self) -> crate::ton::lite_server::Signature {
        crate::ton::lite_server::Signature::LiteServer_Signature(self)
    }
}
