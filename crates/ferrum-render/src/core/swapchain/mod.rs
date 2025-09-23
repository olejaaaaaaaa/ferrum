pub mod device;
pub mod surface;
pub mod swapchain;
pub mod color_space;
pub mod present_mode;
pub mod extent;
// pub mod transform;
pub mod format;
pub mod image_count;
// pub mod clipped;
// pub mod image_array_layers;
pub mod instance;

use ash::vk::{
    Image,
    SurfaceTransformFlagsKHR,
    SwapchainKHR
};

use image_count::WithImageCount;

use crate::{VulkanError, VulkanResult};

pub struct Swapchain {
    pub raw: SwapchainKHR,
    pub swapchain_loader: ash::khr::swapchain::Device,
    #[cfg(debug_assertions)]
    destroyed: bool
}

impl Swapchain {
    /// Get Current swapchain images
    pub fn get_swapchain_images(&self) -> VulkanResult<Vec<Image>> {
        let images = unsafe {
            self.swapchain_loader.get_swapchain_images(self.raw)
                .map_err(|_| VulkanError::Unknown)?
        };
        Ok(images)
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
        unsafe {
            self.swapchain_loader.destroy_swapchain(self.raw, None);
        }
    }
}

pub struct SwapchainBuilder<S = ()> {
    pub transform: ash::vk::SurfaceTransformFlagsKHR,
    pub clipped: bool,
    pub image_array_layers: u32,
    pub composite_alpha: ash::vk::CompositeAlphaFlagsKHR,
    pub image_sharing_mode: ash::vk::SharingMode,
    pub image_usage: ash::vk::ImageUsageFlags,
    pub state: S
}

impl SwapchainBuilder<()> {
    pub fn new() -> Self {
        Self {
            state: (),
            transform: ash::vk::SurfaceTransformFlagsKHR::IDENTITY,
            clipped: true,
            image_array_layers: 1,
            composite_alpha: ash::vk::CompositeAlphaFlagsKHR::OPAQUE,
            image_sharing_mode: ash::vk::SharingMode::EXCLUSIVE,
            image_usage: ash::vk::ImageUsageFlags::COLOR_ATTACHMENT
        }
    }
}

impl<'n> SwapchainBuilder<WithImageCount<'n>> {

    pub fn build(self) -> VulkanResult<Swapchain> {

        let swapchain_info = ash::vk::SwapchainCreateInfoKHR::default()
            .surface(*self.state.surface)
            .min_image_count(self.state.image_count)
            .image_color_space(self.state.color_space)
            .image_format(self.state.format)
            .image_extent(self.state.extent)
            .image_usage(self.image_usage)
            .image_sharing_mode(self.image_sharing_mode)
            .pre_transform(self.transform)
            .composite_alpha(self.composite_alpha)
            .present_mode(self.state.present_mode)
            .clipped(self.clipped)
            .image_array_layers(self.image_array_layers);

        let swapchain_loader = ash::khr::swapchain::Device::new(self.state.instance, self.state.device);
        let swapchain = unsafe {
            swapchain_loader.create_swapchain(&swapchain_info, None)
                .map_err(|e| {
                    VulkanError::Unknown
                }
            )?
        };

        Ok(Swapchain {
            raw: swapchain,
            swapchain_loader,
            #[cfg(debug_assertions)]
            destroyed: false
        })
    }
}


#[cfg(debug_assertions)]
impl Drop for Swapchain {
    fn drop(&mut self) {
        if !self.destroyed {
            log::warn!("Swapchain was not destroyed before being dropped!");
        }
    }
}