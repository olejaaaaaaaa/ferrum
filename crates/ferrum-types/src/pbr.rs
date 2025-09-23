use crate::{AttributeDescriptions, BindingDescriptions};
use ash::vk;
use std::mem::offset_of;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PBRVertex {
    pub pos: [f32; 4],
    pub normal: [f32; 4],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    pub tangent: [f32; 4],
}

impl PBRVertex {
    pub fn new(x: f32, y: f32, z: f32) -> PBRVertex {
        PBRVertex {
            pos: [x, y, z, 0.0],
            normal: [0.0, 0.0, 0.0, 0.0],
            uv: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            tangent: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl AttributeDescriptions for PBRVertex {
    fn attr_desc() -> Vec<ash::vk::VertexInputAttributeDescription> {
        vec![
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(PBRVertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(PBRVertex, normal) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 2,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: offset_of!(PBRVertex, uv) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 3,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(PBRVertex, color) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 4,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(PBRVertex, tangent) as u32,
            },
        ]
    }
}

impl BindingDescriptions for PBRVertex {
    fn bind_desc() -> Vec<ash::vk::VertexInputBindingDescription> {
        vec![vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<PBRVertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }]
    }
}
