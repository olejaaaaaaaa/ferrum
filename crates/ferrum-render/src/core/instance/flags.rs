use ash::vk::{ApplicationInfo, InstanceCreateFlags};
use crate::InstanceBuilder;
use super::api_version::WithApiVersion;

pub struct WithFlags<'n> {
    pub app_info: ApplicationInfo<'n>,
    pub entry: ash::Entry,
    pub api_version: u32,
    pub flags: InstanceCreateFlags
}

impl<'n> InstanceBuilder<WithApiVersion<'n>> {
    pub fn with_flags(self, flags: Option<InstanceCreateFlags>) -> InstanceBuilder<WithFlags<'n>> {

        #[allow(unused_mut)]
        let mut flags = flags.unwrap_or(InstanceCreateFlags::empty());

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        if !flags.contains(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR) {
            flags = flags | InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR;
        }

        InstanceBuilder { state: WithFlags {
            app_info: self.state.app_info,
            entry: self.state.entry,
            api_version: self.state.api_version,
            flags
        }}
    }
}