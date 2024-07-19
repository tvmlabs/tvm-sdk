use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `db.files.index.value`\n\n```text\ndb.files.index.value packages:(vector int) key_packages:(vector int) temp_packages:(vector int) = db.files.index.Value;\n```\n"]
pub struct Value {
    pub packages: crate::ton::vector<crate::ton::int>,
    pub key_packages: crate::ton::vector<crate::ton::int>,
    pub temp_packages: crate::ton::vector<crate::ton::int>,
}
impl Eq for Value {}
impl crate::BareSerialize for Value {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa2b1dafc)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Value { packages, key_packages, temp_packages } = self;
        (packages as &dyn crate::ton::VectoredBare<crate::ton::int>).serialize(_ser)?;
        (key_packages as &dyn crate::ton::VectoredBare<crate::ton::int>).serialize(_ser)?;
        (temp_packages as &dyn crate::ton::VectoredBare<crate::ton::int>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Value {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let packages =
                <Vec<crate::ton::int> as crate::ton::VectoredBare<crate::ton::int>>::deserialize(
                    _de,
                )?;
            let key_packages = <Vec<crate::ton::int> as crate::ton::VectoredBare<
                crate::ton::int,
            >>::deserialize(_de)?;
            let temp_packages = <Vec<crate::ton::int> as crate::ton::VectoredBare<
                crate::ton::int,
            >>::deserialize(_de)?;
            Ok(Self { packages, key_packages, temp_packages })
        }
    }
}
impl crate::IntoBoxed for Value {
    type Boxed = crate::ton::db::files::index::Value;

    fn into_boxed(self) -> crate::ton::db::files::index::Value {
        crate::ton::db::files::index::Value::Db_Files_Index_Value(self)
    }
}
