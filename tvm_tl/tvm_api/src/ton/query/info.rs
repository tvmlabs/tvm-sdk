use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `query.info`\n\n```text\nquery.info id:int53 valid_until:int53 body_hash:bytes body:bytes init_state:bytes = query.Info;\n```\n"]
pub struct Info {
    pub id: crate::ton::int53,
    pub valid_until: crate::ton::int53,
    pub body_hash: crate::ton::bytes,
    pub body: crate::ton::bytes,
    pub init_state: crate::ton::bytes,
}
impl Eq for Info {}
impl crate::BareSerialize for Info {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5689dc70)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Info { id, valid_until, body_hash, body, init_state } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        _ser.write_bare::<crate::ton::int53>(valid_until)?;
        _ser.write_bare::<crate::ton::bytes>(body_hash)?;
        _ser.write_bare::<crate::ton::bytes>(body)?;
        _ser.write_bare::<crate::ton::bytes>(init_state)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Info {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            let valid_until = _de.read_bare::<crate::ton::int53>()?;
            let body_hash = _de.read_bare::<crate::ton::bytes>()?;
            let body = _de.read_bare::<crate::ton::bytes>()?;
            let init_state = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, valid_until, body_hash, body, init_state })
        }
    }
}
impl crate::IntoBoxed for Info {
    type Boxed = crate::ton::query::Info;

    fn into_boxed(self) -> crate::ton::query::Info {
        crate::ton::query::Info::Query_Info(self)
    }
}
