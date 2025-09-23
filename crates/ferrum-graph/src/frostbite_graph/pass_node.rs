use crate::frostbite_graph::frame_graph_resource::FrameGraphResource;

struct FrameGraphPassConcept;

pub struct PassNode {
    pub exec: FrameGraphPassConcept,
    pub creates: Vec<FrameGraphResource>,
    pub reads: Vec<AccessDeclaration>,
    pub writes: Vec<AccessDeclaration>,
    pub has_side_effect: bool,
    pub name: &'static str,
    pub id: u32,
    pub ref_count: i32
}

impl PassNode {

    pub fn creates(id: FrameGraphResource) -> bool {
        true
    }

    pub fn reads(id: FrameGraphResource ) -> bool {
        true
    }

    pub fn writes(id: FrameGraphResource) -> bool {
        true
    }

    fn has_side_effect(&self) -> bool {
        self.has_side_effect
    }

    fn can_execute(&self) -> bool {
        return self.get_ref_count() > 0 || self.has_side_effect();
    }

    fn get_ref_count(&self) -> i32 {
        self.ref_count
    }

}

pub struct AccessDeclaration {
    id: FrameGraphResource,
    flags: u32
}


