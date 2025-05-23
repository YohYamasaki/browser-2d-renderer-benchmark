use bouncing_rect::BouncingRect;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use std::rc::Rc;
use tiny_skia::{Color, Pixmap};
use wasm_bindgen::prelude::*;
use web_sys::{Event, Performance, window};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
use winit::window::{Window, WindowId};

mod bouncing_rect;

pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn performance() -> Performance {
    window()
        .and_then(|win| win.performance())
        .expect("Could not access the performance")
}

fn get_rect_velocity() -> f32 {
    let mut res = (1.0 + random() * 5.0) as f32;
    if random() < 0.5 {
        res *= -1.0
    }
    res
}

struct App<'a> {
    window: Rc<Window>,
    pixels: Pixels<'a>,
    pixmap: Pixmap,
    rects: Vec<BouncingRect>,
    start_time: f64,
    frame_count: i32,
}

impl<'a> App<'a> {
    fn new(window: Rc<Window>, pixels: Pixels<'a>, pixmap: Pixmap) -> App<'a> {
        App {
            window,
            pixels,
            rects: vec![],
            pixmap,
            start_time: 0.0,
            frame_count: 0,
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, _: &ActiveEventLoop) {
        // Set sample rectangle
        for _ in 0..1000 {
            let rect = BouncingRect::new(get_rect_velocity(), get_rect_velocity(), WIDTH, HEIGHT);
            self.rects.push(rect);
        }

        // Append window to canvas element
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access the document");
        let body = document.body().expect("Could not access document.body");

        use winit::platform::web::WindowExtWebSys;
        let winit_canvas = self
            .window
            .clone()
            .canvas()
            .expect("Failed to load canvas element");
        body.append_child(winit_canvas.as_ref())
            .expect("failed to append canvas");

        self.start_time = performance().now();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.pixmap.fill(Color::from_rgba8(0, 0, 0, 255));

                self.rects.iter().for_each(|rect| {
                    rect.draw(&mut self.pixmap);
                });

                self.pixels.frame_mut().copy_from_slice(self.pixmap.data());
                // Draw the current frame
                if let Err(err) = self.pixels.render() {
                    console_log!("pixels.render: {}", err);
                    event_loop.exit();
                    return;
                }
                if let Err(err) = self.pixels.render() {
                    console_log!("Failed to execute pixel.render(): {}", err.to_string());
                    event_loop.exit();
                    drop(self.window.clone());
                    return;
                }

                self.frame_count += 1;
                self.rects
                    .iter_mut()
                    .for_each(|rect| rect.update(WIDTH as i16, HEIGHT as i16));
                self.window.request_redraw();

                let now_ms = performance().now();
                if now_ms > self.start_time + (1000 * 10) as f64 {
                    let elapsed_secs = (now_ms - self.start_time) / 1000.0;
                    let fps = (self.frame_count as f32) / (elapsed_secs as f32);
                    console_log!("{:.2}fps", fps);

                    event_loop.exit();
                    drop(self.window.clone());
                }
            }
            _ => (),
        }
    }
}

async fn start_win() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let win = event_loop
        .create_window(
            Window::default_attributes().with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
        )
        .unwrap();

    // initialise surface with pixels
    let win = Rc::new(win);

    let pixels = {
        #[cfg(target_arch = "wasm32")]
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, win.clone());
        let builder = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture);

        #[cfg(target_arch = "wasm32")]
        let builder = {
            // Web targets do not support the default texture format
            let texture_format = pixels::wgpu::TextureFormat::Bgra8Unorm;
            builder
                .texture_format(texture_format)
                .surface_texture_format(texture_format)
        };

        builder.build_async().await
    }
    .expect("Failed to build Pixels");

    let pixmap = Pixmap::new(WIDTH, HEIGHT).unwrap();
    // Store all to fields
    let app = App::new(win, pixels, pixmap);
    event_loop.spawn_app(app);
}

fn main() {
    console_error_panic_hook::set_once();

    // insert start button
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");
    let body = document.body().expect("Could not access document.body");
    let button = document.create_element("button").unwrap();

    let cb = Closure::wrap(Box::new(|_: Event| {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_bindgen_futures::spawn_local(start_win());
    }) as Box<dyn FnMut(_)>);

    button.set_text_content(Some("Start"));
    button
        .add_event_listener_with_callback("click", &cb.as_ref().unchecked_ref())
        .unwrap();
    body.append_child(button.as_ref()).unwrap();
    cb.forget();
}
