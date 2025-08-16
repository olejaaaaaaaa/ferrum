#![warn(unused_qualifications)]

use std::{collections::HashMap, error::Error, ffi::CStr, fs::{read_dir, write, DirEntry, File}, io::Read, mem::offset_of, panic, path::Path, process::Command, rc::Rc, sync::Arc, time::Instant, u64};

use ash::vk::{self, AttachmentReference, BufferUsageFlags, CommandBuffer, CommandBufferLevel, Extent2D, Extent3D, Fence, FenceCreateFlags, Format, PhysicalDeviceType, PipelineBindPoint, PresentModeKHR, PrimitiveTopology, SurfaceFormatKHR, VertexInputAttributeDescription, VertexInputBindingDescription, API_VERSION_1_0, API_VERSION_1_3};

use ferrum_assets::*;
use ferrum_render::*;
use ferrum_graph::*;

use winit::raw_window_handle::*;
use log::*;
use winit::{dpi::PhysicalSize, raw_window_handle::HasDisplayHandle};


#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

pub fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}

impl Vertex {

    fn get_binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Self>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }]
    }

    fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Self, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Self, color) as u32,
            },
        ]
    }
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


#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
struct UniformBufferObject {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}





fn main() -> Result<(), Box<dyn Error>> {

    unsafe { std::env::set_var("RUST_LOG", "DEBUG") };
    env_logger::init();

    let main_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_title("Game")
        .build(&main_loop)
        .unwrap();

    let mut ctx: RenderContext = RenderContext::default(window);


    //------
    // Загрузка изображения shared\assets\texture\texture0.jpg

    let image = image::open(r"..\..\shared\assets\texture\texture0.jpg")
        .expect("Error open image")
        .to_rgba8();

    let (image_width, image_height) = image.dimensions();

    let image_bytes = image.into_raw();

    let image_buffer = GPUBuffer::new(
        ctx.device.raw_device(),
        &ctx.device.phys_dev.phys_info.memory_prop,
        (size_of::<u8>() * image_bytes.len()) as u64,
         vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    ).unwrap();

    image_buffer.upload_data(ctx.device.raw_device(), &image_bytes);

    let texture = Texture::new(ctx.device.raw_device(), Extent3D {
        width: image_width,
        height: image_height,
        depth: 1
    }, ctx.window.surface_format_khr.format);

    let mem_requirements = unsafe {
        ctx.device.raw_device().get_image_memory_requirements(texture.raw)
    };

    let texture_image_memory = unsafe {
        ctx.device.raw_device().allocate_memory(
            &vk::MemoryAllocateInfo::default()
                .allocation_size(mem_requirements.size)
                .memory_type_index(
                    find_memorytype_index(
                        &mem_requirements,
                        &ctx.device.phys_dev.phys_info.memory_prop,
                        vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    )
                    .expect("Failed to find suitable memory type"),
                ),
            None,
        )
        .expect("Failed to allocate image memory")
    };

    unsafe {
        ctx.device.raw_device().bind_image_memory(
            texture.raw,
            texture_image_memory,
            0,
        )
        .expect("Failed to bind image memory");
    }


    let image_view = unsafe {
        ctx.device.raw_device().create_image_view(
            &vk::ImageViewCreateInfo::default()
                .image(texture.raw)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(ctx.window.surface_format_khr.format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                }),
            None,
        )
        .expect("Failed to create image view")
    };

    let sampler = unsafe {
        ctx.device.raw_device().create_sampler(
            &vk::SamplerCreateInfo::default()
                .mag_filter(vk::Filter::LINEAR)
                .min_filter(vk::Filter::LINEAR)
                .address_mode_u(vk::SamplerAddressMode::REPEAT)
                .address_mode_v(vk::SamplerAddressMode::REPEAT)
                .address_mode_w(vk::SamplerAddressMode::REPEAT)
                .anisotropy_enable(false)
                .max_anisotropy(1.0)
                .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
                .unnormalized_coordinates(false)
                .compare_enable(false)
                .compare_op(vk::CompareOp::ALWAYS)
                .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
                .mip_lod_bias(0.0)
                .min_lod(0.0)
                .max_lod(0.0),
            None,
        )
        .expect("Failed to create sampler")
    };


    //------


    let buffer_size = size_of::<UniformBufferObject>() as u64;

    let uniform_buffer = GPUBuffer::new(
        &ctx.device.logical_device.raw,
        &ctx.device.phys_dev.phys_info.memory_prop,
        buffer_size,
        vk::BufferUsageFlags::UNIFORM_BUFFER,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    ).unwrap();

    let mut ubo = UniformBufferObject {

        model: [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],

        view: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 5.0, 1.0],
        ],

        projection: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    uniform_buffer.upload_data(&ctx.device.logical_device.raw, &[ubo]);

    let layout = DescriptorSetLayoutBuilder::new()
        .with_device(ctx.device.raw_device())
        .with_bindings(&[
            vk::DescriptorSetLayoutBinding::default()
                .binding(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::VERTEX),

            vk::DescriptorSetLayoutBinding::default()
                .binding(1)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            ]
        )
        .build();

    let descriptor_pool = DescriptorPoolBuilder::new()
        .with_device(ctx.device.raw_device())
        .with_max_sets(1)
        .with_pool_sizes(&[
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(1),

            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
        ])
        .build();

    // Выделяем Descriptor Set
    let layout = std::slice::from_ref(&layout.raw);
    let allocate_info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(descriptor_pool.raw)
        .set_layouts(layout);

    let descriptor_sets = unsafe {
        ctx.device.logical_device.raw
            .allocate_descriptor_sets(&allocate_info)
            .unwrap()
    };

    let descriptor_set = descriptor_sets[0];

    let buffer_info = [vk::DescriptorBufferInfo::default()
        .buffer(uniform_buffer.raw)
        .offset(0)
        .range(buffer_size)
    ];

    let image_info = [vk::DescriptorImageInfo {
        sampler,           // Ваш созданный sampler
        image_view,        // Ваш созданный image_view
        image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    }];

    let write_descriptor1 = vk::WriteDescriptorSet::default()
        .dst_set(descriptor_set)
        .dst_binding(0)  // Как в layout_binding
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .buffer_info(&buffer_info);

    let write_descriptor2 = vk::WriteDescriptorSet::default()
        .dst_set(descriptor_set)
        .dst_binding(1)  // Как в layout_binding
        .dst_array_element(0)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&image_info);

    unsafe {
        ctx.device.logical_device.raw
            .update_descriptor_sets(&[write_descriptor1, write_descriptor2], &[]);
    }

    let pipeline = StandartPipelineBuilder::new()
        .with_graphics_device(&ctx)
        .with_fragment_shader(load_spv(r"..\..\shared\shaders\spv\triangle-frag.spv"))
        .with_vertex_shader(load_spv(r"..\..\shared\shaders\spv\triangle-vert.spv"))
        .build(layout[0]);

    //let (data, index) = &load_model(r"..\..\shared\assets\models\cube.obj").expect("EEEER");

    let mesh = load_gltf_model(r"C:\Users\Oleja\Desktop\ferrum\shared\assets\models\girl2.glb").expect("EEEER");
    let mut data = vec![];

    for (index, _data) in mesh.positions.iter().enumerate() {
        data.push(Vertex { pos: *_data, color: mesh.colors[index] });
    }

    let index = mesh.indices;

    let command_pool = CommandPoolBuilder::new()
        .device(&ctx.device.raw_device())
        .family_index(ctx.device.universal_queue.graphics_index())
        .build();

    let gpu_buffer = GPUBuffer::new(
        &ctx.device.raw_device(),
        &ctx.device.phys_dev.phys_info.memory_prop,
        (size_of::<Vertex>() * data.len()) as u64,
        BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    ).unwrap();

    gpu_buffer.upload_data(ctx.device.raw_device(), &data);

    let index_buffer = GPUBuffer::new(
        &ctx.device.raw_device(),
        &ctx.device.phys_dev.phys_info.memory_prop,
        (std::mem::size_of::<u32>() * index.len()) as u64,
        BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    ).unwrap();

    index_buffer.upload_data(ctx.device.raw_device(), &index);


    info!("SIZE: {}", size_of::<Arc<RenderContext>>());
    //------------------------------
    let mut graph = RenderGraph::new();
    graph.register_command_pool("pool", command_pool);
    graph.register_buffer("buf", gpu_buffer);
    graph.register_buffer("index_buf", index_buffer);
    graph.register_texture("image", texture);
    graph.register_buffer("image_buffer", image_buffer);
    graph.register_pipeline("pipe", pipeline);
    graph.register_descriptor_set("set", descriptor_set);

    graph.add_raw_pass("Simple", move |res, ctx, image_index| {

        let device = ctx.device.raw_device();
        let buffer = res.buffers.get("buf").ok_or("ERR")?;
        let index_buffer = res.buffers.get("index_buf").ok_or("ERR")?;
        let pipeline = res.pipeline.get("pipe").ok_or("ERR")?;
        let command_pool = res.command_pool.get("pool").ok_or("ERR")?;
        let set = res.descriptor_set.get("set").unwrap();
        let tex = res.texture.get("image").unwrap();
        let image_buffer = res.buffers.get("image_buffer").unwrap();

        let current_extent = ctx.window.caps.current_extent;

        let command_buffer = if let Some(cbuf) = res.command_buffers.get(&image_index) {
            *cbuf
        } else {
            let command_buffer = command_pool.create_command_buffers(device, 1, CommandBufferLevel::PRIMARY)[0];
            res.command_buffers.insert(image_index, command_buffer);
            command_buffer
        };

        let render_pass = &ctx.window.render_pass;
        let frame_buffer = ctx.window.frame_buffers.raw[image_index as usize];

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [5.0/255.0, 5.0/255.0, 5.0/255.0, 1.0],
            },
        }];

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



            // /////

            let barrier = vk::ImageMemoryBarrier::default()
                    .old_layout(vk::ImageLayout::UNDEFINED)
                    .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .image(tex.raw)
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    });

                let src_stage;
                let dst_stage;

                match (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) {
                    (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => {
                        barrier.src_access_mask(vk::AccessFlags::empty());
                        barrier.dst_access_mask(vk::AccessFlags::TRANSFER_WRITE);
                        src_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
                        dst_stage = vk::PipelineStageFlags::TRANSFER;
                    }
                    (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => {
                        barrier.src_access_mask(vk::AccessFlags::TRANSFER_WRITE);
                        barrier.dst_access_mask(vk::AccessFlags::SHADER_READ);
                        src_stage = vk::PipelineStageFlags::TRANSFER;
                        dst_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;
                    }
                    _ => panic!("Unsupported layout transition!"),
                }

            unsafe {
                ctx.device.raw_device().cmd_pipeline_barrier(
                    command_buffer,
                    src_stage,
                    dst_stage,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[barrier],
                );
            }

            /////


            // Копирование данных
            let buffer_copy = vk::BufferImageCopy::default()
                .buffer_offset(0)
                .buffer_row_length(0)
                .buffer_image_height(0)
                .image_subresource(vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .image_extent(Extent3D {
                    width: image_width,
                    height: image_height,
                    depth: 1,
                });


            unsafe {
                ctx.device.raw_device().cmd_copy_buffer_to_image(
                    command_buffer,
                    image_buffer.raw,
                    texture.raw,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[buffer_copy],
                );
            }

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

            device.cmd_bind_descriptor_sets(command_buffer, PipelineBindPoint::GRAPHICS, pipeline.raw_layout, 0, &[*set], &[]);

            device.cmd_bind_pipeline(
                command_buffer,
                PipelineBindPoint::GRAPHICS,
                pipeline.raw,
            );

            device.cmd_bind_vertex_buffers(command_buffer, 0, &[buffer.raw], &[0]);
            device.cmd_bind_index_buffer(command_buffer, index_buffer.raw, 0, vk::IndexType::UINT32);
            device.cmd_draw_indexed(
                command_buffer,
                index.len() as u32,
                1,
                0,
                0,
                0
            );

            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer)?;
        }

        Ok(())
    });

    let mut global_time = Instant::now();
    let mut time = Instant::now();
    let mut count_frame = 0;

    let mut angle = 0.0f32; // Угол в радианах
    let rotation_speed = 0.0001; // Скорость вращения (рад/сек)

    let _ = main_loop.run(move |ev, ev_window| {
    match ev {
        winit::event::Event::WindowEvent { window_id: _, event } => match event {
            winit::event::WindowEvent::KeyboardInput { event, .. } => {
                match event {
                    _ => {}
                }
            },

            winit::event::WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_x, y) => {
                        ubo.view[3][2] -= y/10.0;
                        uniform_buffer.upload_data(&ctx.device.raw_device(), &[ubo]);
                    },

                    _ => {}
                }
            },
            winit::event::WindowEvent::CloseRequested => ev_window.exit(),
            winit::event::WindowEvent::RedrawRequested => {

                angle += rotation_speed * global_time.elapsed().as_millis() as f32;
                global_time = Instant::now();

                graph.execute(&ctx);
                if time.elapsed().as_secs() >= 1 {
                    info!("FPS: {}", count_frame);
                    time = Instant::now();
                    count_frame = 0;

                } else {
                    count_frame += 1;
                }

                ubo.model = rotation_matrix(angle, [0.0 , 1.0, 0.0]);
                uniform_buffer.upload_data(&ctx.device.raw_device(), &[ubo]);
            },
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

                uniform_buffer.upload_data(&ctx.device.logical_device.raw, &[ubo]);

            },
            _ => {}
        },
        winit::event::Event::AboutToWait => {
            ctx.window.raw.request_redraw();
        }
        _ => {}
    }
    });

    Ok(())
}