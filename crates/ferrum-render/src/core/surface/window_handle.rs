
use winit::raw_window_handle::RawWindowHandle;

use super::instance::WithInstance;
use super::SurfaceBuilder;

pub struct WithWindowHandle<'n> {
    pub entry: &'n ash::Entry,
    pub instance: &'n ash::Instance,
    pub window_handle: &'n RawWindowHandle,
}

impl<'n> SurfaceBuilder<WithInstance<'n>> {
    pub fn with_window_handle(self, window_handle: &'n RawWindowHandle) -> SurfaceBuilder<WithWindowHandle<'n>> {
        SurfaceBuilder { state: WithWindowHandle {
            entry: self.state.entry,
            instance: self.state.instance,
            window_handle
        }}
    }
}