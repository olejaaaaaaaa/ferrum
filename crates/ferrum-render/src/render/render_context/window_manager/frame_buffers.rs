use ash::vk::SurfaceCapabilitiesKHR;

use super::WindowManagerBuilder;
use super::WindowManager;
use super::image_views::WithImageViews;
use crate::core::frame_buffers::FrameBuffers;
use crate::core::frame_buffers::FrameBuffersBuilder;

impl WindowManagerBuilder<WithImageViews> {

    pub fn with_frame_buffers<F>(self, build_fn: F) -> WindowManager
        where F: FnOnce(&ash::Device, &Vec<ash::vk::ImageView>, &ash::vk::RenderPass, &SurfaceCapabilitiesKHR) -> FrameBuffers {

            let device = &self.state.device.logical_device.raw;
            let frame_buffers = build_fn(
                &device,
                &self.state.image_views.raw,
                &self.state.render_pass.raw,
                &self.state.caps
            );

            WindowManager {
                raw: self.state.window,
                surface: self.state.surface,
                surface_format_khr: self.state.format,
                mode: self.state.mode,
                caps: self.state.caps,
                swapchain: self.state.swapchain,
                render_pass: self.state.render_pass,
                image_views: self.state.image_views,
                frame_buffers,
            }
    }

    pub fn with_default_frame_buffers(self) -> WindowManager {
        self.with_frame_buffers(|device, image_views, render_pass, caps| {
            FrameBuffersBuilder::default()
                .with_device(device)
                .with_render_pass(render_pass)
                .with_extent(caps.current_extent)
                .with_image_views(image_views)
                .build()
                .unwrap()
        })
    }
}