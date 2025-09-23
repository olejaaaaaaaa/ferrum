use ash::vk::{self, Extent3D};
use ash::vk::Format;

#[derive(Clone, Copy)]
pub struct Image {
    pub raw: vk::Image
}

impl Image {

    pub fn new(device: &ash::Device, extent: Extent3D, format: Format) -> Self {

        let create_info = vk::ImageCreateInfo {
            image_type: vk::ImageType::TYPE_2D,
            format,
            extent,
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };

        let image = unsafe { device.create_image(&create_info, None).unwrap() };

        Self {
            raw: image
        }
    }
}