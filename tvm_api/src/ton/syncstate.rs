use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `syncStateInProgress`\n\n```text\nsyncStateInProgress from_seqno:int32 to_seqno:int32 current_seqno:int32 = SyncState;\n```\n"]
pub struct SyncStateInProgress {
    pub from_seqno: crate::ton::int32,
    pub to_seqno: crate::ton::int32,
    pub current_seqno: crate::ton::int32,
}
impl Eq for SyncStateInProgress {}
impl crate::BareSerialize for SyncStateInProgress {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x066bc4c7)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let SyncStateInProgress { from_seqno, to_seqno, current_seqno } = self;
        _ser.write_bare::<crate::ton::int32>(from_seqno)?;
        _ser.write_bare::<crate::ton::int32>(to_seqno)?;
        _ser.write_bare::<crate::ton::int32>(current_seqno)?;
        Ok(())
    }
}
impl crate::BareDeserialize for SyncStateInProgress {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let from_seqno = _de.read_bare::<crate::ton::int32>()?;
            let to_seqno = _de.read_bare::<crate::ton::int32>()?;
            let current_seqno = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { from_seqno, to_seqno, current_seqno })
        }
    }
}
impl crate::IntoBoxed for SyncStateInProgress {
    type Boxed = crate::ton::SyncState;

    fn into_boxed(self) -> crate::ton::SyncState {
        crate::ton::SyncState::SyncStateInProgress(self)
    }
}
