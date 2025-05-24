use super::bouncing_rect::BouncingRect;
use crate::js_bindings::{performance, send_results_to_js};
use softbuffer::{Context, Surface};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::rc::Rc;
use tiny_skia::{Color, Pixmap};
use web_sys::window;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
#[cfg(target_arch = "wasm32")]
use winit::window::{Window, WindowId};

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 300;

#[derive(Default)]
pub struct TinySkiaApp {
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
    pixmap: Option<Pixmap>,
    rects: Vec<BouncingRect>,
    start_time: f64,
    last_measure_time: f64,
    frame_count: i32,
    results: HashMap<i32, f32>,
}

impl ApplicationHandler for TinySkiaApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let win = event_loop
            .create_window(
                Window::default_attributes().with_inner_size(LogicalSize::new(WIDTH, HEIGHT)),
            )
            .unwrap();

        // initialise surface with pixels
        let rc_win = Rc::new(win);
        let ctx = Context::new(rc_win.clone()).unwrap();
        let surf = Surface::new(&ctx, rc_win.clone()).unwrap();

        // Append winit window to canvas element
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access the document");
        let window_wrap_el = document
            .body()
            .unwrap()
            .query_selector("#window-wrap")
            .unwrap()
            .unwrap();

        use winit::platform::web::WindowExtWebSys;
        let winit_canvas = rc_win
            .clone()
            .canvas()
            .expect("Failed to load canvas element");
        window_wrap_el
            .append_child(winit_canvas.as_ref())
            .expect("failed to append canvas");

        // Store all data to the fields
        self.window = Some(rc_win);
        self.context = Some(ctx);
        self.surface = Some(surf);
        self.pixmap = Some(Pixmap::new(WIDTH, HEIGHT).unwrap());
        BouncingRect::generate_rect(&mut self.rects, 100, WIDTH, HEIGHT);
        self.start_time = performance().now();
        self.last_measure_time = self.start_time;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let mut pixmap = self.pixmap.as_mut().unwrap();
                pixmap.fill(Color::from_rgba8(0, 0, 0, 255));
                self.rects.iter().for_each(|rect| rect.draw(&mut pixmap));

                self.surface
                    .as_mut()
                    .unwrap()
                    .resize(
                        NonZeroU32::new(WIDTH).unwrap(),
                        NonZeroU32::new(HEIGHT).unwrap(),
                    )
                    .unwrap();
                let mut buffer = self.surface.as_mut().unwrap().buffer_mut().unwrap();

                for index in 0..(WIDTH * HEIGHT) as usize {
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4 + 0] as u32) << 16;
                }
                buffer.present().unwrap();

                self.frame_count += 1;
                self.rects
                    .iter_mut()
                    .for_each(|rect| rect.update(WIDTH as i16, HEIGHT as i16));
                self.window.as_ref().unwrap().request_redraw();

                let now_ms = performance().now();
                if now_ms - self.last_measure_time > 3000.0 {
                    let elapsed_secs = (now_ms - self.last_measure_time) / 1000.0;
                    let fps = (self.frame_count as f32) / (elapsed_secs as f32);
                    self.results.insert(self.rects.len() as i32, fps);
                    self.frame_count = 0;
                    self.last_measure_time = now_ms;
                    BouncingRect::generate_rect(&mut self.rects, 100, WIDTH, HEIGHT);

                    // exiting the event loop after 10 seconds
                    if now_ms - self.start_time > 10_000.0 {
                        send_results_to_js(&self.results, "#tiny-skia");
                        event_loop.exit();
                    }
                }
            }
            _ => (),
        }
    }
}
