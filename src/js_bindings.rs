// src/js_bindings.rs
use serde_wasm_bindgen;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::Performance;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = window)]
    pub fn receiveResults(results: JsValue, selector: &str);
}

pub fn performance() -> Performance {
    web_sys::window()
        .and_then(|win| win.performance())
        .expect("Could not access the performance")
}

pub fn send_results_to_js(results: &HashMap<i32, f32>, selector: &str) {
    let js_value = serde_wasm_bindgen::to_value(results).unwrap();
    receiveResults(js_value, selector);
}
