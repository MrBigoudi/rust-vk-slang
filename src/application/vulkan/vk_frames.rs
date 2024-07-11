use crate::application::vk_app::{FrameData, VulkanApp, FRAME_OVERLAP};

impl VulkanApp {
    pub fn get_current_frame(&self) -> &FrameData {
        &self.frames[self.frame_number % FRAME_OVERLAP]
    }

    pub fn init_frames() -> ([FrameData; FRAME_OVERLAP], usize) {
        let frames: [FrameData; FRAME_OVERLAP] = [Default::default(); FRAME_OVERLAP];
        (frames, 0)
    }

    pub fn clear_frames(&self){
        for &frame in self.frames.iter() {
            unsafe {
                self.device.destroy_semaphore(frame.swapchain_semaphore, None);
                self.device.destroy_semaphore(frame.render_semaphore, None);
                self.device.destroy_fence(frame.render_fence, None);
                self.device.destroy_command_pool(frame.command_pool, None);
            }
        }
    }
}
