use std::str::FromStr;

use tvm_block::ExternalInboundMessageHeader;
use tvm_block::GetRepresentationHash;
use tvm_block::MsgAddressExt;
use tvm_block::StateInit;

use crate::boc::BocCacheType;
use crate::boc::internal::deserialize_cell_from_boc;
use crate::boc::internal::deserialize_object_from_boc;
use crate::boc::internal::serialize_object_to_boc;
use crate::client::ClientContext;
use crate::encoding::account_decode;
use crate::encoding::slice_from_cell;
use crate::error::ClientResult;

#[derive(Serialize, Deserialize, Clone, Debug, ApiType, Default)]
pub struct ParamsOfEncodeExternalInMessage {
    /// Source address.
    pub src: Option<String>,

    /// Destination address.
    pub dst: String,

    /// Bag of cells with state init (used in deploy messages).
    pub init: Option<String>,

    /// Bag of cells with the message body encoded as base64.
    pub body: Option<String>,

    /// Cache type to put the result. The BOC itself returned if no cache type
    /// provided
    pub boc_cache: Option<BocCacheType>,
}

#[derive(Serialize, Deserialize, ApiType, Default)]
pub struct ResultOfEncodeExternalInMessage {
    /// Message BOC encoded with `base64`.
    pub message: String,

    /// Message id.
    pub message_id: String,
}

/// Encodes a message
///
/// Allows to encode any external inbound message.
#[api_function]
pub fn encode_external_in_message(
    context: std::sync::Arc<ClientContext>,
    params: ParamsOfEncodeExternalInMessage,
) -> ClientResult<ResultOfEncodeExternalInMessage> {
    let src = params.src.clone();
    let header = ExternalInboundMessageHeader {
        dst: account_decode(&params.dst)?,
        src: src
            .as_ref()
            .map(|x| MsgAddressExt::from_str(x.as_str()))
            .unwrap_or_else(|| Ok(MsgAddressExt::AddrNone))
            .map_err(|err| {
                crate::client::errors::Error::invalid_address(
                    err.to_string(),
                    &src.unwrap_or_default(),
                )
            })?,
        ..Default::default()
    };

    let mut msg = tvm_block::Message::with_ext_in_header(header);
    if let Some(init) = params.init {
        msg.set_state_init(
            deserialize_object_from_boc::<StateInit>(&context, &init, "state init")?.object,
        );
    }
    if let Some(body) = params.body {
        let (_, cell) = deserialize_cell_from_boc(&context, &body, "message body")?;
        msg.set_body(slice_from_cell(cell)?);
    }

    let hash = msg.hash().map_err(crate::client::errors::Error::internal_error)?;
    let boc = serialize_object_to_boc(&context, &msg, "message", params.boc_cache)?;
    Ok(ResultOfEncodeExternalInMessage { message: boc, message_id: hex::encode(hash) })
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tvm_block::Message;
    use tvm_types::BuilderData;
    use tvm_types::IBitstring;
    use tvm_types::base64_encode;

    use super::*;

    fn context() -> Arc<ClientContext> {
        Arc::new(ClientContext::new(crate::ClientConfig::default()).unwrap())
    }

    #[test]
    fn rejects_invalid_source_address() {
        let err = encode_external_in_message(
            context(),
            ParamsOfEncodeExternalInMessage {
                src: Some("not-an-address".into()),
                dst: "0:1111111111111111111111111111111111111111111111111111111111111111".into(),
                ..Default::default()
            },
        )
        .err()
        .unwrap();

        assert!(err.message().contains("Invalid address ["));
    }

    #[test]
    fn encodes_message_with_optional_init_and_body() {
        let context = context();
        let state_init =
            serialize_object_to_boc(&context, &StateInit::default(), "state init", None).unwrap();

        let mut body = BuilderData::new();
        body.append_u32(0xdead_beef).unwrap();
        let body = base64_encode(tvm_types::boc::write_boc(&body.into_cell().unwrap()).unwrap());

        let result = encode_external_in_message(
            context.clone(),
            ParamsOfEncodeExternalInMessage {
                src: Some(":abcd".into()),
                dst: "0:1111111111111111111111111111111111111111111111111111111111111111".into(),
                init: Some(state_init),
                body: Some(body),
                ..Default::default()
            },
        )
        .unwrap();

        let message = deserialize_object_from_boc::<Message>(&context, &result.message, "message")
            .unwrap()
            .object;
        assert!(message.state_init().is_some());
        assert!(message.body().is_some());
        assert_eq!(hex::encode(message.hash().unwrap()), result.message_id);
    }
}
