



#[derive(Clone, Copy)]
pub enum MemoryProperties {
    V1(ash::vk::PhysicalDeviceMemoryProperties),
    V2(ash::vk::PhysicalDeviceMemoryProperties2<'static>),
    None
}

impl Default for MemoryProperties {
    fn default() -> Self {
        MemoryProperties::None
    }
}

impl std::ops::Deref for MemoryProperties {
    type Target = ash::vk::PhysicalDeviceMemoryProperties;

    fn deref(&self) -> &Self::Target {
        match self {
            MemoryProperties::V2(x) => &x.memory_properties,
            MemoryProperties::V1(x) => x,
            MemoryProperties::None => panic!("Physical Memory Properties is not initialize")
        }
    }
}