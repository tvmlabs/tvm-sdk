use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.difference`\n\n```text\ncatchain.difference sent_upto:(vector int) = catchain.Difference;\n```\n"]
pub struct Difference {
    pub sent_upto: crate::ton::vector<crate::ton::Bare, crate::ton::int>,
}
impl Eq for Difference {}
impl crate::BareSerialize for Difference {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1415d1ca)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Difference { sent_upto } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>(sent_upto)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Difference {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let sent_upto =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int>>()?;
            Ok(Self { sent_upto })
        }
    }
}
impl crate::IntoBoxed for Difference {
    type Boxed = crate::ton::catchain::Difference;

    fn into_boxed(self) -> crate::ton::catchain::Difference {
        crate::ton::catchain::Difference::Catchain_Difference(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.differenceFork`\n\n```text\ncatchain.differenceFork left:catchain.block.dep right:catchain.block.dep = catchain.Difference;\n```\n"]
pub struct DifferenceFork {
    pub left: crate::ton::catchain::block::dep::Dep,
    pub right: crate::ton::catchain::block::dep::Dep,
}
impl Eq for DifferenceFork {}
impl crate::BareSerialize for DifferenceFork {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x4927c06f)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let DifferenceFork { left, right } = self;
        _ser.write_bare::<crate::ton::catchain::block::dep::Dep>(left)?;
        _ser.write_bare::<crate::ton::catchain::block::dep::Dep>(right)?;
        Ok(())
    }
}
impl crate::BareDeserialize for DifferenceFork {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let left = _de.read_bare::<crate::ton::catchain::block::dep::Dep>()?;
            let right = _de.read_bare::<crate::ton::catchain::block::dep::Dep>()?;
            Ok(Self { left, right })
        }
    }
}
impl crate::IntoBoxed for DifferenceFork {
    type Boxed = crate::ton::catchain::Difference;

    fn into_boxed(self) -> crate::ton::catchain::Difference {
        crate::ton::catchain::Difference::Catchain_DifferenceFork(self)
    }
}
