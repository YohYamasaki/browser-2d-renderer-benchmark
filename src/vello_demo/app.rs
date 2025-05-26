use crate::js_bindings::{log, performance};
use std::cell::RefCell;
use std::rc::Rc;
use vello::peniko::color::palette;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Renderer, RendererOptions, Scene, wgpu};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use super::bouncing_rect::BouncingRect;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub enum RenderState {
    /// `RenderSurface` and `Window` for active rendering.
    Active {
        // The `RenderSurface` and the `Window` must be in this order, so that the surface is dropped first.
        context: RenderContext,
        window: Rc<Window>,
        surface: Box<RenderSurface<'static>>,
        renderers: Vec<Option<Renderer>>,
    },
    Suspend,
}

pub struct VelloApp {
    state: Rc<RefCell<RenderState>>,
    scene: Scene,
    rects: Vec<BouncingRect>,
    last_measure_time: f64,
    frame_count: i32,
}

impl<'s> VelloApp {
    pub fn new() -> VelloApp {
        VelloApp {
            state: Rc::new(RefCell::new(RenderState::Suspend)),
            scene: Scene::new(),
            rects: vec![],
            last_measure_time: 0 as f64,
            frame_count: 0 as i32,
        }
    }
}

impl ApplicationHandler for VelloApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Need to drop the borrow of the state before using it from the closure for spawn_local
        let is_suspend = {
            let state = self.state.borrow();
            matches!(*state, RenderState::Suspend)
        };

        if is_suspend {
            use winit::platform::web::WindowExtWebSys;

            // Create boucing rects
            BouncingRect::generate_rect(&mut self.rects, 10000, WIDTH as f64, HEIGHT as f64);

            // Initialize the winit window
            let window = Rc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
                            .with_resizable(true),
                    )
                    .unwrap(),
            );

            // Append the canvas element from the winit window to the #window-wrap element
            let canvas = window.canvas().unwrap();
            let window_wrap_el = web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.query_selector("#window-wrap").unwrap())
                .expect("Could not find canvas wrapper");
            window_wrap_el.append_child(canvas.as_ref()).ok();

            // Spawn a local asynchronous task (WASM single-threaded) to initialize rendering
            // We need an async closure for the creation of RenderSurface with wgpu
            let state_rc = self.state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut state = state_rc.borrow_mut();

                let mut context = RenderContext::new();
                let surface = context
                    .create_surface(window.clone(), WIDTH, HEIGHT, wgpu::PresentMode::AutoVsync)
                    .await
                    .unwrap();

                // Create a vello Renderer for the surface (using its device id)
                let mut renderers = vec![];
                renderers.resize_with((context.devices).len(), || None);
                renderers[surface.dev_id]
                    .get_or_insert_with(|| create_vello_renderer(&context, &surface));

                // Save the Window and Surface to the state
                *state = RenderState::Active {
                    context,
                    surface: Box::new(surface),
                    renderers,
                    window: window.clone(),
                };
                log("state initialized!");
            });
        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Ok(mut state) = self.state.try_borrow_mut() {
            let (context, surface, window, renderers) = match &mut *state {
                RenderState::Active {
                    context,
                    surface,
                    window,
                    renderers,
                } if window.id() == window_id => (context, surface, window, renderers),
                _ => {
                    log("not initialized yet");
                    return;
                }
            };

            // Only process events for our window, and only when we have a surface.
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    self.scene.reset();
                    let width = surface.config.width;
                    let height = surface.config.height;
                    let device_handle = &context.devices[surface.dev_id];

                    // Re-add the objects to draw to the scene
                    self.rects
                        .iter()
                        .for_each(|rect| rect.draw(&mut self.scene));

                    // Render to a texture, which we will later copy into the surface
                    renderers[surface.dev_id]
                        .as_mut()
                        .unwrap()
                        .render_to_texture(
                            &device_handle.device,
                            &device_handle.queue,
                            &self.scene,
                            &surface.target_view,
                            &vello::RenderParams {
                                base_color: palette::css::BLACK,
                                width,
                                height,
                                antialiasing_method: AaConfig::Area,
                            },
                        )
                        .expect("failed to render to surface");

                    // Get the surface's texture
                    let surface_texture = surface
                        .surface
                        .get_current_texture()
                        .expect("failed to get surface texture");

                    // Perform the copy
                    let mut encoder = device_handle.device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {
                            label: Some("Surface Blit"),
                        },
                    );
                    surface.blitter.copy(
                        &device_handle.device,
                        &mut encoder,
                        &surface.target_view,
                        &surface_texture
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    );
                    device_handle.queue.submit([encoder.finish()]);

                    // Queue the texture to be presented on the surface
                    surface_texture.present();
                    device_handle.device.poll(wgpu::Maintain::Poll);

                    // Update the position of the rects
                    self.rects
                        .iter_mut()
                        .for_each(|rect| rect.update(WIDTH as f64, HEIGHT as f64));

                    window.request_redraw();

                    // Calculate the fps
                    self.frame_count += 1;
                    let now_ms = performance().now();
                    if now_ms - self.last_measure_time > 3000.0 {
                        let elapsed_secs = (now_ms - self.last_measure_time) / 1000.0;
                        let fps = (self.frame_count as f32) / (elapsed_secs as f32);
                        log(&format!("fps: {}", fps));
                        self.frame_count = 0;
                        self.last_measure_time = now_ms;
                        // exiting the event loop after 10 seconds
                        // if now_ms - self.start_time > 10_000.0 {
                        //     event_loop.exit();
                        // }
                    }
                }
                _ => (),
            }
        } else {
            // Ignore all events if the state is borrowed
            log("state is still borrowed");
            match event {
                _ => (),
            }
        }
    }
}

/// Helper function that creates a vello `Renderer` for a given `RenderContext` and `RenderSurface`
fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface<'_>) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions::default(),
    )
    .expect("Couldn't create renderer")
}
