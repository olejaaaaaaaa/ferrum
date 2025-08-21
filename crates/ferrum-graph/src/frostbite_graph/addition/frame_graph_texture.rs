use std::any::Any;

use ferrum_render::Texture;
use crate::frostbite_graph::resource_entry::Resource;

pub struct FrameGraphTexture {
    pub texture: Texture
}

pub struct TextureDesc {
    pub width: u32,
    pub height: u32,
    pub format: ash::vk::Format
}

impl Resource for FrameGraphTexture {

    type Desc = TextureDesc;

    fn create(&mut self, descriptor: &Self::Desc, allocator: &dyn Any) {
        todo!()
    }

    fn destroy(&mut self, descriptor: &Self::Desc, allocator: &dyn Any) {

    }

    fn pre_read(&self, descriptor: &Self::Desc, flags: u32, ctx: &dyn std::any::Any) {
        todo!()
    }

    fn pre_write(&self, descriptor: &Self::Desc, flags: u32, ctx: &dyn std::any::Any) {
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