use super::AppBuilder;

pub struct WithApiVersion {
    pub api_version: u32
}

impl<'n> AppBuilder<()> {
    ///
    /// Chooses the Minimal Vulkan API version the application will use
    ///
    pub fn with_min_required_api_version(self, api_version: u32) -> AppBuilder<WithApiVersion> {
        AppBuilder {
            state: WithApiVersion { api_version },
            app_name: self.app_name,
            engine_name: self.engine_name,
            app_version: self.app_version,
            engine_version: self.engine_version
        }
    }
}