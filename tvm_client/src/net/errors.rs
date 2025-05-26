use std::fmt::Display;

use serde_json::Value;
use serde_json::json;

use crate::client::FetchResult;
use crate::error::ClientError;
use crate::error::format_time;

#[derive(ApiType)]
pub enum ErrorCode {
    QueryFailed = 601,
    SubscribeFailed = 602,
    WaitForFailed = 603,
    GetSubscriptionResultFailed = 604,
    InvalidServerResponse = 605,
    ClockOutOfSync = 606,
    WaitForTimeout = 607,
    GraphqlError = 608,
    NetworkModuleSuspended = 609,
    WebsocketDisconnected = 610,
    NotSupported = 611,
    NoEndpointsProvided = 612,
    GraphqlWebsocketInitError = 613,
    NetworkModuleResumed = 614,
    Unauthorized = 615,
    QueryTransactionTreeTimeout = 616,
    GraphqlConnectionError = 617,
    WrongWebscoketProtocolSequence = 618,
    ParseUrlFailed = 619,
    ModifyUrlFailed = 620,
    SendMessageFailed = 621,
}

pub struct Error;

fn error(code: ErrorCode, message: String) -> ClientError {
    ClientError::with_code_message(code as u32, message)
}

impl Error {
    pub(crate) fn unauthorized(response: &FetchResult) -> ClientError {
        let message = match serde_json::from_str(&response.body) {
            Err(_) => response.body.clone(),
            Ok(value) => match Self::try_extract_graphql_error(&value) {
                Some(err) => err.message,
                None => response.body.clone(),
            },
        };
        error(ErrorCode::Unauthorized, message)
    }

    pub fn try_extract_graphql_error(value: &Value) -> Option<ClientError> {
        let errors = if let Some(payload) = value.get("payload") {
            payload.get("errors")
        } else {
            value.get("errors")
        };

        if let Some(errors) = errors {
            if let Some(errors) = errors.as_array() {
                return Some(Self::graphql_server_error(None, errors));
            }
        }

        None
    }

    pub fn try_extract_send_messages_error(value: &Value) -> Option<ClientError> {
        if let Some(error) = value.get("error") {
            if !error.is_null() {
                return Some(Self::send_message_server_error(error));
            }
        }

        None
    }

    pub fn queries_query_failed(mut err: ClientError) -> ClientError {
        if err.code != ErrorCode::Unauthorized as u32 {
            err.code = ErrorCode::QueryFailed as u32;
        }
        err.message = format!("Query failed: {err}");
        err
    }

    pub fn queries_subscribe_failed(mut err: ClientError) -> ClientError {
        if err.code != ErrorCode::Unauthorized as u32 {
            err.code = ErrorCode::SubscribeFailed as u32;
        }
        err.message = format!("Subscribe failed: {err}");
        err
    }

    pub fn queries_wait_for_failed(
        mut err: ClientError,
        filter: Option<Value>,
        timestamp: u32,
    ) -> ClientError {
        if err.code != ErrorCode::Unauthorized as u32
            && err.code != ErrorCode::WaitForTimeout as u32
        {
            err.code = ErrorCode::WaitForFailed as u32;
        }
        err.message = format!("WaitFor failed: {err}");
        err.data["filter"] = filter.into();
        err.data["timestamp"] = format_time(timestamp).into();
        err
    }

    pub fn queries_get_subscription_result_failed<E: Display>(err: E) -> ClientError {
        error(
            ErrorCode::GetSubscriptionResultFailed,
            format!("Receive subscription result failed: {err}"),
        )
    }

    pub fn invalid_server_response<E: Display>(err: E) -> ClientError {
        error(ErrorCode::InvalidServerResponse, format!("Invalid server response: {err}"))
    }

    pub fn wait_for_timeout() -> ClientError {
        error(
            ErrorCode::WaitForTimeout,
            "wait_for operation did not return anything during the specified timeout".to_owned(),
        )
    }

    fn try_get_error_details(
        server_errors: &[Value],
    ) -> (Option<String>, Option<i64>, Option<Value>) {
        for error in server_errors.iter() {
            if let Some(message) = error["message"].as_str() {
                let code = error["extensions"]["exception"]["code"]
                    .as_i64()
                    .or_else(|| error["extensions"]["code"].as_i64());
                return (Some(message.to_string()), code, Some(error.clone()));
            }
        }
        (None, None, None)
    }

    pub fn send_message_server_error(orig_error: &Value) -> ClientError {
        let message = if let Some(message) = orig_error["message"].as_str() {
            message.to_string()
        } else {
            "Unknown error".to_string()
        };

        let mut err = error(ErrorCode::SendMessageFailed, message.clone());
        if let Some(code) = orig_error.get("code") {
            err.data["node_error"]["extensions"]["code"] = code.clone();
        }
        if let Some(message) = orig_error.get("message") {
            err.data["node_error"]["extensions"]["message"] = message.clone();
        }
        if let Some(data) = orig_error.get("data") {
            err.data["node_error"]["extensions"]["details"] = data.clone();
        }

        err
    }

    pub fn graphql_server_error(operation: Option<&str>, errors: &[Value]) -> ClientError {
        let (message, code, details) = Self::try_get_error_details(errors);
        let message = match (operation, message) {
            (None, None) => "Graphql server returned error.".to_string(),
            (None, Some(message)) => format!("Graphql server returned error: {message}."),
            (Some(operation), None) => format!("Graphql {operation} error."),
            (Some(operation), Some(message)) => {
                format!("Graphql {operation} error: {message}.")
            }
        };
        let mut err = error(ErrorCode::GraphqlError, message);

        if let Some(code) = code {
            err.data["server_code"] = code.into();
        }

        if let Some(mut node_error) = details {
            if node_error["extensions"]["code"].is_string()
                && node_error["extensions"]["details"].is_object()
            {
                if let Value::Object(ref mut map) = node_error {
                    map.remove("locations");
                    map.remove("path");
                }
                err.data["node_error"] = node_error;
            }
        }

        err
    }

    pub fn graphql_connection_error(errors: &[Value]) -> ClientError {
        let mut err = Self::graphql_server_error(Some("connection"), errors);
        err.code = ErrorCode::GraphqlConnectionError as u32;
        err
    }

    pub fn websocket_disconnected<E: Display>(err: E) -> ClientError {
        error(
            ErrorCode::WebsocketDisconnected,
            format!("Websocket unexpectedly disconnected: {err}"),
        )
    }

    pub fn network_module_suspended() -> ClientError {
        error(ErrorCode::NetworkModuleSuspended, "Network module is suspended".to_owned())
    }

    pub fn not_supported(request: &str) -> ClientError {
        error(
            ErrorCode::NotSupported,
            format!("Server does not support the following request: {request}"),
        )
    }

    pub fn no_endpoints_provided() -> ClientError {
        error(ErrorCode::NoEndpointsProvided, "No endpoints provided".to_owned())
    }

    pub fn graphql_websocket_init_error(mut err: ClientError) -> ClientError {
        err.code = ErrorCode::GraphqlWebsocketInitError as u32;
        err.message = format!("GraphQL websocket init failed: {err}");
        err
    }

    pub fn network_module_resumed() -> ClientError {
        error(ErrorCode::NetworkModuleResumed, "Network module has been resumed".to_owned())
    }

    pub fn query_transaction_tree_timeout(timeout: u32) -> ClientError {
        let mut err = error(
            ErrorCode::QueryTransactionTreeTimeout,
            "Query transaction tree failed: some messages has not appeared during the timeout. Possible reason: sync problems on server side.".to_owned(),
        );
        err.data = json!({ "timeout": timeout });
        err
    }

    pub fn wrong_ws_protocol_sequence(err: &str) -> ClientError {
        error(
            ErrorCode::WrongWebscoketProtocolSequence,
            format!("Wrong webscoket protocol sequence: {err}"),
        )
    }

    pub fn parse_url_failed<E: Display>(err: E) -> ClientError {
        error(ErrorCode::ParseUrlFailed, format!("Failed to parse url: {err}"))
    }

    pub fn modify_url_failed(err: &str) -> ClientError {
        error(ErrorCode::ParseUrlFailed, format!("Failed to modify url: {err}"))
    }
}
