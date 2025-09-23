use super::instance::WithInstance;
use super::SwapchainBuilder;

pub struct WithExtent<'n> {
    pub device: &'n ash::Device,
    pub surface: &'n ash::vk::SurfaceKHR,
    pub instance: &'n ash::Instance,
    pub extent: ash::vk::Extent2D,
}

impl<'n> SwapchainBuilder<WithInstance<'n>> {
    pub fn with_extent(self, extent: ash::vk::Extent2D) -> SwapchainBuilder<WithExtent<'n>> {
        SwapchainBuilder {
            state: WithExtent {
                device: self.state.device,
                surface: self.state.surface,
                instance: self.state.instance,
                extent
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

