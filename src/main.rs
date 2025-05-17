use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, window};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;
use winit::window::{Window, WindowId};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let win = event_loop
            .create_window(
                Window::default_attributes().with_inner_size(LogicalSize::new(800.0, 600.0)),
            )
            .unwrap();

        // 2. initialise context and surface
        let rc_win = Rc::new(win);
        let ctx = Context::new(rc_win.clone()).unwrap();
        let surf = Surface::new(&ctx, rc_win.clone()).unwrap();

        // Append window to canvas element
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access the document");
        let body = document.body().expect("Could not access document.body");

        use winit::platform::web::WindowExtWebSys;
        let winit_canvas = rc_win
            .clone()
            .canvas()
            .expect("Failed to load canvas element");
        body.append_child(winit_canvas.as_ref())
            .expect("failed to append canvas");

        // Store all to fields
        self.window = Some(rc_win);
        self.context = Some(ctx);
        self.surface = Some(surf);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
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
                let path =
                    PathBuilder::from_circle((width / 2) as f32, (height / 2) as f32, 100 as f32)
                        .unwrap();

                let mut paint = Paint::default();
                paint.set_color_rgba8(128, 0, 0, 255);
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::EvenOdd,
                    Transform::identity(),
                    None,
                );

                let mut buffer = self.surface.as_mut().unwrap().buffer_mut().unwrap();
                for index in 0..(width * height) as usize {
                    buffer[index] = pixmap.data()[index * 4 + 2] as u32
                        | (pixmap.data()[index * 4 + 1] as u32) << 8
                        | (pixmap.data()[index * 4 + 0] as u32) << 16;
                }
                buffer.present().unwrap();
                // self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let app = App::default();
    event_loop.spawn_app(app);
}
