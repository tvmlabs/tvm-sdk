use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `smc.Info`\n\n```text\nsmc.info id:int53 = smc.Info;\n```\n"]
pub enum Info {
    Smc_Info(crate::ton::smc::info::Info),
}
impl Info {
    pub fn id(&self) -> &crate::ton::int53 {
        match self {
            Info::Smc_Info(ref x) => &x.id,
        }
    }

    pub fn only(self) -> crate::ton::smc::info::Info {
        match self {
            Info::Smc_Info(x) => x,
        }
    }
}
impl Eq for Info {}
impl Default for Info {
    fn default() -> Self {
        Info::Smc_Info(crate::ton::smc::info::Info::default())
    }
}
impl crate::BoxedSerialize for Info {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            Info::Smc_Info(x) => (crate::ConstructorNumber(0x439b963c), x),
        }
    }
}
impl crate::BoxedDeserialize for Info {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x439b963c)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x439b963c) => {
                Ok(Info::Smc_Info(_de.read_bare::<crate::ton::smc::info::Info>()?))
            }
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `smc.MethodId`\n\n```text\nsmc.methodIdName name:string = smc.MethodId;\n\nsmc.methodIdNumber number:int32 = smc.MethodId;\n```\n"]
pub enum MethodId {
    Smc_MethodIdName(crate::ton::smc::methodid::MethodIdName),
    Smc_MethodIdNumber(crate::ton::smc::methodid::MethodIdNumber),
}
impl MethodId {
    pub fn name(&self) -> Option<&crate::ton::string> {
        match self {
            MethodId::Smc_MethodIdName(ref x) => Some(&x.name),
            _ => None,
        }
    }

    pub fn number(&self) -> Option<&crate::ton::int32> {
        match self {
            MethodId::Smc_MethodIdNumber(ref x) => Some(&x.number),
            _ => None,
        }
    }
}
impl Eq for MethodId {}
impl Default for MethodId {
    fn default() -> Self {
        MethodId::Smc_MethodIdName(crate::ton::smc::methodid::MethodIdName::default())
    }
}
impl crate::BoxedSerialize for MethodId {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            MethodId::Smc_MethodIdName(x) => (crate::ConstructorNumber(0xf127ff94), x),
            MethodId::Smc_MethodIdNumber(x) => (crate::ConstructorNumber(0xa423b9fc), x),
        }
    }
}
impl crate::BoxedDeserialize for MethodId {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0xf127ff94), crate::ConstructorNumber(0xa423b9fc)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0xf127ff94) => Ok(MethodId::Smc_MethodIdName(
                _de.read_bare::<crate::ton::smc::methodid::MethodIdName>()?,
            )),
            crate::ConstructorNumber(0xa423b9fc) => Ok(MethodId::Smc_MethodIdNumber(
                _de.read_bare::<crate::ton::smc::methodid::MethodIdNumber>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `smc.RunResult`\n\n```text\nsmc.runResult gas_used:int53 stack:vector<tvm.StackEntry> exit_code:int32 = smc.RunResult;\n```\n"]
pub enum RunResult {
    Smc_RunResult(crate::ton::smc::runresult::RunResult),
}
impl RunResult {
    pub fn exit_code(&self) -> &crate::ton::int32 {
        match self {
            RunResult::Smc_RunResult(ref x) => &x.exit_code,
        }
    }

    pub fn gas_used(&self) -> &crate::ton::int53 {
        match self {
            RunResult::Smc_RunResult(ref x) => &x.gas_used,
        }
    }

    pub fn stack(&self) -> &crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry> {
        match self {
            RunResult::Smc_RunResult(ref x) => &x.stack,
        }
    }

    pub fn only(self) -> crate::ton::smc::runresult::RunResult {
        match self {
            RunResult::Smc_RunResult(x) => x,
        }
    }
}
impl Eq for RunResult {}
impl Default for RunResult {
    fn default() -> Self {
        RunResult::Smc_RunResult(crate::ton::smc::runresult::RunResult::default())
    }
}
impl crate::BoxedSerialize for RunResult {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RunResult::Smc_RunResult(x) => (crate::ConstructorNumber(0x5444f3f3), x),
        }
    }
}
impl crate::BoxedDeserialize for RunResult {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x5444f3f3)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x5444f3f3) => Ok(RunResult::Smc_RunResult(
                _de.read_bare::<crate::ton::smc::runresult::RunResult>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[doc = "TL-derived from `smc.RunTvmResult`\n\n```text\nsmc.runTvmResultException block_root_hash:int256 exit_code:int exit_arg:tvm.StackEntry = smc.RunTvmResult;\n\nsmc.runTvmResultOk mode:# block_root_hash:int256 exit_code:int stack:mode.0?vector<tvm.StackEntry> init_c7:mode.1?tvm.StackEntry messages:mode.2?vector<bytes> data:mode.3?bytes code:mode.4?bytes = smc.RunTvmResult;\n```\n"]
pub enum RunTvmResult {
    Smc_RunTvmResultException(crate::ton::smc::runtvmresult::RunTvmResultException),
    Smc_RunTvmResultOk(crate::ton::smc::runtvmresult::RunTvmResultOk),
}
impl RunTvmResult {
    pub fn block_root_hash(&self) -> &crate::ton::int256 {
        match self {
            RunTvmResult::Smc_RunTvmResultException(ref x) => &x.block_root_hash,
            RunTvmResult::Smc_RunTvmResultOk(ref x) => &x.block_root_hash,
        }
    }

    pub fn code(&self) -> Option<&crate::ton::bytes> {
        match self {
            RunTvmResult::Smc_RunTvmResultOk(ref x) => x.code.as_ref(),
            _ => None,
        }
    }

    pub fn data(&self) -> Option<&crate::ton::bytes> {
        match self {
            RunTvmResult::Smc_RunTvmResultOk(ref x) => x.data.as_ref(),
            _ => None,
        }
    }

    pub fn exit_arg(&self) -> Option<&crate::ton::tvm::StackEntry> {
        match self {
            RunTvmResult::Smc_RunTvmResultException(ref x) => Some(&x.exit_arg),
            _ => None,
        }
    }

    pub fn exit_code(&self) -> &crate::ton::int {
        match self {
            RunTvmResult::Smc_RunTvmResultException(ref x) => &x.exit_code,
            RunTvmResult::Smc_RunTvmResultOk(ref x) => &x.exit_code,
        }
    }

    pub fn init_c7(&self) -> Option<&crate::ton::tvm::StackEntry> {
        match self {
            RunTvmResult::Smc_RunTvmResultOk(ref x) => x.init_c7.as_ref(),
            _ => None,
        }
    }

    pub fn messages(&self) -> Option<&crate::ton::vector<crate::ton::Bare, crate::ton::bytes>> {
        match self {
            RunTvmResult::Smc_RunTvmResultOk(ref x) => x.messages.as_ref(),
            _ => None,
        }
    }

    pub fn mode(&self) -> Option<&crate::ton::int> {
        match self {
            RunTvmResult::Smc_RunTvmResultOk(ref x) => Some(&x.mode),
            _ => None,
        }
    }

    pub fn stack(
        &self,
    ) -> Option<&crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>> {
        match self {
            RunTvmResult::Smc_RunTvmResultOk(ref x) => x.stack.as_ref(),
            _ => None,
        }
    }
}
impl Eq for RunTvmResult {}
impl Default for RunTvmResult {
    fn default() -> Self {
        RunTvmResult::Smc_RunTvmResultException(
            crate::ton::smc::runtvmresult::RunTvmResultException::default(),
        )
    }
}
impl crate::BoxedSerialize for RunTvmResult {
    fn serialize_boxed(&self) -> (crate::ConstructorNumber, &dyn crate::BareSerialize) {
        match self {
            RunTvmResult::Smc_RunTvmResultException(x) => (crate::ConstructorNumber(0x2e68fba6), x),
            RunTvmResult::Smc_RunTvmResultOk(x) => (crate::ConstructorNumber(0x19ce6fd6), x),
        }
    }
}
impl crate::BoxedDeserialize for RunTvmResult {
    fn possible_constructors() -> Vec<crate::ConstructorNumber> {
        vec![crate::ConstructorNumber(0x2e68fba6), crate::ConstructorNumber(0x19ce6fd6)]
    }

    fn deserialize_boxed(
        _id: crate::ConstructorNumber,
        _de: &mut crate::Deserializer,
    ) -> crate::Result<Self> {
        match _id {
            crate::ConstructorNumber(0x2e68fba6) => Ok(RunTvmResult::Smc_RunTvmResultException(
                _de.read_bare::<crate::ton::smc::runtvmresult::RunTvmResultException>()?,
            )),
            crate::ConstructorNumber(0x19ce6fd6) => Ok(RunTvmResult::Smc_RunTvmResultOk(
                _de.read_bare::<crate::ton::smc::runtvmresult::RunTvmResultOk>()?,
            )),
            id => _invalid_id!(id),
        }
    }
}
pub mod info;
pub mod methodid;
pub mod runresult;
pub mod runtvmresult;
