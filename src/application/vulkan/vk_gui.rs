use std::sync::Mutex;

use ash::vk::{
    DescriptorPoolCreateFlags, DescriptorPoolCreateInfo, DescriptorPoolSize, DescriptorType,
};
use once_cell::sync::Lazy;
use winit::window::Window;

use crate::application::vk_app::VulkanApp;

static FONT_DATA_GLOBAL: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(load_font_data()));

fn load_font_data() -> Vec<u8> {
    let crate_path = env!("CARGO_MANIFEST_DIR");
    let font_path = format!("{}/src/assets/fonts/Roboto-Regular.ttf", crate_path);
    std::fs::read(font_path).expect("Failed to read font file")
}

impl VulkanApp {
    pub fn init_gui(&mut self, window: &Window) {
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

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[
            imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    size_pixels: font_size,
                    ..imgui::FontConfig::default()
                }),
            },
            imgui::FontSource::TtfData {
                data: FONT_DATA_GLOBAL.lock().unwrap().as_slice(),
                size_pixels: font_size,
                config: Some(imgui::FontConfig::default()),
            },
        ]);
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

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
            self.immediate_submit.command_pool,
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
            self.device
                .destroy_descriptor_pool(self.gui_parameters.descriptor_pool, None);
            self.gui_parameters.context = None;
            self.gui_parameters.platform = None;
            self.gui_parameters.renderer = None;
        };
    }
}
