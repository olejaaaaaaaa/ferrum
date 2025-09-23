
use super::features::WithFeatures;
use super::DeviceBuilder;
use ash::vk::PhysicalDeviceFeatures;
use std::ffi::CStr;

pub struct WithExtensions<'n> {
    pub instance: &'n ash::Instance,
    pub phys_dev: &'n ash::vk::PhysicalDevice,
    pub features: PhysicalDeviceFeatures,
    pub extensions: Vec<*const i8>
}

impl<'n> DeviceBuilder<WithFeatures<'n>> {
    pub fn with_extensions<F, N>(self, extensions: F) -> DeviceBuilder<WithExtensions<'n>>
    where F: IntoIterator<Item = N>, N: AsRef<CStr> {

        let extensions = extensions.into_iter().map(|name| name.as_ref().as_ptr()).collect::<Vec<_>>();

        DeviceBuilder { state: WithExtensions {
            instance: self.state.instance,
            phys_dev: self.state.phys_dev,
            features: self.state.features,
            extensions
        }}
    }
}