
use crate::QueueFamilies;

use super::DeviceBuilder;
use ash::vk::PhysicalDeviceFeatures;
use super::extensions::WithExtensions;

pub struct WithQueueFamilies<'n> {
    pub instance: &'n ash::Instance,
    pub phys_dev: &'n ash::vk::PhysicalDevice,
    pub features: PhysicalDeviceFeatures,
    pub extensions: Vec<*const i8>,
    pub families: &'n [ash::vk::QueueFamilyProperties]
}

impl<'n> DeviceBuilder<WithExtensions<'n>> {
    pub fn queue_families(self, families: &'n [ash::vk::QueueFamilyProperties]) -> DeviceBuilder<WithQueueFamilies<'n>> {
        DeviceBuilder { state: WithQueueFamilies {
            instance: self.state.instance,
            phys_dev: self.state.phys_dev,
            features: self.state.features,
            extensions: self.state.extensions,
            families
        }}
    }
}