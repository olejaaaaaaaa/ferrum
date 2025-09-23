
use super::present_mode::WithPresentMode;
use super::SwapchainBuilder;
use ash::vk;

pub struct WithImageCount<'n> {
    pub device: &'n ash::Device,
    pub surface: &'n ash::vk::SurfaceKHR,
    pub instance: &'n ash::Instance,
    pub format: vk::Format,
    pub color_space: vk::ColorSpaceKHR,
    pub extent: ash::vk::Extent2D,
    pub present_mode: vk::PresentModeKHR,
    pub image_count: u32,
}

impl<'n> SwapchainBuilder<WithPresentMode<'n>> {
    pub fn with_min_count_image(self, image_count: u32) -> SwapchainBuilder<WithImageCount<'n>> {
        SwapchainBuilder {

            state: WithImageCount {
                device: self.state.device,
                instance: self.state.instance,
                format: self.state.format,
                color_space: self.state.color_space,
                extent: self.state.extent,
                present_mode: self.state.present_mode,
                surface: self.state.surface,
                image_count
            },

            transform: self.transform,
            clipped: self.clipped,
            image_array_layers: self.image_array_layers,
            composite_alpha: self.composite_alpha,
            image_sharing_mode: self.image_sharing_mode,
            image_usage: self.image_usage
        }
    }
}