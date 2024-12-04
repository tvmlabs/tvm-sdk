use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `logStreamFile`\n\n```text\nlogStreamFile path:string max_file_size:int53 = LogStream;\n```\n"]
pub struct LogStreamFile {
    pub path: crate::ton::string,
    pub max_file_size: crate::ton::int53,
}
impl Eq for LogStreamFile {}
impl crate::BareSerialize for LogStreamFile {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x8ff02a56)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let LogStreamFile { path, max_file_size } = self;
        _ser.write_bare::<crate::ton::string>(path)?;
        _ser.write_bare::<crate::ton::int53>(max_file_size)?;
        Ok(())
    }
}
impl crate::BareDeserialize for LogStreamFile {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let path = _de.read_bare::<crate::ton::string>()?;
            let max_file_size = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { path, max_file_size })
        }
    }
}
impl crate::IntoBoxed for LogStreamFile {
    type Boxed = crate::ton::LogStream;

    fn into_boxed(self) -> crate::ton::LogStream {
        crate::ton::LogStream::LogStreamFile(self)
    }
}
