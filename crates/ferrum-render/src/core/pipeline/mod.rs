
use ash::vk;
use crate::VulkanError;

pub struct RenderPipeline {
    pub raw: vk::Pipeline,
    pub layout: vk::PipelineLayout
}

pub struct RenderPipelineBuilder<'n, S = ()> {
    state: S,
    descriptor_set_layout: Option<&'n [vk::DescriptorSetLayout]>,
    color_blend_attachment_state: Option<Vec<vk::PipelineColorBlendAttachmentState>>,
    color_blending_info: Option<vk::PipelineColorBlendStateCreateInfo<'n>>,
    vertex_input_info: Option<vk::PipelineVertexInputStateCreateInfo<'n>>,
    shader_state_infos: Option<vk::PipelineShaderStageCreateInfo<'n>>,
    input_assembly_info: Option<vk::PipelineInputAssemblyStateCreateInfo<'n>>,
    multisampling_info: Option<vk::PipelineMultisampleStateCreateInfo<'n>>,
    rasterization: Option<vk::PipelineRasterizationStateCreateInfo<'n>>,
    viewport: Option<Vec<vk::Viewport>>,
    scissors: Option<Vec<vk::Rect2D>>,
    dynamic_state: Option<Vec<vk::DynamicState>>,
    vertex_shader: Option<vk::ShaderModule>,
    fragment_shader: Option<vk::ShaderModule>
}

impl<'n> RenderPipelineBuilder<'n, ()> {

    pub fn default(extent: vk::Extent2D) -> RenderPipelineBuilder<'n, ()> {

        let color_blend_attachment = vec![vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(vk::ColorComponentFlags::RGBA)
            .blend_enable(false)
        ];

        let viewport = vk::Viewport::default()
            .x(0.0)
            .y(0.0)
            .width(extent.width as f32)
            .height(extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);

        let scissor = vk::Rect2D::default()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(vk::Extent2D {
                width: extent.width,
                height: extent.height
            }
        );

        let mut res = RenderPipelineBuilder { state: (),
            viewport: Some(vec![viewport]),
            scissors: Some(vec![scissor]),
            dynamic_state: Some(
                vec![
                    vk::DynamicState::VIEWPORT,
                    vk::DynamicState::SCISSOR
                ]
            ),
            vertex_shader: None,
            fragment_shader: None,
            descriptor_set_layout: None,
            color_blend_attachment_state: Some(color_blend_attachment),
            color_blending_info: None,
            vertex_input_info: None,
            shader_state_infos: None,
            input_assembly_info: Some(
                vk::PipelineInputAssemblyStateCreateInfo::default()
                    .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
                    .primitive_restart_enable(false)
                ),
            rasterization: Some(
                vk::PipelineRasterizationStateCreateInfo::default()
                    .depth_clamp_enable(false)
                    .rasterizer_discard_enable(false)
                    .polygon_mode(vk::PolygonMode::FILL)
                    .line_width(1.0)
                    .cull_mode(vk::CullModeFlags::NONE)
                    .front_face(vk::FrontFace::CLOCKWISE)
                    .depth_bias_enable(false)
            ),
            multisampling_info: Some(
                vk::PipelineMultisampleStateCreateInfo::default()
                    .sample_shading_enable(false)
                    .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            )
        };

        let attachments = res.color_blend_attachment_state.as_ref().unwrap().as_ptr();

        res.color_blending_info = Some(vk::PipelineColorBlendStateCreateInfo {
            logic_op_enable: vk::FALSE,
            logic_op: vk::LogicOp::COPY,
            attachment_count: 1,
            p_attachments: attachments,
            ..Default::default()
        });

        res
    }
}

pub struct WithDevice<'n> {
    pub device: &'n ash::Device
}

impl<'n> RenderPipelineBuilder<'n, ()> {
    pub fn with_device(self, device: &'n ash::Device) -> RenderPipelineBuilder<'n, WithDevice<'n>> {
        RenderPipelineBuilder {
            state: WithDevice { device },
            descriptor_set_layout: self.descriptor_set_layout,
            color_blend_attachment_state: self.color_blend_attachment_state,
            color_blending_info: self.color_blending_info,
            vertex_input_info: self.vertex_input_info,
            shader_state_infos: self.shader_state_infos,
            input_assembly_info: self.input_assembly_info,
            multisampling_info: self.multisampling_info,
            rasterization: self.rasterization,
            viewport: self.viewport,
            scissors: self.scissors,
            dynamic_state: self.dynamic_state,
            vertex_shader: self.vertex_shader,
            fragment_shader: self.fragment_shader
        }
    }
}


pub struct WithRenderPass<'n> {
    pub device: &'n ash::Device,
    pub render_pass: &'n vk::RenderPass
}

impl<'n> RenderPipelineBuilder<'n, WithDevice<'n>> {
    pub fn with_render_pass(self, render_pass: &'n vk::RenderPass) -> RenderPipelineBuilder<'n, WithRenderPass<'n>> {
        RenderPipelineBuilder {
            state: WithRenderPass { device: self.state.device, render_pass },
            descriptor_set_layout: self.descriptor_set_layout,
            color_blend_attachment_state: self.color_blend_attachment_state,
            color_blending_info: self.color_blending_info,
            vertex_input_info: self.vertex_input_info,
            shader_state_infos: self.shader_state_infos,
            input_assembly_info: self.input_assembly_info,
            multisampling_info: self.multisampling_info,
            rasterization: self.rasterization,
            viewport: self.viewport,
            scissors: self.scissors,
            dynamic_state: self.dynamic_state,
            vertex_shader: self.vertex_shader,
            fragment_shader: self.fragment_shader
        }
    }
}


impl<'n> RenderPipelineBuilder<'n, WithRenderPass<'n>> {

    pub fn with_vertex_shader(mut self, vertex_shader: vk::ShaderModule) -> Self {
        self.vertex_shader = Some(vertex_shader);
        self
    }

    pub fn with_fragment_shader(mut self, fragment_shader: vk::ShaderModule) -> Self {
        self.fragment_shader = Some(fragment_shader);
        self
    }

    pub fn with_vertex_input(mut self, input: vk::PipelineVertexInputStateCreateInfo<'n>) -> Self {
        self.vertex_input_info = Some(input);
        self
    }

    pub fn with_descriptor_set_layouts(mut self, layout: &'n [vk::DescriptorSetLayout]) -> Self {
        self.descriptor_set_layout = Some(layout);
        self
    }
}


use crate::VulkanResult;

impl<'n> RenderPipelineBuilder<'n, WithRenderPass<'n>> {
    pub fn build(self) -> VulkanResult<RenderPipeline> {

        let mut shader_states_infos = vec![];

        if let Some(vertex) = self.vertex_shader {
            shader_states_infos.push(
                vk::PipelineShaderStageCreateInfo::default()
                    .module(vertex)
                    .name(c"main")
                    .stage(vk::ShaderStageFlags::VERTEX)
            )
        }

        if let Some(fragment) = self.fragment_shader {
            shader_states_infos.push(
                vk::PipelineShaderStageCreateInfo::default()
                    .module(fragment)
                    .name(c"main")
                    .stage(vk::ShaderStageFlags::FRAGMENT)
            )
        }

        let vertex_input_info = self.vertex_input_info.unwrap_or(vk::PipelineVertexInputStateCreateInfo::default());
        let input_assembly_info = self.input_assembly_info.unwrap();

        let viewport = self.viewport.ok_or(VulkanError::Unknown)?;
        let scissors = self.scissors.ok_or(VulkanError::Unknown)?;

        let viewport_info = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewport)
            .scissors(&scissors);

        let rasterizer_info= self.rasterization.ok_or(VulkanError::Unknown)?;
        let multisampling_info = self.multisampling_info.ok_or(VulkanError::Unknown)?;
        let color_blending_info = self.color_blending_info.ok_or(VulkanError::Unknown)?;
        let binding = self.descriptor_set_layout.unwrap_or(&[]);

        let layout_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&binding);

        let pipeline_layout = unsafe { self.state.device.create_pipeline_layout(&layout_info, None).unwrap() };

        let mut dynamic_state_info_opt = None;
        if let Some(dynamic_states) = self.dynamic_state.as_ref() {
            dynamic_state_info_opt = Some(
                vk::PipelineDynamicStateCreateInfo::default()
                    .dynamic_states(dynamic_states)
            );
        }

        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false);

        let mut pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .depth_stencil_state(&depth_stencil_state)
            .stages(&shader_states_infos)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly_info)
            .viewport_state(&viewport_info)
            .rasterization_state(&rasterizer_info)
            .multisample_state(&multisampling_info)
            .color_blend_state(&color_blending_info)
            .layout(pipeline_layout)
            .render_pass(*self.state.render_pass);

        if let Some(ref dynamic_state_info) = dynamic_state_info_opt {
            pipeline_info = pipeline_info.dynamic_state(dynamic_state_info);
        }

        let pipeline = unsafe {
            self.state.device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    std::slice::from_ref(&pipeline_info),
                    None,
                )
                .map_err(|e| VulkanError::Unknown)?[0]
        };

        Ok(RenderPipeline {
            layout: pipeline_layout,
            raw: pipeline
        })
    }
}