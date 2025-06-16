use wasm_bindgen::prelude::*;

#[wasm_bindgen]
fn hello() -> &'static str {
    "Hello, world!"
}
#[wasm_bindgen]
fn answer() -> i32 {
    42
}
#[wasm_bindgen]
fn question() -> &'static str {
    "What is the meaning of life? The Universe? Everything?"
}
#[wasm_bindgen]
fn hello_there(kenobi: &str) -> String {
    format!("General {}!", kenobi)
}
#[wasm_bindgen]
fn main() {
    ()
}
