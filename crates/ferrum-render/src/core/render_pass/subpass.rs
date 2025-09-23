
use ash::vk;

#[derive(Default)]
pub struct Subpass {
    pub raw: ash::vk::SubpassDescription<'static>,
    p_color_attachment_ref: *const vk::AttachmentReference,
    p_depth_attachment_ref: *const vk::AttachmentReference,
    // p_resolve_attachments: *const AttachmentReference,
    // p_preserve_attachments: *const AttachmentReference
}


#[derive(Default)]
pub struct SubpassBuilder {
    color_attachment_ref: Vec<vk::AttachmentReference>,
    depth_attachment_ref: Option<vk::AttachmentReference>,
    bind_point: Option<vk::PipelineBindPoint>,
    flags: Option<vk::SubpassDescriptionFlags>
}

impl SubpassBuilder {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn add_color_attachment_ref(mut self, color_attachment_ref: vk::AttachmentReference) -> Self {
        self.color_attachment_ref.push(color_attachment_ref);
        self
    }

    pub fn with_bind_point(mut self, bind_point: vk::PipelineBindPoint) -> Self {
        self.bind_point = Some(bind_point);
        self
    }

    pub fn add_depth_attachment_ref(mut self, depth_attachment_ref: vk::AttachmentReference) -> Self {
        self.depth_attachment_ref = Some(depth_attachment_ref);
        self
    }

    pub fn build(self) -> Subpass {

        let bind_point = self.bind_point.unwrap();
        let mut subpass = Subpass::default();

        subpass.raw.pipeline_bind_point = bind_point;

        if let Some(bind_point) = self.bind_point {
            subpass.raw.pipeline_bind_point = bind_point;
        }
        if let Some(flags) = self.flags {
            subpass.raw.flags = flags;
        }

        if !self.color_attachment_ref.is_empty() {
            subpass.p_color_attachment_ref = self.color_attachment_ref.as_ptr();
            subpass.raw.color_attachment_count = self.color_attachment_ref.len() as u32;
            subpass.raw.p_color_attachments = subpass.p_color_attachment_ref;
        }

        if let Some(depth) = self.depth_attachment_ref {
            subpass.p_depth_attachment_ref = Box::into_raw(Box::new(depth));
            subpass.raw.p_depth_stencil_attachment = subpass.p_depth_attachment_ref;
        }

        std::mem::forget(self.color_attachment_ref);

        subpass

    }

}