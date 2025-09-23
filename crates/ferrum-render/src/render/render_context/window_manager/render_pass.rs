use std::sync::Arc;

use ash::vk::{self, AttachmentReference, Format, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use winit::window::Window;

use super::{
    Device, GraphicsDevice, RenderPass, RenderPassBuilder, Surface, Swapchain, WindowManagerBuilder, WithSwapchain
};

pub struct WithRenderPass {
    pub device: Arc<GraphicsDevice>,
    pub window: Window,
    pub surface: Surface,
    pub format: SurfaceFormatKHR,
    pub mode: PresentModeKHR,
    pub caps: SurfaceCapabilitiesKHR,
    pub swapchain: Swapchain,
    pub render_pass: RenderPass
}

impl<'n> WindowManagerBuilder<WithSwapchain> {

    pub fn with_render_pass<F>(self, build_fn: F) -> WindowManagerBuilder<WithRenderPass>
        where F: FnOnce(&ash::Device, &Format) -> RenderPass {

            let device = &self.state.device.logical_device.raw;
            let render_pass = build_fn(device, &self.state.format.format);

            WindowManagerBuilder { state: WithRenderPass {
                device: self.state.device,
                window: self.state.window,
                surface: self.state.surface,
                format: self.state.format,
                mode: self.state.mode,
                swapchain: self.state.swapchain,
                caps: self.state.caps,
                render_pass
            }}
    }

    pub fn with_default_render_pass(self) -> WindowManagerBuilder<WithRenderPass> {
        self.with_render_pass(|device, format| {
            RenderPassBuilder::default(*format)
                .with_device(device)
                .build()
                .unwrap()
        })
    }
}