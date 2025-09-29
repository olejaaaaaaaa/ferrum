#![warn(unused_qualifications)]

use std::{
    cell::RefCell, collections::HashMap, error::Error, ffi::CStr, fs::{read_dir, write, DirEntry, File}, io::Read, mem::{offset_of, ManuallyDrop}, panic, path::{Path, PathBuf}, process::Command, rc::Rc, sync::Arc, time::{Duration, Instant}, u64
};

use ash::{
    khr::uniform_buffer_standard_layout,
    vk::{
        self, AttachmentReference, BufferUsageFlags, ClearValue, CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, DescriptorType, Extent2D, Extent3D, Fence, FenceCreateFlags, FenceCreateInfo, Format, Handle, IndexType, MemoryPropertyFlags, Offset2D, PhysicalDeviceType, PipelineBindPoint, PipelineVertexInputStateCreateInfo, PresentModeKHR, PrimitiveTopology, QueueFlags, Rect2D, ShaderStageFlags, SurfaceFormatKHR, VertexInputAttributeDescription, VertexInputBindingDescription, Viewport, API_VERSION_1_0, API_VERSION_1_3
    },
};

use egui::TextureId;
use ferrum_assets::load_gltf;
use ferrum_graph::RenderGraph;
use ferrum_render::{
   CommandPoolBuilder,
   RenderContext,
   RenderPipelineBuilder,
};
use ferrum_resources::DescriptorManager;
use log::*;
use winit::{dpi::PhysicalSize, raw_window_handle::HasDisplayHandle};
use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopWindowTarget},
    raw_window_handle::*,
    window::{self, Window},
};

use ferrum_types::{AttributeDescriptions, BindingDescriptions, PBRVertex, Vertex};
use ferrum_render::ShaderProgramBuilder;
use ferrum_ui::*;

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
struct UniformBufferObject {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}


fn rotation_matrix(angle_rad: f32, axis: [f32; 3]) -> [[f32; 4]; 4] {
    let (sin, cos) = angle_rad.sin_cos();
    let [x, y, z] = {
        let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        [axis[0]/len, axis[1]/len, axis[2]/len] // Нормализуем ось
    };

    [
        [cos + x*x*(1.0-cos),    x*y*(1.0-cos) - z*sin, x*z*(1.0-cos) + y*sin, 0.0],
        [y*x*(1.0-cos) + z*sin,  cos + y*y*(1.0-cos),   y*z*(1.0-cos) - x*sin, 0.0],
        [z*x*(1.0-cos) - y*sin,  z*y*(1.0-cos) + x*sin, cos + z*z*(1.0-cos),   0.0],
        [0.0,                    0.0,                    0.0,                  1.0]
    ]
}

use mlua::prelude::*;


struct ScriptManager {
    scripts: HashMap<PathBuf, String>,
    lua: Lua
}

impl ScriptManager {
    pub fn new() -> Self {
        let lua = Lua::new();
        Self {
            scripts: HashMap::new(),
            lua
        }
    }

    pub fn update(&self) {
        for (path, script_text) in &self.scripts {
            let lua = &self.lua;
            let res = lua.load(script_text).exec();
            if let Err(x) = res {
                tracing::error!("Error execute lua script: {:?} with Error: {:?}", path.as_path(), x);
            }
        }
    }

    pub fn load_script(&mut self, path: PathBuf) {
        let mut file = File::open(&path);

        if let Ok(mut file) = file {
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            self.scripts.insert(path, text);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    unsafe { std::env::set_var("RUST_LOG", "DEBUG") };

    tracing_subscriber::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        .with_line_number(false)
        .with_timer(false)
        .with_file(false)
        .with_target(false)
        .event_format(tracing_subscriber::fmt::format().pretty())
        .init();

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_title("Game")
        .build(&main_loop)
        .unwrap();

    let mut scripts = ScriptManager::new();

    // let egui_ctx = egui::Context::default();
    // egui_extras::install_image_loaders(&egui_ctx);

    // let mut egui_winit = RefCell::new(egui_winit::State::new(
    //     egui_ctx.clone(),
    //     egui::ViewportId::ROOT,
    //     &window,
    //     None,
    //     None,
    //     None
    // ));

    let mut ctx = RenderContext::default(window);

    let smpler = ctx.create_default_sampler().unwrap();
    // let mut ui: Arc<RefCell<Renderer>> = Arc::new(RefCell::new(Renderer::with_default_allocator(
    //     ctx.device.raw_instance(),
    //     ctx.device.phys_dev.raw,
    //     ctx.device.logical_device.raw.clone().as_ref().clone(),
    //     ctx.window.render_pass.raw,
    //     Options {
    //         in_flight_frames: ctx.window.frame_buffers.raw.len(),
    //         enable_depth_test: false,
    //         enable_depth_write: false,
    //         srgb_framebuffer: true
    //     }
    // ).unwrap()));

    let model = load_gltf(&ctx, "../../shared/assets/models/girl.glb");

    let shader_program = ShaderProgramBuilder::new()
        .with_device(ctx.device.logical_device.raw.clone())
        .with_vertex_shader(r"../../shared/shaders/spv/triangle-vert.spv")
        .with_fragment_shader(r"../../shared/shaders/spv/triangle-frag.spv")
        .build()
        .unwrap();

    let mut ubo = UniformBufferObject {

        model: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],

        view: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],

        projection: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    let uniform_buffer = ctx.create_dynamic_buffer(BufferUsageFlags::UNIFORM_BUFFER, &[ubo]).unwrap();

    let f = vk::DescriptorBufferInfo::default()
        .buffer(uniform_buffer.raw)
        .offset(0)
        .range(size_of_val(&ubo) as u64);

    let mut m = DescriptorManager::new(ctx.device.clone());
    let index = m.builder()
        .with(0, DescriptorType::UNIFORM_BUFFER, ShaderStageFlags::VERTEX, &f)
        .send()
        .unwrap();

    let set = m.get_descriptor_set(index).unwrap();
    let lay = m.get_layout(index).unwrap();

    let pipeline = RenderPipelineBuilder::default(ctx.window.caps.current_extent)
        .with_device(ctx.device.logical_device.raw.clone())
        .with_render_pass(&ctx.window.render_pass.raw)
        .with_vertex_shader(shader_program.vertex_shader)
        .with_fragment_shader(shader_program.fragment_shader)
        .with_descriptor_set_layouts(&[lay])
        .with_vertex_input(
            PipelineVertexInputStateCreateInfo::default()
                .vertex_attribute_descriptions(&PBRVertex::attr_desc())
                .vertex_binding_descriptions(&PBRVertex::bind_desc()),
        )
        .build()
        .expect("Error create pipeline");

    let command_pool = CommandPoolBuilder::new()
        .device(ctx.device.logical_device.raw.clone())
        .family_index(0)
        .build()
        .unwrap();

    let mut textures_to_free: Option<Vec<TextureId>> = None;

    let mut graph = RenderGraph::new();
    graph.register_command_pool("pool", command_pool);
    graph.add_raw_pass("Simple", move |res, ctx, image_index| {
        let device = ctx.device.raw_device();
        let command_pool = res.command_pool.get("pool").ok_or("ERR")?;

        let current_extent = ctx.window.caps.current_extent;

        let command_buffer = if let Some(cbuf) = res.command_buffers.get(&image_index) {
            *cbuf
        } else {
            let command_buffer =
                command_pool.create_command_buffers(1, CommandBufferLevel::PRIMARY).unwrap().raw[0];
            res.command_buffers.insert(image_index, command_buffer);
            command_buffer
        };

        let render_pass = &ctx.window.render_pass;
        let frame_buffer = ctx.window.frame_buffers.raw[image_index as usize];

        let clear_values = [

            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [5.0 / 255.0, 5.0 / 255.0, 5.0 / 255.0, 1.0],
                },
            },

            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },

        ];

        let render_pass_begin_info = vk::RenderPassBeginInfo::default()
            .render_pass(render_pass.raw)
            .framebuffer(frame_buffer)
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: current_extent,
            })
            .clear_values(&clear_values);

        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

        unsafe {
            device.begin_command_buffer(command_buffer, &begin_info)?;

            let viewport = vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: ctx.window.caps.current_extent.width as f32,
                height: ctx.window.caps.current_extent.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            };

            let scissor = vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: ctx.window.caps.current_extent,
            };

            device.cmd_set_viewport(command_buffer, 0, &[viewport]);
            device.cmd_set_scissor(command_buffer, 0, &[scissor]);

            device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );

            // let raw_input = egui::RawInput::default();

            // let egui::FullOutput {
            //     platform_output,
            //     textures_delta,
            //     shapes,
            //     pixels_per_point,
            //     ..
            // } = egui_ctx.run(raw_input, |ctx| {
            //     egui::CentralPanel::default().show(&ctx, |ui: &mut egui::Ui| {
            //         ui.label("Hello world!");
            //         if ui.button("Click me").clicked() {
            //             // take some action here
            //         }
            //     });
            // });

            //egui_winit.borrow_mut().handle_platform_output(&ctx.window.raw, platform_output);

            // if !textures_delta.free.is_empty() {
            //     //textures_to_free = Some(textures_delta.free.clone());
            // }

            // if !textures_delta.set.is_empty() {
            //     ui.borrow_mut()
            //         .set_textures(
            //             ctx.device.universal_queue.raw_queue(vk::QueueFlags::GRAPHICS),
            //             command_pool.raw,
            //             textures_delta.set.as_slice(),
            //         )
            //         .expect("Failed to update texture");
            // }

            // let clipped_primitives = egui_ctx.tessellate(shapes, pixels_per_point);
            // ui.try_borrow_mut().unwrap().cmd_draw(
            //     command_buffer,
            //     current_extent,
            //     1.0,
            //     clipped_primitives.as_slice()
            // ).unwrap();

            device.cmd_bind_pipeline(command_buffer, PipelineBindPoint::GRAPHICS, pipeline.raw);
            device.cmd_bind_descriptor_sets(command_buffer, PipelineBindPoint::GRAPHICS, pipeline.layout, 0, &[set.raw], &[]);

            for mesh in &model.meshes {

                device.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[mesh.primitive.vertex_buffer.raw],
                    &[0]
                );

                if mesh.primitive.indices.len() > 0 {
                    device.cmd_bind_index_buffer(
                        command_buffer,
                        mesh.primitive.index_buffer.raw,
                        0,
                        vk::IndexType::UINT32
                    );

                    device.cmd_draw_indexed(
                        command_buffer,
                        mesh.primitive.indices.len() as u32,
                        1,
                        0,
                        0,
                        0
                    );
                } else {

                    device.cmd_draw(
                        command_buffer, 
                        mesh.primitive.vertices.len() as u32,
                        1,
                        0, 
                        0
                    );
                }
            }



            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer)?;
        }

        Ok(())
    });

    let mut dt = Instant::now();
    let mut global_time = Instant::now();
    let mut count_frame = 0;

    let mut angle = 0.0f32;
    let rotation_speed = 0.0001;

    let _ = main_loop.run(|ev, ev_window| match ev {
        winit::event::Event::WindowEvent {
            window_id: _,
            event,
        } => match event {
            winit::event::WindowEvent::KeyboardInput { event, .. } => match event {
                _ => {}
            },

            winit::event::WindowEvent::DroppedFile(path) => {
                scripts.load_script(path);
            }

            winit::event::WindowEvent::MouseWheel { delta, .. } => match delta {
                winit::event::MouseScrollDelta::LineDelta(_x, y) => {
                    ubo.view[3][2] -= y/10.0;
                    uniform_buffer.update_buffer(&[ubo]);
                }
                _ => {}
            },
            winit::event::WindowEvent::CloseRequested => ev_window.exit(),
            winit::event::WindowEvent::RedrawRequested => {

                scripts.update();

                angle += rotation_speed * global_time.elapsed().as_millis() as f32;
                global_time = Instant::now();

                if dt.elapsed().as_secs_f32() >= 1.0 {
                    if count_frame < 58 {
                        tracing::warn!("Low FPS: {}", count_frame);
                    }
                    dt = Instant::now();
                    count_frame = 0;
                } else {
                    count_frame += 1;
                }

                graph.execute(&ctx);

                ubo.model = rotation_matrix(angle, [0.0 , 1.0, 0.0]);
                uniform_buffer.update_buffer(&[ubo]);
            }
            winit::event::WindowEvent::Resized(size) => {

                let dev = ctx.device.clone();
                ctx.window.resize(&dev, size.width, size.height);

                let fov = std::f32::consts::PI / 3.0;
                let aspect = size.width as f32 / size.height as f32;
                let near = 0.01;
                let far = 100.0;

                let f = 1.0 / (fov / 2.0).tan();

                ubo.projection = [
                    [f / aspect, 0.0, 0.0, 0.0],
                    [0.0, -f, 0.0, 0.0],
                    [0.0, 0.0, far / (far - near), 1.0],
                    [0.0, 0.0, -(far * near) / (far - near), 0.0],
                ];

                uniform_buffer.update_buffer(&[ubo]);
            }
            _ => {}
        },
        winit::event::Event::AboutToWait => {
            ctx.window.raw.request_redraw();
        }
        _ => {}
    });

    unsafe { ctx.device.raw_device().device_wait_idle() };

    Ok(())
}
