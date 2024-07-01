use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `query.estimateFees`\n\n```text\nquery.estimateFees id:int53 ignore_chksig:Bool = query.Fees;\n```\n"]
pub struct EstimateFees {
    pub id: crate::ton::int53,
    pub ignore_chksig: crate::ton::Bool,
}
impl Eq for EstimateFees {}
impl crate::BareSerialize for EstimateFees {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xc6f54e41)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let EstimateFees { id, ignore_chksig } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        _ser.write_boxed::<crate::ton::Bool>(ignore_chksig)?;
        Ok(())
    }
}
impl crate::BareDeserialize for EstimateFees {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            let ignore_chksig = _de.read_boxed::<crate::ton::Bool>()?;
            Ok(Self { id, ignore_chksig })
        }
    }
}
impl crate::BoxedDeserialize for EstimateFees {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xc6f54e41)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xc6f54e41) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for EstimateFees {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xc6f54e41), self)
    }
}
impl crate::Function for EstimateFees {
    type Reply = crate::ton::query::Fees;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `query.forget`\n\n```text\nquery.forget id:int53 = Ok;\n```\n"]
pub struct Forget {
    pub id: crate::ton::int53,
}
impl Eq for Forget {}
impl crate::BareSerialize for Forget {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb7c2925f)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Forget { id } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Forget {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for Forget {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xb7c2925f)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xb7c2925f) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for Forget {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xb7c2925f), self)
    }
}
impl crate::Function for Forget {
    type Reply = crate::ton::Ok;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `query.getInfo`\n\n```text\nquery.getInfo id:int53 = query.Info;\n```\n"]
pub struct GetInfo {
    pub id: crate::ton::int53,
}
impl Eq for GetInfo {}
impl crate::BareSerialize for GetInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xd05b22db)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let GetInfo { id } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for GetInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for GetInfo {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xd05b22db)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0xd05b22db) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for GetInfo {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0xd05b22db), self)
    }
}
impl crate::Function for GetInfo {
    type Reply = crate::ton::query::Info;
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `query.send`\n\n```text\nquery.send id:int53 = Ok;\n```\n"]
pub struct Send {
    pub id: crate::ton::int53,
}
impl Eq for Send {}
impl crate::BareSerialize for Send {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x37261573)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Send { id } = self;
        _ser.write_bare::<crate::ton::int53>(id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Send {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int53>()?;
            Ok(Self { id })
        }
    }
}
impl crate::BoxedDeserialize for Send {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x37261573)]
    }
    fn deserialize_boxed(
        id: crate::ConstructorNumber,
        de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        if id == crate::ConstructorNumber(0x37261573) {
            de.read_bare()
        } else {
            _invalid_id!(id)
        }
    }
}
impl crate::BoxedSerialize for Send {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        (crate::ConstructorNumber(0x37261573), self)
    }
}
impl crate::Function for Send {
    type Reply = crate::ton::Ok;
}
