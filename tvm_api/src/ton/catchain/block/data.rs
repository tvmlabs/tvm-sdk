use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `catchain.block.data`\n\n```text\ncatchain.block.data prev:catchain.block.dep deps:(vector catchain.block.dep) = catchain.block.Data;\n```\n"]
pub struct Data {
    pub prev: crate::ton::catchain::block::dep::Dep,
    pub deps: crate::ton::vector<crate::ton::Bare, crate::ton::catchain::block::dep::Dep>,
}
impl Eq for Data {}
impl crate::BareSerialize for Data {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xf8aca620)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Data { prev, deps } = self;
        _ser.write_bare::<crate::ton::catchain::block::dep::Dep>(prev)?;
        _ser . write_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: catchain :: block :: dep :: Dep > > (deps) ? ;
        Ok(())
    }
}
impl crate::BareDeserialize for Data {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let prev = _de.read_bare::<crate::ton::catchain::block::dep::Dep>()?;
            let deps = _de . read_bare :: < crate :: ton :: vector < crate :: ton :: Bare , crate :: ton :: catchain :: block :: dep :: Dep > > () ? ;
            Ok(Self { prev, deps })
        }
    }
}
impl crate::IntoBoxed for Data {
    type Boxed = crate::ton::catchain::block::Data;

    fn into_boxed(self) -> crate::ton::catchain::block::Data {
        crate::ton::catchain::block::Data::Catchain_Block_Data(self)
    }
}
