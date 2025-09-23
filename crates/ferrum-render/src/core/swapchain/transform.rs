
use crate::transform;

use super::surface::WithSurface;
use super::SwapchainBuilder;
use ash::vk::{self, SurfaceTransformFlagsKHR};

pub struct WithTransform<'n> {
    pub device: &'n ash::Device,
    pub instance: &'n ash::Instance,
    pub format: vk::Format,
    pub color_space: vk::ColorSpaceKHR,
    pub extent: ash::vk::Extent2D,
    pub present_mode: vk::PresentModeKHR,
    pub image_count: u32,
    pub surface: &'n vk::SurfaceKHR,
    pub transform: vk::SurfaceTransformFlagsKHR,
}

impl<'n> SwapchainBuilder<WithSurface<'n>> {

    /// default value is IDENTITY
    pub fn with_transform(self, transform: Option<SurfaceTransformFlagsKHR>) -> SwapchainBuilder<WithTransform<'n>> {

        let transform = transform.unwrap_or(vk::SurfaceTransformFlagsKHR::IDENTITY);

        SwapchainBuilder { state: WithTransform {
            device: self.state.device,
            instance: self.state.instance,
            format: self.state.format,
            color_space: self.state.color_space,
            extent: self.state.extent,
            present_mode: self.state.present_mode,
            image_count: self.state.image_count,
            surface: self.state.surface,
            transform
        }}
    }
}