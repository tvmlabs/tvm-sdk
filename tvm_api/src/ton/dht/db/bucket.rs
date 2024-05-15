use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.db.bucket`\n\n```text\ndht.db.bucket nodes:dht.nodes = dht.db.Bucket;\n```\n"]
pub struct Bucket {
    pub nodes: crate::ton::dht::nodes::Nodes,
}
impl Eq for Bucket {}
impl crate::BareSerialize for Bucket {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb39cfa6c)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Bucket { nodes } = self;
        _ser.write_bare::<crate::ton::dht::nodes::Nodes>(nodes)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Bucket {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let nodes = _de.read_bare::<crate::ton::dht::nodes::Nodes>()?;
            Ok(Self { nodes })
        }
    }
}
impl crate::IntoBoxed for Bucket {
    type Boxed = crate::ton::dht::db::Bucket;
    fn into_boxed(self) -> crate::ton::dht::db::Bucket {
        crate::ton::dht::db::Bucket::Dht_Db_Bucket(self)
    }
}
