


use super::RenderPassBuilder;

pub struct WithDevice<'n> {
    pub device: &'n ash::Device
}

impl<'n> RenderPassBuilder<()> {
    pub fn with_device(self, device: &'n ash::Device) -> RenderPassBuilder<WithDevice<'n>> {
        RenderPassBuilder {
            state: WithDevice { device },
            attachments: self.attachments,
            dependencies: self.dependencies,
            subpass: self.subpass,
            allocation_callback: self.allocation_callback
        }
    }
}