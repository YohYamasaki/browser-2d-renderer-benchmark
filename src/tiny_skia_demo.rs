pub mod app;
mod bouncing_rect;

use app::TinySkiaApp;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::web::EventLoopExtWebSys,
};

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let app = TinySkiaApp::default();
    event_loop.spawn_app(app);
}
