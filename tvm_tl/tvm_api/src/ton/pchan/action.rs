use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.actionClose`\n\n```text\npchan.actionClose extra_A:int64 extra_B:int64 promise:pchan.promise = pchan.Action;\n```\n"]
pub struct ActionClose {
    pub extra_A: crate::ton::int64,
    pub extra_B: crate::ton::int64,
    pub promise: crate::ton::pchan::promise::Promise,
}
impl Eq for ActionClose {}
impl crate::BareSerialize for ActionClose {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x639c4b16)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionClose { extra_A, extra_B, promise } = self;
        _ser.write_bare::<crate::ton::int64>(extra_A)?;
        _ser.write_bare::<crate::ton::int64>(extra_B)?;
        _ser.write_bare::<crate::ton::pchan::promise::Promise>(promise)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionClose {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let extra_A = _de.read_bare::<crate::ton::int64>()?;
            let extra_B = _de.read_bare::<crate::ton::int64>()?;
            let promise = _de.read_bare::<crate::ton::pchan::promise::Promise>()?;
            Ok(Self { extra_A, extra_B, promise })
        }
    }
}
impl crate::IntoBoxed for ActionClose {
    type Boxed = crate::ton::pchan::Action;

    fn into_boxed(self) -> crate::ton::pchan::Action {
        crate::ton::pchan::Action::Pchan_ActionClose(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.actionInit`\n\n```text\npchan.actionInit inc_A:int64 inc_B:int64 min_A:int64 min_B:int64 = pchan.Action;\n```\n"]
pub struct ActionInit {
    pub inc_A: crate::ton::int64,
    pub inc_B: crate::ton::int64,
    pub min_A: crate::ton::int64,
    pub min_B: crate::ton::int64,
}
impl Eq for ActionInit {}
impl crate::BareSerialize for ActionInit {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x1a2bf68a)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ActionInit { inc_A, inc_B, min_A, min_B } = self;
        _ser.write_bare::<crate::ton::int64>(inc_A)?;
        _ser.write_bare::<crate::ton::int64>(inc_B)?;
        _ser.write_bare::<crate::ton::int64>(min_A)?;
        _ser.write_bare::<crate::ton::int64>(min_B)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ActionInit {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let inc_A = _de.read_bare::<crate::ton::int64>()?;
            let inc_B = _de.read_bare::<crate::ton::int64>()?;
            let min_A = _de.read_bare::<crate::ton::int64>()?;
            let min_B = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { inc_A, inc_B, min_A, min_B })
        }
    }
}
impl crate::IntoBoxed for ActionInit {
    type Boxed = crate::ton::pchan::Action;

    fn into_boxed(self) -> crate::ton::pchan::Action {
        crate::ton::pchan::Action::Pchan_ActionInit(self)
    }
}
