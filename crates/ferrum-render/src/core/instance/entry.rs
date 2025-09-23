use ash::vk::ApplicationInfo;
use crate::{
    InstanceBuilder
};

pub struct WithEntry {
    pub entry: ash::Entry
}

impl InstanceBuilder<()> {
    pub fn with_entry(self, entry: ash::Entry) -> InstanceBuilder<WithEntry> {
        InstanceBuilder {
            state: WithEntry { entry },
            allocation_callback: self.allocation_callback,
            flags: self.flags
        }
    }
}