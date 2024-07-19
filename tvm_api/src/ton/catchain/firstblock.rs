use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.firstblock`\n\n```text\ncatchain.firstblock unique_hash:int256 nodes:(vector int256) = catchain.FirstBlock;\n```\n"]
pub struct Firstblock {
    pub unique_hash: crate::ton::int256,
    pub nodes: crate::ton::vector<crate::ton::int256>,
}
impl Eq for Firstblock {}
impl crate::BareSerialize for Firstblock {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x10c904fb)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Firstblock { unique_hash, nodes } = self;
        _ser.write_bare::<crate::ton::int256>(unique_hash)?;
        (nodes as &dyn crate::ton::VectoredBare<crate::ton::int256>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Firstblock {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let unique_hash = _de.read_bare::<crate::ton::int256>()?;
            let nodes = <Vec<crate::ton::int256> as crate::ton::VectoredBare<
                crate::ton::int256,
            >>::deserialize(_de)?;
            Ok(Self { unique_hash, nodes })
        }
    }
}
impl crate::IntoBoxed for Firstblock {
    type Boxed = crate::ton::catchain::FirstBlock;

    fn into_boxed(self) -> crate::ton::catchain::FirstBlock {
        crate::ton::catchain::FirstBlock::Catchain_Firstblock(self)
    }
}
