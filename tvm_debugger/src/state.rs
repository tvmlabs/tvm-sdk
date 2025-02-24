use crate::{
    StateDecodeArgs, StateEncodeArgs, helper::get_base64_or_read_from_file, read_file_as_base64,
};
use anyhow::bail;
use std::sync::Arc;
use tvm_client::{
    ClientConfig, ClientContext,
    boc::{
        ParamsOfDecodeStateInit, ParamsOfEncodeStateInit, ResultOfDecodeStateInit,
        ResultOfEncodeStateInit, decode_state_init, encode_state_init,
    },
};

pub fn encode(args: &StateEncodeArgs) -> anyhow::Result<ResultOfEncodeStateInit> {
    let code = get_base64_or_read_from_file(args.code.as_deref())?;
    let data = get_base64_or_read_from_file(args.data.as_deref())?;

    let params = ParamsOfEncodeStateInit { code, data, ..Default::default() };
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);
    Ok(encode_state_init(client, params)?)
}

pub fn decode(args: &StateDecodeArgs) -> anyhow::Result<ResultOfDecodeStateInit> {
    let state_init = match get_base64_or_read_from_file(Some(args.state_init.as_ref())).transpose()
    {
        Some(Ok(res)) => res,
        Some(Err(_)) => read_file_as_base64(args.state_init.as_ref())?,
        None => bail!("state-init is required"),
    };
    let params = ParamsOfDecodeStateInit { state_init, boc_cache: None };
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);

    Ok(decode_state_init(client, params)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_decode_state_init() {
        let code = "te6ccgECGAEAAsUABCSK7VMg4wMgwP/jAiDA/uMC8gsVAgEXArAh2zzTAAGOH4MI1xgg+CjIzs7J+QAB0wABlNP/UDOTAvhC4vkQ8qiV0wAB8nri0z8B+EMhufK0IPgjgQPoqIIIG3dAoLnytPhj0x8B+CO88rnTHwHbPPI8EwMDUu1E0IEBQNch1woA+GYi0NcLA6k4ANwhxwDjAiHXDR/yvCHjAwHbPPI8FBQDBFAgghBMKH52u+MCIIIQa/J+w7vjAiCCEHgGAWu64wIgghB8EhYwuuMCCwgFBAFQMNHbPPhKIY4cjQRwAAAAAAAAAAAAAAAAPwSFjCDIzsv/yXD7AN7yABMCGjD4RvLgTNHbPOMA8gAHBgAo7UTQ0//TPzH4Q1jIy//LP87J7VQAVPgAcPgoyM+FiM6CmBzEtAAAAAAAAAAAAAAAAAAAMiU73c8Lpsv/yXD7AAIoIIIQaLVfP7rjAiCCEGvyfsO64wIKCQFQMNHbPPhLIY4cjQRwAAAAAAAAAAAAAAAAOvyfsODIzssfyXD7AN7yABMCLjD4Qm7jAHD4anD4a/hG8nPR+ADbPPIAExEDPCCCEC9vzzq64wIgghAyJTvduuMCIIIQTCh+drrjAhAODAMkMPhG8uBM+EJu4wDR2zzbPPIAEw0RAAz4APgt+GsDNDD4RvLgTPhCbuMAIZPU0dDe0//R2zzbPPIAEw8RAIb4ACDBBo46cJMgwQSOMfhK+CSg+GohpPgoyM+FiM6CmBzEtAAAAAAAAAAAAAAAAAAAMiU73c8Lpsv/yXD7AKToMN4wAyQw+Eby4Ez4Qm7jANHbPNs88gATEhEALPhL+Er4Q/hCyMv/yz/Pg8v/yx/J7VQADvgA+Eqk+GoAMO1E0NP/0z/TANP/0x/R+Gv4avhm+GP4YgAK+Eby4EwCEPSkIPS98sBOFxYAFHNvbCAwLjczLjAAAA==";
        let data = "te6ccgEBAQEATwAAmUZwR+gdaoRh0jd9rKUTxUo+LDi6IC1Q6uAMv9e/P1dwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABA";
        let expected_state_init = read_file_as_base64("tests/contract/contract.tvc").unwrap();

        // Encode
        let result =
            encode(&StateEncodeArgs { code: Some(code.to_string()), data: Some(data.to_string()) });

        assert!(result.is_ok());
        let state_init = result.unwrap().state_init;
        assert_eq!(state_init, expected_state_init);

        // Decode
        let result = decode(&StateDecodeArgs { state_init });
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.code.unwrap(), code);
        assert_eq!(result.data.unwrap(), data);
    }

    #[test]
    fn test_decode_state_from_base64_file() {
        let result = decode(&StateDecodeArgs {
            state_init: "tests/contract/contract.tvc.base64".to_string(),
        });
        assert!(result.is_ok());
    }
    #[test]
    fn test_decode_state_from_binary_file() {
        let result =
            decode(&StateDecodeArgs { state_init: "tests/contract/contract.tvc".to_string() });
        assert!(result.is_ok());
    }
    #[test]
    fn test_decode_state_from_base64_string() {
        let state_init="te6ccgECGgEAAxkAAgE0AgEAmUZwR+gdaoRh0jd9rKUTxUo+LDi6IC1Q6uAMv9e/P1dwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABABCSK7VMg4wMgwP/jAiDA/uMC8gsXBAMZArAh2zzTAAGOH4MI1xgg+CjIzs7J+QAB0wABlNP/UDOTAvhC4vkQ8qiV0wAB8nri0z8B+EMhufK0IPgjgQPoqIIIG3dAoLnytPhj0x8B+CO88rnTHwHbPPI8FQUDUu1E0IEBQNch1woA+GYi0NcLA6k4ANwhxwDjAiHXDR/yvCHjAwHbPPI8FhYFBFAgghBMKH52u+MCIIIQa/J+w7vjAiCCEHgGAWu64wIgghB8EhYwuuMCDQoHBgFQMNHbPPhKIY4cjQRwAAAAAAAAAAAAAAAAPwSFjCDIzsv/yXD7AN7yABUCGjD4RvLgTNHbPOMA8gAJCAAo7UTQ0//TPzH4Q1jIy//LP87J7VQAVPgAcPgoyM+FiM6CmBzEtAAAAAAAAAAAAAAAAAAAMiU73c8Lpsv/yXD7AAIoIIIQaLVfP7rjAiCCEGvyfsO64wIMCwFQMNHbPPhLIY4cjQRwAAAAAAAAAAAAAAAAOvyfsODIzssfyXD7AN7yABUCLjD4Qm7jAHD4anD4a/hG8nPR+ADbPPIAFRMDPCCCEC9vzzq64wIgghAyJTvduuMCIIIQTCh+drrjAhIQDgMkMPhG8uBM+EJu4wDR2zzbPPIAFQ8TAAz4APgt+GsDNDD4RvLgTPhCbuMAIZPU0dDe0//R2zzbPPIAFRETAIb4ACDBBo46cJMgwQSOMfhK+CSg+GohpPgoyM+FiM6CmBzEtAAAAAAAAAAAAAAAAAAAMiU73c8Lpsv/yXD7AKToMN4wAyQw+Eby4Ez4Qm7jANHbPNs88gAVFBMALPhL+Er4Q/hCyMv/yz/Pg8v/yx/J7VQADvgA+Eqk+GoAMO1E0NP/0z/TANP/0x/R+Gv4avhm+GP4YgAK+Eby4EwCEPSkIPS98sBOGRgAFHNvbCAwLjczLjAAAA==".to_string();
        let result = decode(&StateDecodeArgs { state_init });
        assert!(result.is_ok());
    }
    #[test]
    fn test_decode_state_from_non_existent_file() {
        let result =
            decode(&StateDecodeArgs { state_init: "tests/contract/non-existent.tvc".to_string() });
        assert!(result.is_err());
    }
}
