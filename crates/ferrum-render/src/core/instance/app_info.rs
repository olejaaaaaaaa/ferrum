use ash::vk::ApplicationInfo;
use super::entry::WithEntry;
use super::InstanceBuilder;

pub struct WithAppInfo<'n> {
    pub entry: ash::Entry,
    pub app_info: ApplicationInfo<'n>
}

impl InstanceBuilder<WithEntry> {
    pub fn with_app_info(self, app_info: ApplicationInfo) -> InstanceBuilder<WithAppInfo<'_>> {
        InstanceBuilder {

            state: WithAppInfo {
                entry: self.state.entry,
                app_info,
            },

            allocation_callback: self.allocation_callback,
            flags: self.flags
        }
    }
}

