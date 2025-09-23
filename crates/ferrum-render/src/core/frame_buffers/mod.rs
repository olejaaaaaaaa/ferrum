

// pub mod frame_buffers;
// pub use frame_buffers::*;

pub struct FrameBuffers {
    pub raw: Vec<ash::vk::Framebuffer>
}

#[derive(Default)]
pub struct FrameBuffersBuilder<S> {
    state: S,
    allocation_callbacks: Option<&'static ash::vk::AllocationCallbacks<'static>>,
    depth_image_view: Option<vk::ImageView>,
}


impl FrameBuffersBuilder<()> {
    pub fn default() -> FrameBuffersBuilder<()> {
        FrameBuffersBuilder {
            state: (),
            allocation_callbacks: None,
            depth_image_view: None
        }
    }
}

use ash::vk::{self, Image};

use crate::{ImageViewsBuilder, VulkanError};

pub struct WithDevice<'n> {
    pub device: &'n ash::Device
}

impl<'n> FrameBuffersBuilder<()> {
    pub fn with_device(self, device: &'n ash::Device) -> FrameBuffersBuilder<WithDevice<'n>> {
        FrameBuffersBuilder {
            state: WithDevice { device },
            allocation_callbacks: self.allocation_callbacks,
            depth_image_view: None
        }
    }
}

pub struct WithRenderPass<'n> {
    pub device: &'n ash::Device,
    pub render_pass: &'n vk::RenderPass
}

impl<'n> FrameBuffersBuilder<WithDevice<'n>> {
    pub fn with_render_pass(self, render_pass: &'n vk::RenderPass) -> FrameBuffersBuilder<WithRenderPass<'n>> {
        FrameBuffersBuilder {
            state: WithRenderPass {
                device: self.state.device,
                render_pass
            },
            allocation_callbacks: self.allocation_callbacks,
            depth_image_view: None
        }
    }
}

pub struct WithExtent<'n> {
    pub device: &'n ash::Device,
    pub render_pass: &'n vk::RenderPass,
    pub extent: vk::Extent2D
}

impl<'n> FrameBuffersBuilder<WithRenderPass<'n>> {
    pub fn with_extent(self, extent: vk::Extent2D) -> FrameBuffersBuilder<WithExtent<'n>> {
        FrameBuffersBuilder {
            state: WithExtent {
                device: self.state.device,
                render_pass: self.state.render_pass,
                extent
            },
            allocation_callbacks: self.allocation_callbacks,
            depth_image_view: None
        }
    }
}

pub struct WithImageViews<'n> {
    pub device: &'n ash::Device,
    pub render_pass: &'n vk::RenderPass,
    pub extent: vk::Extent2D,
    pub image_views: &'n [vk::ImageView]
}

impl<'n> FrameBuffersBuilder<WithExtent<'n>> {
    pub fn with_image_views(self, image_views: &'n [vk::ImageView]) -> FrameBuffersBuilder<WithImageViews<'n>> {
        FrameBuffersBuilder {
            state: WithImageViews {
                device: self.state.device,
                render_pass: self.state.render_pass,
                extent: self.state.extent,
                image_views
            },
            allocation_callbacks: self.allocation_callbacks,
            depth_image_view: None
        }
    }
}

use crate::{VulkanResult};


impl<'n> FrameBuffersBuilder<WithImageViews<'n>> {

    pub fn with_depth_image(&mut self, depth_image: ash::vk::ImageView) {
        self.depth_image_view = Some(depth_image);
    }

    pub fn build(self) -> VulkanResult<FrameBuffers> {

        let mut frame_buffers = vec![];

        unsafe {

            for i in self.state.image_views.iter() {

                let mut image_view = vec![*i];

                // if let Some(x) = self.depth_image_view {
                //     image_view.push(x);
                // }

                let create_info = ash::vk::FramebufferCreateInfo::default()
                    .render_pass(*self.state.render_pass)
                    .attachments(&image_view)
                    .attachment_count(1)
                    .width(self.state.extent.width)
                    .height(self.state.extent.height)
                    .layers(1);

                let frame = self.state.device.create_framebuffer(&create_info, self.allocation_callbacks).map_err(|x| VulkanError::Unknown)?;
                frame_buffers.push(frame);
            }

            Ok(FrameBuffers { raw: frame_buffers })
        }
    }
}