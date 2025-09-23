use std::{collections::HashMap, rc::Rc};
use ferrum_render::{RenderContext, Texture};
use crate::frostbite_graph::{frame_graph_texture::FrameGraphTexture, resource_entry::Resource};

struct ResourceEntry<T> {
    resource: T,
    life: f32
}

type ResourcePool<T> = ResourceEntry<T>;

pub struct TransientResources {
    ctx: Rc<RenderContext>,
    textures: Vec<Rc<Texture>>,
    buffer: Vec<Rc<bool>>,
    texture_pools: HashMap<usize, ResourcePool<Texture>>,
    buffer_pools: HashMap<usize, ResourcePool<bool>>
}


impl TransientResources {

    pub fn new() -> Self {
       Self { ctx: todo!(), textures: todo!(), buffer: todo!(), texture_pools: todo!(), buffer_pools: todo!() }
    }

    fn update(dt: f32) {

    }

    fn acquireTexture(desc: <FrameGraphTexture as Resource>::Desc) -> Texture {
        todo!()
    }

    fn releaseTexture(desc: <FrameGraphTexture as Resource>::Desc, texture: Texture) {
        todo!()
    }

}
