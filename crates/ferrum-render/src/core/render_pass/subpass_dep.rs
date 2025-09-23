
use ash::vk;

pub struct SubpassDependency {
    pub raw: ash::vk::SubpassDependency
}

impl SubpassDependency {
    pub fn default() -> Self {
        Self {
            raw: ash::vk::SubpassDependency {
                src_subpass: vk::SUBPASS_EXTERNAL,
                src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                ..Default::default()
            }
        }
    }
}
