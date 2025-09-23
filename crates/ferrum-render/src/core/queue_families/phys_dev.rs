
use super::QueueFamiliesBuilder;

pub struct WithPhysDev<'n> {
    pub phys_dev: &'n ash::vk::PhysicalDevice
}

impl<'n> QueueFamiliesBuilder<()> {
    pub fn with_phys_dev(self, phys_dev: &'n ash::vk::PhysicalDevice) -> QueueFamiliesBuilder<WithPhysDev<'n>> {
        QueueFamiliesBuilder { state: WithPhysDev { phys_dev } }
    }
}