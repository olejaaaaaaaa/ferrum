
use std::{
    ffi::CStr,
    sync::Arc
};

use ash::vk::{
    Format,
    PresentModeKHR
};


mod window_manager;
use window_manager::{
    WindowManager,
    WindowManagerBuilder
};

mod graphics_device;
pub use graphics_device::{
    GraphicsDevice,
    GraphicsDeviceBuilder
};

mod buffer;
mod submit_commands;
mod sampler;
pub mod texture;

pub struct RenderContext {
    pub device: Arc<GraphicsDevice>,
    pub window: WindowManager,
}

#[allow(dead_code)]
pub struct RenderContextParams<'n> {
    pub api_version: Option<u32>,
    pub app_name: Option<&'static CStr>,
    pub app_version: Option<u32>,
    pub engine_name: Option<&'static CStr>,
    pub engine_version: Option<u32>,
    pub format_prioriry: Option<&'n [Format]>,
    pub present_mode_priority: Option<&'n [PresentModeKHR]>
}

impl RenderContext {

    pub fn from(device: Arc<GraphicsDevice>, window: WindowManager) -> Self {

        Self {
            device,
            window,
        }
    }

    pub fn default(window: winit::window::Window) -> Self {

        let device = GraphicsDeviceBuilder::new()
            .with_default_app()
            .with_window(&window)
            .with_default_instance();

        let window = WindowManagerBuilder::new(window)
            .with_default_surface(&device.state.instance);

        let device: Arc<GraphicsDevice> = device
            .with_default_phys_dev(&window.state.surface)
            .with_default_queue_families(&window.state.surface)
            .with_default_device()
            .into();

        let window = window
            .with_graphics_device(device.clone())
            .with_default_format()
            .with_default_mode()
            .with_default_swapchain()
            .with_default_render_pass()
            .with_default_image_views()
            .with_default_frame_buffers();

        Self::from(device, window)
    }
}


impl Drop for RenderContext {
    fn drop(&mut self) {
        unsafe {
            self.window.surface.raw_loader.destroy_surface(self.window.surface.raw, None);
            self.device.logical_device.raw.destroy_device(None);
            self.device.instance.raw.destroy_instance(None);
        }
    }
}

