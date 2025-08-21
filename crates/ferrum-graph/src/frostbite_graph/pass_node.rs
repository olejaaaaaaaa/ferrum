use crate::frostbite_graph::frame_graph_resource::FrameGraphResource;

struct FrameGraphPassConcept;

pub struct PassNode {
    m_exec: FrameGraphPassConcept,
    m_creates: Vec<FrameGraphResource>,
    m_reads: Vec<AccessDeclaration>,
    m_writes: Vec<AccessDeclaration>
}

pub struct AccessDeclaration {
    id: FrameGraphResource,
    flags: u32
}


