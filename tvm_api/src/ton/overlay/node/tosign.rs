use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.node.toSign`\n\n```text\noverlay.node.toSign id:adnl.id.short overlay:int256 version:int = overlay.node.ToSign;\n```\n"]
pub struct ToSign {
    pub id: crate::ton::adnl::id::short::Short,
    pub overlay: crate::ton::int256,
    pub version: crate::ton::int,
}
impl Eq for ToSign {}
impl crate::BareSerialize for ToSign {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x03d8a8e1)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let ToSign { id, overlay, version } = self;
        _ser.write_bare::<crate::ton::adnl::id::short::Short>(id)?;
        _ser.write_bare::<crate::ton::int256>(overlay)?;
        _ser.write_bare::<crate::ton::int>(version)?;
        Ok(())
    }
}
impl crate::BareDeserialize for ToSign {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_bare::<crate::ton::adnl::id::short::Short>()?;
            let overlay = _de.read_bare::<crate::ton::int256>()?;
            let version = _de.read_bare::<crate::ton::int>()?;
            Ok(Self { id, overlay, version })
        }
    }
}
impl crate::IntoBoxed for ToSign {
    type Boxed = crate::ton::overlay::node::ToSign;

    fn into_boxed(self) -> crate::ton::overlay::node::ToSign {
        crate::ton::overlay::node::ToSign::Overlay_Node_ToSign(self)
    }
}
