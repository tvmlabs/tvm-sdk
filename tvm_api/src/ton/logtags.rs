use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `logTags`\n\n```text\nlogTags tags:vector<string> = LogTags;\n```\n"]
pub struct LogTags {
    pub tags: crate::ton::vector<crate::ton::string>,
}
impl Eq for LogTags {}
impl crate::BareSerialize for LogTags {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa056b3d7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let LogTags { tags } = self;
        (tags as &dyn crate::ton::VectoredBare<crate::ton::string>).serialize(_ser)?;
        Ok(())
    }
}
impl crate::BareDeserialize for LogTags {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let tags = < Vec < crate :: ton :: string > as crate :: ton :: VectoredBare < crate :: ton :: string >> :: deserialize (_de) ? ;
            Ok(Self { tags })
        }
    }
}
impl crate::IntoBoxed for LogTags {
    type Boxed = crate::ton::LogTags;

    fn into_boxed(self) -> crate::ton::LogTags {
        crate::ton::LogTags::LogTags(self)
    }
}
