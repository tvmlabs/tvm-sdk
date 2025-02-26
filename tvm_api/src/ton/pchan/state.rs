use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.stateClose`\n\n```text\npchan.stateClose signed_A:Bool signed_B:Bool min_A:int64 min_B:int64 expire_at:int53 A:int64 B:int64 = pchan.State;\n```\n"]
pub struct StateClose {
    pub signed_A: crate::ton::Bool,
    pub signed_B: crate::ton::Bool,
    pub min_A: crate::ton::int64,
    pub min_B: crate::ton::int64,
    pub expire_at: crate::ton::int53,
    pub A: crate::ton::int64,
    pub B: crate::ton::int64,
}
impl Eq for StateClose {}
impl crate::BareSerialize for StateClose {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x34e201f3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StateClose { signed_A, signed_B, min_A, min_B, expire_at, A, B } = self;
        _ser.write_boxed::<crate::ton::Bool>(signed_A)?;
        _ser.write_boxed::<crate::ton::Bool>(signed_B)?;
        _ser.write_bare::<crate::ton::int64>(min_A)?;
        _ser.write_bare::<crate::ton::int64>(min_B)?;
        _ser.write_bare::<crate::ton::int53>(expire_at)?;
        _ser.write_bare::<crate::ton::int64>(A)?;
        _ser.write_bare::<crate::ton::int64>(B)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StateClose {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let signed_A = _de.read_boxed::<crate::ton::Bool>()?;
            let signed_B = _de.read_boxed::<crate::ton::Bool>()?;
            let min_A = _de.read_bare::<crate::ton::int64>()?;
            let min_B = _de.read_bare::<crate::ton::int64>()?;
            let expire_at = _de.read_bare::<crate::ton::int53>()?;
            let A = _de.read_bare::<crate::ton::int64>()?;
            let B = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { signed_A, signed_B, min_A, min_B, expire_at, A, B })
        }
    }
}
impl crate::IntoBoxed for StateClose {
    type Boxed = crate::ton::pchan::State;

    fn into_boxed(self) -> crate::ton::pchan::State {
        crate::ton::pchan::State::Pchan_StateClose(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.stateInit`\n\n```text\npchan.stateInit signed_A:Bool signed_B:Bool min_A:int64 min_B:int64 expire_at:int53 A:int64 B:int64 = pchan.State;\n```\n"]
pub struct StateInit {
    pub signed_A: crate::ton::Bool,
    pub signed_B: crate::ton::Bool,
    pub min_A: crate::ton::int64,
    pub min_B: crate::ton::int64,
    pub expire_at: crate::ton::int53,
    pub A: crate::ton::int64,
    pub B: crate::ton::int64,
}
impl Eq for StateInit {}
impl crate::BareSerialize for StateInit {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb92a0cf8)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StateInit { signed_A, signed_B, min_A, min_B, expire_at, A, B } = self;
        _ser.write_boxed::<crate::ton::Bool>(signed_A)?;
        _ser.write_boxed::<crate::ton::Bool>(signed_B)?;
        _ser.write_bare::<crate::ton::int64>(min_A)?;
        _ser.write_bare::<crate::ton::int64>(min_B)?;
        _ser.write_bare::<crate::ton::int53>(expire_at)?;
        _ser.write_bare::<crate::ton::int64>(A)?;
        _ser.write_bare::<crate::ton::int64>(B)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StateInit {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let signed_A = _de.read_boxed::<crate::ton::Bool>()?;
            let signed_B = _de.read_boxed::<crate::ton::Bool>()?;
            let min_A = _de.read_bare::<crate::ton::int64>()?;
            let min_B = _de.read_bare::<crate::ton::int64>()?;
            let expire_at = _de.read_bare::<crate::ton::int53>()?;
            let A = _de.read_bare::<crate::ton::int64>()?;
            let B = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { signed_A, signed_B, min_A, min_B, expire_at, A, B })
        }
    }
}
impl crate::IntoBoxed for StateInit {
    type Boxed = crate::ton::pchan::State;

    fn into_boxed(self) -> crate::ton::pchan::State {
        crate::ton::pchan::State::Pchan_StateInit(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.statePayout`\n\n```text\npchan.statePayout A:int64 B:int64 = pchan.State;\n```\n"]
pub struct StatePayout {
    pub A: crate::ton::int64,
    pub B: crate::ton::int64,
}
impl Eq for StatePayout {}
impl crate::BareSerialize for StatePayout {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x279e1447)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let StatePayout { A, B } = self;
        _ser.write_bare::<crate::ton::int64>(A)?;
        _ser.write_bare::<crate::ton::int64>(B)?;
        Ok(())
    }
}
impl crate::BareDeserialize for StatePayout {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let A = _de.read_bare::<crate::ton::int64>()?;
            let B = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { A, B })
        }
    }
}
impl crate::IntoBoxed for StatePayout {
    type Boxed = crate::ton::pchan::State;

    fn into_boxed(self) -> crate::ton::pchan::State {
        crate::ton::pchan::State::Pchan_StatePayout(self)
    }
}
