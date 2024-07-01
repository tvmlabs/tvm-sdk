use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.broadcastList`\n\n```text\noverlay.broadcastList hashes:(vector int256) = overlay.BroadcastList;\n```\n"]
pub struct BroadcastList {
    pub hashes: crate::ton::vector<crate::ton::Bare, crate::ton::int256>,
}
impl Eq for BroadcastList {}
impl crate::BareSerialize for BroadcastList {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x18d1dedf)
    }
    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let BroadcastList { hashes } = self;
        _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>(hashes)?;
        Ok(())
    }
}
impl crate::BareDeserialize for BroadcastList {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let hashes =
                _de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::int256>>()?;
            Ok(Self { hashes })
        }
    }
}
impl crate::IntoBoxed for BroadcastList {
    type Boxed = crate::ton::overlay::BroadcastList;
    fn into_boxed(self) -> crate::ton::overlay::BroadcastList {
        crate::ton::overlay::BroadcastList::Overlay_BroadcastList(self)
    }
}
