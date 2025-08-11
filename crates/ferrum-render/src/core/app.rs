
use std::ffi::CStr;
use ash::{vk::{self, *}, Entry};
use crate::{AppError, VulkanError, VulkanResult};

const ENGINE_NAME: &'static CStr = c"Ferrum";
const ENGINE_VERSION: u32 = 24_06_2025;

/// Represents a Vulkan application wrapper containing initialization resources.
///
/// Key invariants:
/// - `entry` and `raw` fields must remain immutable after creation for proper Vulkan operation.
/// - Serves as the central configuration and resource container for Vulkan applications.
/// - Use `AppBuilder` for safe instance construction.
///
/// # Example
/// ```rust
/// fn main() {
///     let app = AppBuilder::new()
///         .with_api_version(ash::vk::API_VERSION_1_0)
///         .build()
///         .expect("Failed to create Vulkan application");
/// }
/// ```
///
/// # Safety
/// - Modifying `entry` or `raw` after creation may lead to undefined behavior.
/// - All fields must be properly initialized before use.
#[derive(Clone)]
pub struct App<'n> {
    /// Vulkan Entry instance for loading Vulkan functions
    pub entry: Entry,
    /// Core application information for Vulkan instance creation
    pub raw: ApplicationInfo<'n>,
    /// Vulkan API version (immutable after creation)
    pub api_version: u32,
}

/// Builder pattern implementation for constructing `App` instances.
///
/// Default values:
/// - `api_version`: VK_API_VERSION_1_2
/// - `app_name`: "None"
/// - `app_version`: 0
/// - `engine_name`: "Ferrum"
/// - `engine_version`: 0
///
/// # Important
/// - After calling `.build()`, the resulting `App` instance should be treated as immutable.
/// - Builder performs version compatibility checks during construction.
///
/// # Example
/// ```rust
/// fn main() {
///     let app = AppBuilder::new()
///         .with_api_version(ash::vk::API_VERSION_1_0)
///         .with_app_name(c"Mario")
///         .build()
///         .expect("Failed to configure Vulkan application");
/// }
/// ```
#[derive(Default)]
pub struct AppBuilder<'n> {
    app_name: Option<&'n CStr>,
    app_version: Option<u32>,
    api_version: Option<u32>,
}

impl<'n> AppBuilder<'n> {

    /// Creates a new builder instance with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the application name
    pub fn with_app_name(mut self, name: &'n CStr) -> Self {
        self.app_name = Some(name);
        self
    }

    /// Sets the Vulkan API version
    pub fn with_api_version(mut self, version: u32) -> Self {
        self.api_version = Some(version);
        self
    }

    /// Sets the application version
    pub fn with_app_version(mut self, version: u32) -> Self {
        self.app_version = Some(version);
        self
    }

    /// Constructs the final `App` instance after validation
    pub fn build(self) -> VulkanResult<App<'n>> {

        let app_name = self.app_name.unwrap_or(c"Game");
        let app_version = self.app_version.unwrap_or(0);
        let mut api_version = self.api_version.unwrap_or(API_VERSION_1_2);

        let entry = unsafe {
            Entry::load().map_err(|e| VulkanError::App(AppError::LoadingVulkan(e)))?
        };

        let available_version = unsafe { 
            entry.try_enumerate_instance_version()
                .map_err(|e| VulkanError::App(AppError::LoadingVulkanApiVersion(e)))?
        };

        api_version = select_supported_api_version(api_version, available_version);

        let app_info = ApplicationInfo::default()
            .api_version(api_version)
            .application_name(app_name)
            .application_version(app_version)
            .engine_name(ENGINE_NAME)
            .engine_version(ENGINE_VERSION);

        Ok(App {
            entry,
            raw: app_info,
            api_version
        })
    }
}

/// Selects the appropriate Vulkan API version based on system capabilities
/// 
/// # Parameters
/// - `requested`: The desired Vulkan version
/// - `available`: The highest version supported by the system
/// 
/// # Returns
/// The highest compatible version that can be safely used
fn select_supported_api_version(requested: u32, available: Option<u32>) -> u32 {

    if let Some(available) = available {
        if requested > available {

            log::warn!(
                "Requested Vulkan version {}.{}.{} is not supported. Falling back to {}.{}.{}.",
                vk::api_version_major(requested),
                vk::api_version_minor(requested),
                vk::api_version_patch(requested),
                vk::api_version_major(available),
                vk::api_version_minor(available),
                vk::api_version_patch(available)
            );

            available

        } else {
            requested
        }
    } else {

        if requested > API_VERSION_1_0 {
            log::warn!("Only API_VERSION_1_0 is available");
        }

        API_VERSION_1_0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_app_with_defaults() {
        let result = AppBuilder::new()
            .build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_custom_app_name_and_versions() {

        let app_name = c"MyApp";
        let result = AppBuilder::new()
            .with_app_name(app_name)
            .with_app_version(1)
            .with_api_version(vk::API_VERSION_1_0)
            .build();

        assert!(result.is_ok());

        let app = result.unwrap();
        assert_eq!(app.raw.p_application_name, app_name.as_ptr());
        assert_eq!(app.raw.application_version, 1);
        assert_eq!(app.raw.engine_version, vk::API_VERSION_1_0);
        assert_eq!(app.raw.api_version, vk::API_VERSION_1_0)
    }

    #[test]
    fn test_select_supported_api_version() {

        let requested = vk::API_VERSION_1_2;
        let available = vk::API_VERSION_1_0;
        let result = select_supported_api_version(requested, Some(available));
        assert_eq!(result, vk::API_VERSION_1_0);

        let requested2 = vk::API_VERSION_1_0;
        let available2 = vk::API_VERSION_1_2;
        let result2 = select_supported_api_version(requested2, Some(available2));
        assert_eq!(result2, vk::API_VERSION_1_0);
    }
}