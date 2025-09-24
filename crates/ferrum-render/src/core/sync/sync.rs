use ash::vk::{
    Fence,
    FenceCreateFlags,
    Semaphore,
    SemaphoreCreateFlags
};

use ash::vk;
use std::sync::Arc;

use crate::{VulkanError, VulkanResult};

pub struct FrameSync {
    pub device: Arc<ash::Device>,
    pub image_available: Semaphore,
    pub render_finished: Semaphore,
    pub fence: Fence,
}

impl FrameSync {
    pub fn new(device: Arc<ash::Device>) -> VulkanResult<Self> {

        let image_available = {

            let semaphore_info = vk::SemaphoreCreateInfo::default()
                .flags(SemaphoreCreateFlags::default());

            unsafe {
                device.create_semaphore(&semaphore_info, None).map_err(|e| {
                    VulkanError::Unknown
                })?
            }
        };

        let render_finished = {

            let semaphore_info = vk::SemaphoreCreateInfo::default()
                .flags(SemaphoreCreateFlags::default());

            unsafe {
                device.create_semaphore(&semaphore_info, None).map_err(|e| {
                    VulkanError::Unknown
                })?
            }
        };

        let fence_info = vk::FenceCreateInfo::default()
            .flags(FenceCreateFlags::SIGNALED);

        let fence = unsafe {
            device.create_fence(&fence_info, None).map_err(|e| {
                VulkanError::Unknown
            })?
        };

        Ok(Self { image_available, render_finished, fence, device: device })
    }
}


impl Drop for FrameSync {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_fence(self.fence, None);
            self.device.destroy_semaphore(self.image_available, None);
            self.device.destroy_semaphore(self.render_finished, None);
        }
    }
}



