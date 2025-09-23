
use super::surface::WithSurface;
use super::SwapchainBuilder;

pub struct WithInstance<'n> {
    pub device: &'n ash::Device,
    pub surface: &'n ash::vk::SurfaceKHR,
    pub instance: &'n ash::Instance
}

impl<'n> SwapchainBuilder<WithSurface<'n>> {
    pub fn with_instance(self, instance: &'n ash::Instance) -> SwapchainBuilder<WithInstance<'n>> {
        SwapchainBuilder {

            state: WithInstance {
                device: self.state.device,
                surface: self.state.surface,
                instance
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