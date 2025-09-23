use crate::render_graph::resource::FrameGraphResource;

#[derive(Default)]
pub struct FrameGraphNode {
    reads: Vec<FrameGraphResource>,
    writes: Vec<FrameGraphResource>
}

