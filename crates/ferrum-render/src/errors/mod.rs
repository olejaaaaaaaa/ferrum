
pub mod instance;
pub use instance::InstanceError;

pub mod app;
pub use app::AppError;

pub mod phys_dev;
pub use phys_dev::PhysicalDeviceError;

pub mod buffer;
pub use buffer::BufferError;

pub mod device;
pub use device::LogicalDeviceError;

pub mod sampler;
pub use sampler::SamplerError;

pub mod swapchain;
pub use swapchain::SwapchainError;

pub mod shader;
pub use shader::ShaderError;

pub mod command_pool;
pub use command_pool::CommandPoolError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VulkanError {
    #[error("App error: {0}")]
    App(AppError),
    #[error("Instance error: {0}")]
    Instance(InstanceError),
    #[error("Physical device error: {0}")]
    PhysicalDevice(PhysicalDeviceError),
    #[error("Logical device error: {0}")]
    CommandPool(CommandPoolError),
    #[error("Logical device error: {0}")]
    LogicalDevice(LogicalDeviceError),
    #[error("Sampler error: {0}")]
    Sampler(SamplerError),
    #[error("Swapchain error: {0}")]
    Swapchain(SwapchainError),
    #[error("GPUBuffer error: {0}")]
    GpuBuffer(BufferError),
    #[error("Shader error: {0}")]
    Shader(ShaderError),
    #[error("Unknown error")]
    Unknown,
}

pub type VulkanResult<T> = core::result::Result<T, VulkanError>;