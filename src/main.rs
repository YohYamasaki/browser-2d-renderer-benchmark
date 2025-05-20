use pixels::{PixelsBuilder, SurfaceTexture};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use tiny_skia::Pixmap;
use wasm_bindgen::prelude::*;
use web_sys::{Event, Performance, window};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
use winit::window::{Window, WindowId};

mod shape;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn performance() -> Performance {
    window()
        .and_then(|win| win.performance())
        .expect("Could not access the performance")
}

struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}
struct App<'a> {
    window: Rc<Window>,
    pixels: Result<pixels::Pixels<'a>, pixels::Error>,
    start_time: f64,
    frame_count: i32,
}

impl<'a> App<'a> {
    fn new(window: Rc<Window>, pixels: Result<pixels::Pixels<'a>, pixels::Error>) -> App<'a> {
        App {
            window,
            pixels,
            start_time: 0.0,
            frame_count: 0,
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // ここでwindowの初期化

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
                // レンダリングのsurfaceへのアクセスを変更する
                let (width, height) = {
                    let size = self.window.as_ref().unwrap().inner_size();
                    (size.width, size.height)
                };
                self.surface
                    .as_mut()
                    .unwrap()
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut pixmap = Pixmap::new(width, height).unwrap();
                shape::draw(&mut pixmap, performance().now() as f32);

                let mut buffer = self.surface.as_mut().unwrap().buffer_mut().unwrap();
                for index in 0..(width * height) as usize {
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4 + 0] as u32) << 16;
                }
                buffer.present().unwrap();

                self.frame_count += 1;
                self.window.as_ref().unwrap().request_redraw();

                let now_ms = performance().now();
                if now_ms > self.start_time + (1000 * 5) as f64 {
                    let elapsed_secs = (now_ms - self.start_time) / 1000.0;
                    let fps = (self.frame_count as f32) / (elapsed_secs as f32);
                    console_log!("{:.2}fps", fps);

                    event_loop.exit();
                    if let Some(rc_win) = self.window.take() {
                        drop(rc_win);
                    }
                    let _ = self.context.take();
                    let _ = self.surface.take();
                }
            }
            _ => (),
        }
    }
}

async fn start_win() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // winをOptionか空の状態でAppに保存して、resumed内で初期化する
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
            let texture_format = pixels::wgpu::TextureFormat::Rgba8Unorm;
            builder
                .texture_format(texture_format)
                .surface_texture_format(texture_format)
        };

        builder.build_async().await
    };

    // Store all to fields
    let app = App::new(win, pixels);
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
