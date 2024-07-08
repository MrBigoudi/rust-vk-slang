use crate::application::vk_app::{AppParameters, VulkanApp};

use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

impl VulkanApp {
    /// Initializes the event loop for handling window events.
    pub fn init_event_loop() -> EventLoop<()> {
        EventLoop::new()
            .unwrap_or_else(|err| panic!("Failed to create the event loop: {:?}\n", err))
    }

    /// Initializes the window with given application parameters and event loop.
    pub fn init_window(app_params: &AppParameters, event_loop: &EventLoop<()>) -> Window {
        WindowBuilder::new()
            .with_title(&app_params.name)
            .with_inner_size(winit::dpi::LogicalSize::new(
                f64::from(app_params.window_width),
                f64::from(app_params.window_height),
            ))
            .build(event_loop)
            .unwrap_or_else(|err| panic!("Failed to initialize the window: {:?}\n", err))
    }
}
