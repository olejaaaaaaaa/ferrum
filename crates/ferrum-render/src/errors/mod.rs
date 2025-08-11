
pub mod instance;
pub use instance::InstanceError;

pub mod app;
pub use app::AppError;

#[derive(Debug)]
pub enum VulkanError {
    App(AppError),
    Instance(InstanceError)
}

pub type VulkanResult<T> = core::result::Result<T, VulkanError>;