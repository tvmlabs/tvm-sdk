use crate::{AccountEncodeArgs, helper::get_base64_or_read_from_file};
use std::sync::Arc;
use tvm_client::{
    ClientConfig, ClientContext,
    abi::{ParamsOfEncodeAccount, ResultOfEncodeAccount, encode_account},
};

pub fn encode(args: &AccountEncodeArgs) -> anyhow::Result<ResultOfEncodeAccount> {
    let state_init = get_base64_or_read_from_file(Some(&args.state_init))?
        .ok_or_else(|| anyhow::anyhow!("The state init is required"))?;

    let params = ParamsOfEncodeAccount {
        state_init,
        balance: args.balance,
        last_paid: args.last_paid,
        last_trans_lt: args.last_trans_lt,
        boc_cache: None,
    };
    let client = Arc::new(ClientContext::new(ClientConfig { ..Default::default() })?);
    Ok(encode_account(client, params)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file_as_base64;

    #[test]
    fn test_encode_decode_state_init() {
        let state_init = read_file_as_base64("tests/contract/contract.tvc").unwrap();

        // Encode
        let result = encode(&&AccountEncodeArgs {
            state_init: state_init.clone(),
            balance: Some(100000000000),
            last_paid: Some(5000000),
            last_trans_lt: Some(10000000),
        });

        assert!(result.is_ok());
    }
}
