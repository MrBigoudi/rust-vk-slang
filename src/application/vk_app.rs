use ash::{ext::debug_utils, khr::surface, vk, Entry, Instance};

use winit::{event_loop::EventLoop, window::Window};

/// Structure to hold application parameters such as name, window width, and window height.
pub struct AppParameters {
    pub name: String,
    pub window_width: i32,
    pub window_height: i32,
}

impl Default for AppParameters {
    /// Provides default values for the `AppParameters` structure.
    fn default() -> Self {
        Self {
            name: String::from("Rulkan"),
            window_width: 1280,
            window_height: 720,
        }
    }
}

/// Main structure to hold Vulkan application components.
pub struct VulkanApp {
    pub app_params: AppParameters,
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub entry: Entry,
    pub instance: Instance,
    pub debug_utils_loader: debug_utils::Instance,
    pub debug_call_back: vk::DebugUtilsMessengerEXT,

    pub surface: vk::SurfaceKHR,
    pub surface_loader: surface::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub queue_families: QueueFamilyIndices,
}

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

impl VulkanApp {
    pub fn new(app_params: AppParameters) -> Self {
        Self::init(app_params)
    }
}
