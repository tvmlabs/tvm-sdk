use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.runTvmResultException`\n\n```text\nsmc.runTvmResultException block_root_hash:int256 exit_code:int exit_arg:tvm.StackEntry = smc.RunTvmResult;\n```\n"]
pub struct RunTvmResultException {
    pub block_root_hash: crate::ton::int256,
    pub exit_code: crate::ton::int,
    pub exit_arg: crate::ton::tvm::StackEntry,
}
impl Eq for RunTvmResultException {}
impl crate::BareSerialize for RunTvmResultException {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x2e68fba6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunTvmResultException { block_root_hash, exit_code, exit_arg } = self;
        _ser.write_bare::<crate::ton::int256>(block_root_hash)?;
        _ser.write_bare::<crate::ton::int>(exit_code)?;
        _ser.write_boxed::<crate::ton::tvm::StackEntry>(exit_arg)?;
        Ok(())
    }
}
impl crate::BareDeserialize for RunTvmResultException {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let block_root_hash = _de.read_bare::<crate::ton::int256>()?;
            let exit_code = _de.read_bare::<crate::ton::int>()?;
            let exit_arg = _de.read_boxed::<crate::ton::tvm::StackEntry>()?;
            Ok(Self { block_root_hash, exit_code, exit_arg })
        }
    }
}
impl crate::IntoBoxed for RunTvmResultException {
    type Boxed = crate::ton::smc::RunTvmResult;

    fn into_boxed(self) -> crate::ton::smc::RunTvmResult {
        crate::ton::smc::RunTvmResult::Smc_RunTvmResultException(self)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[doc = "TL-derived from `smc.runTvmResultOk`\n\n```text\nsmc.runTvmResultOk mode:# block_root_hash:int256 exit_code:int stack:mode.0?vector<tvm.StackEntry> init_c7:mode.1?tvm.StackEntry messages:mode.2?vector<bytes> data:mode.3?bytes code:mode.4?bytes = smc.RunTvmResult;\n```\n"]
pub struct RunTvmResultOk {
    pub mode: crate::ton::int,
    pub block_root_hash: crate::ton::int256,
    pub exit_code: crate::ton::int,
    pub stack: Option<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>,
    pub init_c7: Option<crate::ton::tvm::StackEntry>,
    pub messages: Option<crate::ton::vector<crate::ton::Bare, crate::ton::bytes>>,
    pub data: Option<crate::ton::bytes>,
    pub code: Option<crate::ton::bytes>,
}
impl Eq for RunTvmResultOk {}
impl crate::BareSerialize for RunTvmResultOk {
    fn constructor(&self) -> crate::ConstructorNumber {
        crate::ConstructorNumber(0x19ce6fd6)
    }

    fn serialize_bare(&self, _ser: &mut crate::Serializer) -> crate::Result<()> {
        let RunTvmResultOk {
            mode,
            block_root_hash,
            exit_code,
            stack,
            init_c7,
            messages,
            data,
            code,
        } = self;
        _ser.write_bare::<crate::ton::int>(mode)?;
        _ser.write_bare::<crate::ton::int256>(block_root_hash)?;
        _ser.write_bare::<crate::ton::int>(exit_code)?;
        if let Some(inner) = stack {
            _ser.write_bare::<crate::ton::vector<crate::ton::Boxed, crate::ton::tvm::StackEntry>>(
                inner,
            )?;
        }
        if let Some(inner) = init_c7 {
            _ser.write_boxed::<crate::ton::tvm::StackEntry>(inner)?;
        }
        if let Some(inner) = messages {
            _ser.write_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::bytes>>(inner)?;
        }
        if let Some(inner) = data {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        if let Some(inner) = code {
            _ser.write_bare::<crate::ton::bytes>(inner)?;
        }
        Ok(())
    }
}
impl crate::BareDeserialize for RunTvmResultOk {
    fn deserialize_bare(_de: &mut crate::Deserializer) -> crate::Result<Self> {
        {
            let mode = _de.read_bare::<crate::ton::int>()?;
            let block_root_hash = _de.read_bare::<crate::ton::int256>()?;
            let exit_code = _de.read_bare::<crate::ton::int>()?;
            let stack = if mode & (1 << 0u32) != 0 {
                Some (_de . read_bare :: < crate :: ton :: vector < crate :: ton :: Boxed , crate :: ton :: tvm :: StackEntry > > () ?)
            } else {
                None
            };
            let init_c7 = if mode & (1 << 1u32) != 0 {
                Some(_de.read_boxed::<crate::ton::tvm::StackEntry>()?)
            } else {
                None
            };
            let messages = if mode & (1 << 2u32) != 0 {
                Some(_de.read_bare::<crate::ton::vector<crate::ton::Bare, crate::ton::bytes>>()?)
            } else {
                None
            };
            let data = if mode & (1 << 3u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            let code = if mode & (1 << 4u32) != 0 {
                Some(_de.read_bare::<crate::ton::bytes>()?)
            } else {
                None
            };
            Ok(Self { mode, block_root_hash, exit_code, stack, init_c7, messages, data, code })
        }
    }
}
impl crate::IntoBoxed for RunTvmResultOk {
    type Boxed = crate::ton::smc::RunTvmResult;

    fn into_boxed(self) -> crate::ton::smc::RunTvmResult {
        crate::ton::smc::RunTvmResult::Smc_RunTvmResultOk(self)
    }
}
