use super::ImageViewsBuilder;

pub struct WithDevice<'n> {
    pub device: &'n ash::Device
}

impl<'n> ImageViewsBuilder<()> {
    pub fn with_device(self, device: &'n ash::Device) -> ImageViewsBuilder<WithDevice<'n>> {
        ImageViewsBuilder {
            state: WithDevice { device },
            image_sub_res_range: self.image_sub_res_range,
            components: self.components,
            view_type: self.view_type
        }
    }
}