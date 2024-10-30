use crate::decode::tree_of_cells_into_base64;
use serde_json::{json, Value};
use tvm_block::{CommonMsgInfo, Message, Serializable};
use tvm_types::base64_encode;

pub struct ExecutionResult {
    is_json: bool,
    log: Vec<String>,
    messages: Vec<Value>,
    response: Value,
    response_code: i32,
    pub(crate) is_vm_success: bool,
    gas_used: i64,
}

impl ExecutionResult {
    pub(crate) fn new(is_json: bool) -> ExecutionResult {
        return ExecutionResult {
            is_json,
            log: vec![],
            messages: vec![],
            response: "{}".into(),
            response_code: -1,
            is_vm_success: false,
            gas_used: 0,
        };
    }

    pub fn exit_code(&mut self, code: i32) {
        self.response_code = code;
        self.log(format!("TVM terminated with exit code {}", code));
    }

    pub fn vm_success(&mut self, is_vm_success: bool) {
        self.is_vm_success = is_vm_success;
        self.log(format!("Computing phase is success: {}", is_vm_success));
    }

    pub fn gas_used(&mut self, gas: i64) {
        self.gas_used = gas;
        self.log(format!("Gas used: {}", self.gas_used));
        self.log("".to_string());
    }

    pub fn response(&mut self, data: String) {
        self.response = serde_json::from_str(&*data.clone()).expect("Failed to parse JSON");
        self.log(data);
    }

    pub fn add_out_message(&mut self, message: Message) {
        match message.header() {
            CommonMsgInfo::IntMsgInfo(_) => {
                let state_init = match message.state_init() {
                    None => None,
                    Some(state_init) => Some(base64_encode(state_init.write_to_bytes().unwrap())),
                };
                let destination =
                    message.header().get_dst_address().unwrap_or_default().to_string();
                let body =
                    tree_of_cells_into_base64(message.body().map(|s| s.into_cell()).as_ref());
                let boc = base64_encode(message.write_to_bytes().unwrap());
                self.messages.push(json!({
                    "state_init": state_init,
                    "destination": destination,
                    "body": body,
                    "boc": boc,
                }));
            }
            CommonMsgInfo::ExtInMsgInfo(_) => {}
            CommonMsgInfo::ExtOutMsgInfo(_) => {}
        }
    }

    pub fn log(&mut self, data: String) {
        self.log.push(data);
    }

    pub fn to_json(&self) -> Value {
        json!({
            "exit_code": self.response_code,
            "vm_success": self.is_vm_success,
            "gas_used": self.gas_used,
            "response": self.response,
            "messages": self.messages,
        })
    }

    pub fn output(&mut self) -> String {
        return if self.is_json { self.to_json().to_string() } else { self.log.join("\n") };
    }
}
