
use super::{clipped::WithClipped};
use super::SwapchainBuilder;
use ash::vk;

pub struct WithImageArrayLayers<'n> {
    pub device: &'n ash::Device,
    pub instance: &'n ash::Instance,
    pub format: vk::Format,
    pub color_space: vk::ColorSpaceKHR,
    pub extent: ash::vk::Extent2D,
    pub present_mode: vk::PresentModeKHR,
    pub image_count: u32,
    pub surface: &'n vk::SurfaceKHR,
    pub transform: vk::SurfaceTransformFlagsKHR,
    pub clipped: bool,
    pub image_array_layers: u32
}

impl<'n> SwapchainBuilder<WithClipped<'n>> {

    /// default value is 1
    pub fn with_image_array_layers(self, image_array_layers: Option<u32>) -> SwapchainBuilder<WithImageArrayLayers<'n>> {
        let image_array_layers = image_array_layers.unwrap_or(1);
        SwapchainBuilder { state: WithImageArrayLayers {
            device: self.state.device,
            instance: self.state.instance,
            format: self.state.format,
            color_space: self.state.color_space,
            extent: self.state.extent,
            present_mode: self.state.present_mode,
            image_count: self.state.image_count,
            surface: self.state.surface,
            transform: self.state.transform,
            clipped: self.state.clipped,
            image_array_layers
        }}
    }
}