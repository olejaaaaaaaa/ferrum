
use crate::{render_pass::subpass::Subpass, VulkanError::{self, *}, VulkanResult};

mod device;
mod subpass;
mod subpass_dep;
mod subpass_desc;
mod attachment_desc;

pub struct RenderPass {
    pub raw: ash::vk::RenderPass,
    pub subpass: Vec<Subpass>
}

pub struct RenderPassBuilder<S = ()> {
    state: S,
    attachments: Vec<ash::vk::AttachmentDescription>,
    dependencies: Vec<ash::vk::SubpassDependency>,
    subpass: Vec<Subpass>,
    allocation_callback: Option<&'static ash::vk::AllocationCallbacks<'static>>
}

use ash::vk;
use subpass::SubpassBuilder;


impl RenderPassBuilder<()> {

    pub fn new() -> RenderPassBuilder<()> {
        RenderPassBuilder {
            state: (),
            attachments: vec![],
            dependencies: vec![],
            subpass: vec![],
            allocation_callback: None
        }
    }

    pub fn default(format: ash::vk::Format) -> RenderPassBuilder<()> {

        let subpass = SubpassBuilder::new()
            .add_color_attachment_ref(
                vk::AttachmentReference::default()
                    .attachment(0)
                    .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            )
            .with_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .build();

        RenderPassBuilder {
            state: (),
            attachments: vec![
                vk::AttachmentDescription {
                    flags: vk::AttachmentDescriptionFlags::empty(),
                    format: format,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    store_op: vk::AttachmentStoreOp::STORE,
                    stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                    stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
                    initial_layout: vk::ImageLayout::UNDEFINED,
                    final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                },
            ],
            dependencies: vec![
                vk::SubpassDependency {
                    src_subpass: vk::SUBPASS_EXTERNAL,
                    dst_subpass: 0,
                    src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
                    dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
                    src_access_mask: vk::AccessFlags::empty(),
                    dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
                    dependency_flags: vk::DependencyFlags::BY_REGION,
                }
            ],
            subpass: vec![
                subpass
            ],
            allocation_callback: None
        }
    }
}

use device::WithDevice;

impl<'n> RenderPassBuilder<WithDevice<'n>> {

    pub fn build(self) -> VulkanResult<RenderPass> {

        let mut subpasses = vec![];

        for i in &self.subpass {
            subpasses.push(i.raw);
        }

        let create_info = vk::RenderPassCreateInfo::default()
            .attachments(&self.attachments)
            .subpasses(&subpasses)
            .dependencies(&self.dependencies);

        let render_pass = unsafe {
             self.state.device.create_render_pass(&create_info, self.allocation_callback).map_err(|e| {
                VulkanError::Unknown
             })?
        };

        Ok(RenderPass { raw: render_pass, subpass: self.subpass })
    }
}





