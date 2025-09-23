use ash::{
    vk::{
        AllocationCallbacks,
        ApplicationInfo,
        InstanceCreateFlags
    }
};

use crate::{
    debug_messenger::WithDebugMessenger,
    InstanceBuilder
};

pub struct WithAllocationCallback<'n> {
    pub app_info: ApplicationInfo<'n>,
    pub entry: ash::Entry,
    pub api_version: u32,
    pub flags: InstanceCreateFlags,
    pub layers: Vec<*const i8>,
    pub extensions: Vec<*const i8>,
    pub debug_messenger: bool,
    pub allocation_callback: Option<&'n AllocationCallbacks<'static>>
}

impl<'n> InstanceBuilder<WithDebugMessenger<'n>> {

    pub fn with_allocation_callback(self, callback: Option<&'n AllocationCallbacks<'static>>) -> InstanceBuilder<WithAllocationCallback<'n>>
    {
        InstanceBuilder { state: WithAllocationCallback {
            app_info: self.state.app_info,
            entry: self.state.entry,
            api_version: self.state.api_version,
            flags: self.state.flags,
            layers: self.state.layers,
            extensions: self.state.extensions,
            debug_messenger: self.state.debug_messenger,
            allocation_callback: callback
        }}
    }
}


