use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `pchan.promise`\n\n```text\npchan.promise signature:bytes promise_A:int64 promise_B:int64 channel_id:int64 = pchan.Promise;\n```\n"]
pub struct Promise {
    pub signature: crate::ton::bytes,
    pub promise_A: crate::ton::int64,
    pub promise_B: crate::ton::int64,
    pub channel_id: crate::ton::int64,
}
impl Eq for Promise {}
impl crate::Signing for Promise {
    fn signature_mut(&mut self) -> &mut crate::ton::bytes {
        &mut self.signature
    }
}
impl crate::BareSerialize for Promise {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xa20e945d)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let Promise { signature, promise_A, promise_B, channel_id } = self;
        _ser.write_bare::<crate::ton::bytes>(signature)?;
        _ser.write_bare::<crate::ton::int64>(promise_A)?;
        _ser.write_bare::<crate::ton::int64>(promise_B)?;
        _ser.write_bare::<crate::ton::int64>(channel_id)?;
        Ok(())
    }
}
impl crate::BareDeserialize for Promise {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let signature = _de.read_bare::<crate::ton::bytes>()?;
            let promise_A = _de.read_bare::<crate::ton::int64>()?;
            let promise_B = _de.read_bare::<crate::ton::int64>()?;
            let channel_id = _de.read_bare::<crate::ton::int64>()?;
            Ok(Self { signature, promise_A, promise_B, channel_id })
        }
    }
}
impl crate::IntoBoxed for Promise {
    type Boxed = crate::ton::pchan::Promise;

    fn into_boxed(self) -> crate::ton::pchan::Promise {
        crate::ton::pchan::Promise::Pchan_Promise(self)
    }
}
