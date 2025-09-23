// Создание ImageView для depth image

use ash::vk;

use crate::{VulkanError, VulkanResult};

pub struct ImageView {
    pub raw: vk::ImageView,
}

impl ImageView {
    pub fn new_depth(
        device: &ash::Device,
        image: vk::Image,
        format: vk::Format,
    ) -> VulkanResult<Self> {
        
        let create_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::DEPTH,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        let image_view = unsafe {
            device.create_image_view(&create_info, None)
                .map_err(|_| VulkanError::Unknown)?
        };

        Ok(Self { raw: image_view })
    }
}