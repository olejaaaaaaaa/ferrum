use ash::vk::SurfaceCapabilitiesKHR;

use crate::{
    Device,
    FrameBufferBuilder,
    FrameBuffers,
    ImageViews,
    RenderPass,
    WindowManager,
    WindowManagerBuilder,
    WithImageViews
};

impl WindowManagerBuilder<WithImageViews> {

    pub fn with_frame_buffers<F>(self, build_fn: F) -> WindowManager
        where F: FnOnce(&ash::Device, &Vec<ash::vk::ImageView>, &RenderPass, &SurfaceCapabilitiesKHR) -> FrameBuffers {

            let device = &self.state.device.logical_device.raw;
            let frame_buffers = build_fn(
                &device,
                &self.state.image_views.raw,
                &self.state.render_pass,
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
                FrameBufferBuilder::new()
                    .device(device)
                    .image_views(image_views)
                    .resolution(caps.current_extent)
                    .render_pass(&render_pass.raw)
                    .build()
        })
    }
}