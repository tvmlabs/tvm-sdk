use crate::{StateDecodeArgs, StateEncodeArgs, helper::get_value_or_read_file};
use std::sync::Arc;
use tvm_client::{
    ClientConfig, ClientContext,
    boc::{
        ParamsOfDecodeStateInit, ParamsOfEncodeStateInit, ResultOfDecodeStateInit,
        ResultOfEncodeStateInit, decode_state_init, encode_state_init,
    },
};

pub fn encode(args: &StateEncodeArgs) -> anyhow::Result<ResultOfEncodeStateInit> {
    let code = get_value_or_read_file(args.code.as_deref())?;
    let data = get_value_or_read_file(args.data.as_deref())?;
    let library = get_value_or_read_file(args.library.as_deref())?;

    let params = ParamsOfEncodeStateInit { code, data, library, ..Default::default() };
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);
    Ok(encode_state_init(client, params)?)
}

pub fn decode(args: &StateDecodeArgs) -> anyhow::Result<ResultOfDecodeStateInit> {
    let params = ParamsOfDecodeStateInit { state_init: args.state_init.clone(), boc_cache: None };
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);
    Ok(decode_state_init(client, params)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::read_file_as_base64;
    #[test]
    fn test_encode_decode_state_init() {
        let code = "te6ccgECGAEAAsUABCSK7VMg4wMgwP/jAiDA/uMC8gsVAgEXArAh2zzTAAGOH4MI1xgg+CjIzs7J+QAB0wABlNP/UDOTAvhC4vkQ8qiV0wAB8nri0z8B+EMhufK0IPgjgQPoqIIIG3dAoLnytPhj0x8B+CO88rnTHwHbPPI8EwMDUu1E0IEBQNch1woA+GYi0NcLA6k4ANwhxwDjAiHXDR/yvCHjAwHbPPI8FBQDBFAgghBMKH52u+MCIIIQa/J+w7vjAiCCEHgGAWu64wIgghB8EhYwuuMCCwgFBAFQMNHbPPhKIY4cjQRwAAAAAAAAAAAAAAAAPwSFjCDIzsv/yXD7AN7yABMCGjD4RvLgTNHbPOMA8gAHBgAo7UTQ0//TPzH4Q1jIy//LP87J7VQAVPgAcPgoyM+FiM6CmBzEtAAAAAAAAAAAAAAAAAAAMiU73c8Lpsv/yXD7AAIoIIIQaLVfP7rjAiCCEGvyfsO64wIKCQFQMNHbPPhLIY4cjQRwAAAAAAAAAAAAAAAAOvyfsODIzssfyXD7AN7yABMCLjD4Qm7jAHD4anD4a/hG8nPR+ADbPPIAExEDPCCCEC9vzzq64wIgghAyJTvduuMCIIIQTCh+drrjAhAODAMkMPhG8uBM+EJu4wDR2zzbPPIAEw0RAAz4APgt+GsDNDD4RvLgTPhCbuMAIZPU0dDe0//R2zzbPPIAEw8RAIb4ACDBBo46cJMgwQSOMfhK+CSg+GohpPgoyM+FiM6CmBzEtAAAAAAAAAAAAAAAAAAAMiU73c8Lpsv/yXD7AKToMN4wAyQw+Eby4Ez4Qm7jANHbPNs88gATEhEALPhL+Er4Q/hCyMv/yz/Pg8v/yx/J7VQADvgA+Eqk+GoAMO1E0NP/0z/TANP/0x/R+Gv4avhm+GP4YgAK+Eby4EwCEPSkIPS98sBOFxYAFHNvbCAwLjczLjAAAA==";
        let data = "te6ccgEBAQEATwAAmUZwR+gdaoRh0jd9rKUTxUo+LDi6IC1Q6uAMv9e/P1dwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABA";
        let expected_state_init = read_file_as_base64("tests/contract/contract.tvc").unwrap();

        // Encode
        let result = encode(&StateEncodeArgs {
            code: Some(code.to_string()),
            data: Some(data.to_string()),
            library: None,
        });

        assert!(result.is_ok());
        let state_init = result.unwrap().state_init;
        assert_eq!(state_init, expected_state_init);

        // Decode
        let result = decode(&StateDecodeArgs { state_init });
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.code.unwrap(), code);
        assert_eq!(result.data.unwrap(), data);
        assert_eq!(result.library, None);
    }
}
