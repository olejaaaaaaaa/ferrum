use ash::vk::PhysicalDeviceFeatures;
use super::phys_dev::WithPhysDev;
use super::DeviceBuilder;

pub struct WithFeatures<'n> {
    pub instance: &'n ash::Instance,
    pub phys_dev: &'n ash::vk::PhysicalDevice,
    pub features: PhysicalDeviceFeatures
}

impl<'n> DeviceBuilder<WithPhysDev<'n>> {
    pub fn with_features(self, features: Option<PhysicalDeviceFeatures>) -> DeviceBuilder<WithFeatures<'n>> {
        DeviceBuilder { state: WithFeatures {
            instance: self.state.instance,
            phys_dev: self.state.phys_dev,
            features: features.unwrap_or(PhysicalDeviceFeatures::default())
        }}
    }
}