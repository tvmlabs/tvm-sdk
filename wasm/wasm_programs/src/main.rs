#![no_main]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello() -> String {
    "Hello, world!".into()
}
#[wasm_bindgen]
pub fn answer() -> i32 {
    42
}
#[wasm_bindgen]
pub fn question() -> String {
    "What is the meaning of life? The Universe? Everything?".into()
}
// #[wasm_bindgen]
// pub fn hello_there(kenobi: &str) -> String {
//     format!("General {}!", kenobi)
// }
#[wasm_bindgen]
pub fn main() {
    ()
}
