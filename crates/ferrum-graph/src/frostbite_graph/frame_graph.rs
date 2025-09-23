
// use crate::frostbite_graph::frame_graph_resource::FrameGraphResource;
// use crate::frostbite_graph::pass_entry::FrameGraphPass;
// use crate::frostbite_graph::resource_entry::{Resource, ResourceEntry};
// use crate::frostbite_graph::resource_node::ResourceNode;
// use crate::frostbite_graph::pass_node::PassNode;
// use crate::frostbite_graph::render_context::RenderContext;
// use crate::frostbite_graph::transient_resources::TransientResources;
// use vk_mem::Allocator;

// pub type Execute<T> = dyn Fn(&T, &FrameGraphPassResources, &RenderContext);
// pub type Setup<T> = dyn Fn(&FrameGraphBuilder, &mut T);

// #[derive(Default)]
// pub struct FrameGraph {
//     pub allocator: Option<Allocator>,
//     pub pass_nodes: Vec<PassNode>,
//     pub resource_nodes: Vec<ResourceNode>,
//     pub resource_registry: Vec<ResourceEntry>
// }

// pub struct FrameGraphBuilder<'f, 'p> {
//     frame_graph: &'f FrameGraph,
//     pass_node: &'p PassNode
// }

// impl<'f, 'p> FrameGraphBuilder<'f, 'p> {

//     fn create<T: Resource>(&self, name: &'static str, data: T::Desc) -> FrameGraphResource {
//         0
//     }

//     fn read(&self, id: FrameGraphResource, flags: u32) -> FrameGraphResource {
//         0
//     }

//     fn write(&self, id: FrameGraphResource) -> FrameGraphResource {
//         0
//     }
// }

// impl FrameGraph {

//     fn new() -> Self {
//         FrameGraph { ..Default::default() }
//     }

//     fn reserve(&mut self, num_passes: u32, num_resources: u32) {

//     }

//     fn add_callback_pass<T, D>(&mut self, name: &'static str, setup: &Setup<T>, exec: &Execute<T>) -> D {

//         if std::mem::size_of_val(&exec) > 1024 {
//             panic!( "Execute captures too much");
//         }

//         let pass = FrameGraphPass::new::<D>(&exec);
//     }

//     fn compile(&mut self) {

//     }

//     fn execute(&self, ctx: &RenderContext, allocator: &TransientResources) {

//         for i in &self.pass_nodes {

//         }
//     }
// }

// pub struct FrameGraphPassResources<'f, 'p> {
//     frame_graph: &'f FrameGraph,
//     pass_node: &'p PassNode
// }

// impl<'a, 'b> FrameGraphPassResources<'a, 'b> {

//     fn get<T>(&self, res: FrameGraphResource) -> T {
//         todo!()
//     }

// }

// #[cfg(test)]
// mod tests {

//     use winit::{event_loop::EventLoop, window::Window};
//     use crate::frostbite_graph::addition;
//     use crate::frostbite_graph::frame_graph_texture::{FrameGraphTexture, TextureDesc};
//     use crate::frostbite_graph::render_context::RenderContext;
//     use crate::frostbite_graph::transient_resources::TransientResources;
//     use crate::frostbite_graph::{
//         frame_graph::FrameGraph,
//         frame_graph_resource::FrameGraphResource
//     };

//     use addition::*;

//     #[test]
//     fn simple() {


//         let event_loop = EventLoop::new().unwrap();
//         let window = Window::new(&event_loop).unwrap();

//         let ctx = ferrum_render::RenderContext::default(window);

//         let alloc_info = vk_mem::AllocatorCreateInfo::new(ctx.device.instance.raw(), ctx.device.raw_device(), ctx.device.phys_dev.raw);
//         let alloc = unsafe { vk_mem::Allocator::new(alloc_info).unwrap() };

//         let transient_resources = TransientResources::new();

//         let ctx = RenderContext::new(&ctx, alloc);
//         let mut fg = FrameGraph::new();

//         struct PassData {
//             target: FrameGraphResource
//         }

//         fg.add_callback_pass::<PassData>("SimplePass",
//         &|builder, data: &mut PassData| {

//             data.target = builder.create::<FrameGraphTexture>("Foo", TextureDesc {
//                 width: 640,
//                 height: 480,
//                 format: ash::vk::Format::R8G8B8A8_SRGB
//             });
//             data.target = builder.write(data.target);
//         },
//         &|data: &PassData, resources, ctx| {
//             let texture = resources.get::<FrameGraphTexture>(data.target);
//             ctx.begin_rendering();
//             ctx.bind_texture(0, &texture.tex);
//             ctx.draw();
//             ctx.end_rendering();
//         }
//         );

//         fg.compile();
//         fg.execute(&ctx, &transient_resources);

//     }
// }