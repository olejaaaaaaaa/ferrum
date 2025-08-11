use std::{ffi::CStr, sync::Arc};

use ash::vk::{Format, PresentModeKHR};

use crate::{AppBuilder, GraphicsDevice, GraphicsDeviceBuilder, InstanceBuilder, WindowManager, WindowManagerBuilder};


pub struct RenderContext {
    pub device: Arc<GraphicsDevice>,
    pub window: WindowManager
}

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

    pub fn new(window: winit::window::Window, params: RenderContextParams) -> Self {

        let api_version = params.api_version.unwrap_or(ash::vk::API_VERSION_1_0);
        let app_name = params.app_name.unwrap_or(c"None");
        let app_version = params.app_version.unwrap_or(0);

        let device = GraphicsDeviceBuilder::new()
            .with_app(|| {
                AppBuilder::new()
                    .with_api_version(api_version)
                    .with_app_name(app_name)
                    .with_app_version(app_version)
                    .build()
                    .unwrap()
            })
            .with_window(&window)
            .with_instance(|app, _| {
                InstanceBuilder::new()
                    .with_app(app)
                    .build()
                    .unwrap()
            });

        // TODO

        let window = WindowManagerBuilder::new(window)
            .with_default_surface(&device.state.instance);

        let device: Arc<GraphicsDevice> = device
            .with_default_phys_dev(&window.state.surface)
            .with_default_queue_family(&window.state.surface)
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

    pub fn from(device: Arc<GraphicsDevice>, window: WindowManager) -> Self {
        Self {
            device,
            window
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
            .with_default_queue_family(&window.state.surface)
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

