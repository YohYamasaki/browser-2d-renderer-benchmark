use wasm_bindgen::prelude::*;

pub mod js_bindings;

mod tiny_skia_demo;
mod vello_demo;

#[wasm_bindgen]
pub async fn run_tiny_skia_demo(
    canvas_width: u32,
    canvas_height: u32,
    box_size: u32,
    box_number: u32,
) {
    tiny_skia_demo::run(canvas_width, canvas_height, box_size, box_number).await;
}

#[wasm_bindgen]
pub fn run_vello_demo(canvas_width: u32, canvas_height: u32, box_size: u32, box_number: u32) {
    vello_demo::run(canvas_width, canvas_height, box_size, box_number).unwrap();
}
