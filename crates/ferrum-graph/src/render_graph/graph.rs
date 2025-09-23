// use ferrum_render::RenderContext;
// use crate::render_graph::{builder::Builder, node::FrameGraphNode, resource::FrameGraphResources, Concept};

// type Execute<T> = dyn FnOnce(&mut Builder, &T);
// type Setup<T> = dyn FnOnce(&T, &FrameGraphResources, &RenderContext);

// pub struct FrameGraph {
//     nodes: Vec<FrameGraphNode>,
// }

// impl FrameGraph {

//     pub fn new() -> Self {
//         Self { nodes: vec![] }
//     }

//     pub fn add_pass<T>(&mut self, name: &'static str, setup: &Setup<T>, execute: &Execute<T>) {

//     }

//     pub fn compile(&mut self) {

//     }

//     pub fn execute(&self, ctx: &RenderContext) {
//         for i in &self.nodes {

//         }
//     }
// }

// pub struct FrameGraphBuilder<'a, 'b> {
//     frame_graph: &'a FrameGraph,
//     pass_node: &'b PassNode
// }

// impl Concept for FrameGraphBuilder {

//     fn create(desc: Self::Desc) {

//     }

//     fn destroy(desc: Self::Desc) {

//     }

//     fn pre_read(desc: Self::Desc, flags: u32, ctx: &RenderContext) {

//     }

//     fn pre_write(desc: Self::Desc, flags: u32, ctx: &RenderContext) {

//     }

//     fn to_string(desc: Self::Desc) -> String {

//     }

// }

