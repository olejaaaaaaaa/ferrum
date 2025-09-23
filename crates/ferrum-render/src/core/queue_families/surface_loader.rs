use super::surface::WithSurface;
use super::QueueFamiliesBuilder;

pub struct WithSurfaceLoader<'n> {
    pub phys_dev: &'n ash::vk::PhysicalDevice,
    pub surface: &'n ash::vk::SurfaceKHR,
    pub surface_loader: &'n ash::khr::surface::Instance,
}

impl<'n> QueueFamiliesBuilder<WithSurface<'n>> {
    pub fn with_surface_loader(self, surface_loader: &'n ash::khr::surface::Instance) -> QueueFamiliesBuilder<WithSurfaceLoader<'n>> {
        QueueFamiliesBuilder { state: WithSurfaceLoader {
            phys_dev: self.state.phys_dev,
            surface: self.state.surface,
            surface_loader
        }}
    }
}
