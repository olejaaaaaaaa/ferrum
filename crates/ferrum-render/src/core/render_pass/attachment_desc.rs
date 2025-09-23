use ash::vk;

pub struct AttachmentDescription {
    pub raw: ash::vk::AttachmentDescription
}

impl AttachmentDescription {
    pub fn default(format: vk::Format) -> Self {
        Self {
            raw:
            ash::vk::AttachmentDescription {
                format: format,
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            },
        }
    }
}
