
pub mod features;
pub mod instance;
// pub mod phys_device;
pub mod memory_properties;
pub mod surface;
pub mod surface_loader;
pub mod api_version;
pub mod select_phys_device;
pub mod physcial_device_info;
pub mod queue_family_properties;
pub mod physical_device_properties;

use physcial_device_info::PhysicalDeviceInfo;
use surface_loader::WithSurfaceLoader;
use select_phys_device::default_select_device;
use crate::{
    VulkanResult,
    VulkanError,
    PhysicalDeviceError
};

///
/// Wrapper around Vulkan Physical Device
///
/// Contains both the raw Vulkan handle and associated properties and features.
///
pub struct PhysicalDevice {
    pub raw: ash::vk::PhysicalDevice,
    pub phys_info: PhysicalDeviceInfo,
    #[cfg(debug_assertions)]
    pub destroyed: bool
}

pub struct PhysicalDeviceBuilder<S> {
    pub fn_select_phys_dev: Box<dyn FnOnce(&[PhysicalDeviceInfo]) -> usize>,
    pub state: S
}

impl PhysicalDeviceBuilder<()> {
    pub fn new() -> PhysicalDeviceBuilder<()> {
        Self {
            state: (),
            fn_select_phys_dev: Box::new(default_select_device)
        }
    }
}


impl<'n> PhysicalDeviceBuilder<WithSurfaceLoader<'n>> {

    pub fn build(self) -> VulkanResult<super::PhysicalDevice> {

        let phys_devs = unsafe { self.state.instance.enumerate_physical_devices().map_err(|e|
            VulkanError::PhysicalDevice(
                PhysicalDeviceError::EnumeratePhysicalDeviceFailed(e))
            )?
        };

        let phys_infos = phys_devs.iter().filter_map(|dev| {

            let api_version = self.state.api_version;

            if let Ok(x) = self.phys_device_info(dev, &self.state.instance, api_version, &self.state.surface, &self.state.surface_loader) {
                if x.is_support_surface {
                    return Some(x);
                }
            }

            None
        }).collect::<Vec<_>>();

        let index = (self.fn_select_phys_dev)(&phys_infos);

        let phys_dev = phys_devs[index];
        let phys_info = phys_infos[index].clone();

        Ok(crate::PhysicalDevice {
            raw: phys_dev,
            phys_info,
            #[cfg(debug_assertions)]
            destroyed: false
        })
    }
}


