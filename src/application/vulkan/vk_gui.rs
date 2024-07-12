use ash::vk::{
    CommandBufferAllocateInfo, CommandPoolCreateFlags, CommandPoolCreateInfo,
    DescriptorPoolCreateFlags, DescriptorPoolCreateInfo, DescriptorPoolSize, DescriptorType,
    FenceCreateFlags, FenceCreateInfo,
};
use winit::window::Window;

use crate::application::vk_app::VulkanApp;

impl VulkanApp {
    fn init_gui_immediate_submit_structures(&mut self) {
        let command_pool_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(self.queue_families.graphics_family.unwrap());

        unsafe {
            self.gui_parameters.immediate_submit_struct.command_pool = self
                .device
                .create_command_pool(&command_pool_info, None)
                .unwrap();
        }

        let allocate_info = CommandBufferAllocateInfo::default()
            .command_buffer_count(1)
            .command_pool(self.gui_parameters.immediate_submit_struct.command_pool);

        unsafe {
            self.gui_parameters.immediate_submit_struct.command_buffer = self
                .device
                .allocate_command_buffers(&allocate_info)
                .unwrap()[0]
        }

        let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);
        unsafe {
            self.gui_parameters.immediate_submit_struct.fence =
                self.device.create_fence(&fence_create_info, None).unwrap()
        }
    }

    pub fn init_gui(&mut self, window: &Window) {
        self.init_gui_immediate_submit_structures();

        // create descriptor pool for IMGUI
        let descriptor_pool_sizes = [
            DescriptorPoolSize::default()
                .ty(DescriptorType::SAMPLER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::SAMPLED_IMAGE)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::STORAGE_IMAGE)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::UNIFORM_TEXEL_BUFFER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::STORAGE_TEXEL_BUFFER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::STORAGE_BUFFER)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::UNIFORM_BUFFER_DYNAMIC)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::STORAGE_BUFFER_DYNAMIC)
                .descriptor_count(1000),
            DescriptorPoolSize::default()
                .ty(DescriptorType::INPUT_ATTACHMENT)
                .descriptor_count(1000),
        ];

        let descriptor_pool_create_info = DescriptorPoolCreateInfo::default()
            .flags(DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
            .max_sets(1000)
            .pool_sizes(&descriptor_pool_sizes);

        self.gui_parameters.descriptor_pool = unsafe {
            self.device
                .create_descriptor_pool(&descriptor_pool_create_info, None)
                .unwrap()
        };

        // initialize gui library
        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Rounded,
        );

        let dynamic_rendering = imgui_rs_vulkan_renderer::DynamicRendering {
            color_attachment_format: self.swapchain_image_format,
            depth_attachment_format: None,
        };

        let renderer = imgui_rs_vulkan_renderer::Renderer::with_vk_mem_allocator(
            self.allocator.allocator.clone(),
            self.device.clone(),
            self.queue_families.graphics_queue,
            self.gui_parameters.immediate_submit_struct.command_pool,
            dynamic_rendering,
            &mut imgui,
            Some(imgui_rs_vulkan_renderer::Options {
                in_flight_frames: 1,
                ..Default::default()
            }),
        )
        .unwrap();

        self.gui_parameters.renderer = Some(renderer);
        self.gui_parameters.platform = Some(platform);
        self.gui_parameters.context = Some(imgui);
    }

    pub fn clear_gui(&mut self) {
        unsafe {
            self.device.destroy_command_pool(
                self.gui_parameters.immediate_submit_struct.command_pool,
                None,
            );
            self.device
                .destroy_fence(self.gui_parameters.immediate_submit_struct.fence, None);
            self.device
                .destroy_descriptor_pool(self.gui_parameters.descriptor_pool, None);
            self.gui_parameters.context = None;
            self.gui_parameters.platform = None;
            self.gui_parameters.renderer = None;
        };
    }
}
