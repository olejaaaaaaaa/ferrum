use std::any::Any;
use crate::frostbite_graph::frame_graph_resource::FrameGraphResource;
use crate::frostbite_graph::resource_entry::{Resource, ResourceEntry};
use crate::frostbite_graph::resource_node::ResourceNode;
use crate::frostbite_graph::pass_node::PassNode;
use crate::frostbite_graph::render_context::RenderContext;

pub type Execute<T> = dyn Fn(&T, &FrameGraphPassResources, &dyn Any);
pub type Setup<T> = dyn Fn(&FrameGraphBuilder, &mut T);

#[derive(Default)]
struct FrameGraph {
    m_passNodes: Vec<PassNode>,
    m_resourceNodes: Vec<ResourceNode>,
    m_resourceRegistry: Vec<ResourceEntry>
}

struct FrameGraphBuilder<'f, 'p> {
    m_frameGraph: &'f FrameGraph,
    m_passNode: &'p PassNode
}

impl<'f, 'p> FrameGraphBuilder<'f, 'p> {

    fn create<T: Resource>(&self, name: &'static str, data: T::Desc) -> FrameGraphResource {
        0
    }

    fn read(&self, id: FrameGraphResource, flags: u32) -> FrameGraphResource {
        0
    }

    fn write(&self, id: FrameGraphResource) -> FrameGraphResource {
        0
    }
}

impl FrameGraph {

    fn new() -> Self {
        FrameGraph { ..Default::default() }
    }

    fn reserve(&mut self, numPasses: u32, numResources: u32) {

    }

    fn addCallbackPass<T>(&mut self, name: &'static str, setup: &Setup<T>, exec: &Execute<T>) {

    }

    fn compile(&mut self) {

    }

    fn execute(&self, ctx: &dyn Any, allocator: *const ()) {

    }
}

pub struct FrameGraphPassResources<'f, 'p> {
    m_frameGraph: &'f FrameGraph,
    m_passNode: &'p PassNode
}

impl<'a, 'b> FrameGraphPassResources<'a, 'b> {

    fn get<T>(&self, res: FrameGraphResource) -> T {
        todo!()
    }

}

#[cfg(test)]
mod tests {

    use winit::{event_loop::EventLoop, window::Window};
    use crate::frostbite_graph::addition;
    use crate::frostbite_graph::frame_graph_texture::{FrameGraphTexture, TextureDesc};
    use crate::frostbite_graph::render_context::RenderContext;
    use crate::frostbite_graph::{
        frame_graph::FrameGraph,
        frame_graph_resource::FrameGraphResource
    };


    use addition::*;

    #[test]
    fn simple() {

        let event_loop = EventLoop::new().unwrap();
        let window = Window::new(&event_loop).unwrap();

        let ctx = RenderContext::new( );
        let mut fg = FrameGraph::new();

        struct PassData {
            target: FrameGraphResource
        }

        fg.addCallbackPass("SimplePass",
        &|builder, data: &mut PassData| {
            data.target = builder.create::<FrameGraphTexture>("Foo", TextureDesc {
                width: 640,
                height: 480,
                format: ash::vk::Format::R8G8B8A8_SRGB
            });
            data.target = builder.write(data.target);
        },
        &|data: &PassData, resources, ctx| {
            let ctx = ctx.downcast_ref::<RenderContext>().unwrap();
            let texture = resources.get::<FrameGraphTexture>(data.target);
            ctx.beginRendering();
            ctx.bindTexture();
            ctx.draw();
            ctx.endRendering();
        }

        );

        fg.compile();
        fg.execute(&ctx, &() as *const ());

    }
}