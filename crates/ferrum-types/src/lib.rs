mod pbr;
mod simple_mesh;
mod vertex;

use ash::vk;
pub use pbr::PBRVertex;
pub use vertex::Vertex;
pub use simple_mesh::SimpleMesh;

pub trait AttributeDescriptions {
    fn attr_desc() -> Vec<vk::VertexInputAttributeDescription>;
}

pub trait BindingDescriptions {
    fn bind_desc() -> Vec<vk::VertexInputBindingDescription>;
}
