
use super::queue_families::WithQueueFamilies;

use super::DeviceBuilder;
use ash::vk::PhysicalDeviceFeatures;
use super::extensions::WithExtensions;

pub struct WithAllocationCallback<'n> {
    pub instance: &'n ash::Instance,
    pub phys_dev: &'n ash::vk::PhysicalDevice,
    pub features: PhysicalDeviceFeatures,
    pub extensions: Vec<*const i8>,
    pub families: &'n [ash::vk::QueueFamilyProperties],
    pub callback: Option<&'n ash::vk::AllocationCallbacks<'static>>
}

impl<'n> DeviceBuilder<WithQueueFamilies<'n>> {
    pub fn with_allocation_callback(self, callback: Option<&'n ash::vk::AllocationCallbacks<'static>>) -> DeviceBuilder<WithAllocationCallback<'n>> {

        DeviceBuilder { state: WithAllocationCallback {
            instance: self.state.instance,
            phys_dev: self.state.phys_dev,
            features: self.state.features,
            extensions: self.state.extensions,
            families: self.state.families,
            callback
        }}
    }
}