use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.db.key.bucket`\n\n```text\ndht.db.key.bucket id:int = dht.db.Key;\n```\n"]
pub struct Bucket {
    pub id: crate::ton::int,
}
impl Eq for Bucket {}
impl crate::BareSerialize for Bucket {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa368ae4c)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Bucket { id } = self;
        _ser.write_bare::<crate::ton::int>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Bucket {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id })
        }
    }
}
impl crate::IntoBoxed for Bucket {
    type Boxed = crate::ton::dht::db::Key;

    fn into_boxed(self) -> crate::ton::dht::db::Key {
        crate::ton::dht::db::Key::Dht_Db_Key_Bucket(self)
    }
}
