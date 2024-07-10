use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `updateSendLiteServerQuery`\n\n```text\nupdateSendLiteServerQuery id:int64 data:bytes = Update;\n```\n"]
pub struct UpdateSendLiteServerQuery {
    pub id: crate::ton::int64,
    pub data: crate::ton::bytes,
}
impl Eq for UpdateSendLiteServerQuery {}
impl crate::BareSerialize for UpdateSendLiteServerQuery {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa34e95dc)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let UpdateSendLiteServerQuery { id, data } = self;
        _ser.write_bare::<crate::ton::int64>(id)?;
        _ser.write_bare::<crate::ton::bytes>(data)?;
        Ok(())
    }
}
impl crate::BareDeserialize for UpdateSendLiteServerQuery {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::int64>()?;
            let data = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, data })
        }
    }
}
impl crate::IntoBoxed for UpdateSendLiteServerQuery {
    type Boxed = crate::ton::Update;
    fn into_boxed(self) -> crate::ton::Update {
        crate::ton::Update::UpdateSendLiteServerQuery(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `updateSyncState`\n\n```text\nupdateSyncState sync_state:SyncState = Update;\n```\n"]
pub struct UpdateSyncState {
    pub sync_state: crate::ton::SyncState,
}
impl Eq for UpdateSyncState {}
impl crate::BareSerialize for UpdateSyncState {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x47c823de)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let UpdateSyncState { sync_state } = self;
        _ser.write_boxed::<crate::ton::SyncState>(sync_state)?;
        Ok(())
    }
}
impl crate::BareDeserialize for UpdateSyncState {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let sync_state = _de.read_boxed::<crate::ton::SyncState>()?;
            Ok(Self { sync_state })
        }
    }
}
impl crate::IntoBoxed for UpdateSyncState {
    type Boxed = crate::ton::Update;
    fn into_boxed(self) -> crate::ton::Update {
        crate::ton::Update::UpdateSyncState(self)
    }
}
