use std::collections::HashMap;


pub struct ShaderManager {
    pub vertex_shader: HashMap<&'static str, ash::vk::ShaderModule>,
    pub fragment_shader: HashMap<&'static str, ash::vk::ShaderModule>
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            vertex_shader: HashMap::new(),
            fragment_shader: HashMap::new()
        }
    }
}