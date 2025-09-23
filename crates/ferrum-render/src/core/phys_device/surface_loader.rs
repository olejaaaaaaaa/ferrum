use ash::vk::SurfaceKHR;

use super::PhysicalDeviceBuilder;
use super::surface::WithSurface;

pub struct WithSurfaceLoader<'n> {
    pub instance: &'n ash::Instance,
    pub api_version: u32,
    pub surface: &'n SurfaceKHR,
    pub surface_loader: &'n ash::khr::surface::Instance
}

impl<'n> PhysicalDeviceBuilder<WithSurface<'n>> {
    pub fn with_surface_loader(self, surface_loader: &'n ash::khr::surface::Instance) -> PhysicalDeviceBuilder<WithSurfaceLoader<'n>> {
        PhysicalDeviceBuilder {

            state: WithSurfaceLoader {
                instance: self.state.instance,
                api_version: self.state.api_version,
                surface: self.state.surface,
                surface_loader
            },

            fn_select_phys_dev: self.fn_select_phys_dev

        }
    }
}

