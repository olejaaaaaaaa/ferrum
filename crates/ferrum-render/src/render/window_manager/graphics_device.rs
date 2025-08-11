use std::sync::Arc;

use ash::vk::SurfaceFormatKHR;

use crate::{GraphicsDevice, PhysicalDevice, Surface, WindowManagerBuilder, WithFormat, WithSurface};

pub struct WithGraphicsDevice {
    pub device: Arc<GraphicsDevice>,
    pub window: winit::window::Window,
    pub surface: Surface
}

impl WindowManagerBuilder<WithSurface> {
    pub fn with_graphics_device(self, dev: Arc<GraphicsDevice>) -> WindowManagerBuilder<WithGraphicsDevice> {
            WindowManagerBuilder { state: WithGraphicsDevice {
                window: self.state.window,
                surface: self.state.surface,
                device: dev
            }}
    }
}