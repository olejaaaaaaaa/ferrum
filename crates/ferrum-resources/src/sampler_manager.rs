use std::collections::HashMap;
use ash::vk::Sampler;

pub struct SamplerManager {
    pub sampler: HashMap<&'static str, Sampler>
}

impl SamplerManager {
    pub fn new() -> Self {
        Self {
            sampler: HashMap::new()
        }
    }
}