use ash::vk::{CommandBuffer, DescriptorSet, DescriptorSetLayout, Pipeline, PipelineLayout};
use ash::{
    util::read_spv,
    vk::{ShaderModule, ShaderModuleCreateInfo},
    Device,
};

use crate::application::vk_app::VulkanApp;

use super::vk_descriptors::DescriptorAllocator;

#[derive(Default)]
pub struct Descriptors {
    pub global_allocator_descriptor: DescriptorAllocator,
    pub draw_image_descriptors: DescriptorSet,
    pub draw_image_descriptor_layout: DescriptorSetLayout,
}

#[derive(Default)]
pub struct PipelineAttributes {
    pub descriptors: Descriptors,
    pub pipeline: Pipeline,
    pub pipeline_layout: PipelineLayout,
}

pub struct PipelineUtils;

impl PipelineUtils {
    pub fn load_shader_module(file_path: String, device: &Device) -> ShaderModule {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let spv_path = crate_path.to_owned() + &file_path;
        // open the file. With cursor at the end
        let mut spv_file = std::fs::File::open(spv_path.clone())
            .unwrap_or_else(|err| panic!("Failed to open the shader `{}': {:?}", spv_path, err));

        let spv_code = read_spv(&mut spv_file)
            .unwrap_or_else(|err| panic!("Faild to read the shader `{}': {:?}", spv_path, err));

        let create_info = ShaderModuleCreateInfo::default().code(&spv_code);

        unsafe { device.create_shader_module(&create_info, None).unwrap() }
    }
}

pub trait ComputePipeline {
    fn get_attributes(&self) -> &PipelineAttributes;

    fn init_descriptors(&mut self, vulkan_app: &mut VulkanApp);
    fn clear_descriptors(&mut self, device: &Device);
    fn create_pipeline_layout(&mut self, vulkan_app: &mut VulkanApp);
    fn create_compute_pipeline(&mut self, vulkan_app: &mut VulkanApp);

    fn run(&mut self, vulkan_app: &mut VulkanApp, command_buffer: &CommandBuffer);

    fn init(&mut self, vulkan_app: &mut VulkanApp) {
        self.init_descriptors(vulkan_app);
        self.create_pipeline_layout(vulkan_app);
        self.create_compute_pipeline(vulkan_app);
    }

    fn clear(&mut self, device: &Device) {
        self.clear_descriptors(device);
        unsafe {
            device.destroy_pipeline_layout(self.get_attributes().pipeline_layout, None);
            device.destroy_pipeline(self.get_attributes().pipeline, None);
        }
    }
}
