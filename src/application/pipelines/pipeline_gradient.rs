use std::ffi::CStr;

use ash::{
    vk::{
        ComputePipelineCreateInfo, DescriptorImageInfo, DescriptorSetLayoutCreateFlags,
        DescriptorType, ImageLayout, PipelineBindPoint, PipelineCache, PipelineLayoutCreateInfo,
        PipelineShaderStageCreateInfo, ShaderStageFlags, WriteDescriptorSet,
    },
    Device,
};

use crate::application::vk_app::VulkanApp;

use super::{
    pipeline::{ComputePipeline, Descriptors, PipelineAttributes, PipelineUtils},
    vk_descriptors::{DescriptorAllocator, DescriptorLayoutBuilder, PoolSizeRatio},
};

#[derive(Default)]
pub struct PipelineGradient {
    pub base_attributes: PipelineAttributes,
}

impl ComputePipeline for PipelineGradient {
    fn init_descriptors(&mut self, vulkan_app: &mut VulkanApp) {
        // create a descriptor pool that will hold 10 sets with 1 image each
        let pool_size_ratios = [PoolSizeRatio {
            descriptor_type: DescriptorType::STORAGE_IMAGE,
            ratio: 1.0,
        }];

        let mut global_allocator_descriptor = DescriptorAllocator::default();
        global_allocator_descriptor.init_pool(&vulkan_app.device, 10, &pool_size_ratios);

        // make the descriptor set layout for our compute draw
        let mut builder = DescriptorLayoutBuilder::default();
        builder.add_binding(0, DescriptorType::STORAGE_IMAGE);
        let draw_image_descriptor_layout = builder.build(
            &vulkan_app.device,
            ShaderStageFlags::COMPUTE,
            DescriptorSetLayoutCreateFlags::empty(),
        );

        // allocate a descriptor set for our draw image
        let draw_image_descriptors =
            global_allocator_descriptor.allocate(&vulkan_app.device, &draw_image_descriptor_layout);

        let descriptor_image_info = [DescriptorImageInfo::default()
            .image_view(vulkan_app.draw_image.image_view)
            .image_layout(ImageLayout::GENERAL)];
        let descriptor_writes = [WriteDescriptorSet::default()
            .dst_binding(0)
            .dst_set(draw_image_descriptors)
            .descriptor_count(1)
            .descriptor_type(DescriptorType::STORAGE_IMAGE)
            .image_info(&descriptor_image_info)];

        unsafe {
            vulkan_app
                .device
                .update_descriptor_sets(&descriptor_writes, &[]);
        }

        self.base_attributes.descriptors = Descriptors {
            global_allocator_descriptor,
            draw_image_descriptors,
            draw_image_descriptor_layout,
        }
    }

    fn clear_descriptors(&mut self, device: &Device) {
        unsafe {
            self.base_attributes
                .descriptors
                .global_allocator_descriptor
                .clear_pool(device);
            device.destroy_descriptor_set_layout(
                self.base_attributes
                    .descriptors
                    .draw_image_descriptor_layout,
                None,
            );
        }
    }

    fn create_pipeline_layout(&mut self, vulkan_app: &mut VulkanApp) {
        let layouts = [self
            .base_attributes
            .descriptors
            .draw_image_descriptor_layout];
        let create_info = PipelineLayoutCreateInfo::default().set_layouts(&layouts);

        unsafe {
            self.base_attributes.pipeline_layout = vulkan_app
                .device
                .create_pipeline_layout(&create_info, None)
                .unwrap();
        }
    }

    fn create_compute_pipeline(&mut self, vulkan_app: &mut VulkanApp) {
        let shader_module = PipelineUtils::load_shader_module(
            String::from("/src/shaders/gradient.spv"),
            &vulkan_app.device,
        );
        let shader_stage_create_info = PipelineShaderStageCreateInfo::default()
            .stage(ShaderStageFlags::COMPUTE)
            .module(shader_module)
            .name(CStr::from_bytes_with_nul(b"main\0").unwrap());

        let compute_pipeline_create_info = [ComputePipelineCreateInfo::default()
            .layout(self.base_attributes.pipeline_layout)
            .stage(shader_stage_create_info)];

        unsafe {
            self.base_attributes.pipeline = vulkan_app
                .device
                .create_compute_pipelines(
                    PipelineCache::null(),
                    &compute_pipeline_create_info,
                    None,
                )
                .unwrap()[0];
        }

        unsafe { vulkan_app.device.destroy_shader_module(shader_module, None) };
    }

    fn get_attributes(&self) -> &PipelineAttributes {
        &self.base_attributes
    }

    fn run(&mut self, vulkan_app: &mut VulkanApp, command_buffer: &ash::vk::CommandBuffer) {
        unsafe {
            // bind the gradient drawing compute pipeline
            vulkan_app.device.cmd_bind_pipeline(
                *command_buffer,
                PipelineBindPoint::COMPUTE,
                self.base_attributes.pipeline,
            );

            // bind the descriptor set containing the draw image for the compute pipeline
            vulkan_app.device.cmd_bind_descriptor_sets(
                *command_buffer,
                PipelineBindPoint::COMPUTE,
                self.base_attributes.pipeline_layout,
                0,
                &[self.base_attributes.descriptors.draw_image_descriptors],
                &[],
            );

            // execute the compute pipeline dispatch. We are using 16x16 workgroup size so we need to divide by it
            vulkan_app.device.cmd_dispatch(
                *command_buffer,
                vulkan_app.draw_extent.width / 16,
                vulkan_app.draw_extent.height / 16,
                1,
            );
        }
    }
}
