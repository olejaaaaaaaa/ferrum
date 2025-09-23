#![allow(warnings)]
use crate::VulkanError;
use crate::VulkanResult;

use std::error::Error;
use std::path::Path;
use std::sync::Arc;
use std::{
    fs::File, io::Read
};

use ash::vk::{
    AllocationCallbacks, ShaderModule, ShaderModuleCreateInfo
};

pub struct ShaderProgram {
    pub vertex_shader: ShaderModule,
    pub fragment_shader: ShaderModule,
    device: Arc<ash::Device>,
    allocation_callbacks: Option<&'static AllocationCallbacks<'static>>
}

pub struct ShaderProgramBuilder<S = ()> {
    pub state: S,
    pub allocation_callbacks: Option<&'static AllocationCallbacks<'static>>
}


impl ShaderProgramBuilder<()> {
    pub fn new() -> Self {
        ShaderProgramBuilder {
            state: (),
            allocation_callbacks: None
        }
    }
}

pub struct WithDevice {
    pub device: Arc<ash::Device>
}

impl<'n> ShaderProgramBuilder<()> {
    pub fn with_device(self, device: Arc<ash::Device>) -> ShaderProgramBuilder<WithDevice> {
        ShaderProgramBuilder {
            state: WithDevice { device },
            allocation_callbacks: self.allocation_callbacks
        }
    }
}

pub struct WithVertexShader<T: AsRef<Path>> {
    pub device: Arc<ash::Device>,
    pub vertex: T
}

impl ShaderProgramBuilder<WithDevice> {
    pub fn with_vertex_shader<T: AsRef<Path>>(self, path: T) -> ShaderProgramBuilder<WithVertexShader<T>> {
        ShaderProgramBuilder {
            state: WithVertexShader { device: self.state.device, vertex: path },
            allocation_callbacks: self.allocation_callbacks
        }
    }
}

pub struct WithFragmentShader<T: AsRef<Path>> {
    pub device: Arc<ash::Device>,
    pub vertex: T,
    pub fragment: T
}

impl<T: AsRef<Path>> ShaderProgramBuilder<WithVertexShader<T>> {
    pub fn with_fragment_shader(self, path: T) -> ShaderProgramBuilder<WithFragmentShader<T>> {
        ShaderProgramBuilder {
            state: WithFragmentShader {
                device: self.state.device,
                vertex: self.state.vertex,
                fragment: path
            },
            allocation_callbacks: self.allocation_callbacks
        }
    }
}

pub fn read_shader_from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut cursor = std::io::Cursor::new(bytes);
    Ok(ash::util::read_spv(&mut cursor)?)
}

pub fn load_spv(path: &Path) -> Vec<u32> {

    let mut file = File::open(path).unwrap();
    let mut text = Vec::new();
    file.read_to_end(&mut text).unwrap();

    assert_eq!(text.len() % 4, 0);
    assert_eq!(0x07230203, u32::from_le_bytes([text[0], text[1], text[2], text[3]]));

    read_shader_from_bytes(&text).unwrap()
}


impl<T: AsRef<Path>> ShaderProgramBuilder<WithFragmentShader<T>> {

    pub fn build(self) -> VulkanResult<ShaderProgram> {

        let spv =  load_spv(self.state.fragment.as_ref());

        let binding = spv;
        let create_info = ShaderModuleCreateInfo::default()
            .code(&binding);

        let fs = unsafe {
            self.state.device.create_shader_module(&create_info, self.allocation_callbacks).map_err(|e| {
                VulkanError::Unknown
            })?
        };

        //---------------------------------------------------

        let spv =  load_spv(self.state.vertex.as_ref());
        let binding = spv;
        let create_info = ShaderModuleCreateInfo::default()
            .code(&binding);

        let vs = unsafe {
            self.state.device.create_shader_module(&create_info, self.allocation_callbacks).map_err(|e| {
                VulkanError::Unknown
            })?
        };

        Ok(ShaderProgram {
            vertex_shader: vs,
            fragment_shader: fs,
            device: self.state.device,
            allocation_callbacks: self.allocation_callbacks
        })
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_shader_module(self.vertex_shader, self.allocation_callbacks);
            self.device.destroy_shader_module(self.fragment_shader, self.allocation_callbacks);
        }
    }
}


