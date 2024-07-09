use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `accountAddress`\n\n```text\naccountAddress account_address:string = AccountAddress;\n```\n"]
pub struct AccountAddress {
    pub account_address: crate::ton::string,
}
impl Eq for AccountAddress {}
impl crate::BareSerialize for AccountAddress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2d09bdab)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountAddress { account_address } = self;
        _ser.write_bare::<crate::ton::string>(account_address)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountAddress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let account_address = _de.read_bare::<crate::ton::string>()?;
            Ok(Self { account_address })
        }
    }
}
impl crate::IntoBoxed for AccountAddress {
    type Boxed = crate::ton::AccountAddress;
    fn into_boxed(self) -> crate::ton::AccountAddress {
        crate::ton::AccountAddress::AccountAddress(self)
    }
}
