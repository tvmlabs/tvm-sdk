use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `tonNode.packInfo`\n\n```text\ntonNode.packInfo flags:# gen_utime_ms:uint64 mc_block:uint32 prev1:int256 prev2:flags.0?int256 round:uint64 seqno:uint64 workchain:int32 shard:uint64 = tonNode.PackInfo;\n```\n"]
pub struct PackInfo {
    pub gen_utime_ms: crate::ton::uint64,
    pub mc_block: crate::ton::uint32,
    pub prev1: crate::ton::int256,
    pub prev2: Option<crate::ton::int256>,
    pub round: crate::ton::uint64,
    pub seqno: crate::ton::uint64,
    pub workchain: crate::ton::int32,
    pub shard: crate::ton::uint64,
}
impl Eq for PackInfo {}
impl crate::BareSerialize for PackInfo {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0xcc90bd44)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let PackInfo { gen_utime_ms, mc_block, prev1, prev2, round, seqno, workchain, shard } =
            self;
        let mut _flags = 0u32;
        if prev2.is_some() {
            _flags |= 1 << 0u32;
        }
        _ser.write_bare::<crate::ton::Flags>(&_flags)?;
        _ser.write_bare::<crate::ton::uint64>(gen_utime_ms)?;
        _ser.write_bare::<crate::ton::uint32>(mc_block)?;
        _ser.write_bare::<crate::ton::int256>(prev1)?;
        if let Some(inner) = prev2 {
            _ser.write_bare::<crate::ton::int256>(inner)?;
        }
        _ser.write_bare::<crate::ton::uint64>(round)?;
        _ser.write_bare::<crate::ton::uint64>(seqno)?;
        _ser.write_bare::<crate::ton::int32>(workchain)?;
        _ser.write_bare::<crate::ton::uint64>(shard)?;
        Ok(())
    }
}
impl crate::BareDeserialize for PackInfo {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let flags = _de.read_bare::<crate::ton::Flags>()?;
            let gen_utime_ms = _de.read_bare::<crate::ton::uint64>()?;
            let mc_block = _de.read_bare::<crate::ton::uint32>()?;
            let prev1 = _de.read_bare::<crate::ton::int256>()?;
            let prev2 = if flags & (1 << 0u32) != 0 {
                Some(_de.read_bare::<crate::ton::int256>()?)
            } else {
                None
            };
            let round = _de.read_bare::<crate::ton::uint64>()?;
            let seqno = _de.read_bare::<crate::ton::uint64>()?;
            let workchain = _de.read_bare::<crate::ton::int32>()?;
            let shard = _de.read_bare::<crate::ton::uint64>()?;
            Ok(Self { gen_utime_ms, mc_block, prev1, prev2, round, seqno, workchain, shard })
        }
    }
}
impl crate::IntoBoxed for PackInfo {
    type Boxed = crate::ton::ton_node::PackInfo;

    fn into_boxed(self) -> crate::ton::ton_node::PackInfo {
        crate::ton::ton_node::PackInfo::TonNode_PackInfo(self)
    }
}
