use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.node`\n\n```text\noverlay.node id:PublicKey overlay:int256 version:int signature:bytes = overlay.Node;\n```\n"]
pub struct Node {
    pub id: crate::ton::PublicKey,
    pub overlay: crate::ton::int256,
    pub version: crate::ton::int,
    pub signature: crate::ton::bytes,
}
impl Eq for Node {}
impl crate::Signing for Node {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Node {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xb86b8a83)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Node { id, overlay, version, signature } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(id)?;
        _ser.write_bare::<crate::ton::int256>(overlay)?;
        _ser.write_bare::<crate::ton::int>(version)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Node {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_boxed::<crate::ton::PublicKey>()?;
            let overlay = _de.read_bare::<crate::ton::int256>()?;
            let version = _de.read_bare::<crate::ton::int>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, overlay, version, signature })
        }
    }
}
impl crate::IntoBoxed for Node {
    type Boxed = crate::ton::overlay::Node;

    fn into_boxed(self) -> crate::ton::overlay::Node {
        crate::ton::overlay::Node::Overlay_Node(self)
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `overlay.node.ToSign`\n\n```text\noverlay.node.toSign id:adnl.id.short overlay:int256 version:int = overlay.node.ToSign;\n```\n"]
pub enum ToSign {
    Overlay_Node_ToSign(crate::ton::overlay::node::tosign::ToSign),
}
impl ToSign {
    pub fn id(&self) -> &crate::ton::adnl::id::short::Short {
        match self {
            ToSign::Overlay_Node_ToSign(ref x) => &x.id,
        }
    }

    pub fn overlay(&self) -> &crate::ton::int256 {
        match self {
            ToSign::Overlay_Node_ToSign(ref x) => &x.overlay,
        }
    }

    pub fn version(&self) -> &crate::ton::int {
        match self {
            ToSign::Overlay_Node_ToSign(ref x) => &x.version,
        }
    }

    pub fn only(self) -> crate::ton::overlay::node::tosign::ToSign {
        match self {
            ToSign::Overlay_Node_ToSign(x) => x,
        }
    }
}
impl Eq for ToSign {}
impl Default for ToSign {
    fn default() -> Self {
        ToSign::Overlay_Node_ToSign(crate::ton::overlay::node::tosign::ToSign::default())
    }
}
impl crate::BoxedSerialize for ToSign {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            ToSign::Overlay_Node_ToSign(x) => (crate::ConstructorNumber(0x03d8a8e1), x),
        }
    }
}
impl crate::BoxedDeserialize for ToSign {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x03d8a8e1)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x03d8a8e1) => Ok(ToSign::Overlay_Node_ToSign(
                _de.read_bare::<crate::ton::overlay::node::tosign::ToSign>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod tosign;
