use ash::vk;
use crate::extent::WithExtent;
use super::SwapchainBuilder;

pub struct WithFormat<'n> {
    pub device: &'n ash::Device,
    pub surface: &'n ash::vk::SurfaceKHR,
    pub instance: &'n ash::Instance,
    pub extent: ash::vk::Extent2D,
    pub format: vk::Format,
}

impl<'n> SwapchainBuilder<WithExtent<'n>> {
    pub fn with_format(self, format: vk::Format) -> SwapchainBuilder<WithFormat<'n>> {
        SwapchainBuilder {

            state: WithFormat {
                device: self.state.device,
                surface: self.state.surface,
                instance: self.state.instance,
                extent: self.state.extent,
                format,
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