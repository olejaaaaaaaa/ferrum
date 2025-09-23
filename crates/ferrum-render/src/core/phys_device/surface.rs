use ash::vk::SurfaceKHR;

use super::PhysicalDeviceBuilder;
use super::api_version::WithApiVersion;

pub struct WithSurface<'n> {
    pub instance: &'n ash::Instance,
    pub api_version: u32,
    pub surface: &'n SurfaceKHR
}

impl<'n> PhysicalDeviceBuilder<WithApiVersion<'n>> {
    pub fn with_surface(self, surface: &'n SurfaceKHR) -> PhysicalDeviceBuilder<WithSurface<'n>> {
        PhysicalDeviceBuilder {

            state: WithSurface {
                instance: self.state.instance,
                api_version: self.state.api_version,
                surface: surface
            },

            fn_select_phys_dev: self.fn_select_phys_dev

        }
    }
}

impl<S> PhysicalDeviceBuilder<S> {
    /// Check if the physical device supports presentation to the given surface
    pub fn check_support_surface(phys_dev: &ash::vk::PhysicalDevice, surface: &SurfaceKHR, surface_loader: &ash::khr::surface::Instance, count_queue_familes: usize) -> bool {

        for index in 0..count_queue_familes{

            let res = unsafe { surface_loader.get_physical_device_surface_support(*phys_dev, index as u32, *surface) };

            match res {
                Ok(support) => {
                    if support {
                        return true
                    }
                },

                Err(err) => {
                    log::warn!("Error check support surface: {}", err);
                }
            }
        }

        return false;
    }
}


