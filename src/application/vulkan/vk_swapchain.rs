use ash::{
    khr::{surface, swapchain},
    vk::{self, PhysicalDevice, PresentModeKHR, SurfaceFormatKHR, SurfaceKHR, SwapchainKHR},
    Device, Instance,
};

use crate::application::vk_app::{AppParameters, SwapChainSupportDetails, VulkanApp};

impl VulkanApp {
    pub fn query_swapchain_support(
        surface: &SurfaceKHR,
        surface_loader: &surface::Instance,
        physical_device: &PhysicalDevice,
    ) -> SwapChainSupportDetails {
        let surface_capabilities = unsafe {
            surface_loader
                .get_physical_device_surface_capabilities(*physical_device, *surface)
                .unwrap()
        };

        let surface_format = unsafe {
            surface_loader
                .get_physical_device_surface_formats(*physical_device, *surface)
                .unwrap()
        };

        let surface_present_modes = unsafe {
            surface_loader
                .get_physical_device_surface_present_modes(*physical_device, *surface)
                .unwrap()
        };

        SwapChainSupportDetails {
            capabilities: surface_capabilities,
            formats: surface_format,
            present_modes: surface_present_modes,
        }
    }

    pub fn choose_swapchain_format(available_formats: &[SurfaceFormatKHR]) -> SurfaceFormatKHR {
        available_formats
            .iter()
            .cloned()
            .find(|format| {
                format.format == vk::Format::B8G8R8A8_SRGB
                    && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or(available_formats[0])
    }

    pub fn choose_present_mode(available_present_modes: &[PresentModeKHR]) -> PresentModeKHR {
        available_present_modes
            .iter()
            .cloned()
            .find(|&present_mode| present_mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO)
    }

    pub fn init_swapchain(
        app_params: &AppParameters,
        instance: &Instance,
        device: &Device,
        surface: &SurfaceKHR,
        surface_loader: &surface::Instance,
        physical_device: &PhysicalDevice,
    ) -> SwapchainKHR {
        let swapchain_support =
            Self::query_swapchain_support(surface, surface_loader, physical_device);
        let mut desired_image_count = swapchain_support.capabilities.min_image_count + 1;
        if swapchain_support.capabilities.max_image_count > 0
            && desired_image_count > swapchain_support.capabilities.max_image_count
        {
            desired_image_count = swapchain_support.capabilities.max_image_count;
        }
        let surface_resolution = match swapchain_support.capabilities.current_extent.width {
            u32::MAX => vk::Extent2D {
                width: app_params.window_width as u32,
                height: app_params.window_height as u32,
            },
            _ => swapchain_support.capabilities.current_extent,
        };
        let pre_transform = if swapchain_support
            .capabilities
            .supported_transforms
            .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            swapchain_support.capabilities.current_transform
        };

        let surface_format = Self::choose_swapchain_format(&swapchain_support.formats);
        let surface_present_mode = Self::choose_present_mode(&swapchain_support.present_modes);

        let swapchain_loader = swapchain::Device::new(instance, device);

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(*surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(surface_present_mode)
            .clipped(true)
            .image_array_layers(1);

        unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap_or_else(|err| panic!("Failed to initialized the swapchain: {:?}", err))
        }
    }
}
