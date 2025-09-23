
pub struct GraphNode {
    name: &'static str,
    id: u32,
    ref_count: i32
}

impl GraphNode {

    pub fn new(name: &'static str, id: u32) -> Self {
        Self { name: name, id: id, ref_count: 0 }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_ref_count(&self) -> i32 {
        self.ref_count
    }
}
