
pub mod entry;
pub mod instance;
pub mod window_handle;
pub mod display_handle;
pub mod allocation_callback;

use crate::{VulkanError, VulkanResult};
use allocation_callback::WithAllocationCallback;

use ash::{self,
    vk::{
        PresentModeKHR,
        SurfaceCapabilitiesKHR,
        SurfaceFormatKHR
    }
};

pub struct Surface {
    pub raw: ash::vk::SurfaceKHR,
    pub raw_loader: ash::khr::surface::Instance,
}

impl Surface {

    pub fn get_surface_formats(&self, phys_dev: &ash::vk::PhysicalDevice) -> Vec<SurfaceFormatKHR>{
        let formats = unsafe { self.raw_loader.get_physical_device_surface_formats(*phys_dev, self.raw).unwrap() };
        formats
    }

    pub fn get_surface_capabilities(&self, phys_dev: &ash::vk::PhysicalDevice) -> SurfaceCapabilitiesKHR {
        let caps = unsafe { self.raw_loader.get_physical_device_surface_capabilities(*phys_dev, self.raw).unwrap() };
        caps
    }

    pub fn get_surface_present_modes(&self, phys_dev: &ash::vk::PhysicalDevice) -> Vec<PresentModeKHR> {
        let present = unsafe { self.raw_loader.get_physical_device_surface_present_modes(*phys_dev, self.raw).unwrap() };
        present
    }
}

pub struct SurfaceBuilder<S> {
    pub state: S
}

impl SurfaceBuilder<()> {
    pub fn new() -> Self {
        SurfaceBuilder { state: () }
    }
}

impl<'n> SurfaceBuilder<WithAllocationCallback<'n>> {

    pub fn build(self) -> VulkanResult<Surface> {

        let display_handle = self.state.display_handle;
        let window_handle = self.state.window_handle;
        let entry = self.state.entry;
        let instance = self.state.instance;

        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                *display_handle,
                *window_handle,
                self.state.allocation_callback
            ).map_err(|x| VulkanError::Unknown)?
        };

        let surface_loader = ash::khr::surface::Instance::new(entry, instance);

        Ok(Surface {
            raw: surface,
            raw_loader: surface_loader
        })
    }
}