
use ash::vk;
use super::SwapchainBuilder;
use super::format::WithFormat;

pub struct WithColorSpace<'n> {
    pub format: vk::Format,
    pub surface: &'n ash::vk::SurfaceKHR,
    pub device: &'n ash::Device,
    pub instance: &'n ash::Instance,
    pub extent: ash::vk::Extent2D,
    pub color_space: vk::ColorSpaceKHR,
}

impl<'n> SwapchainBuilder<WithFormat<'n>> {
    pub fn with_color_space(self, color_space: vk::ColorSpaceKHR) -> SwapchainBuilder<WithColorSpace<'n>> {
        SwapchainBuilder {

            state: WithColorSpace {
                device: self.state.device,
                surface: self.state.surface,
                instance: self.state.instance,
                extent: self.state.extent,
                format: self.state.format,
                color_space
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