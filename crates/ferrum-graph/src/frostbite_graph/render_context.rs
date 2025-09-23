use std::collections::HashMap;
use ash::vk;
use ferrum_render::GPUBuffer;
use ferrum_render::RenderPipeline;
use ferrum_render::Texture;
use vk_mem::Allocator;

type UniformBuffer = GPUBuffer;
type StorageBuffer = GPUBuffer;

pub struct RenderContext<'c> {

    ctx: &'c ferrum_render::RenderContext,

    allocator: vk_mem::Allocator,

    command_pool: Option<vk::CommandPool>,
    current_command_buffer: Option<vk::CommandBuffer>,

    pipelines: HashMap<u64, RenderPipeline>,
    framebuffers: HashMap<u64, vk::Framebuffer>,

    current_pipeline: Option<RenderPipeline>,
    rendering_started: bool,
}

impl<'c> RenderContext<'c> {

    pub fn new(ctx: &'c ferrum_render::RenderContext, alloc: Allocator) -> Self {
        Self {
            ctx,
            allocator: alloc,
            command_pool: todo!(),
            current_command_buffer: todo!(),
            pipelines: todo!(),
            framebuffers: todo!(),
            current_pipeline: todo!(),
            rendering_started: todo!()
        }
    }

    pub fn begin_rendering(&self) {

    }

    pub fn end_rendering(&self)   {

    }

    pub fn create_graphics_program(&self) {

    }

    pub fn create_vertex_buffer(&self, ) {
        
    }

    pub fn create_index_buffer(&self) {

    }

    pub fn create_texture2D(&self) {

    }

    pub fn create_texture3D(&self) {

    }

    pub fn create_sampler() {

    }

    pub fn clear_texture() {

    }

    pub fn get_swapchain_size(&self) -> (u32, u32) {
        let extent = self.ctx.window.caps.current_extent;
        (extent.width, extent.height)
    }

    pub fn draw(vertex: Option<GPUBuffer>, indeces: Option<GPUBuffer>, info: GeometryInfo, num_instance: u32) {

    }

    pub fn drawCube(&self) {
        //self.draw();
    }
    pub fn drawFullScreenTriangle(&self) {
        //self.draw();
    }

    pub fn bind_texture(&self, bind: u32, texture: &Texture) {

    }

    pub fn bind_image(&self) {

    }

    pub fn bind_storage_buffer(&self) {

    }

    pub fn bind_uniform_buffer(&self) {

    }
    pub fn set_viewport(&self) {

    }

    pub fn set_uniform1f(&self) {

    }

    pub fn set_uniform_vec3(&self) {

    }

    pub fn set_graphics_pipeline(&self) {

    }

    pub fn setup_sampler(&self)  {

    }
}

pub struct GeometryInfo {
    pub topology: ash::vk::PrimitiveTopology,
    pub vertex_offset: u32,
    pub num_vertices: u32,
    pub index_offset: u32,
    pub num_indices: u32
}
