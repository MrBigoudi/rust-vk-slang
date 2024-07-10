use ash::{
    vk::{FenceCreateFlags, FenceCreateInfo, SemaphoreCreateInfo},
    Device,
};

use crate::application::vk_app::{FrameData, VulkanApp, FRAME_OVERLAP};

impl VulkanApp {
    pub fn init_sync_structuress(device: &Device, frames: &mut [FrameData; FRAME_OVERLAP]) {
        let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);
        let semaphore_create_info = SemaphoreCreateInfo::default();

        frames.iter_mut().for_each(|frame| {
            let render_fence = unsafe { device.create_fence(&fence_create_info, None).unwrap() };
            let render_semaphore = unsafe {
                device
                    .create_semaphore(&semaphore_create_info, None)
                    .unwrap()
            };
            let swapchain_semaphore = unsafe {
                device
                    .create_semaphore(&semaphore_create_info, None)
                    .unwrap()
            };
            frame.render_fence = render_fence;
            frame.render_semaphore = render_semaphore;
            frame.swapchain_semaphore = swapchain_semaphore;
        });
    }
}
