use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `engine.validator.dhtServersStatus`\n\n```text\nengine.validator.dhtServersStatus servers:(vector engine.validator.dhtServerStatus) = engine.validator.DhtServersStatus;\n```\n"]
pub struct DhtServersStatus {
    pub servers:
        crate::ton::vector<crate::ton::engine::validator::dhtserverstatus::DhtServerStatus>,
}
impl Eq for DhtServersStatus {}
impl crate::BareSerialize for DhtServersStatus {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2b38fd28)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DhtServersStatus { servers } = self;
        (servers
            as &dyn crate::ton::VectoredBare<
                crate::ton::engine::validator::dhtserverstatus::DhtServerStatus,
            >)
            .serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DhtServersStatus {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let servers = < Vec < crate :: ton :: engine :: validator :: dhtserverstatus :: DhtServerStatus > as crate :: ton :: VectoredBare < crate :: ton :: engine :: validator :: dhtserverstatus :: DhtServerStatus >> :: deserialize (_de) ? ;
            Ok(Self { servers })
        }
    }
}
impl crate::IntoBoxed for DhtServersStatus {
    type Boxed = crate::ton::engine::validator::DhtServersStatus;

    fn into_boxed(self) -> crate::ton::engine::validator::DhtServersStatus {
        crate::ton::engine::validator::DhtServersStatus::Engine_Validator_DhtServersStatus(self)
    }
}
