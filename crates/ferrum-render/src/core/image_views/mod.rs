pub mod image_views;
pub use image_views::*;

use crate::VulkanResult;

mod format;
mod device;

pub struct ImageViews {
    pub raw: Vec<ash::vk::ImageView>,
    #[cfg(debug_assertions)]
    destroyed: bool
}

pub struct ImageViewsBuilder<S = ()> {
    image_sub_res_range: ash::vk::ImageSubresourceRange,
    components: ash::vk::ComponentMapping,
    view_type: ash::vk::ImageViewType,
    state: S
}

impl ImageViewsBuilder<()> {
    pub fn new() -> ImageViewsBuilder<()> {
        ImageViewsBuilder {
            image_sub_res_range: ash::vk::ImageSubresourceRange {
                aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            components: ash::vk::ComponentMapping {
                r: ash::vk::ComponentSwizzle::R,
                g: ash::vk::ComponentSwizzle::G,
                b: ash::vk::ComponentSwizzle::B,
                a: ash::vk::ComponentSwizzle::A,
            },
            view_type: ash::vk::ImageViewType::TYPE_2D,
            state: (),
        }
    }
}

impl<'n> ImageViewsBuilder<WithImageViews<'n>> {

    pub fn build(self) -> VulkanResult<ImageViews> {

        let mut image_views = vec![];

        for i in self.state.image_views {

            let create_view_info = ash::vk::ImageViewCreateInfo::default()
                .view_type(self.view_type)
                .format(self.state.format)
                .components(self.components)
                .subresource_range(self.image_sub_res_range)
                .image(*i);

            let image_view = unsafe {
                 self.state.device.create_image_view(&create_view_info, None).unwrap()
            };

            image_views.push(image_view)
        }

        log::debug!("Create ImageViews: {}", image_views.len());

        Ok(ImageViews {
            raw: image_views,
            #[cfg(debug_assertions)]
            destroyed: false
        })
    }
}


#[cfg(debug_assertions)]
impl Drop for ImageViews {
    fn drop(&mut self) {
        if !self.destroyed {
            log::warn!("ImageViews dont't destroyed before Drop")
        }
    }
}