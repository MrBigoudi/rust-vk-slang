use crate::application::vk_app::{AppParameters, VulkanApp};

use log::debug;

impl VulkanApp {
    /// Creates a new VulkanApp instance and initializes Vulkan components.
    pub fn init(app_params: AppParameters) -> VulkanApp {
        debug!("Init Event Loop...");
        let event_loop = Self::init_event_loop();
        debug!("Ok\n");

        debug!("Init Window...");
        let window = Self::init_window(&app_params, &event_loop);
        debug!("Ok\n");

        debug!("Init Entry...");
        let entry = Self::init_entry();
        debug!("Ok\n");

        debug!("Init Instance...");
        let instance = Self::init_instance(&app_params, &window, &entry);
        debug!("Ok\n");

        debug!("Init Debug Callback...");
        let (debug_utils_loader, debug_call_back) = Self::init_debug_callback(&entry, &instance);
        debug!("Ok\n");

        debug!("Init Surface...");
        let surface = Self::init_surface(&entry, &instance, &window);
        debug!("Ok\n");

        debug!("Init Surface Loader...");
        let surface_loader = Self::init_surface_loader(&entry, &instance);
        debug!("Ok\n");

        debug!("Init Physical Device and Queue Families...");
        let (physical_device, queue_families) =
            Self::init_physical_device_and_queue_families(&instance, &surface, &surface_loader);
        debug!("Ok\n");

        debug!("Init Device...");
        let device = Self::init_device(&physical_device, &instance, &queue_families);
        debug!("Ok\n");

        debug!("Init Swapchain...");
        let (
            swapchain,
            swapchain_images,
            swapchain_image_format,
            swapchain_extent,
            swapchain_image_views,
        ) = Self::init_swapchain(
            &app_params,
            &instance,
            &device,
            &surface,
            &surface_loader,
            &physical_device,
        );
        debug!("Ok\n");

        debug!("Init Frames...");
        let (frames, frame_number) = Self::init_frames();
        debug!("Ok\n");

        VulkanApp {
            app_params,
            event_loop,
            window,
            entry,
            instance,
            debug_utils_loader,
            debug_call_back,
            surface,
            surface_loader,
            physical_device,
            queue_families,
            device,
            swapchain,
            swapchain_images,
            swapchain_image_format,
            swapchain_extent,
            swapchain_image_views,
            frames,
            frame_number,
        }
    }
}
