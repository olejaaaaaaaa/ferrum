use std::sync::Arc;

use crate::*;
use ash::vk::{Extent2D, Format, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use winit::{raw_window_handle::*, window::Window};

pub(crate) mod window;
pub use window::*;

pub(crate) mod surface;
pub use surface::*;

pub(crate) mod present_formats;
pub use present_formats::*;

pub(crate) mod present_modes;
pub use present_modes::*;

pub(crate) mod swapchain;
pub use swapchain::*;

pub(crate) mod render_pass;
pub use render_pass::*;

pub(crate) mod image_views;
pub use image_views::*;

pub(crate) mod frame_buffers;
pub use frame_buffers::*;

pub(crate) mod graphics_device;
pub use graphics_device::*;

pub struct WindowManagerBuilder<S> {
    pub state: S
}

pub struct WindowManager {
    pub raw: Window,
    pub surface: Surface,
    pub surface_format_khr: SurfaceFormatKHR,
    pub mode: PresentModeKHR,
    pub caps: SurfaceCapabilitiesKHR,
    pub swapchain: Swapchain,
    pub render_pass: RenderPass,
    pub image_views: ImageViews,
    pub frame_buffers: FrameBuffers
}


impl WindowManager {

    pub fn resize(&mut self, dev: &Arc<GraphicsDevice>, width: u32, height: u32) {

        if width == 0 || height == 0 {
            return;
        }

        let format = &self.surface_format_khr;
        let device = &dev.logical_device.raw;
        let instance = &dev.instance.raw;
        let transform = &self.caps.current_transform;
        let mode = &self.mode;
        let surface = &self.surface.raw;

        unsafe { device.device_wait_idle().expect("Failed to wait for device idle"); }

        let caps = self.surface.get_surface_capabilities(&dev.phys_dev.raw);

        let swapchain = SwapchainBuilder::new()
                .with_color_space(format.color_space)
                .with_format(format.format)
                .with_resolution(caps.current_extent)
                .with_transform(*transform)
                .with_present_mode(*mode)
                .with_instance(instance)
                .with_device(device)
                .with_surface(surface)
                .build();

        let swapchain_images = swapchain.get_swapchain_images();

        let image_views = ImageViewsBuilder::new()
            .with_device(device)
            .with_format(format.format)
            .with_image_views(&swapchain_images)
            .build();

        let frame_buffers = FrameBufferBuilder::new()
            .device(device)
            .image_views(&image_views.raw)
            .resolution(caps.current_extent)
            .render_pass(&self.render_pass.raw)
            .build();

        unsafe {
            for i in &self.frame_buffers.raw {
                device.destroy_framebuffer(*i, None);
            }

            for i in &self.image_views.raw {
                device.destroy_image_view(*i, None);
            }

            self.swapchain.swapchain_load.destroy_swapchain(self.swapchain.raw, None);
        }

        self.caps = caps;
        self.swapchain = swapchain;
        self.image_views = image_views;
        self.frame_buffers = frame_buffers;

        log::debug!("New size: {:?}", self.caps.current_extent)
    }


}