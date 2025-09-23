use std::collections::HashMap;

pub struct PipelineManager {
    pub pipeline: HashMap<&'static str, ash::vk::Pipeline>,
    pub cache: HashMap<&'static str, ash::vk::PipelineCache>,
    pub layout: HashMap<&'static str, ash::vk::PipelineLayout>
}

impl PipelineManager {

    pub fn new() -> Self {
        Self {
            pipeline: HashMap::new(),
            cache: HashMap::new(),
            layout: HashMap::new()
        }
    }

    pub fn store_pipeline(&mut self, name: &'static str, pipeline: ash::vk::Pipeline) {
        self.pipeline.insert(name, pipeline);
    }

    pub fn store_pipeline_layout(&mut self, name: &'static str, layout: ash::vk::PipelineLayout) {
        self.layout.insert(name, layout);
    }

    pub fn get_pipeline(&self, path: &'static str) -> Option<&ash::vk::Pipeline> {
        self.pipeline.get(path)
    }

    pub fn get_pipeline_layout(&self, path: &'static str) -> Option<&ash::vk::PipelineLayout> {
        self.layout.get(path)
    }

}


