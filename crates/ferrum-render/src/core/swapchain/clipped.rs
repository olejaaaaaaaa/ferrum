
use crate::transform::WithTransform;
use super::SwapchainBuilder;
use ash::vk;

pub struct WithClipped<'n> {
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
}

impl<'n> SwapchainBuilder<WithTransform<'n>> {
    /// default value is `true`
    pub fn with_clipped(self, clipped: Option<bool>) -> SwapchainBuilder<WithClipped<'n>> {

        let clipped = clipped.unwrap_or(true);

        SwapchainBuilder { state: WithClipped {
            device: self.state.device,
            instance: self.state.instance,
            format: self.state.format,
            color_space: self.state.color_space,
            extent: self.state.extent,
            present_mode: self.state.present_mode,
            image_count: self.state.image_count,
            surface: self.state.surface,
            transform: self.state.transform,
            clipped
        }}
    }
}