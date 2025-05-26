use wasm_bindgen::prelude::*;

pub mod js_bindings;

mod tiny_skia_demo;
mod vello_demo;

#[wasm_bindgen]
pub async fn run_tiny_skia_benchmark() {
    tiny_skia_demo::run().await;
}

#[wasm_bindgen]
pub fn run_vello_benchmark() {
    vello_demo::run().unwrap();
}
