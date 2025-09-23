use super::vertex::Vertex;

pub struct SimpleMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
}
