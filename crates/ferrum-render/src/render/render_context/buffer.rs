use crate::RenderContext;
use crate::GpuBuffer;

use vk_mem::*;
use ash::vk;

impl RenderContext {

    #[cfg(feature = "vma")]
    pub fn create_static_buffer<T: Copy>(&self, _type: vk::BufferUsageFlags, data: &[T]) -> crate::VulkanResult<GpuBuffer> {

        let buffer_size = data.len() * size_of::<T>();

        let buffer_info = vk::BufferCreateInfo::default()
            .size(buffer_size as u64)
            .usage(_type | vk::BufferUsageFlags::TRANSFER_DST)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let allocation_info = AllocationCreateInfo {
            usage: MemoryUsage::GpuOnly,
            ..Default::default()
        };

        let (buffer, allocation) = unsafe {
            self.device.allocator.allocator().create_buffer(&buffer_info, &allocation_info)
            .unwrap()
        };

        // TODO: Cache Staging Buffer
        let staging_info = vk::BufferCreateInfo::default()
            .size(buffer_size as u64)
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let staging_alloc_info = AllocationCreateInfo {
            usage: MemoryUsage::CpuOnly,
            flags: AllocationCreateFlags::MAPPED,
            ..Default::default()
        };

        let (staging_buffer, mut staging_allocation) = unsafe {
            self.device.allocator.allocator().create_buffer(&staging_info, &staging_alloc_info)
                .unwrap()
        };

        let staging_allocation_info = self.device.allocator.allocator().get_allocation_info(&staging_allocation);

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                staging_allocation_info.mapped_data as *mut u8,
                buffer_size
            );
        }

        self.submit_commands(|cmd| {
            unsafe {

                let region = vk::BufferCopy::default()
                    .dst_offset(0)
                    .src_offset(0)
                    .size(buffer_size as u64);

                self.device.raw_device().cmd_copy_buffer(*cmd, staging_buffer, buffer, &[region]);
            };
        });

        unsafe {
            self.device.allocator.allocator().destroy_buffer(staging_buffer, &mut staging_allocation);
        }

        Ok(GpuBuffer {
            raw: buffer,
            device: self.device.clone(),
            allocation,
            size: buffer_size as u64,
        })
    }

    #[cfg(feature = "vma")]
    pub fn create_dynamic_buffer<T: Copy>(&self, _type: vk::BufferUsageFlags, data: &[T]) -> crate::VulkanResult<GpuBuffer> {

        let buffer_size = data.len() * size_of::<T>();

        let buffer_info = vk::BufferCreateInfo::default()
            .size(buffer_size as u64)
            .usage(_type)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let allocation_info = AllocationCreateInfo {
            usage: MemoryUsage::CpuToGpu,
            flags: AllocationCreateFlags::MAPPED,
            ..Default::default()
        };

        let (buffer, allocation) = unsafe {
            self.device.allocator.allocator().create_buffer(&buffer_info, &allocation_info)
                .map_err(|x| crate::VulkanError::Unknown)?
        };

        let allocation_info = self.device.allocator.allocator().get_allocation_info(&allocation);
        let mapped_ptr = allocation_info.mapped_data;

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                mapped_ptr as *mut u8,
                buffer_size
            );
        }

        self.device.allocator.allocator().flush_allocation(&allocation, 0, buffer_size as u64)
            .map_err(|e| crate::VulkanError::Unknown)?;

        Ok(GpuBuffer {
            raw: buffer,
            device: self.device.clone(),
            allocation,
            size: buffer_size as u64
        })
    }
}