use std::alloc;
use std::path::Path;
use ash::vk::{BufferUsageFlags, Extent3D};
use ash::vk;
use image::GenericImageView;
use vk_mem::{Alloc, AllocationCreateInfo};

use crate::{Image, RenderContext, VulkanError, VulkanResult};

impl RenderContext {

    #[cfg(feature = "vma")]
    pub fn create_texture(&self, raw_image: &RawImage) -> VulkanResult<Texture> {

        let image = Image::new(self.device.raw_device(), raw_image.extent, raw_image.format);

        let alloc_info = AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::GpuOnly,
            ..Default::default()
        };

        // TODO: Required free memory
        let image_alloc = unsafe {
            self.device.allocator
                .allocator()
                .allocate_memory_for_image(image.raw, &alloc_info).unwrap()
        };

        unsafe {
            self.device.allocator.allocator()
                .bind_image_memory(&image_alloc, image.raw).unwrap()
        };

        let buffer_size = (raw_image.raw_bytes.len() * size_of::<u8>()) as u64;

        let staging_info = vk::BufferCreateInfo::default()
            .size(buffer_size)
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let staging_alloc_info = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::CpuOnly,
            flags: vk_mem::AllocationCreateFlags::MAPPED,
            ..Default::default()
        };

        let (staging_buffer, mut staging_allocation) = unsafe {
            self.device.allocator.allocator().create_buffer(&staging_info, &staging_alloc_info).unwrap()
        };

        let allocation_info = self.device.allocator.allocator().get_allocation_info(&staging_allocation);

        unsafe {
            std::ptr::copy_nonoverlapping(
                raw_image.raw_bytes.as_ptr(),
                allocation_info.mapped_data as *mut u8,
                raw_image.raw_bytes.len()
            );
        }

        self.transition_image_layout(
            image.raw,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        )?;

        self.copy_buffer_to_image(staging_buffer, image.raw, raw_image.extent)?;

        self.transition_image_layout(
            image.raw,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )?;

        unsafe {
            self.device.allocator.allocator().destroy_buffer(staging_buffer, &mut staging_allocation);
        }

        let image_view = unsafe {
            self.device.raw_device().create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(image.raw)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(raw_image.format)
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


        Ok(Texture {
            image: image,
            image_view,
        })
    }


    fn copy_buffer_to_image(
        &self,
        buffer: vk::Buffer,
        image: vk::Image,
        extent: vk::Extent3D,
    ) -> VulkanResult<()> {

        self.submit_commands(|cmd| {

            let region = vk::BufferImageCopy::default()
                .buffer_offset(0)
                .buffer_row_length(0)
                .buffer_image_height(0)
                .image_subresource(vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
                .image_extent(extent);

            unsafe {
                self.device.raw_device().cmd_copy_buffer_to_image(
                    *cmd,
                    buffer,
                    image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[region],
                );
            }
        });


        Ok(())
    }

    pub fn transition_image_layout(
        &self,
        image: vk::Image,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
    ) -> VulkanResult<()> {

        self.submit_commands(|cmd| {

            let mut barrier = vk::ImageMemoryBarrier::default()
                .old_layout(old_layout)
                .new_layout(new_layout)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .image(image)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            let (source_stage, destination_stage) = match (old_layout, new_layout) {
                (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => {
                    barrier.src_access_mask = vk::AccessFlags::empty();
                    barrier.dst_access_mask = vk::AccessFlags::TRANSFER_WRITE;
                    (vk::PipelineStageFlags::TOP_OF_PIPE, vk::PipelineStageFlags::TRANSFER)
                }
                (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => {
                    barrier.src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
                    barrier.dst_access_mask = vk::AccessFlags::SHADER_READ;
                    (vk::PipelineStageFlags::TRANSFER, vk::PipelineStageFlags::FRAGMENT_SHADER)
                }
                _ => { panic!("AAA") }
            };

            unsafe {
                self.device.raw_device().cmd_pipeline_barrier(
                    *cmd,
                    source_stage,
                    destination_stage,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[barrier],
                );
            }
        });

        Ok(())
    }

    pub fn load_image_blocking<T: AsRef<Path>>(&self, path: T) -> VulkanResult<RawImage> {

        let image = image::open(path).map_err(|_| VulkanError::Unknown)?;

        let (width, height) = image.dimensions();

        let image = image
            .to_rgba8()
            .into_raw();

        Ok(RawImage {
            extent: Extent3D {
                width: width,
                height: height,
                depth: 1
            },
            raw_bytes: image,
            format: ash::vk::Format::R8G8B8A8_SRGB
        })
    }
}


pub struct RawImage {
    pub extent: ash::vk::Extent3D,
    pub format: vk::Format,
    pub raw_bytes: Vec<u8>
}


pub struct Texture {
    pub image: Image,
    pub image_view: vk::ImageView,
}