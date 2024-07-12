use ash::{
    vk::{
        self, AccessFlags2, BlitImageInfo2, CommandBuffer, DependencyInfo, Extent2D, Extent3D,
        Filter, Format, Image, ImageAspectFlags, ImageBlit2, ImageCreateInfo, ImageLayout,
        ImageMemoryBarrier2, ImageSubresourceLayers, ImageSubresourceRange, ImageTiling, ImageType,
        ImageUsageFlags, ImageViewCreateInfo, ImageViewType, MemoryPropertyFlags, Offset3D,
        PipelineStageFlags2, SampleCountFlags,
    },
    Device,
};
use vk_mem::{Alloc, AllocationCreateInfo, Allocator};

use crate::application::vk_app::{AllocatedImage, AppParameters, VulkanApp};

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

    pub fn copy_image_to_image(
        command_buffer: &CommandBuffer,
        device: &Device,
        image_src: &Image,
        image_dst: &Image,
        size_src: &Extent2D,
        size_dst: &Extent2D,
    ) {
        let src_offsets: [Offset3D; 2] = [
            Offset3D::default(),
            Offset3D::default()
                .x(size_src.width as i32)
                .y(size_src.height as i32)
                .z(1),
        ];
        let dst_offsets: [Offset3D; 2] = [
            Offset3D::default(),
            Offset3D::default()
                .x(size_dst.width as i32)
                .y(size_dst.height as i32)
                .z(1),
        ];

        let src_subresource = ImageSubresourceLayers::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_array_layer(0)
            .layer_count(1)
            .mip_level(0);

        let dst_subresource = ImageSubresourceLayers::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_array_layer(0)
            .layer_count(1)
            .mip_level(0);

        let blit_regions = [ImageBlit2::default()
            .src_offsets(src_offsets)
            .dst_offsets(dst_offsets)
            .src_subresource(src_subresource)
            .dst_subresource(dst_subresource)];

        let blit_image_info = BlitImageInfo2::default()
            .dst_image(*image_dst)
            .dst_image_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_image(*image_src)
            .src_image_layout(ImageLayout::TRANSFER_SRC_OPTIMAL)
            .filter(Filter::LINEAR)
            .regions(&blit_regions);

        unsafe {
            device.cmd_blit_image2(*command_buffer, &blit_image_info);
        }
    }

    pub fn init_images(
        app_params: &AppParameters,
        device: &Device,
        allocator: &Allocator,
    ) -> AllocatedImage {
        //draw image size will match the window
        let draw_image_extent = Extent3D::default()
            .width(app_params.window_width as u32)
            .height(app_params.window_height as u32)
            .depth(1);

        // hardcoding the draw format to 32 bit float
        let draw_image_format = Format::R16G16B16A16_SFLOAT;
        let draw_image_usages = ImageUsageFlags::default()
            | ImageUsageFlags::TRANSFER_SRC
            | ImageUsageFlags::TRANSFER_DST
            | ImageUsageFlags::STORAGE
            | ImageUsageFlags::COLOR_ATTACHMENT;

        let image_info = ImageCreateInfo::default()
            .image_type(ImageType::TYPE_2D)
            .format(draw_image_format)
            .extent(draw_image_extent)
            .usage(draw_image_usages)
            .mip_levels(1)
            .array_layers(1)
            .samples(SampleCountFlags::TYPE_1)
            .tiling(ImageTiling::OPTIMAL);

        // for the draw image, we want to allocate it from gpu local memory
        let image_allocation_info = AllocationCreateInfo {
            required_flags: MemoryPropertyFlags::DEVICE_LOCAL,
            ..Default::default()
        };

        //allocate and create the image
        let (image, allocation) = unsafe {
            allocator
                .create_image(&image_info, &image_allocation_info)
                .unwrap()
        };

        // build a image-view for the draw image to use for rendering
        let image_subresource_range = ImageSubresourceRange::default()
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .aspect_mask(ImageAspectFlags::COLOR);

        let image_view_info = ImageViewCreateInfo::default()
            .view_type(ImageViewType::TYPE_2D)
            .image(image)
            .format(draw_image_format)
            .subresource_range(image_subresource_range);

        let image_view = unsafe { device.create_image_view(&image_view_info, None).unwrap() };

        AllocatedImage {
            image,
            image_view,
            image_extent: draw_image_extent,
            image_format: draw_image_format,
            allocation,
        }
    }

    pub fn clear_images(&mut self) {
        unsafe {
            for &image_view in self.swapchain_image_views.iter() {
                self.device.destroy_image_view(image_view, None);
            }

            self.device
                .destroy_image_view(self.draw_image.image_view, None);

            let allocator = self.allocator.allocator.lock().unwrap();
            allocator.destroy_image(self.draw_image.image, &mut self.draw_image.allocation);
        }
    }
}
