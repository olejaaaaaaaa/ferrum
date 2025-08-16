use ash::vk;


#[derive(Debug)]
pub enum PhysicalDeviceError {
    EnumeratePhysicalDeviceFailed(vk::Result),
    EnumerateDeviceExtensionPropertiesFailed(vk::Result),
    EnumerateDeviceLayerPropertiesFailed(vk::Result)
}
