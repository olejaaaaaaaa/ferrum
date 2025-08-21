use std::ffi::CStr;
use thiserror::Error;
use ash::vk;

#[derive(Debug, Error)]
pub enum InstanceError {
    #[error("Missing required AppInfo structure")]
    MissingApp,
    #[error("Failed to create Vulkan instance (VkResult: {0:?})")]
    InstanceCreationFailed(vk::Result),
    #[error("Failed to create debug utils messenger (VkResult: {0:?})")]
    DebugUtilsMessengerCreationFailed(vk::Result),
    #[error("Required Vulkan extension not available: {0}")]
    MissingRequiredExtension(String),
    #[error("Required Vulkan layer not available: {0}")]
    MissingRequiredLayer(String),
    #[error("Failed to enumerate instance layers (VkResult: {0:?})")]
    EnumerateInstanceLayerPropertiesFailed(vk::Result),
    #[error("Vulkan layer is not supported: {0:?}")]
    NotSupportRequiredLayer(&'static CStr)
}
