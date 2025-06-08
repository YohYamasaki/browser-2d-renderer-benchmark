use crate::vello_demo::app::VelloApp;
use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod bouncing_rect;

pub fn run(canvas_width: u32, canvas_height: u32, box_size: u32, box_number: u32) -> Result<()> {
    #[cfg(target_arch = "wasm32")]
    {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Wait);
        // Create a vello Surface
        let app: VelloApp = VelloApp::new(canvas_width, canvas_height, box_size, box_number);
        use winit::platform::web::EventLoopExtWebSys;
        event_loop.spawn_app(app);
    }
    Ok(())
}
