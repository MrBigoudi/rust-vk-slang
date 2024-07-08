use ash::Entry;

use crate::application::vk_app::VulkanApp;

impl VulkanApp {
    /// Loads the Vulkan library entry points.
    pub fn init_entry() -> Entry {
        unsafe {
            Entry::load()
                .unwrap_or_else(|err| panic!("Failed to load the vulkan library: {:?}", err))
        }
    }
}
