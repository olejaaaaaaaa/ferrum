use winit::raw_window_handle::RawDisplayHandle;
use winit::raw_window_handle::RawWindowHandle;

use super::SurfaceBuilder;
use super::window_handle::WithWindowHandle;

pub struct WithDisplayHandle<'n> {
    pub entry: &'n ash::Entry,
    pub instance: &'n ash::Instance,
    pub window_handle: &'n RawWindowHandle,
    pub display_handle: &'n RawDisplayHandle,
}

impl<'n> SurfaceBuilder<WithWindowHandle<'n>> {
    pub fn with_display_handle(self, display_handle: &'n RawDisplayHandle) -> SurfaceBuilder<WithDisplayHandle<'n>> {
        SurfaceBuilder { state: WithDisplayHandle {
            entry: self.state.entry,
            instance: self.state.instance,
            window_handle: self.state.window_handle,
            display_handle
        }}
    }
}
