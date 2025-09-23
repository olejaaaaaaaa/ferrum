pub struct FrameGraphResources {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrameGraphResource(i32);

impl FrameGraphResource {
    pub fn new(id: i32) -> Self {
        FrameGraphResource(id)
    }
}
