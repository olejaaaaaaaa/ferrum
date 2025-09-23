use crate::frostbite_graph::resource_entry::Type;

pub struct ResourceNode {
    ty: Type,
    id: u32,
    version: u32,
}

impl ResourceNode {

    fn get_resource_id(&self) -> u32 {
        self.id
    }
    fn get_version(&self) -> u32 {
        self.version
    }
    
}