use crate::frostbite_graph::resource_entry::Type;

pub struct ResourceNode {
    m_type: Type,
    m_id: u32,
    m_version: u32,
}

impl ResourceNode {
    fn getResourceId(&self) -> u32 { self.m_id }
    fn getVersion(&self) -> u32 { self.m_version }
}