
use crate::{InstanceBuilder};
use super::allocation_callback::WithAllocationCallback;
use ash::vk::ApplicationInfo;
use ash::vk::AllocationCallbacks;
use ash::vk::InstanceCreateFlags;
use ash::vk::ExtendsInstanceCreateInfo;

pub struct WithPushNext<'n, T: ExtendsInstanceCreateInfo + ?Sized + 'static> {
    pub app_info: ApplicationInfo<'n>,
    pub entry: ash::Entry,
    pub api_version: u32,
    pub flags: InstanceCreateFlags,
    pub layers: Vec<*const i8>,
    pub extensions: Vec<*const i8>,
    pub debug_messenger: bool,
    pub allocation_callback: Option<&'n AllocationCallbacks<'static>>,
    pub push_next: Vec<&'static mut T>
}

impl<'n> InstanceBuilder<WithAllocationCallback<'n>> {
    pub fn push_next<T: ExtendsInstanceCreateInfo + ?Sized + 'static>(self, next: Option<&'static mut T>) -> InstanceBuilder<WithPushNext<'n, T>> {
        InstanceBuilder { state: WithPushNext {
            app_info: self.state.app_info,
            entry: self.state.entry,
            api_version: self.state.api_version,
            flags: self.state.flags,
            layers: self.state.layers,
            extensions: self.state.extensions,
            debug_messenger: self.state.debug_messenger,
            allocation_callback: self.state.allocation_callback,
            push_next: if next.is_some() { vec![next.unwrap()] } else { vec![] }
        }}
    }
}

impl<'n, N: ExtendsInstanceCreateInfo + ?Sized + 'static> InstanceBuilder<WithPushNext<'n, N>> {
    pub fn push_next(mut self, next: &'static mut N) -> InstanceBuilder<WithPushNext<'n, N>> {
        self.state.push_next.push(next);
        self
    }
}
