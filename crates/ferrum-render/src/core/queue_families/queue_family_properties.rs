
use super::surface_loader::WithSurfaceLoader;
use super::QueueFamiliesBuilder;

pub struct WithQueueFamilyProperties<'n> {
    pub phys_dev: &'n ash::vk::PhysicalDevice,
    pub surface: &'n ash::vk::SurfaceKHR,
    pub surface_loader: &'n ash::khr::surface::Instance,
    pub properties: &'n [ash::vk::QueueFamilyProperties],
}

impl<'n> QueueFamiliesBuilder<WithSurfaceLoader<'n>> {
    pub fn with_queue_family_properties<>(self, properties: &'n [ash::vk::QueueFamilyProperties]) -> QueueFamiliesBuilder<WithQueueFamilyProperties<'n>> {
        QueueFamiliesBuilder { state: WithQueueFamilyProperties {
            phys_dev: self.state.phys_dev,
            surface: self.state.surface,
            surface_loader: self.state.surface_loader,
            properties
        }}
    }
}