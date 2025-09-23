use ash::vk;

#[derive(Debug, Clone)]
pub struct FrameGraphTextureDesc {
    pub width: u32,
    pub height: u32,
    pub format: vk::Format,
    pub usage: vk::ImageUsageFlags,
    pub samples: vk::SampleCountFlags,
}

