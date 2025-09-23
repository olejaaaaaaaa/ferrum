
use std::ffi::CStr;

use ash::{vk::*, Entry};

use crate::{
    AppError, VulkanError, VulkanResult
};

mod api_version;
use api_version::WithApiVersion;

/// Wrapper around Vulkan application information. Need for creation Vulkan [`crate::core::Instance`]
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
#[derive(Clone)]
pub struct App<'n> {
    /// Vulkan Entry is needed to load instance functions
    pub entry: Entry,
    /// raw Vulkan ApplicationInfo structure
    pub raw: ApplicationInfo<'n>,
    /// Vulkan API version
    pub api_version: u32,
}

/// Builder pattern implementation for constructing `App` instances.
///
/// Default values:
/// - `app_name`: "App"
/// - `app_version`: 0
/// - `engine_name`: "Ferrum"
/// - `engine_version`: 0
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
pub struct AppBuilder<S = ()> {
    pub app_name: Option<&'static CStr>,
    pub app_version: Option<u32>,
    pub engine_name: Option<&'static CStr>,
    pub engine_version: Option<u32>,
    pub state: S
}

impl AppBuilder {

    pub fn default() -> AppBuilder<()> {
        AppBuilder {
            state: (),
            app_name:       Some(CStr::from_bytes_until_nul(b"App\0").unwrap()),
            app_version:    Some(0),
            engine_name:    Some(CStr::from_bytes_until_nul(b"Ferrum\0").unwrap()),
            engine_version: Some(0),
        }
    }

    pub fn new() -> AppBuilder<()> {
        AppBuilder {
            state: (),
            app_name:       None,
            app_version:    None,
            engine_name:    None,
            engine_version: None,
        }
    }
}

impl<'n> AppBuilder<WithApiVersion> {

    pub fn with_app_name(mut self, name: &'static CStr) -> Self {
        self.app_name = Some(name);
        self
    }

    pub fn with_app_version(mut self, version: u32) -> Self {
        self.app_version = Some(version);
        self
    }

    pub fn build(self) -> VulkanResult<App<'n>> {

        let entry = unsafe {
            Entry::load().map_err(|e| VulkanError::App(AppError::LoadingVulkan(e)))?
        };

        let available_version = unsafe {
            entry.try_enumerate_instance_version()
                .map_err(|e| VulkanError::App(AppError::LoadingVulkanApiVersion(e)))?
        };

        let app_name = self.app_name.ok_or(VulkanError::Unknown)?;
        let app_version = self.app_version.ok_or(VulkanError::Unknown)?;
        let engine_name = self.engine_name.ok_or(VulkanError::Unknown)?;
        let engine_version = self.engine_version.ok_or(VulkanError::Unknown)?;

        let api_version = select_supported_api_version(self.state.api_version, available_version).unwrap();

        let app_info = ApplicationInfo::default()
            .api_version(api_version)
            .application_name(app_name)
            .application_version(app_version)
            .engine_name(engine_name)
            .engine_version(engine_version);

        Ok(App {
            entry,
            raw: app_info,
            api_version
        })
    }
}

pub fn select_supported_api_version(requested: u32, available: Option<u32>) -> Option<u32> {

    if let Some(available) = available {
        if requested > available {
            None
        } else {
            Some(available)
        }
    } else {

        if requested > API_VERSION_1_0 {
            return None;
        }

        Some(API_VERSION_1_0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_app_with_defaults() {

        // let result = AppBuilder::new()
        //     .with_min_required_api_version(api_version)
        //     .
        //     .with_app_name(None)
        //     .with_app_version(None)
        //     .with_api_version(None, None)
        //     .with_engine_name(None)
        //     .with_engine_version(None)
        //     .build();

        // assert!(result.is_ok());
    }

    #[test]
    fn test_select_supported_api_version() {

        // let requested = vk::API_VERSION_1_2;
        // let available = vk::API_VERSION_1_0;
        // let result = select_supported_api_version(requested, Some(available));
        // //assert_eq!(result, vk::API_VERSION_1_0);

        // let requested2 = vk::API_VERSION_1_0;
        // let available2 = vk::API_VERSION_1_2;
        // let result2 = select_supported_api_version(requested2, Some(available2));
        //assert_eq!(result2, vk::API_VERSION_1_0);
    }
}