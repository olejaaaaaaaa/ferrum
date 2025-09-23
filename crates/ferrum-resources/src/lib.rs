#![feature(downcast_unchecked)]

use std::{any::{Any, TypeId}, path::Path};
use ash::vk::{Handle, Pipeline};

use crate::{
    image_manager::ImageManager, pipeline_manager::PipelineManager, sampler_manager::SamplerManager, shader_manager::ShaderManager
};

mod image_manager;
mod mesh_manager;
mod shader_manager;
mod pipeline_manager;
mod sampler_manager;
mod descriptor_manager;

pub use descriptor_manager::DescriptorManager;


pub struct ResourceManager {
    pipeline: PipelineManager,
    shader: ShaderManager,
    sampler: SamplerManager,
    image: ImageManager
}

impl ResourceManager {

    pub fn new() -> Self {
        Self {
            pipeline: PipelineManager::new(),
            shader: ShaderManager::new(),
            sampler: SamplerManager::new(),
            image: ImageManager::new()
        }
    }

    pub fn store(&mut self, name: &'static str, data: &dyn Any) {

    }

    pub fn load_blocking(&self, name: &'static str) {

    }
}
