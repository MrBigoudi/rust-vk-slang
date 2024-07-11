use crate::application::vk_app::VulkanApp;

use super::{pipeline::ComputePipeline, pipeline_gradient::PipelineGradient};

impl VulkanApp {
    pub fn init_pipelines(&mut self) {
        let mut gradient_pipeline: PipelineGradient = Default::default();
        gradient_pipeline.init(self);

        self.pipelines = vec![Box::new(gradient_pipeline)]
    }

    pub fn clear_pipelines(&mut self) {
        // Clear each pipeline
        for pipeline in self.pipelines.iter_mut() {
            pipeline.clear(&self.device);
        }
    }
}
