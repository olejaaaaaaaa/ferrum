use ash::vk;

use crate::core::{
    PhysicalDevice,
    PhysicalDeviceBuilder,
    Instance,
    InstanceBuilder,
    Surface,
    SamplerBuilder
};

use super::{GraphicsDevice, GraphicsDeviceBuilder};
use super::instance::WithInstance;
pub struct WithPhysicalDevice {
    pub instance: Instance,
    pub phys_dev: PhysicalDevice
}

impl<'n> GraphicsDeviceBuilder<WithInstance<'n>> {

    pub fn with_phys_dev<F>(self, surface: &'n Surface, build_fn: F) -> GraphicsDeviceBuilder<WithPhysicalDevice>
    where F: FnOnce(&Instance, &Surface) -> PhysicalDevice {

        let phys_dev = build_fn(&self.state.instance, surface);

        GraphicsDeviceBuilder {
            state: WithPhysicalDevice {
                instance: self.state.instance,
                phys_dev
            }
        }
    }

    pub fn with_default_phys_dev(self, surface: &Surface) -> GraphicsDeviceBuilder<WithPhysicalDevice> {
        self.with_phys_dev(surface, |instance, surface| {

            const PRIORITY_GPU: &[vk::PhysicalDeviceType] = &[
                vk::PhysicalDeviceType::DISCRETE_GPU,
                vk::PhysicalDeviceType::INTEGRATED_GPU,
                vk::PhysicalDeviceType::VIRTUAL_GPU,
                vk::PhysicalDeviceType::CPU,
                vk::PhysicalDeviceType::OTHER
            ];

            PhysicalDeviceBuilder::new()
                .with_instance(&instance.raw)
                .with_api_version(instance.api_version)
                .with_surface(&surface.raw)
                .with_surface_loader(&surface.raw_loader)
                .build()
                // .select_physical_device(Some(|phys_infos: &[PhysicalDeviceInfo] | {

                //         for &priority_type in PRIORITY_GPU {
                //             if let Some((index, _)) = phys_infos.iter().enumerate().find(|(_, info)| {
                //                 info.is_support_surface && info.phys_prop.device_type == priority_type
                //             }) {
                //                 return index;
                //             }
                //         }

                //         panic!("No suitable device found");
                //     }))
                // .build()
                .unwrap()
        })
    }
}