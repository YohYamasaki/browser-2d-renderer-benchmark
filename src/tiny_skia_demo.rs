pub mod app;
mod bouncing_rect;

use app::TinySkiaApp;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::web::EventLoopExtWebSys,
};

pub async fn run(canvas_width: u32, canvas_height: u32, box_size: u32, box_number: u32) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let app = TinySkiaApp::new(canvas_width, canvas_height, box_size, box_number);
    event_loop.spawn_app(app);
}
