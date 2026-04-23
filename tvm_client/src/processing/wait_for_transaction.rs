use std::sync::Arc;

use tvm_block::Message;

use crate::abi::Abi;
use crate::boc::internal::deserialize_object_from_boc;
use crate::client::ClientContext;
use crate::error::AddNetworkUrl;
use crate::error::ClientResult;
use crate::net::EndpointStat;
use crate::processing::Error;
use crate::processing::ProcessingEvent;
use crate::processing::ResultOfProcessMessage;
use crate::processing::fetching;
use crate::processing::internal::get_message_expiration_time;
use crate::processing::internal::resolve_error;

//--------------------------------------------------------------------------- wait_for_transaction

#[derive(Serialize, Deserialize, ApiType, Default, Debug)]
pub struct ParamsOfWaitForTransaction {
    /// Optional ABI for decoding the transaction result.
    ///
    /// If it is specified, then the output messages' bodies will be
    /// decoded according to this ABI.
    ///
    /// The `abi_decoded` result field will be filled out.
    pub abi: Option<Abi>,

    /// Message BOC. Encoded with `base64`.
    pub message: String,

    /// The last generated block id of the destination account shard before the
    /// message was sent.
    ///
    /// Deprecated: no longer used. Block walking has been removed.
    #[serde(default)]
    pub shard_block_id: String,

    /// Flag that enables/disables intermediate events.
    /// Default is `false`.
    #[serde(default)]
    pub send_events: bool,

    /// The list of endpoints to which the message was sent.
    ///
    /// Use this field to get more informative errors.
    /// Provide the same value as the `send_message` has returned.
    /// If the message was not delivered (expired), SDK will log the endpoint
    /// URLs, used for its sending.
    pub sending_endpoints: Option<Vec<String>>,

    /// Transaction hash returned by `send_message`.
    /// Used to poll for the transaction directly by hash.
    #[serde(default)]
    pub tx_hash: Option<String>,
}

pub async fn wait_for_transaction<F: futures::Future<Output = ()> + Send>(
    context: Arc<ClientContext>,
    params: ParamsOfWaitForTransaction,
    callback: impl Fn(ProcessingEvent) -> F + Send + Sync,
) -> ClientResult<ResultOfProcessMessage> {
    let net = context.get_server_link()?;

    // Deserialize message to get message_id and destination address
    let message = deserialize_object_from_boc::<Message>(&context, &params.message, "message")?;
    let message_id = message.cell.repr_hash().as_hex_string();
    let address =
        message.object.dst_ref().cloned().ok_or(Error::message_has_not_destination_address())?;

    let tx_hash = params
        .tx_hash
        .as_deref()
        .ok_or_else(|| Error::invalid_data("tx_hash is required for wait_for_transaction"))?;

    // Compute timeout
    let message_expiration_time =
        get_message_expiration_time(context.clone(), params.abi.as_ref(), &params.message)?;
    let processing_timeout = net.config().message_processing_timeout;
    let max_block_time =
        message_expiration_time.unwrap_or(context.env.now_ms() + processing_timeout as u64);
    let now = context.env.now_ms();
    let timeout_ms = std::cmp::max(max_block_time, now) - now + processing_timeout as u64;

    log::debug!(
        "wait_for_transaction: tx_hash={}, message_expiration_time={}, timeout_ms={}",
        tx_hash,
        message_expiration_time.unwrap_or_default() / 1000,
        timeout_ms
    );

    if params.send_events {
        callback(ProcessingEvent::WillFetchNextBlock {
            shard_block_id: String::new(),
            message_id: message_id.clone(),
            message_dst: address.to_string(),
            message: params.message.clone(),
        })
        .await;
    }

    // Poll blockchain.transaction(hash) until found or timeout
    let result = fetching::fetch_transaction_result(
        &context,
        tx_hash,
        &message_id,
        &params.message,
        &params.abi,
        address.clone(),
        (max_block_time / 1000) as u32,
        (context.env.now_ms() / 1000) as u32,
        timeout_ms,
    )
    .await
    .add_network_url_from_context(&context)
    .await;

    match result {
        Ok(output) => {
            if let Some(endpoints) = &params.sending_endpoints {
                context
                    .get_server_link()?
                    .update_stat(endpoints, EndpointStat::MessageDelivered)
                    .await;
            }
            Ok(output)
        }
        Err(err) => {
            // Check if this is a timeout (transaction not found)
            let is_timeout =
                err.code() == crate::processing::ErrorCode::InvalidBlockReceived as u32;

            if is_timeout {
                let waiting_expiration_time = (max_block_time / 1000) as u32;
                let now_secs = (context.env.now_ms() / 1000) as u32;
                let error = if message_expiration_time.is_some() {
                    Error::message_expired(
                        &message_id,
                        "",
                        waiting_expiration_time,
                        now_secs,
                        &address,
                    )
                } else {
                    Error::transaction_wait_timeout(
                        &message_id,
                        "",
                        waiting_expiration_time,
                        processing_timeout,
                        now_secs,
                        &address,
                    )
                };
                let resolved = resolve_error(
                    context.clone(),
                    &address,
                    params.message.clone(),
                    error,
                    waiting_expiration_time - 1,
                    true,
                )
                .await
                .add_network_url_from_context(&context)
                .await;
                if let (Some(endpoints), Err(ref e)) = (&params.sending_endpoints, &resolved) {
                    if e.data()["local_error"].is_null() {
                        context
                            .get_server_link()?
                            .update_stat(endpoints, EndpointStat::MessageUndelivered)
                            .await;
                    }
                }
                resolved?;
            }
            Err(err)
        }
    }
}
