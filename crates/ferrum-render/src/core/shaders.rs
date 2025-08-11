#![allow(warnings)]

use std::{
    fs::File, io::Read
};
use ash::vk::{
    AllocationCallbacks, ShaderModule, ShaderModuleCreateInfo
};

pub struct ShaderProgram {
    pub vertex_shader: ShaderModule,
    pub fragment_shader: ShaderModule
}

#[derive(Default)]
pub struct ShaderProgramBuilder<'n> {
    pub device: Option<&'n ash::Device>,
    pub vertex_shader_source: Option<Vec<u32>>,
    pub fragment_shader_source: Option<Vec<u32>>,
    pub allocation_callbacks: Option<&'n AllocationCallbacks<'n>>
}

impl<'n> ShaderProgramBuilder<'n> {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn with_allocation_callbacks(mut self, callback: &'n AllocationCallbacks<'n>) -> Self {
        self.allocation_callbacks = Some(callback);
        self
    }

    pub fn with_device(mut self, device: &'n ash::Device) -> Self {
        self.device = Some(device);
        self
    }

    pub fn with_vertex_shader(mut self, bytes: Vec<u32>) -> Self {
        self.vertex_shader_source = Some(bytes);
        self
    }

    pub fn with_fragment_shader(mut self, bytes: Vec<u32>) -> Self {
        self.fragment_shader_source = Some(bytes);
        self
    }

    pub fn build(self) -> ShaderProgram {

        let callback = self.allocation_callbacks;
        let binding = self.fragment_shader_source.unwrap();
        let create_info = ShaderModuleCreateInfo::default()
            .code(&binding);

        let fs = unsafe { self.device.unwrap().create_shader_module(&create_info, callback) };

        //---------------------------------------------------

        let binding = self.vertex_shader_source.unwrap();
        let create_info = ShaderModuleCreateInfo::default()
            .code(&binding);

        let vs = unsafe { self.device.unwrap().create_shader_module(&create_info, callback) };

        ShaderProgram { vertex_shader: vs.unwrap(), fragment_shader: fs.unwrap() }
    }
}