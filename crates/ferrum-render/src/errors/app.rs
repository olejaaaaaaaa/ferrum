use ash::LoadingError;
use ash::vk::Result;

#[derive(Debug)]
pub enum AppError {
    LoadingVulkan(LoadingError),
    LoadingVulkanApiVersion(Result)
}