

use super::device::WithDevice;
use super::SwapchainBuilder;
use ash::vk;

pub struct WithSurface<'n> {
    pub device: &'n ash::Device,
    pub surface: &'n vk::SurfaceKHR,
}

impl<'n> SwapchainBuilder<WithDevice<'n>> {
    pub fn with_surface(self, surface: &'n ash::vk::SurfaceKHR) -> SwapchainBuilder<WithSurface<'n>> {
        SwapchainBuilder {

            state: WithSurface {
                device: self.state.device,
                surface
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