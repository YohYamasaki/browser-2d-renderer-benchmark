use std::cell::RefCell;
use std::rc::Rc;

// src/js_bindings.rs
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::Event;
use web_sys::Performance;
use winit::event_loop::ActiveEventLoop;

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

pub fn update_fps(fps: f64) {
    let fps = (fps * 100.0).round() / 100.0;
    let document = web_sys::window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let fps_el = document
        .body()
        .unwrap()
        .query_selector("#fps")
        .unwrap()
        .unwrap();
    fps_el.set_text_content(Some(fps.to_string().as_str()));
}

pub fn stop_updating_fps() {
    let document = web_sys::window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let fps_el = document
        .body()
        .unwrap()
        .query_selector("#fps")
        .unwrap()
        .unwrap();
    fps_el.set_text_content(Some("--.--"));
}

pub fn add_event_listener_on_stop_button(stopped_rc: Rc<RefCell<bool>>) {
    let document = web_sys::window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    // Add event listener to the "Stop demo" button to close the winit
    let stop_button_el = document
        .body()
        .unwrap()
        .query_selector("#stop")
        .unwrap()
        .unwrap();

    // Pass the flag whether the application should keep running to the callback for the stop button
    let cb = Closure::wrap(Box::new(move |_: Event| {
        let mut stopped = stopped_rc.try_borrow_mut().unwrap();
        *stopped = true;
    }) as Box<dyn FnMut(_)>);
    let _ = stop_button_el.add_event_listener_with_callback("click", &cb.as_ref().unchecked_ref());
    cb.forget();
}

pub fn stop_app(event_loop: &ActiveEventLoop) {
    event_loop.exit();
    stop_updating_fps();
    // Remove the canvas element
    let document = web_sys::window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let canvas_el: web_sys::Element = document
        .body()
        .unwrap()
        .query_selector("#window-wrap canvas")
        .unwrap()
        .unwrap();
    canvas_el.remove();
}
