use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.runResult`\n\n```text\nsmc.runResult gas_used:int53 stack:vector<tvm.StackEntry> exit_code:int32 = smc.RunResult;\n```\n"]
pub struct RunResult {
    pub gas_used: crate::ton::int53,
    pub stack: crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>,
    pub exit_code: crate::ton::int32,
}
impl Eq for RunResult {}
impl crate::BareSerialize for RunResult {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x5444f3f3)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunResult { gas_used, stack, exit_code } = self;
        _ser.write_bare::<crate::ton::int53>(gas_used)?;
        _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
            stack,
        )?;
        _ser.write_bare::<crate::ton::int32>(exit_code)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunResult {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let gas_used = _de.read_bare::<crate::ton::int53>()?;
            let stack = _de
                .read_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
                )?;
            let exit_code = _de.read_bare::<crate::ton::int32>()?;
            Ok(Self { gas_used, stack, exit_code })
        }
    }
}
impl crate::IntoBoxed for RunResult {
    type Boxed = crate::ton::smc::RunResult;

    fn into_boxed(self) -> crate::ton::smc::RunResult {
        crate::ton::smc::RunResult::Smc_RunResult(self)
    }
}
