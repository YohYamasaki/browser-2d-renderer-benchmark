use std::num::NonZeroU32;
use std::rc::Rc;
use softbuffer::{Context, Surface};
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    context: Option<Context<Rc<Window>>>,
    surface: Option<Surface<Rc<Window>, Rc<Window>>>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let win = event_loop
            .create_window(Window::default_attributes()
                .with_inner_size(LogicalSize::new(800.0, 600.0)))
            .unwrap();

        // 2. Context & Surface の初期化
        let rc_win = Rc::new(win);
        let ctx = Context::new(rc_win.clone()).unwrap();
        let surf = Surface::new(&ctx, rc_win.clone()).unwrap();

        // 3. フィールドに格納
        self.window = Some(rc_win);
        self.context = Some(ctx);
        self.surface = Some(surf);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let (width, height) = {
                    let size = self.window.as_ref().unwrap().inner_size();
                    (size.width, size.height)
                };
                self.surface.as_mut().unwrap().resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                ).unwrap();

                let mut pixmap = Pixmap::new(width, height).unwrap();
                let path = PathBuilder::from_circle(
                    (width / 2) as f32,
                    (height / 2) as f32,
                    100 as f32,
                ).unwrap();

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
                    buffer[index] =
                        pixmap.data()[index * 4 + 2] as u32
                            | (pixmap.data()[index * 4 + 1] as u32) << 8
                            | (pixmap.data()[index * 4 + 0] as u32) << 16;
                }
                buffer.present().unwrap();
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app);
}