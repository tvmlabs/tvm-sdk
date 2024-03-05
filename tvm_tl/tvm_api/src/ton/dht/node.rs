use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `dht.node`\n\n```text\ndht.node id:PublicKey addr_list:adnl.addressList version:int signature:bytes = dht.Node;\n```\n"]
pub struct Node {
    pub id: crate::ton::PublicKey,
    pub addr_list: crate::ton::adnl::addresslist::AddressList,
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
        crate::ConstructorNumber(0x84533248)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Node { id, addr_list, version, signature } = self;
        _ser.write_boxed::<crate::ton::PublicKey>(id)?;
        _ser.write_bare::<crate::ton::adnl::addresslist::AddressList>(addr_list)?;
        _ser.write_bare::<crate::ton::int>(version)?;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Node {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let id = _de.read_boxed::<crate::ton::PublicKey>()?;
            let addr_list = _de.read_bare::<crate::ton::adnl::addresslist::AddressList>()?;
            let version = _de.read_bare::<crate::ton::int>()?;
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            Ok(Self { id, addr_list, version, signature })
        }
    }
}
impl crate::IntoBoxed for Node {
    type Boxed = crate::ton::dht::Node;

    fn into_boxed(self) -> crate::ton::dht::Node {
        crate::ton::dht::Node::Dht_Node(self)
    }
}
