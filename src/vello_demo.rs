use crate::vello_demo::app::VelloApp;
use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod bouncing_rect;

pub fn run() -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Wait);
        // Create a vello Surface
        let app: VelloApp = VelloApp::new();
        use winit::platform::web::EventLoopExtWebSys;
        event_loop.spawn_app(app);
    }
    Ok(())
}
