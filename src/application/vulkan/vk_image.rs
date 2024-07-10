use ash::{
    vk::{
        self, AccessFlags2, CommandBuffer, DependencyInfo, Image, ImageAspectFlags, ImageLayout,
        ImageMemoryBarrier2, ImageSubresourceRange, PipelineStageFlags2,
    },
    Device,
};

use crate::application::vk_app::VulkanApp;

impl VulkanApp {
    pub fn transition_image(
        device: &Device,
        command_buffer: &CommandBuffer,
        image: &Image,
        current_layout: &ImageLayout,
        new_layout: &ImageLayout,
    ) {
        let aspect_mask = if *new_layout == ImageLayout::DEPTH_ATTACHMENT_OPTIMAL {
            ImageAspectFlags::DEPTH
        } else {
            ImageAspectFlags::COLOR
        };

        let image_barrier_subresource_range = ImageSubresourceRange::default()
            .aspect_mask(aspect_mask)
            .base_mip_level(0)
            .level_count(vk::REMAINING_MIP_LEVELS)
            .base_array_layer(0)
            .layer_count(vk::REMAINING_ARRAY_LAYERS);

        let image_barrier = ImageMemoryBarrier2::default()
            .src_stage_mask(PipelineStageFlags2::ALL_COMMANDS)
            .src_access_mask(AccessFlags2::MEMORY_WRITE)
            .dst_stage_mask(PipelineStageFlags2::ALL_COMMANDS)
            .dst_access_mask(AccessFlags2::MEMORY_WRITE | AccessFlags2::MEMORY_READ)
            .old_layout(*current_layout)
            .new_layout(*new_layout)
            .subresource_range(image_barrier_subresource_range)
            .image(*image);

        let binding = [image_barrier];
        let dependency_info = DependencyInfo::default().image_memory_barriers(&binding);

        unsafe { device.cmd_pipeline_barrier2(*command_buffer, &dependency_info) };
    }
}
