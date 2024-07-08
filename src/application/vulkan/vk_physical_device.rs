use std::ffi::CStr;

use crate::application::vk_app::{QueueFamilyIndices, VulkanApp, DEVICE_EXTENSION_NAMES_RAW};

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
            present_family: None,
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
                    let support_present = surface_loader
                        .get_physical_device_surface_support(
                            *physical_device,
                            index as u32,
                            *surface,
                        )
                        .unwrap();
                    if support_graphics && support_present {
                        queue_families.present_family = Some(index as u32);
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
    ) -> bool {
        let are_queue_families_completed =
            Self::find_queue_families(physical_device, instance, surface, surface_loader)
                .is_complete();
        let are_extensions_found = Self::check_device_extension_support(instance, physical_device);
        let is_swap_chain_supported =
            Self::query_swapchain_support(surface, surface_loader, physical_device).is_complete();
        are_queue_families_completed && are_extensions_found && is_swap_chain_supported
    }

    pub fn init_physical_device_and_queue_families(
        instance: &Instance,
        surface: &vk::SurfaceKHR,
        surface_loader: &surface::Instance,
    ) -> (vk::PhysicalDevice, QueueFamilyIndices) {
        let physical_device = Self::init_physical_devices(instance)
            .iter()
            .find_map(|physical_device| {
                let is_suitable =
                    Self::is_device_suitable(physical_device, instance, surface, surface_loader);
                if is_suitable {
                    Some(*physical_device)
                } else {
                    None
                }
            })
            .expect("Failed to find a suitable physical device!\n");
        (
            physical_device,
            Self::find_queue_families(&physical_device, instance, surface, surface_loader),
        )
    }

    pub fn init_physical_devices(instance: &Instance) -> Vec<vk::PhysicalDevice> {
        unsafe {
            instance
                .enumerate_physical_devices()
                .unwrap_or_else(|err| panic!("Failed to fetch the physical devices: {:?}\n", err))
        }
    }

    pub fn check_device_extension_support(
        instance: &Instance,
        physical_device: &vk::PhysicalDevice,
    ) -> bool {
        let extension_properties = unsafe {
            instance
                .enumerate_device_extension_properties(*physical_device)
                .unwrap()
        };

        'cur_extension: for required_extension in DEVICE_EXTENSION_NAMES_RAW {
            let required_extension_cstr = unsafe { CStr::from_ptr(required_extension) };
            for found_extension in &extension_properties {
                let found_extension_cstr =
                    unsafe { CStr::from_ptr(found_extension.extension_name.as_ptr()) };
                if found_extension_cstr == required_extension_cstr {
                    continue 'cur_extension;
                }
            }
            return false;
        }
        true
    }
}
