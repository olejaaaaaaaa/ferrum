use std::sync::Arc;

use ash::vk::{PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR};
use winit::window::Window;

use crate::{ Device, GraphicsDevice, Instance, Surface, Swapchain, SwapchainBuilder, WindowManagerBuilder, WithMode };


pub struct WithSwapchain {
    pub device: Arc<GraphicsDevice>,
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR,
    pub caps: SurfaceCapabilitiesKHR,
    pub mode: PresentModeKHR,
    pub swapchain: Swapchain
}

impl WindowManagerBuilder<WithMode> {
    pub fn with_swapchain<F>(self, build_fn: F) -> WindowManagerBuilder<WithSwapchain>
        where F: FnOnce(&ash::Instance, &ash::Device, &SurfaceKHR, &SurfaceFormatKHR, &PresentModeKHR, &SurfaceCapabilitiesKHR) -> Swapchain {

            let instance = &self.state.device.instance.raw;
            let device = &self.state.device.logical_device.raw;
            let surface = &self.state.surface.raw;
            let format = &self.state.format;
            let mode = &self.state.mode;
            let caps = &self.state.caps;

            let swapchain = build_fn(
                instance,
                device,
                surface,
                format,
                mode,
                caps
            );

            WindowManagerBuilder { state: WithSwapchain {
                device: self.state.device,
                window: self.state.window,
                surface: self.state.surface,
                format: self.state.format,
                mode: self.state.mode,
                caps: self.state.caps,
                swapchain
            }}
    }

    pub fn with_default_swapchain(self) -> WindowManagerBuilder<WithSwapchain> {

        self.with_swapchain(|instance, device, surface, format, mode, caps| {

            let extent = caps.current_extent;
            let transform = caps.current_transform;

            SwapchainBuilder::new()
                .with_color_space(format.color_space)
                .with_format(format.format)
                .with_resolution(extent)
                .with_transform(transform)
                .with_present_mode(*mode)
                .with_instance(instance)
                .with_device(device)
                .with_surface(&surface)
                .build()
        })
    }
}