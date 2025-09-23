use ferrum_render::GPUBuffer;
use vk_mem::Alloc;

use crate::frostbite_graph::{render_context::RenderContext, resource_entry::Resource, transient_resources::TransientResources};



pub struct GPUBufferDesc {
    pub size: usize
}

pub struct FrameGraphBuffer {
    pub buffer: ash::vk::Buffer,
    pub desc: GPUBufferDesc
}


impl Resource for FrameGraphBuffer {

    type Desc = GPUBufferDesc;

    fn create(&mut self, descriptor: &Self::Desc, allocator: &TransientResources) {
        //allocator.create_buffer(&Buffer, create_info)
        todo!()
    }

    fn destroy(&mut self, descriptor: &Self::Desc, allocator: &TransientResources) {

    }

    fn pre_read(&self, descriptor: &Self::Desc, flags: u32, ctx: &RenderContext) {
        todo!()
    }

    fn pre_write(&self, descriptor: &Self::Desc, flags: u32, ctx: &RenderContext) {
        todo!()
    }

    fn to_string(descriptor: &Self::Desc) -> String {
        format!("Buffer with size: {}",
            descriptor.size
        )
    }
}