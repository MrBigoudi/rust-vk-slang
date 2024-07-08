use crate::application::vk_app::{FrameData, VulkanApp, FRAME_OVERLAP};

impl VulkanApp {
    pub fn get_current_frame(&self) -> &FrameData {
        &self.frames[self.frame_number % FRAME_OVERLAP]
    }

    pub fn init_frames() -> ([FrameData; FRAME_OVERLAP], usize) {
        let frames: [FrameData; FRAME_OVERLAP] = [Default::default(); FRAME_OVERLAP];
        (frames, 0)
    }
}
