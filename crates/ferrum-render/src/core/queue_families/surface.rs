
use super::phys_dev::WithPhysDev;
use super::QueueFamiliesBuilder;

pub struct WithSurface<'n> {
    pub phys_dev: &'n ash::vk::PhysicalDevice,
    pub surface: &'n ash::vk::SurfaceKHR,
}

impl<'n> QueueFamiliesBuilder<WithPhysDev<'n>> {
    pub fn with_surface(self, surface: &'n ash::vk::SurfaceKHR) -> QueueFamiliesBuilder<WithSurface<'n>> {
        QueueFamiliesBuilder { state: WithSurface {
            phys_dev: self.state.phys_dev,
            surface
        }}
    }
}