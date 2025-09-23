

use super::format::WithFormat;
use super::ImageViewsBuilder;

pub struct WithImageViews<'n> {
    pub device: &'n ash::Device,
    pub format: ash::vk::Format,
    pub image_views: &'n Vec<ash::vk::Image>
}

impl<'n> ImageViewsBuilder<WithFormat<'n>> {
    pub fn with_image_views(self, image_views: &'n Vec<ash::vk::Image>) -> ImageViewsBuilder<WithImageViews<'n>> {
        ImageViewsBuilder {
            state: WithImageViews {
                device: self.state.device,
                format: self.state.format,
                image_views
            },
            image_sub_res_range: self.image_sub_res_range,
            components: self.components,
            view_type: self.view_type
        }
    }
}
