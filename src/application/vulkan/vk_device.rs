use std::collections::HashSet;

use ash::{
    vk::{self, DeviceQueueCreateInfo, PhysicalDevice}, Device, Instance
};

use crate::application::vk_app::{QueueFamilyIndices, VulkanApp, DEVICE_EXTENSION_NAMES_RAW};

impl VulkanApp {
    pub fn init_device(
        physical_device: &PhysicalDevice,
        instance: &Instance,
        queue_families: &mut QueueFamilyIndices,
    ) -> Device {
        let features = vk::PhysicalDeviceFeatures::default()
            .shader_clip_distance(true);
        let mut features_12 = vk::PhysicalDeviceVulkan12Features::default()
            .buffer_device_address(true)
            .descriptor_indexing(true);
        let mut features_13 = vk::PhysicalDeviceVulkan13Features::default()
            .synchronization2(true)
            .dynamic_rendering(true);


        let priority = [1.0];
        let mut queue_create_infos: Vec<DeviceQueueCreateInfo> = Vec::new();
        let unique_queue_families: HashSet<u32> = queue_families.get_unique_queues();

        for queue_family_index in unique_queue_families {
            let queue_info = vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family_index)
                .queue_priorities(&priority);
            queue_create_infos.push(queue_info);
        }

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(&DEVICE_EXTENSION_NAMES_RAW)
            .enabled_features(&features)
            .push_next(&mut features_12)
            .push_next(&mut features_13);

        let device = unsafe {
            instance
                .create_device(*physical_device, &device_create_info, None)
                .unwrap()
        };

        queue_families.graphics_queue =
            unsafe { device.get_device_queue(queue_families.graphics_family.unwrap(), 0) };

        queue_families.present_queue =
            unsafe { device.get_device_queue(queue_families.present_family.unwrap(), 0) };

        device
    }
}
