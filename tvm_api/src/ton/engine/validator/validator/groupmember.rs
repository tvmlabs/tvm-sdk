use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `validator.groupMember`\n\n```text\nvalidator.groupMember public_key_hash:int256 adnl:int256 weight:long = engine.validator.GroupMember;\n```\n"]
pub struct GroupMember {
    pub public_key_hash: crate::ton::int256,
    pub adnl: crate::ton::int256,
    pub weight: crate::ton::long,
}
impl Eq for GroupMember {}
impl crate::BareSerialize for GroupMember {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8b9465e4)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GroupMember { public_key_hash, adnl, weight } = self;
        _ser.write_bare::<crate::ton::int256>(public_key_hash)?;
        _ser.write_bare::<crate::ton::int256>(adnl)?;
        _ser.write_bare::<crate::ton::long>(weight)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GroupMember {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let public_key_hash = _de.read_bare::<crate::ton::int256>()?;
            let adnl = _de.read_bare::<crate::ton::int256>()?;
            let weight = _de.read_bare::<crate::ton::long>()?;
            Ok(Self { public_key_hash, adnl, weight })
        }
    }
}
impl crate::IntoBoxed for GroupMember {
    type Boxed = crate::ton::engine::validator::GroupMember;

    fn into_boxed(self) -> crate::ton::engine::validator::GroupMember {
        crate::ton::engine::validator::GroupMember::Validator_GroupMember(self)
    }
}
