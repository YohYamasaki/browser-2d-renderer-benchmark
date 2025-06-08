use super::bouncing_rect::BouncingRect;
use crate::js_bindings::{
    add_event_listener_on_stop_button, log, performance, stop_app, stop_updating_fps, update_fps,
};
use softbuffer::{Context, Surface};
use std::cell::RefCell;
use std::num::NonZeroU32;
use std::rc::Rc;
use tiny_skia::{Color, Pixmap};
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Event, window};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
#[cfg(target_arch = "wasm32")]
use winit::window::{Window, WindowId};

#[derive(Default)]

struct Config {
    canvas_width: u32,
    canvas_height: u32,
    box_size: u32,
    box_number: u32,
}

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
    config: Config,
    stopped: Rc<RefCell<bool>>,
}

impl TinySkiaApp {
    pub fn new(
        canvas_width: u32,
        canvas_height: u32,
        box_size: u32,
        box_number: u32,
    ) -> TinySkiaApp {
        TinySkiaApp {
            config: Config {
                canvas_width,
                canvas_height,
                box_size,
                box_number,
            },
            ..TinySkiaApp::default()
        }
    }
}

impl ApplicationHandler for TinySkiaApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let canvas_width = self.config.canvas_width;
        let canvas_height = self.config.canvas_height;

        let win = event_loop
            .create_window(
                Window::default_attributes()
                    .with_inner_size(LogicalSize::new(canvas_width, canvas_height)),
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
        self.pixmap = Some(Pixmap::new(canvas_width, canvas_height).unwrap());
        BouncingRect::generate_rect(
            &mut self.rects,
            self.config.box_number,
            self.config.box_size as f32,
            canvas_width,
            canvas_height,
        );
        self.start_time = performance().now();
        self.last_measure_time = self.start_time;

        add_event_listener_on_stop_button(self.stopped.clone());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Ok(stopped) = self.stopped.try_borrow() {
                    if *stopped {
                        if let Ok(stopped) = self.stopped.try_borrow() {
                            if *stopped {
                                stop_app(event_loop);
                            }
                        }
                    }
                }

                let canvas_width = self.config.canvas_width;
                let canvas_height = self.config.canvas_height;

                let mut pixmap = self.pixmap.as_mut().unwrap();
                pixmap.fill(Color::from_rgba8(0, 0, 0, 255));
                self.rects.iter().for_each(|rect| rect.draw(&mut pixmap));

                self.surface
                    .as_mut()
                    .unwrap()
                    .resize(
                        NonZeroU32::new(canvas_width).unwrap(),
                        NonZeroU32::new(canvas_height).unwrap(),
                    )
                    .unwrap();
                let mut buffer = self.surface.as_mut().unwrap().buffer_mut().unwrap();

                for index in 0..(canvas_width * canvas_height) as usize {
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4 + 0] as u32) << 16;
                }
                buffer.present().unwrap();

                self.frame_count += 1;
                self.rects
                    .iter_mut()
                    .for_each(|rect| rect.update(canvas_width as i16, canvas_height as i16));
                self.window.as_ref().unwrap().request_redraw();

                let now_ms = performance().now();
                if now_ms - self.last_measure_time > 1000.0 {
                    let elapsed_secs = (now_ms - self.last_measure_time) / 1000.0;
                    let fps = (self.frame_count as f64) / elapsed_secs;
                    self.frame_count = 0;
                    self.last_measure_time = now_ms;

                    update_fps(fps);
                }
            }
            _ => (),
        }
    }
}
