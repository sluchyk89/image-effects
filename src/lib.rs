use base64::decode;
use image::load_from_memory;
use web_sys::console::log_1 as log;

use wasm_bingen::prelude::*;

#[wasm_bingen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale scalled".into());

    let base64_to_vector = decode(encoded_file).unwrap();

    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();

    log(&"Image loaded".into());

    img = img.grayscale();

    log(&"Grayscale effect applied".into());
}