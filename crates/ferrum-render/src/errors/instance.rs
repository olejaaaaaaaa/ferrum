use std::ffi::CStr;

use ash::vk;

#[derive(Debug)]
pub enum InstanceError {
    MissingApp,
    InstanceCreationFailed(vk::Result),
    DebugUtilsMessengerCreationFailed(vk::Result),
    MissingRequiredExtension(String),
    MissingRequiredLayer(String),
    EnumerateInstanceLayerPropertiesFailed(vk::Result),
    NotSupportRequiredLayer(&'static CStr)
}

