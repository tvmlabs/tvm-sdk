use crate::decode::tree_of_cells_into_base64;
use serde_json::{json, Value};
use tvm_block::{CommonMsgInfo, Message, Serializable};
use tvm_types::base64_encode;

pub struct ExecutionResult {
    is_json: bool,
    log: Vec<String>,
    messages: Vec<Value>,
    response: String,
}

impl ExecutionResult {
    pub(crate) fn new(is_json: bool) -> ExecutionResult {
        return ExecutionResult {
            is_json,
            log: vec![],
            messages: vec![],
            response: "{}".to_string(),
        };
    }

    pub fn response(&mut self, data: String) {
        self.response = data.clone();
        self.push(data);
    }

    pub fn add_out_message(&mut self, message: Message) {
        match message.header() {
            CommonMsgInfo::IntMsgInfo(_) => {
                let destination =
                    message.header().get_dst_address().unwrap_or_default().to_string();
                let body =
                    tree_of_cells_into_base64(message.body().map(|s| s.into_cell()).as_ref());
                let boc = base64_encode(message.write_to_bytes().unwrap());
                self.messages.push(json!({
                    "destination": destination,
                    "body": body,
                    "boc": boc,
                }));
            }
            CommonMsgInfo::ExtInMsgInfo(_) => {}
            CommonMsgInfo::ExtOutMsgInfo(_) => {}
        }
    }

    pub fn push(&mut self, data: String) {
        self.log.push(data);
    }

    pub fn print(&mut self) {
        if self.is_json {
            let messages =
                serde_json::to_string(&self.messages).unwrap_or_else(|_| "[]".to_string());
            println!(r#"{{"response":{},"messages":{}}}"#, self.response, messages);
        } else {
            for item in self.log.clone() {
                println!("{}", item);
            }
        }
    }
}
