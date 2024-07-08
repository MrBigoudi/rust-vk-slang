use crate::application::vk_app::{QueueFamilyIndices, VulkanApp};

use ash::{khr::surface, vk, Instance};

impl VulkanApp {
    pub fn find_queue_families(
        physical_device: &vk::PhysicalDevice,
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> QueueFamilyIndices {
        let mut queue_families = QueueFamilyIndices {
            graphics_family: None,
        };
        unsafe {
            instance
                .get_physical_device_queue_family_properties(*physical_device)
                .iter()
                .enumerate()
                .find_map(|(index, queue_family_properties)| {
                    let support_graphics = queue_family_properties
                        .queue_flags
                        .contains(vk::QueueFlags::GRAPHICS);
                    let support_surface = surface_loader
                        .get_physical_device_surface_support(
                            *physical_device,
                            index as u32,
                            *surface,
                        )
                        .unwrap();
                    if support_graphics && support_surface {
                        queue_families.graphics_family = Some(index as u32);
                        Some(())
                    } else {
                        None
                    }
                })
                .expect("Failed to find a suitable")
        };
        queue_families
    }

    pub fn is_device_suitable(
        physical_device: &vk::PhysicalDevice,
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (bool, QueueFamilyIndices) {
        let queue_families =
            Self::find_queue_families(physical_device, instance, surface, surface_loader);
        (queue_families.is_complete(), queue_families)
    }

    pub fn init_physical_device_and_queue_families(
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (vk::PhysicalDevice, QueueFamilyIndices) {
        Self::init_physical_devices(instance)
            .iter()
            .find_map(|physical_device| {
                let (is_suitable, queue_families) =
                    Self::is_device_suitable(physical_device, instance, surface, surface_loader);
                if is_suitable {
                    Some((*physical_device, queue_families))
                } else {
                    None
                }
            })
            .expect("Failed to find a suitable physical device!\n")
    }

    pub fn init_physical_devices(instance: &Instance) -> Vec<vk::PhysicalDevice> {
        unsafe {
            instance
                .enumerate_physical_devices()
                .unwrap_or_else(|err| panic!("Failed to fetch the physical devices: {:?}\n", err))
        }
    }
}
