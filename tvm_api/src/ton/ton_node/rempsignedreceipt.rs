use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.rempSignedReceipt`\n\n```text\ntonNode.rempSignedReceipt receipt:bytes signature:int512 = tonNode.RempSignedReceipt;\n```\n"]
pub struct RempSignedReceipt {
    pub receipt: crate::ton::bytes,
    pub signature: crate::ton::int512,
}
impl Eq for RempSignedReceipt {}
impl crate::BareSerialize for RempSignedReceipt {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb361c8dd)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RempSignedReceipt { receipt, signature } = self;
        _ser.write_bare::<crate::ton::bytes>(receipt)?;
        _ser.write_bare::<crate::ton::int512>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RempSignedReceipt {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let receipt = _de.read_bare::<crate::ton::bytes>()?;
            let signature = _de.read_bare::<crate::ton::int512>()?;
            Ok(Self { receipt, signature })
        }
    }
}
impl crate::IntoBoxed for RempSignedReceipt {
    type Boxed = crate::ton::ton_node::RempSignedReceipt;

    fn into_boxed(self) -> crate::ton::ton_node::RempSignedReceipt {
        crate::ton::ton_node::RempSignedReceipt::TonNode_RempSignedReceipt(self)
    }
}
