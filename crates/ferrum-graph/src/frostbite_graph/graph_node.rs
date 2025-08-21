

pub struct GraphNode {
    m_name: &'static str,
    m_id: u32,
    m_refCount: i32
}

impl GraphNode {

    pub fn new(name: &'static str, id: u32) -> Self {
        Self { m_name: name, m_id: id, m_refCount: 0 }
    }

    pub fn getId(&self) -> u32 { return self.m_id; }
    pub fn getName(&self) -> &'static str { return self.m_name; }
    pub fn getRefCount(&self) -> i32 { return self.m_refCount; }
}
