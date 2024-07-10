use ash::{vk::PhysicalDevice, Device, Instance};
use vk_mem::{Allocator, AllocatorCreateInfo};

use crate::application::vk_app::VulkanApp;

impl VulkanApp {
    pub fn init_allocator(
        instance: &Instance,
        device: &Device,
        physical_device: &PhysicalDevice,
    ) -> Allocator {
        let allocator_info = AllocatorCreateInfo::new(instance, device, *physical_device);
        unsafe {
            Allocator::new(allocator_info)
                .unwrap_or_else(|err| panic!("Failed to create the memory allocator: {:?}\n", err))
        }
    }
}
