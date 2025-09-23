use std::sync::Arc;

use ash::vk::{Format, Image, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use winit::window::Window;

use crate::core::Surface;
use crate::core::RenderPass;
use crate::core::Swapchain;
use crate::GraphicsDevice;
use super::WindowManagerBuilder;
use super::render_pass::WithRenderPass;
use crate::core::image_views as imgv;

pub struct WithImageViews {
    pub device: Arc<GraphicsDevice>,
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR,
    pub mode: PresentModeKHR,
    pub caps: SurfaceCapabilitiesKHR,
    pub swapchain: Swapchain,
    pub render_pass: RenderPass,
    pub image_views: imgv::ImageViews
}

impl<'n> WindowManagerBuilder<WithRenderPass> {
    pub fn with_image_views<F>(self, build_fn: F) -> WindowManagerBuilder<WithImageViews>
        where F: FnOnce(&ash::Device, Format, Vec<Image>) -> imgv::ImageViews {

            let swapchain_images = self.state.swapchain.get_swapchain_images().unwrap();
            let device = &self.state.device.logical_device.raw;
            let image_views = build_fn(&device, self.state.format.format, swapchain_images);

            WindowManagerBuilder { state: WithImageViews {
                device: self.state.device,
                window: self.state.window,
                surface: self.state.surface,
                format: self.state.format,
                mode: self.state.mode,
                caps: self.state.caps,
                swapchain: self.state.swapchain,
                render_pass: self.state.render_pass,
                image_views
            }}
    }

    pub fn with_default_image_views(self) -> WindowManagerBuilder<WithImageViews> {
        self.with_image_views(|device, format, swapchain_images| {
            imgv::ImageViewsBuilder::new()
                .with_device(device)
                .with_format(format)
                .with_image_views(&swapchain_images)
                .build()
                .unwrap()
        })
    }
}