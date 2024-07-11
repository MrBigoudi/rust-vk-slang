use crate::application::vk_app::VulkanApp;

use ash::{khr::surface, vk, Entry, Instance};

use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

impl VulkanApp {
    /// Init the surface
    pub fn init_surface(entry: &Entry, instance: &Instance, window: &Window) -> vk::SurfaceKHR {
        unsafe {
            ash_window::create_surface(
                entry,
                instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .unwrap_or_else(|err| panic!("Failed to create the surface: {:?}\n", err))
        }
    }

    pub fn init_surface_loader(entry: &Entry, instance: &Instance) -> surface::Instance {
        surface::Instance::new(entry, instance)
    }

    pub fn clear_surface(&self){
        unsafe{
            self.surface_loader.destroy_surface(self.surface, None);
        }
    }
}
