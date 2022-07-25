use wasm_bingen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bingen]
pub fn grayscale(encoded_file: &str) {
  log(&encoded_file.into())
    
}