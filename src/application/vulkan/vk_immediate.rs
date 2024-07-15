use ash::vk::{
    CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferResetFlags,
    CommandBufferSubmitInfo, CommandBufferUsageFlags, CommandPoolCreateFlags,
    CommandPoolCreateInfo, FenceCreateFlags, FenceCreateInfo, SubmitInfo2,
};

use crate::application::vk_app::VulkanApp;

impl VulkanApp {
    pub fn init_immediate_submit_structures(&mut self) {
        let command_pool_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(self.queue_families.graphics_family.unwrap());

        unsafe {
            self.immediate_submit.command_pool = self
                .device
                .create_command_pool(&command_pool_info, None)
                .unwrap();
        }

        let allocate_info = CommandBufferAllocateInfo::default()
            .command_buffer_count(1)
            .command_pool(self.immediate_submit.command_pool);

        unsafe {
            self.immediate_submit.command_buffer = self
                .device
                .allocate_command_buffers(&allocate_info)
                .unwrap()[0]
        }

        let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);
        unsafe {
            self.immediate_submit.fence =
                self.device.create_fence(&fence_create_info, None).unwrap()
        }
    }

    pub fn clear_immediate_submit_structures(&mut self) {
        unsafe {
            self.device
                .destroy_command_pool(self.immediate_submit.command_pool, None);
            self.device.destroy_fence(self.immediate_submit.fence, None);
        }
    }

    pub fn immediate_submit(&self, fct: &dyn Fn(&Self, CommandBuffer)) {
        unsafe {
            self.device
                .reset_fences(&[self.immediate_submit.fence])
                .unwrap();
            self.device
                .reset_command_buffer(
                    self.immediate_submit.command_buffer,
                    CommandBufferResetFlags::empty(),
                )
                .unwrap();
        }

        let command_buffer_begin_info =
            CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.device
                .begin_command_buffer(
                    self.immediate_submit.command_buffer,
                    &command_buffer_begin_info,
                )
                .unwrap();
        }

        fct(self, self.immediate_submit.command_buffer);

        unsafe {
            self.device
                .end_command_buffer(self.immediate_submit.command_buffer)
                .unwrap();
        }

        let command_buffer_submit_info =
            [CommandBufferSubmitInfo::default()
                .command_buffer(self.immediate_submit.command_buffer)];

        let submit_info =
            [SubmitInfo2::default().command_buffer_infos(&command_buffer_submit_info)];

        // submit command buffer to the queue and execute it.
        //  _renderFence will now block until the graphic commands finish execution
        unsafe {
            self.device
                .queue_submit2(
                    self.queue_families.graphics_queue,
                    &submit_info,
                    self.immediate_submit.fence,
                )
                .unwrap();
            self.device
                .wait_for_fences(&[self.immediate_submit.fence], true, 9999999999)
                .unwrap();
        }
    }
}
