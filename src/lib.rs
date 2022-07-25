use base64::decode;
use web_sys::console::log_1 as log;

use wasm_bingen::prelude::*;

#[wasm_bingen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale scalled".into());

    let base64_to_vector = decode(encoded_file).unwrap();

    log(&"Image decoded".into())
}