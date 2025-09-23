use std::sync::Arc;
use ash::vk::{self, PhysicalDeviceMemoryProperties};
use log::warn;

use crate::render_context::GraphicsDevice;
///
/// Wraper around [`ash::vk::Buffer`] for simple use
/// # Panic
/// if size == 0
///
/// # Example:
///
/// ```
/// fn main() {
///     let uniform_buffer = GPUBuffer::new(
///         &device.raw,
///         &memory_prop,
///         buffer_size,
///         vk::BufferUsageFlags::UNIFORM_BUFFER,
///         vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
///     ).unwrap();
///
///     uniform_buffer.upload_data(&ctx.graphics_device.device.raw, &[data]);
/// }
/// ```
///
#[derive(Clone)]
pub struct GpuBuffer {
    pub raw: vk::Buffer,
    pub device: Arc<GraphicsDevice>,
    #[cfg(feature = "vma")]
    pub allocation: vk_mem::Allocation,
    pub size: u64,
}

impl GpuBuffer {

    #[cfg(feature = "vma")]
    pub fn update_buffer<T: Copy>(&self, data: &[T]) -> crate::VulkanResult<()> {
        use vk_mem::Alloc;

        let new_size = (data.len() * size_of::<T>()) as u64;

        if new_size > self.size {
            return Err(crate::VulkanError::Unknown);
        }

        let allocation_info = self.device.allocator.allocator().get_allocation_info(&self.allocation);
        let mapped_ptr = allocation_info.mapped_data;

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                mapped_ptr as *mut u8,
                new_size as usize
            );
        }

        self.device.allocator.allocator().flush_allocation(&self.allocation, 0, new_size as u64).unwrap();

        Ok(())
    }

}

impl Drop for GpuBuffer {
    fn drop(&mut self) {
        unsafe { self.device.allocator.destroy_buffer(self.raw, &mut self.allocation) };
    }
}