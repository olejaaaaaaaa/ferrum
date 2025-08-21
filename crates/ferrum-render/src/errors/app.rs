use ash::LoadingError;
use ash::vk::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error dynamic load vulkan entry point")]
    LoadingVulkan(LoadingError),
    #[error("Error get support vulkan api version")]
    LoadingVulkanApiVersion(Result)
}
