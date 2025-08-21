
pub mod instance;
pub use instance::InstanceError;

pub mod app;
pub use app::AppError;

pub mod phys_dev;
pub use phys_dev::PhysicalDeviceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VulkanError {
    #[error("App error: {0}")]
    App(AppError),
    #[error("Instance error: {0}")]
    Instance(InstanceError),
    #[error("Physical device error: {0}")]
    PhysicalDevice(PhysicalDeviceError),
    #[error("Unknown error")]
    Unknown,
}

pub type VulkanResult<T> = core::result::Result<T, VulkanError>;