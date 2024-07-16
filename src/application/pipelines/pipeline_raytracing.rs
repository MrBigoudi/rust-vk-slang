use std::ffi::CStr;

use ash::{
    vk::{
        ComputePipelineCreateInfo, DescriptorBufferInfo, DescriptorImageInfo,
        DescriptorSetLayoutCreateFlags, DescriptorType, ImageLayout, PipelineBindPoint,
        PipelineCache, PipelineLayoutCreateInfo, PipelineShaderStageCreateInfo, ShaderStageFlags,
        WriteDescriptorSet, WHOLE_SIZE,
    },
    Device,
};

use crate::application::vk_app::VulkanApp;

use super::{
    pipeline::{ComputePipeline, Descriptors, PipelineAttributes, PipelineUtils},
    vk_descriptors::{DescriptorAllocator, DescriptorLayoutBuilder, PoolSizeRatio},
};

#[derive(Default)]
pub struct PipelineRaytracing {
    pub base_attributes: PipelineAttributes,
}

impl ComputePipeline for PipelineRaytracing {
    fn init_descriptors(&mut self, vulkan_app: &mut VulkanApp) {
        let pool_size_ratios = [
            // framebuffer
            PoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_IMAGE,
                ratio: 1.0,
            },
            // triangles buffer
            PoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_BUFFER,
                ratio: 1.0,
            },
            // models buffer
            PoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_BUFFER,
                ratio: 1.0,
            },
            // materials buffer
            PoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_BUFFER,
                ratio: 1.0,
            },
        ];

        let mut global_allocator_descriptor = DescriptorAllocator::default();
        global_allocator_descriptor.init_pool(&vulkan_app.device, 10, &pool_size_ratios);

        // make the descriptor set layout for our compute draw
        let mut builder = DescriptorLayoutBuilder::default();
        // framebuffer
        builder.add_binding(0, DescriptorType::STORAGE_IMAGE);
        // triangles buffer
        builder.add_binding(1, DescriptorType::STORAGE_BUFFER);
        // models buffer
        builder.add_binding(2, DescriptorType::STORAGE_BUFFER);
        // materials buffer
        builder.add_binding(3, DescriptorType::STORAGE_BUFFER);

        let descriptor_set_layout = builder.build(
            &vulkan_app.device,
            ShaderStageFlags::COMPUTE,
            DescriptorSetLayoutCreateFlags::empty(),
        );

        let scene_buffers_gpu = {
            let scene = &vulkan_app.scene;
            scene.upload_buffers(vulkan_app)
        };

        // allocate a descriptor set for our draw image and buffer
        let descriptor_set =
            global_allocator_descriptor.allocate(&vulkan_app.device, &descriptor_set_layout);

        // frame buffer
        let descriptor_framebuffer_info = [DescriptorImageInfo::default()
            .image_view(vulkan_app.draw_image.image_view)
            .image_layout(ImageLayout::GENERAL)];
        // triangles buffer
        let descriptor_triangles_buffer_info = [DescriptorBufferInfo::default()
            .buffer(scene_buffers_gpu.triangles_buffer.buffer.buffer)
            .range(WHOLE_SIZE)
            .offset(0)];
        // models buffer
        let descriptor_models_buffer_info = [DescriptorBufferInfo::default()
            .buffer(scene_buffers_gpu.models_buffer.buffer.buffer)
            .range(WHOLE_SIZE)
            .offset(0)];
        // materials buffer
        let descriptor_materials_buffer_info = [DescriptorBufferInfo::default()
            .buffer(scene_buffers_gpu.materials_buffer.buffer.buffer)
            .range(WHOLE_SIZE)
            .offset(0)];

        let descriptor_writes = [
            // framebuffer binding in set 0
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(0) // binding within the set
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_IMAGE)
                .image_info(&descriptor_framebuffer_info),
            // triangles buffer in set 0
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(1) // binding within the set
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_BUFFER)
                .buffer_info(&descriptor_triangles_buffer_info),
            // models buffer in set 0
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(2) // binding within the set
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_BUFFER)
                .buffer_info(&descriptor_models_buffer_info),
            // materials buffer in set 0
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(3) // binding within the set
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_BUFFER)
                .buffer_info(&descriptor_materials_buffer_info),
        ];

        unsafe {
            vulkan_app
                .device
                .update_descriptor_sets(&descriptor_writes, &[]);
        }

        self.base_attributes.descriptors = Descriptors {
            global_allocator_descriptor,
            draw_image_descriptors: descriptor_set,
            draw_image_descriptor_layout: descriptor_set_layout,
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
            String::from("/src/shaders/raytracing.spv"),
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
