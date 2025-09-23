use ash::vk;
use ferrum_render::{GpuBuffer, RenderContext};
use ferrum_types::PBRVertex;
use glam::{Vec2, Vec4};

pub struct Primitive {
    pub index_buffer: GpuBuffer,
    pub vertex_buffer: GpuBuffer,
    pub indices: Vec<u32>,
    pub vertices: Vec<PBRVertex>,
}



impl Primitive {
    pub fn new(ctx: &RenderContext, indices: Vec<u32>, vertices: Vec<PBRVertex>) -> Primitive {

        let index_buffer = ctx.create_dynamic_buffer(vk::BufferUsageFlags::INDEX_BUFFER, &indices).unwrap();
        let vertex_buffer = ctx.create_dynamic_buffer(vk::BufferUsageFlags::VERTEX_BUFFER, &vertices).unwrap();

        Primitive {
            index_buffer,
            vertex_buffer,
            indices,
            vertices,
        }
    }

}