use std::mem::ManuallyDrop;

use crate::application::vk_app::{AppParameters, VulkanApp};

use ash::vk::Extent2D;
use log::debug;
use winit::window::Window;

impl VulkanApp {
    /// Creates a new VulkanApp instance and initializes Vulkan components.
    pub fn init(app_params: AppParameters, window: &Window) -> VulkanApp {
        debug!("Init Entry...");
        let entry = Self::init_entry();
        debug!("Ok\n");

        debug!("Init Instance...");
        let instance = Self::init_instance(&app_params, window, &entry);
        debug!("Ok\n");

        debug!("Init Debug Callback...");
        let (debug_utils_loader, debug_call_back) = Self::init_debug_callback(&entry, &instance);
        debug!("Ok\n");

        debug!("Init Surface...");
        let surface = Self::init_surface(&entry, &instance, window);
        debug!("Ok\n");

        debug!("Init Surface Loader...");
        let surface_loader = Self::init_surface_loader(&entry, &instance);
        debug!("Ok\n");

        debug!("Init Physical Device and Queue Families...");
        let (physical_device, mut queue_families) =
            Self::init_physical_device_and_queue_families(&instance, &surface, &surface_loader);
        debug!("Ok\n");

        debug!("Init Device...");
        let device = Self::init_device(&physical_device, &instance, &mut queue_families);
        debug!("Ok\n");

        debug!("Init Swapchain...");
        let (
            swapchain_loader,
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
        let (mut frames, frame_number) = Self::init_frames();
        debug!("Ok\n");

        debug!("Init Commands...");
        Self::init_commands(&device, &queue_families, &mut frames);
        debug!("Ok\n");

        debug!("Init Sync Structures...");
        Self::init_sync_structuress(&device, &mut frames);
        debug!("Ok\n");

        // check frames
        frames.iter().for_each(|frame| frame.check());

        debug!("Init Memory Allocator...");
        let allocator = ManuallyDrop::new(Self::init_allocator(&instance, &device, &physical_device));
        debug!("Ok\n");

        debug!("Init Images...");
        let draw_image = Self::init_images(&app_params, &device, &allocator);
        let draw_extent = Extent2D::default();
        debug!("Ok\n");

        VulkanApp {
            app_params,
            entry,
            instance,
            debug_utils_loader,
            debug_call_back,
            surface,
            surface_loader,
            physical_device,
            queue_families,
            device,
            swapchain_loader,
            swapchain,
            swapchain_images,
            swapchain_image_format,
            swapchain_extent,
            swapchain_image_views,
            frames,
            frame_number,
            allocator,
            draw_image,
            draw_extent,
        }
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            for &frame in self.frames.iter() {
                self.device.destroy_semaphore(frame.swapchain_semaphore, None);
                self.device.destroy_semaphore(frame.render_semaphore, None);
                self.device.destroy_fence(frame.render_fence, None);
                self.device.destroy_command_pool(frame.command_pool, None);
            }

            for &image_view in self.swapchain_image_views.iter() {
                self.device.destroy_image_view(image_view, None);
            }

            self.device.destroy_image_view(self.draw_image.image_view, None);
            self.allocator.destroy_image(self.draw_image.image, &mut self.draw_image.allocation);

            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);

            // drop allocator before device
            ManuallyDrop::drop(&mut self.allocator);

            self.device.destroy_device(None);

            self.surface_loader.destroy_surface(self.surface, None);

            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_call_back, None);

            self.instance.destroy_instance(None);
        }
    }
}
