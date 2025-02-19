use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `liteServer.accountId`\n\n```text\nliteServer.accountId workchain:int id:int256 = liteServer.AccountId;\n```\n"]
pub struct AccountId {
    pub workchain: crate::ton::int,
    pub id: crate::ton::int256,
}
impl Eq for AccountId {}
impl crate::BareSerialize for AccountId {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x75a0e2c5)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let AccountId { workchain, id } = self;
        _ser.write_bare::<crate::ton::int>(workchain)?;
        _ser.write_bare::<crate::ton::int256>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for AccountId {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let workchain = _de.read_bare::<crate::ton::int>()?;
            let id = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { workchain, id })
        }
    }
}
impl crate::IntoBoxed for AccountId {
    type Boxed = crate::ton::lite_server::AccountId;

    fn into_boxed(self) -> crate::ton::lite_server::AccountId {
        crate::ton::lite_server::AccountId::LiteServer_AccountId(self)
    }
}
