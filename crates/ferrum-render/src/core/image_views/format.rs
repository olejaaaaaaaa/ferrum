use super::device::WithDevice;
use super::ImageViewsBuilder;

pub struct WithFormat<'n> {
    pub device: &'n ash::Device,
    pub format: ash::vk::Format
}

impl<'n> ImageViewsBuilder<WithDevice<'n>> {
    pub fn with_format(self, format: ash::vk::Format) -> ImageViewsBuilder<WithFormat<'n>> {
        ImageViewsBuilder {
            state: WithFormat {
                device: self.state.device,
                format,
            },
            image_sub_res_range: self.image_sub_res_range,
            components: self.components,
            view_type: self.view_type
        }
    }
}