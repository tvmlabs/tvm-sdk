use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.blockSignature`\n\n```text\ntonNode.blockSignature who:int256 signature:bytes = tonNode.BlockSignature;\n```\n"]
pub struct BlockSignature {
    pub who: crate::ton::int256,
    pub signature: crate::ton::bytes,
}
impl Eq for BlockSignature {}
impl crate::Signing for BlockSignature {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for BlockSignature {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x50f03c33)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BlockSignature { who, signature } = self;
        _ser.write_bare::<crate::ton::int256>(who)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BlockSignature {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let who = _de.read_bare::<crate::ton::int256>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { who, signature })
        }
    }
}
impl crate::IntoBoxed for BlockSignature {
    type Boxed = crate::ton::ton_node::BlockSignature;
    fn into_boxed(self) -> crate::ton::ton_node::BlockSignature {
        crate::ton::ton_node::BlockSignature::TonNode_BlockSignature(self)
    }
}
