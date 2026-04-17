use std::sync::Arc;

use serde_json::Value;
use tvm_block::MsgAddressInt;

use crate::abi::Abi;
use crate::boc::internal::deserialize_object_from_base64;
use crate::client::ClientContext;
use crate::error::AddNetworkUrl;
use crate::error::ClientResult;
use crate::processing::Error;
use crate::processing::ResultOfProcessMessage;
use crate::processing::internal::can_retry_network_error;
use crate::processing::internal::resolve_error;
use crate::processing::parsing::decode_output;
use crate::processing::parsing::parse_transaction_boc;
use crate::tvm::check_transaction::calc_transaction_fees;
use crate::tvm::check_transaction::extract_error;

#[derive(Deserialize)]
pub(crate) struct MessageBoc {
    pub boc: String,
}

#[derive(Deserialize)]
pub(crate) struct TransactionBoc {
    pub boc: String,
    pub out_messages: Vec<MessageBoc>,
}

impl TransactionBoc {
    fn from(value: Value, message_id: &str) -> ClientResult<Self> {
        serde_json::from_value::<TransactionBoc>(value).map_err(|err| {
            Error::fetch_transaction_result_failed(
                format!("Transaction can't be parsed: {}", err),
                message_id,
                "",
            )
        })
    }
}

/// Polls `blockchain.transaction(hash)` until the transaction appears or
/// timeout is reached.
pub(crate) async fn fetch_transaction_by_hash(
    context: &Arc<ClientContext>,
    tx_hash: &str,
    message_id: &str,
    timeout_ms: u64,
) -> ClientResult<TransactionBoc> {
    let start = context.env.now_ms();

    loop {
        let result = crate::net::query(
            context.clone(),
            crate::net::ParamsOfQuery {
                query: "query transaction($hash:String!){blockchain{transaction(hash:$hash){boc out_messages{boc}}}}".into(),
                variables: Some(json!({
                    "hash": tx_hash,
                })),
            },
        )
        .await;

        match result {
            Ok(mut result) => {
                if let Some(value) = result.result.pointer_mut("/data/blockchain/transaction") {
                    let value = value.take();
                    if !value.is_null() {
                        return TransactionBoc::from(value, message_id);
                    }
                }
                // Transaction not found yet — check timeout then retry
            }
            Err(error) => {
                if !crate::client::Error::is_network_error(&error)
                    || !can_retry_network_error(context, start)
                {
                    return Err(error);
                }
            }
        }

        if context.env.now_ms() - start > timeout_ms {
            return Err(Error::fetch_transaction_result_failed(
                "Transaction not found within timeout",
                message_id,
                "",
            ));
        }

        let _ = context.env.set_timer(1000).await;
    }
}

pub(crate) async fn fetch_account(
    context: Arc<ClientContext>,
    address: &MsgAddressInt,
    result: &str,
) -> ClientResult<Value> {
    let mut result = crate::net::query(
        context,
        crate::net::ParamsOfQuery {
            query: format!("query account($address:String!){{blockchain{{account(address:$address){{info{{{}}}}}}}}}", result),
            variables: Some(json!({
                "address": address.to_string(),
            })),
        },
    )
    .await?;

    result
        .result
        .pointer_mut("/data/blockchain/account/info")
        .map(|value| value.take())
        .ok_or(crate::tvm::Error::account_missing(address))
}

#[derive(Deserialize)]
struct AccountBalance {
    #[serde(with = "tvm_sdk::json_helper::uint")]
    balance: u64,
}

async fn fetch_contract_balance(
    context: Arc<ClientContext>,
    address: &MsgAddressInt,
) -> ClientResult<u64> {
    let account = fetch_account(context, address, "balance").await?;

    let balance: AccountBalance = serde_json::from_value(account)
        .map_err(|err| Error::invalid_data(format!("can not parse account balance: {}", err)))?;

    Ok(balance.balance)
}

#[allow(clippy::too_many_arguments)]
pub async fn fetch_transaction_result(
    context: &Arc<ClientContext>,
    tx_hash: &str,
    message_id: &str,
    message: &str,
    abi: &Option<Abi>,
    address: MsgAddressInt,
    expiration_time: u32,
    block_time: u32,
    timeout_ms: u64,
) -> ClientResult<ResultOfProcessMessage> {
    let transaction_boc =
        fetch_transaction_by_hash(context, tx_hash, message_id, timeout_ms).await?;
    let context_copy = context.clone();
    let address_copy = address.clone();
    let get_contract_info = || async move {
        let balance = fetch_contract_balance(context_copy, &address_copy).await?;
        Ok((address_copy, balance))
    };
    let transaction_object = deserialize_object_from_base64(&transaction_boc.boc, "transaction")?;

    let transaction = tvm_sdk::Transaction::try_from(&transaction_object.object)
        .map_err(crate::tvm::Error::can_not_read_transaction)?;

    let local_result = if transaction.is_aborted() {
        let error = match extract_error(&transaction, get_contract_info.clone(), true).await {
            Err(err) => err,
            Ok(_) => crate::tvm::Error::transaction_aborted(),
        };

        Some(
            resolve_error(
                Arc::clone(context),
                &address,
                message.to_string(),
                error,
                expiration_time - 1,
                false,
            )
            .await
            .add_network_url_from_context(context)
            .await
            .map_err(|mut error| {
                error.data_mut()["transaction_id"] = transaction.id().to_string().into();
                error
            }),
        )
    } else {
        None
    };

    let fees = calc_transaction_fees(&transaction, true, false, get_contract_info, true)
        .await
        .map_err(|err| {
            const EXIT_CODE_FIELD: &str = "exit_code";
            let exit_code = &err.data()[EXIT_CODE_FIELD];
            if err.code() == crate::tvm::ErrorCode::ContractExecutionError as u32
                && (exit_code == crate::tvm::StdContractError::ReplayProtection as i32
                    || exit_code == crate::tvm::StdContractError::ExtMessageExpired as i32)
            {
                Error::message_expired(message_id, "", expiration_time, block_time, &address)
            } else {
                if let Some(Err(local_error)) = local_result {
                    if local_error.data()[EXIT_CODE_FIELD] == *exit_code {
                        return local_error;
                    }
                }
                err
            }
        })?;

    let (transaction, out_messages) = parse_transaction_boc(context.clone(), transaction_boc)?;
    let abi_decoded = if let Some(abi) = abi {
        Some(decode_output(context, abi, out_messages.clone())?)
    } else {
        None
    };

    Ok(ResultOfProcessMessage { transaction, out_messages, decoded: abi_decoded, fees })
}
