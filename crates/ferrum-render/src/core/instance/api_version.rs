use ash::vk::ApplicationInfo;
use super::app_info::WithAppInfo;
use super::InstanceBuilder;

pub struct WithApiVersion<'n> {
    pub entry: ash::Entry,
    pub app_info: ApplicationInfo<'n>,
    pub api_version: u32
}

impl<'n> InstanceBuilder<WithAppInfo<'n>> {
    pub fn with_api_version(self, version: u32) -> InstanceBuilder<WithApiVersion<'n>> {
        InstanceBuilder {
            state: WithApiVersion {
                app_info: self.state.app_info,
                entry: self.state.entry,
                api_version: version,
            },
            allocation_callback: self.allocation_callback,
            flags: self.flags
        }
    }
}