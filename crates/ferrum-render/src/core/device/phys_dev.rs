
use super::instance::WithInstance;
use super::DeviceBuilder;

pub struct WithPhysDev<'n> {
    pub instance: &'n ash::Instance,
    pub phys_dev: &'n ash::vk::PhysicalDevice,
}

impl<'n> DeviceBuilder<WithInstance<'n>> {
    pub fn with_phys_dev(self, phys_dev: &'n ash::vk::PhysicalDevice) -> DeviceBuilder<WithPhysDev<'n>> {
        DeviceBuilder { state: WithPhysDev {
            instance: self.state.instance,
            phys_dev
        }}
    }
}