use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `adnlAddress`\n\n```text\nadnlAddress adnl_address:string = AdnlAddress;\n```\n"]
pub struct AdnlAddress {
    pub adnl_address: crate::ton::string,
}
impl Eq for AdnlAddress {}
impl crate::BareSerialize for AdnlAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x0431950c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AdnlAddress { adnl_address } = self;
        _ser.write_bare::<crate::ton::string>(adnl_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AdnlAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let adnl_address = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { adnl_address })
        }
    }
}
impl crate::IntoBoxed for AdnlAddress {
    type Boxed = crate::ton::AdnlAddress;

    fn into_boxed(self) -> crate::ton::AdnlAddress {
        crate::ton::AdnlAddress::AdnlAddress(self)
    }
}
