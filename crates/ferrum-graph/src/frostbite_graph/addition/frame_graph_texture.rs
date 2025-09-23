use std::any::Any;

use ferrum_render::Texture;
use crate::frostbite_graph::{render_context::RenderContext, resource_entry::Resource, transient_resources::TransientResources};

pub struct FrameGraphTexture {
    pub tex: Texture,
    pub tex_desc: TextureDesc
}

pub struct TextureDesc {
    pub width: u32,
    pub height: u32,
    pub format: ash::vk::Format
}

impl Resource for FrameGraphTexture {

    type Desc = TextureDesc;

    fn create(&mut self, descriptor: &Self::Desc, allocator: &TransientResources) {
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
        format!("Texture({}x{}) with Format: {:?}",
            descriptor.width,
            descriptor.height,
            descriptor.format
        )
    }
}