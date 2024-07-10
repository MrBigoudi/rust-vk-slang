use ash::{
    vk::{
        CommandBufferAllocateInfo, CommandBufferLevel, CommandPoolCreateFlags,
        CommandPoolCreateInfo,
    },
    Device,
};

use crate::application::vk_app::{FrameData, QueueFamilyIndices, VulkanApp, FRAME_OVERLAP};

impl VulkanApp {
    pub fn init_commands(
        device: &Device,
        queue_families: &QueueFamilyIndices,
        frames: &mut [FrameData; FRAME_OVERLAP],
    ) {
        let command_pool_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_families.graphics_family.unwrap());

        frames.iter_mut().for_each(|frame| {
            frame.command_pool = unsafe {
                device
                    .create_command_pool(&command_pool_info, None)
                    .unwrap()
            };
            let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
                .command_pool(frame.command_pool)
                .command_buffer_count(1)
                .level(CommandBufferLevel::PRIMARY);
            frame.command_buffer = unsafe {
                device
                    .allocate_command_buffers(&command_buffer_allocate_info)
                    .unwrap()[0]
            };
        });
    }
}
