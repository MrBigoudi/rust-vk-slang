use std::collections::HashSet;

use ash::{
    ext::debug_utils,
    khr::{surface, swapchain},
    vk::{self, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SwapchainKHR},
    Device, Entry, Instance,
};

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
    pub device: Device,
    pub swapchain: SwapchainKHR,
}

pub const DEVICE_EXTENSION_NAMES_RAW: [*const i8; 1] = [swapchain::NAME.as_ptr()];

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub present_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }

    pub fn get_unique_queues(&self) -> HashSet<u32> {
        let mut set: HashSet<u32> = HashSet::new();
        set.insert(self.graphics_family.unwrap());
        set.insert(self.present_family.unwrap());
        set
    }
}

pub struct SwapChainSupportDetails {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>,
}

impl SwapChainSupportDetails {
    pub fn is_complete(&self) -> bool {
        !self.formats.is_empty() && !self.present_modes.is_empty()
    }
}

impl VulkanApp {
    pub fn new(app_params: AppParameters) -> Self {
        Self::init(app_params)
    }
}
