use winit::raw_window_handle::RawDisplayHandle;
use winit::raw_window_handle::RawWindowHandle;

use super::SurfaceBuilder;
use super::display_handle::WithDisplayHandle;

pub struct WithAllocationCallback<'n> {
    pub entry: &'n ash::Entry,
    pub instance: &'n ash::Instance,
    pub window_handle: &'n RawWindowHandle,
    pub display_handle: &'n RawDisplayHandle,
    pub allocation_callback: Option<&'n ash::vk::AllocationCallbacks<'static>>,
}

impl<'n> SurfaceBuilder<WithDisplayHandle<'n>> {
    pub fn with_allocation_callback(self, allocation_callback: Option<&'n ash::vk::AllocationCallbacks<'static>>) -> SurfaceBuilder<WithAllocationCallback<'n>> {
        SurfaceBuilder { state: WithAllocationCallback {
            entry: self.state.entry,
            instance: self.state.instance,
            window_handle: self.state.window_handle,
            display_handle: self.state.display_handle,
            allocation_callback
        }}
    }
}
