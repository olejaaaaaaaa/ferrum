
use super::color_space::WithColorSpace;
use super::SwapchainBuilder;
use ash::vk;

pub struct WithPresentMode<'n> {
    pub device: &'n ash::Device,
    pub surface: &'n ash::vk::SurfaceKHR,
    pub instance: &'n ash::Instance,
    pub format: vk::Format,
    pub color_space: vk::ColorSpaceKHR,
    pub extent: ash::vk::Extent2D,
    pub present_mode: vk::PresentModeKHR,
}

impl<'n> SwapchainBuilder<WithColorSpace<'n>> {
    pub fn with_present_mode(self, mode: vk::PresentModeKHR) -> SwapchainBuilder<WithPresentMode<'n>> {
        SwapchainBuilder {

            state: WithPresentMode {
                device: self.state.device,
                surface: self.state.surface,
                format: self.state.format,
                color_space: self.state.color_space,
                extent: self.state.extent,
                instance: self.state.instance,
                present_mode: mode
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