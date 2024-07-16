use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.id`\n\n```text\ncatchain.block.id incarnation:int256 src:int256 height:int data_hash:int256 = catchain.block.Id;\n```\n"]
pub struct Id {
    pub incarnation: crate::ton::int256,
    pub src: crate::ton::int256,
    pub height: crate::ton::int,
    pub data_hash: crate::ton::int256,
}
impl Eq for Id {}
impl crate::BareSerialize for Id {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x24fe98ba)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Id { incarnation, src, height, data_hash } = self;
        _ser.write_bare::<crate::ton::int256>(incarnation)?;
        _ser.write_bare::<crate::ton::int256>(src)?;
        _ser.write_bare::<crate::ton::int>(height)?;
        _ser.write_bare::<crate::ton::int256>(data_hash)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Id {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let incarnation = _de.read_bare::<crate::ton::int256>()?;
            let src = _de.read_bare::<crate::ton::int256>()?;
            let height = _de.read_bare::<crate::ton::int>()?;
            let data_hash = _de.read_bare::<crate::ton::int256>()?;
            Ok(Self { incarnation, src, height, data_hash })
        }
    }
}
impl crate::IntoBoxed for Id {
    type Boxed = crate::ton::catchain::block::Id;

    fn into_boxed(self) -> crate::ton::catchain::block::Id {
        crate::ton::catchain::block::Id::Catchain_Block_Id(self)
    }
}
