use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ApiType)]
#[serde(tag = "type")]
pub enum MonitoredMessage {
    Boc { boc: String },
    HashAddress { hash: String, address: String },
}

impl Default for MonitoredMessage {
    fn default() -> Self {
        Self::Boc { boc: String::new() }
    }
}

impl From<tvm_client_processing::MonitoredMessage> for MonitoredMessage {
    fn from(value: tvm_client_processing::MonitoredMessage) -> Self {
        match value {
            tvm_client_processing::MonitoredMessage::Boc { boc } => Self::Boc { boc },
            tvm_client_processing::MonitoredMessage::HashAddress { hash, address } => {
                Self::HashAddress { hash, address }
            }
        }
    }
}

impl From<MonitoredMessage> for tvm_client_processing::MonitoredMessage {
    fn from(value: MonitoredMessage) -> Self {
        match value {
            MonitoredMessage::Boc { boc } => Self::Boc { boc },
            MonitoredMessage::HashAddress { hash, address } => Self::HashAddress { hash, address },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ApiType, Default)]
pub struct MessageMonitoringParams {
    pub message: MonitoredMessage,
    pub wait_until: u32,
    pub user_data: Option<Value>,
}

impl From<tvm_client_processing::MessageMonitoringParams> for MessageMonitoringParams {
    fn from(value: tvm_client_processing::MessageMonitoringParams) -> Self {
        Self { message: value.message.into(), wait_until: value.wait_until, user_data: None }
    }
}

impl From<MessageMonitoringParams> for tvm_client_processing::MessageMonitoringParams {
    fn from(value: MessageMonitoringParams) -> Self {
        Self { message: value.message.into(), wait_until: value.wait_until, user_data: None }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ApiType, Default)]
pub enum MessageMonitoringStatus {
    Finalized,
    Timeout,
    #[default]
    Reserved,
}

impl From<tvm_client_processing::MessageMonitoringStatus> for MessageMonitoringStatus {
    fn from(value: tvm_client_processing::MessageMonitoringStatus) -> Self {
        match value {
            tvm_client_processing::MessageMonitoringStatus::Finalized => Self::Finalized,
            tvm_client_processing::MessageMonitoringStatus::Timeout => Self::Timeout,
            tvm_client_processing::MessageMonitoringStatus::Reserved => Self::Reserved,
        }
    }
}

impl From<MessageMonitoringStatus> for tvm_client_processing::MessageMonitoringStatus {
    fn from(value: MessageMonitoringStatus) -> Self {
        match value {
            MessageMonitoringStatus::Finalized => Self::Finalized,
            MessageMonitoringStatus::Timeout => Self::Timeout,
            MessageMonitoringStatus::Reserved => Self::Reserved,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ApiType)]
pub struct MessageMonitoringTransactionCompute {
    pub exit_code: i32,
}

impl From<tvm_client_processing::MessageMonitoringTransactionCompute>
    for MessageMonitoringTransactionCompute
{
    fn from(value: tvm_client_processing::MessageMonitoringTransactionCompute) -> Self {
        Self { exit_code: value.exit_code }
    }
}

impl From<MessageMonitoringTransactionCompute>
    for tvm_client_processing::MessageMonitoringTransactionCompute
{
    fn from(value: MessageMonitoringTransactionCompute) -> Self {
        Self { exit_code: value.exit_code }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ApiType)]
pub struct MessageMonitoringTransaction {
    pub hash: Option<String>,
    pub aborted: bool,
    pub compute: Option<MessageMonitoringTransactionCompute>,
}

impl From<tvm_client_processing::MessageMonitoringTransaction> for MessageMonitoringTransaction {
    fn from(value: tvm_client_processing::MessageMonitoringTransaction) -> Self {
        Self { hash: value.hash, aborted: value.aborted, compute: value.compute.map(Into::into) }
    }
}

impl From<MessageMonitoringTransaction> for tvm_client_processing::MessageMonitoringTransaction {
    fn from(value: MessageMonitoringTransaction) -> Self {
        Self { hash: value.hash, aborted: value.aborted, compute: value.compute.map(Into::into) }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, ApiType, Default)]
pub struct MessageMonitoringResult {
    pub hash: String,
    pub status: MessageMonitoringStatus,
    pub transaction: Option<MessageMonitoringTransaction>,
    pub error: Option<String>,
    pub user_data: Option<Value>,
}

impl From<tvm_client_processing::MessageMonitoringResult> for MessageMonitoringResult {
    fn from(value: tvm_client_processing::MessageMonitoringResult) -> Self {
        Self {
            hash: value.hash,
            status: value.status.into(),
            transaction: value.transaction.map(Into::into),
            error: value.error,
            user_data: None,
        }
    }
}

impl From<MessageMonitoringResult> for tvm_client_processing::MessageMonitoringResult {
    fn from(value: MessageMonitoringResult) -> Self {
        Self {
            hash: value.hash,
            status: value.status.into(),
            transaction: value.transaction.map(Into::into),
            error: value.error,
            user_data: None,
        }
    }
}

#[derive(Deserialize, Serialize, ApiType, Copy, Clone)]
pub enum MonitorFetchWaitMode {
    AtLeastOne,
    All,
    NoWait,
}

impl From<tvm_client_processing::MonitorFetchWaitMode> for MonitorFetchWaitMode {
    fn from(value: tvm_client_processing::MonitorFetchWaitMode) -> Self {
        match value {
            tvm_client_processing::MonitorFetchWaitMode::AtLeastOne => Self::AtLeastOne,
            tvm_client_processing::MonitorFetchWaitMode::All => Self::All,
            tvm_client_processing::MonitorFetchWaitMode::NoWait => Self::NoWait,
        }
    }
}

impl From<MonitorFetchWaitMode> for tvm_client_processing::MonitorFetchWaitMode {
    fn from(value: MonitorFetchWaitMode) -> Self {
        match value {
            MonitorFetchWaitMode::AtLeastOne => Self::AtLeastOne,
            MonitorFetchWaitMode::All => Self::All,
            MonitorFetchWaitMode::NoWait => Self::NoWait,
        }
    }
}

#[derive(Deserialize, Serialize, ApiType, Default)]
pub struct MonitoringQueueInfo {
    pub unresolved: u32,
    pub resolved: u32,
}

impl From<tvm_client_processing::MonitoringQueueInfo> for MonitoringQueueInfo {
    fn from(value: tvm_client_processing::MonitoringQueueInfo) -> Self {
        Self { unresolved: value.unresolved, resolved: value.resolved }
    }
}
