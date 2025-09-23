use std::{collections::HashMap, error::Error};

// use std::{collections::HashMap, error::Error};
use ash::vk::{self, CommandBuffer, DescriptorSet, QueueFlags};
use ferrum_render::{CommandPool, FrameSync, RenderContext, RenderPass, RenderPipeline};

mod render_graph;
//mod frostbite_graph;


#[derive(Default)]
pub struct RenderGraphResource {
    pub pipeline: HashMap<&'static str, RenderPipeline>,
    pub buffers: HashMap<&'static str, bool>,
    pub descriptor_set: HashMap<&'static str, DescriptorSet>,
    pub command_buffers: HashMap<u32, CommandBuffer>,
    pub command_pool: HashMap<&'static str, CommandPool>,
    pub render_pass: HashMap<&'static str, RenderPass>
}

#[derive(Default)]
pub struct RenderGraph {
    pub resources: RenderGraphResource,
    pub nodes: HashMap<&'static str, Box<dyn Fn(&mut RenderGraphResource, &RenderContext, u32) -> Result<(), Box<dyn Error>>>>,
    pub sync: Vec<FrameSync>,
    pub current_frame: usize
}

impl RenderGraph {

    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn register_descriptor_set(&mut self, name: &'static str, set: DescriptorSet) {
        self.resources.descriptor_set.insert(name, set);
    }

    pub fn register_render_pass(&mut self, name: &'static str, pass: RenderPass) {
        self.resources.render_pass.insert(name, pass);
    }

    pub fn register_command_pool(&mut self, name: &'static str, pool: CommandPool) {
        self.resources.command_pool.insert(name, pool);
    }

    pub fn register_pipeline(&mut self, name: &'static str, pipeline: RenderPipeline) {
        self.resources.pipeline.insert(name, pipeline);
    }

    pub fn add_raw_pass<F>(&mut self, name: &'static str, clojure: F)
        where F: Fn(&mut RenderGraphResource, &RenderContext, u32) -> Result<(), Box<dyn Error>> + 'static
    {
        self.nodes.insert(name, Box::new(clojure));
    }

    pub fn compile(&mut self) {

    }

    pub fn execute(&mut self, ctx: &RenderContext) {

        for (name, pass) in &self.nodes {

            if self.sync.is_empty() {
                let frame_count = ctx.window.frame_buffers.raw.len();
                for _ in 0..frame_count {
                    self.sync.push(FrameSync::new(ctx.device.raw_device()));
                }
            }

            let current_frame = self.current_frame;
            let fence = self.sync[current_frame].fence;
            let swapchain = &ctx.window.swapchain;
            let queue = ctx.device.universal_queue.raw_queue(QueueFlags::GRAPHICS);
            let device = ctx.device.raw_device();
            let sync = &self.sync;

            // 2. Дождаться завершения предыдущего кадра
            unsafe {
                device.wait_for_fences(&[fence], false, u64::MAX).unwrap();
                device.reset_fences(&[fence]).unwrap();
            }

            let (image_index, _) = unsafe {
                swapchain.swapchain_loader.acquire_next_image(
                    swapchain.raw,
                    u64::MAX,
                    sync[current_frame].image_available,
                    vk::Fence::null(),
                )
            }.unwrap();

            if let Err(err) = pass(&mut self.resources, ctx, image_index) {
                log::error!("Error in {:?} pass: {:?}", name, err);
                continue;
            }

            let binding1 = [sync[current_frame].image_available];
            let binding2 = [sync[current_frame].render_finished];
            let cbuf = &[*self.resources.command_buffers.get(&image_index).unwrap()];

            let submit_info = vk::SubmitInfo::default()
                .wait_semaphores(&binding1)
                .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
                .command_buffers(cbuf)
                .signal_semaphores(&binding2);

            unsafe {
                device.queue_submit(queue, &[submit_info], fence).unwrap();
            }

            let binding1 = [sync[current_frame].render_finished];
            let binding2 = [swapchain.raw];
            let binding3 = [image_index];

            let present_info = vk::PresentInfoKHR::default()
                .wait_semaphores(&binding1)
                .swapchains(&binding2)
                .image_indices(&binding3);

            unsafe {
                swapchain.swapchain_loader.queue_present(queue, &present_info).unwrap();
            }

            self.current_frame = (current_frame + 1) % self.sync.len();
        }
    }
}



// impl Drop for RenderGraph {
//     fn drop(&mut self) {
//         //TODO: Destroy Commands Buffers
//     }
// }