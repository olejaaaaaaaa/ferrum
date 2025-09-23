
use super::SwapchainBuilder;

pub struct WithDevice<'n> {
    pub device: &'n ash::Device,
}

impl SwapchainBuilder<()> {
    pub fn with_device<'n>(self, device: &'n ash::Device) -> SwapchainBuilder<WithDevice<'n>> {
        SwapchainBuilder {

            state: WithDevice {
                device
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